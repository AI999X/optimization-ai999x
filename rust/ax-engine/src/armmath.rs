//! exp, log, pow, exp2 and log2 of ARM optimized-routines (Copyright (c) 2018-2026, Arm Limited; MIT OR Apache-2.0 WITH
//! LLVM-exception), the algorithms the GNU C library uses since 2.28: its FMA build (x86-64 with FMA, which the
//! Linux reference machines run), so x ** y, math.exp and math.log give the reference Python's bits: 0 differences
//! in 2.1 million random cases against glibc 2.43 (the libm crate differed in 7.7% of exp, 2.2% of log, 9.6% of pow).
//! GCC fuses a*b + c when every use of the product is in its own basic block; `fm` marks each such site. exp2 and log2
//! (added 2026-10-30) equal glibc in 100,000 random cases each (the libm crate: 91 and 76 differences).

use crate::armmath_data::*;

#[inline(always)]
fn d(b: u64) -> f64 {
    f64::from_bits(b)
}

/// a * b + c with one rounding, where GCC's contraction fuses it in the C library's FMA build (Rust's mul_add is a
/// true fused multiply-add on every platform, the browser included)
#[inline(always)]
fn fm(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}

#[inline(always)]
fn top12(x: f64) -> u32 {
    (x.to_bits() >> 52) as u32
}

const N: u64 = 128;
const EXP_TABLE_BITS: u64 = 7;
const SIGN_BIAS: u64 = 0x800 << EXP_TABLE_BITS;

fn exp_specialcase(tmp: f64, sbits: u64, ki: u64) -> f64 {
    if ki & 0x80000000 == 0 {
        // k > 0: the exponent of scale might have overflowed by <= 460
        let scale = d(sbits.wrapping_sub(1009u64 << 52));
        let y = f64::from_bits(0x7f00000000000000) * fm(scale, tmp, scale);   // 0x1p1009
        return y;
    }
    // k < 0: care in the subnormal range. scale * tmp is also used inside the branch below, and GCC fuses a
    // multiplication only when all its uses are in its own basic block: here it is rounded on its own
    let scale = d(sbits.wrapping_add(1022u64 << 52));
    let m = scale * tmp;
    let mut y = scale + m;
    if y < 1.0 {
        let lo = scale - y + m;
        let hi = 1.0 + y;
        let lo = 1.0 - hi + y + lo;
        y = (hi + lo) - 1.0;
        if y == 0.0 {
            y = 0.0;
        }
    }
    f64::from_bits(0x0010000000000000) * y                     // 0x1p-1022
}

/// The C library's exp.
pub fn exp(x: f64) -> f64 {
    let (inv_ln2_n, neg_ln2_hi_n, neg_ln2_lo_n, shift) = (d(EXP_CONST[0]), d(EXP_CONST[1]), d(EXP_CONST[2]), d(EXP_CONST[3]));
    let (c2, c3, c4, c5) = (d(EXP_POLY[0]), d(EXP_POLY[1]), d(EXP_POLY[2]), d(EXP_POLY[3]));
    let mut abstop = top12(x) & 0x7ff;
    if abstop.wrapping_sub(top12(f64::from_bits(0x3c90000000000000))) >= top12(512.0) - top12(f64::from_bits(0x3c90000000000000)) {
        if abstop.wrapping_sub(top12(f64::from_bits(0x3c90000000000000))) >= 0x80000000 {
            return 1.0 + x;                                    // tiny x (0 is common)
        }
        if abstop >= top12(1024.0) {
            if x.to_bits() == f64::NEG_INFINITY.to_bits() {
                return 0.0;
            }
            if abstop >= top12(f64::INFINITY) {
                return 1.0 + x;
            }
            return if x.to_bits() >> 63 != 0 { 0.0 } else { f64::INFINITY };
        }
        abstop = 0;                                            // large x: special-cased below
    }
    let kd = fm(inv_ln2_n, x, shift);
    let ki = kd.to_bits();
    let kd = kd - shift;
    let r = fm(kd, neg_ln2_lo_n, fm(kd, neg_ln2_hi_n, x));
    let idx = (2 * (ki % N)) as usize;
    let top = ki << (52 - EXP_TABLE_BITS);
    let tail = d(EXP_TAB[idx]);
    let sbits = EXP_TAB[idx + 1].wrapping_add(top);
    let r2 = r * r;
    let tmp = fm(r2 * r2, fm(r, c5, c4), fm(r2, fm(r, c3, c2), tail + r));
    if abstop == 0 {
        return exp_specialcase(tmp, sbits, ki);
    }
    let scale = d(sbits);
    fm(scale, tmp, scale)
}

/// The C library's log.
pub fn log(x: f64) -> f64 {
    let (ln2hi, ln2lo) = (d(LOG_CONST[0]), d(LOG_CONST[1]));
    let a = |i: usize| d(LOG_POLY[i]);
    let b = |i: usize| d(LOG_POLY1[i]);
    let mut ix = x.to_bits();
    let top = (ix >> 48) as u32;
    let lo_b = (1.0f64 - f64::from_bits(0x3fb0000000000000)).to_bits();             // 1.0 - 0x1p-4
    let hi_b = (1.0f64 + f64::from_bits(0x3fb0900000000000)).to_bits();             // 1.0 + 0x1.09p-4
    if ix.wrapping_sub(lo_b) < hi_b - lo_b {
        if ix == 1.0f64.to_bits() {
            return 0.0;
        }
        let r = x - 1.0;
        let r2 = r * r;
        let r3 = r * r2;
        let inner = fm(r3, b(10), fm(r2, b(9), fm(r, b(8), b(7))));
        let mid = fm(r3, inner, fm(r2, b(6), fm(r, b(5), b(4))));
        let outer = fm(r3, mid, fm(r2, b(3), fm(r, b(2), b(1))));
        let w = r * f64::from_bits(0x41a0000000000000);                             // r * 0x1p27
        let rhi = r + w - w;
        let rlo = r - rhi;
        let w = rhi * rhi * b(0);
        let hi = r + w;
        let mut lo = r - hi + w;
        lo = fm(b(0) * rlo, rhi + r, lo);
        let y = fm(r3, outer, lo);
        return y + hi;
    }
    if top.wrapping_sub(0x0010) >= 0x7ff0 - 0x0010 {
        if ix.wrapping_mul(2) == 0 {
            return f64::NEG_INFINITY;
        }
        if ix == f64::INFINITY.to_bits() {
            return x;
        }
        if (top & 0x8000) != 0 || (top & 0x7ff0) == 0x7ff0 {
            return f64::NAN;
        }
        ix = (x * f64::from_bits(0x4330000000000000)).to_bits();                    // x * 0x1p52
        ix = ix.wrapping_sub(52u64 << 52);
    }
    const OFF: u64 = 0x3fe6000000000000;
    let tmp = ix.wrapping_sub(OFF);
    let i = ((tmp >> (52 - 7)) % N) as usize;
    let k = (tmp as i64) >> 52;
    let iz = ix.wrapping_sub(tmp & (0xfffu64 << 52));
    let (invc, logc) = (d(LOG_TAB[2 * i]), d(LOG_TAB[2 * i + 1]));
    let z = d(iz);
    let r = z.mul_add(invc, -1.0);
    let kd = k as f64;
    let w = fm(kd, ln2hi, logc);
    let hi = w + r;
    let lo = fm(kd, ln2lo, w - hi + r);
    let r2 = r * r;
    let q = fm(r2, fm(r, a(4), a(3)), fm(r, a(2), a(1)));
    let y = fm(r * r2, q, fm(r2, a(0), lo));
    y + hi
}

/// log(x) as y + tail for pow (about 15 more bits); ix normalized in the subnormal range.
fn pow_log(ix: u64) -> (f64, f64) {
    let (ln2hi, ln2lo) = (d(POW_LOG_CONST[0]), d(POW_LOG_CONST[1]));
    let a = |i: usize| d(POW_LOG_POLY[i]);
    const OFF: u64 = 0x3fe6955500000000;
    let tmp = ix.wrapping_sub(OFF);
    let i = ((tmp >> (52 - 7)) % N) as usize;
    let k = (tmp as i64) >> 52;
    let iz = ix.wrapping_sub(tmp & (0xfffu64 << 52));
    let z = d(iz);
    let kd = k as f64;
    let (invc, logc, logctail) = (d(POW_LOG_TAB[4 * i]), d(POW_LOG_TAB[4 * i + 2]), d(POW_LOG_TAB[4 * i + 3]));
    let r = z.mul_add(invc, -1.0);
    let t1 = fm(kd, ln2hi, logc);
    let t2 = t1 + r;
    let lo1 = fm(kd, ln2lo, logctail);
    let lo2 = t1 - t2 + r;
    let ar = a(0) * r;
    let ar2 = r * ar;
    let ar3 = r * ar2;
    let hi = t2 + ar2;
    let lo3 = ar.mul_add(r, -ar2);
    let lo4 = t2 - hi + ar2;
    let inner = fm(ar2, fm(r, a(6), a(5)), fm(r, a(4), a(3)));
    let poly = fm(ar2, inner, fm(r, a(2), a(1)));
    let lo = fm(ar3, poly, lo1 + lo2 + lo3 + lo4);
    let y = hi + lo;
    (y, hi - y + lo)
}

fn pow_specialcase(tmp: f64, sbits: u64, ki: u64) -> f64 {
    if ki & 0x80000000 == 0 {
        let scale = d(sbits.wrapping_sub(1009u64 << 52));
        let y = fm(scale, tmp, scale);
        return y * f64::from_bits(0x7f00000000000000);                              // 0x1p1009
    }
    // as in exp_specialcase: scale * tmp has a use in the branch, so it is not fused
    let sbits = sbits.wrapping_add(1022u64 << 52);
    let scale = d(sbits);
    let m = scale * tmp;
    let mut y = scale + m;
    if y.abs() < 1.0 {
        let one = if y < 0.0 { -1.0 } else { 1.0 };
        let lo = scale - y + m;
        let hi = one + y;
        let lo = one - hi + y + lo;
        y = (hi + lo) - one;
        if y == 0.0 {
            y = d(sbits & 0x8000000000000000);
        }
    }
    f64::from_bits(0x0010000000000000) * y
}

fn pow_exp(x: f64, xtail: f64, sign_bias: u64) -> f64 {
    let (inv_ln2_n, neg_ln2_hi_n, neg_ln2_lo_n, shift) = (d(EXP_CONST[0]), d(EXP_CONST[1]), d(EXP_CONST[2]), d(EXP_CONST[3]));
    let (c2, c3, c4, c5) = (d(EXP_POLY[0]), d(EXP_POLY[1]), d(EXP_POLY[2]), d(EXP_POLY[3]));
    let mut abstop = top12(x) & 0x7ff;
    let t54 = top12(f64::from_bits(0x3c90000000000000));
    if abstop.wrapping_sub(t54) >= top12(512.0) - t54 {
        if abstop.wrapping_sub(t54) >= 0x80000000 {
            let one = 1.0 + x;
            return if sign_bias != 0 { -one } else { one };
        }
        if abstop >= top12(1024.0) {
            let s = if sign_bias != 0 { -1.0 } else { 1.0 };
            return if x.to_bits() >> 63 != 0 { s * 0.0 } else { s * f64::INFINITY };
        }
        abstop = 0;
    }
    let kd = fm(inv_ln2_n, x, shift);
    let ki = kd.to_bits();
    let kd = kd - shift;
    let r = fm(kd, neg_ln2_lo_n, fm(kd, neg_ln2_hi_n, x)) + xtail;
    let idx = (2 * (ki % N)) as usize;
    let top = ki.wrapping_add(sign_bias) << (52 - EXP_TABLE_BITS);
    let tail = d(EXP_TAB[idx]);
    let sbits = EXP_TAB[idx + 1].wrapping_add(top);
    let r2 = r * r;
    let tmp = fm(r2 * r2, fm(r, c5, c4), fm(r2, fm(r, c3, c2), tail + r));
    if abstop == 0 {
        return pow_specialcase(tmp, sbits, ki);
    }
    let scale = d(sbits);
    fm(scale, tmp, scale)
}

/// 0 if not an integer, 1 if odd, 2 if even (iy: a non-zero finite value's bits)
fn checkint(iy: u64) -> u32 {
    let e = (iy >> 52 & 0x7ff) as i64;
    if e < 0x3ff {
        return 0;
    }
    if e > 0x3ff + 52 {
        return 2;
    }
    if iy & ((1u64 << (0x3ff + 52 - e)) - 1) != 0 {
        return 0;
    }
    if iy & (1u64 << (0x3ff + 52 - e)) != 0 {
        return 1;
    }
    2
}

fn zeroinfnan(i: u64) -> bool {
    i.wrapping_mul(2).wrapping_sub(1) >= f64::INFINITY.to_bits().wrapping_mul(2).wrapping_sub(1)
}

/// The C library's pow.
pub fn pow(x: f64, y: f64) -> f64 {
    let mut sign_bias = 0u64;
    let mut ix = x.to_bits();
    let iy = y.to_bits();
    let mut topx = top12(x);
    let topy = top12(y);
    if topx.wrapping_sub(0x001) >= 0x7ff - 0x001 || (topy & 0x7ff).wrapping_sub(0x3be) >= 0x43e - 0x3be {
        if zeroinfnan(iy) {
            if iy.wrapping_mul(2) == 0 {
                return 1.0;
            }
            if ix == 1.0f64.to_bits() {
                return 1.0;
            }
            if ix.wrapping_mul(2) > f64::INFINITY.to_bits().wrapping_mul(2)
                || iy.wrapping_mul(2) > f64::INFINITY.to_bits().wrapping_mul(2) {
                return x + y;
            }
            if ix.wrapping_mul(2) == 1.0f64.to_bits().wrapping_mul(2) {
                return 1.0;
            }
            if (ix.wrapping_mul(2) < 1.0f64.to_bits().wrapping_mul(2)) == (iy >> 63 == 0) {
                return 0.0;
            }
            return y * y;
        }
        if zeroinfnan(ix) {
            let mut x2 = x * x;
            if ix >> 63 != 0 && checkint(iy) == 1 {
                x2 = -x2;
            }
            return if iy >> 63 != 0 { 1.0 / x2 } else { x2 };
        }
        if ix >> 63 != 0 {
            let yint = checkint(iy);
            if yint == 0 {
                return f64::NAN;
            }
            if yint == 1 {
                sign_bias = SIGN_BIAS;
            }
            ix &= 0x7fffffffffffffff;
            topx &= 0x7ff;
        }
        if (topy & 0x7ff).wrapping_sub(0x3be) >= 0x43e - 0x3be {
            if ix == 1.0f64.to_bits() {
                return 1.0;
            }
            if (topy & 0x7ff) < 0x3be {
                return if ix > 1.0f64.to_bits() { 1.0 + y } else { 1.0 - y };
            }
            return if (ix > 1.0f64.to_bits()) == (topy < 0x800) { f64::INFINITY } else { 0.0 };
        }
        if topx == 0 {
            ix = (x * f64::from_bits(0x4330000000000000)).to_bits();                // subnormal x: normalize
            ix &= 0x7fffffffffffffff;
            ix = ix.wrapping_sub(52u64 << 52);
        }
    }
    let (hi, lo) = pow_log(ix);
    let ehi = y * hi;
    let elo = fm(y, lo, y.mul_add(hi, -ehi));
    pow_exp(ehi, elo, sign_bias)
}

/// a * b + c rounded twice: glibc's exp2 is its plain (not FMA) build
#[inline(always)]
fn plain(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}

fn exp2_specialcase(tmp: f64, sbits: u64, ki: u64) -> f64 {
    if ki & 0x80000000 == 0 {
        let scale = d(sbits.wrapping_sub(1u64 << 52));
        return 2.0 * plain(scale, tmp, scale);
    }
    let scale = d(sbits.wrapping_add(1022u64 << 52));
    let m = scale * tmp;
    let mut y = scale + m;
    if y < 1.0 {
        let lo = scale - y + m;
        let hi = 1.0 + y;
        let lo = 1.0 - hi + y + lo;
        y = (hi + lo) - 1.0;
        if y == 0.0 {
            y = 0.0;
        }
    }
    f64::from_bits(0x0010000000000000) * y
}

/// The C library's exp2.
pub fn exp2(x: f64) -> f64 {
    let shift = d(EXP2_CONST[0]);
    let c = |i: usize| d(EXP2_POLY[i]);
    let mut abstop = top12(x) & 0x7ff;
    let t54 = top12(f64::from_bits(0x3c90000000000000));
    if abstop.wrapping_sub(t54) >= top12(512.0) - t54 {
        if abstop.wrapping_sub(t54) >= 0x80000000 {
            return 1.0 + x;
        }
        if abstop >= top12(1024.0) {
            if x.to_bits() == f64::NEG_INFINITY.to_bits() {
                return 0.0;
            }
            if abstop >= top12(f64::INFINITY) {
                return 1.0 + x;
            }
            if x.to_bits() >> 63 == 0 {
                return f64::INFINITY;
            } else if x.to_bits() >= (-1075.0f64).to_bits() {
                return 0.0;
            }
        }
        if x.to_bits().wrapping_mul(2) > 928.0f64.to_bits().wrapping_mul(2) {
            abstop = 0;
        }
    }
    let kd = x + shift;
    let ki = kd.to_bits();
    let kd = kd - shift;
    let r = x - kd;
    let idx = (2 * (ki % N)) as usize;
    let top = ki << (52 - EXP_TABLE_BITS);
    let tail = d(EXP_TAB[idx]);
    let sbits = EXP_TAB[idx + 1].wrapping_add(top);
    let r2 = r * r;
    let tmp = plain(r2 * r2, plain(r, c(4), c(3)), plain(r2, plain(r, c(2), c(1)), plain(r, c(0), tail)));
    if abstop == 0 {
        return exp2_specialcase(tmp, sbits, ki);
    }
    let scale = d(sbits);
    plain(scale, tmp, scale)
}

/// The C library's log2.
pub fn log2(x: f64) -> f64 {
    let (ih, il) = (d(LOG2_CONST[0]), d(LOG2_CONST[1]));
    let a = |i: usize| d(LOG2_POLY[i]);
    let b = |i: usize| d(LOG2_POLY1[i]);
    let mut ix = x.to_bits();
    let top = (ix >> 48) as u32;
    let lo_b = (1.0f64 - f64::from_bits(0x3fa5b51000000000)).to_bits();     // 1.0 - 0x1.5b51p-5
    let hi_b = (1.0f64 + f64::from_bits(0x3fa6ab2000000000)).to_bits();     // 1.0 + 0x1.6ab2p-5
    if ix.wrapping_sub(lo_b) < hi_b - lo_b {
        if ix == 1.0f64.to_bits() {
            return 0.0;
        }
        let r = x - 1.0;
        let hi = r * ih;
        let mut lo = fm(r, il, r.mul_add(ih, -hi));
        let r2 = r * r;
        let r4 = r2 * r2;
        let q = fm(r, b(1), b(0));
        let y = fm(r2, q, hi);
        lo += fm(r2, q, hi - y);
        let big = fm(r4, fm(r2, fm(r, b(9), b(8)), fm(r, b(7), b(6))),
                          fm(r2, fm(r, b(5), b(4)), fm(r, b(3), b(2))));
        lo = fm(r4, big, lo);
        return y + lo;
    }
    if top.wrapping_sub(0x0010) >= 0x7ff0 - 0x0010 {
        if ix.wrapping_mul(2) == 0 {
            return f64::NEG_INFINITY;
        }
        if ix == f64::INFINITY.to_bits() {
            return x;
        }
        if (top & 0x8000) != 0 || (top & 0x7ff0) == 0x7ff0 {
            return f64::NAN;
        }
        ix = (x * f64::from_bits(0x4330000000000000)).to_bits();
        ix = ix.wrapping_sub(52u64 << 52);
    }
    const OFF: u64 = 0x3fe6000000000000;
    let tmp = ix.wrapping_sub(OFF);
    let i = ((tmp >> (52 - 6)) % 64) as usize;
    let k = (tmp as i64) >> 52;
    let iz = ix.wrapping_sub(tmp & (0xfffu64 << 52));
    let (invc, logc) = (d(LOG2_TAB[2 * i]), d(LOG2_TAB[2 * i + 1]));
    let z = d(iz);
    let kd = k as f64;
    let r = z.mul_add(invc, -1.0);
    let t1 = r * ih;
    let t2 = fm(r, il, r.mul_add(ih, -t1));
    let t3 = kd + logc;
    let hi = t3 + t1;
    let lo = t3 - hi + t1 + t2;
    let r2 = r * r;
    let r4 = r2 * r2;
    let p = fm(r4, fm(r, a(5), a(4)), fm(r2, fm(r, a(3), a(2)), fm(r, a(1), a(0))));
    fm(r2, p, lo) + hi
}
