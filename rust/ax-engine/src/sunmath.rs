//! expm1, log1p, log10, sinh, cosh, tanh as the GNU C library computes them (its dbl-64 files from Sun's fdlibm:
//! "Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved. Developed at SunSoft, a Sun Microsystems,
//! Inc. business. Permission to use, copy, modify, and distribute this software is freely granted, provided that
//! this notice is preserved."), calling the exp and log of crate::armmath as glibc's call its own. expm1 is the
//! library's FMA build (GCC fuses a*b + c at each site marked mul_add): sinh, cosh, tanh, expm1 and log10 equal glibc
//! 2.43 in 100,000 random cases each; log1p differs in 27 (the libm crate: 36), cosh 0 (the libm crate: 9,506).

use crate::armmath;

fn hi(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

fn lo(x: f64) -> u32 {
    x.to_bits() as u32
}

fn with_hi(x: f64, h: u32) -> f64 {
    f64::from_bits(((h as u64) << 32) | (x.to_bits() & 0xffff_ffff))
}

pub fn expm1(mut x: f64) -> f64 {
    const HUGE: f64 = 1.0e300;
    const TINY: f64 = 1.0e-300;
    const O_THRESHOLD: f64 = 7.09782712893383973096e+02;
    const LN2_HI: f64 = 6.93147180369123816490e-01;
    const LN2_LO: f64 = 1.90821492927058770002e-10;
    const INVLN2: f64 = 1.44269504088896338700e+00;
    const Q: [f64; 6] = [1.0, -3.33333333333331316428e-02, 1.58730158725481460165e-03, -7.93650757867487942473e-05,
                         4.00821782732936239552e-06, -2.01099218183624371326e-07];
    let mut hx = hi(x);
    let xsb = hx & 0x8000_0000;
    hx &= 0x7fff_ffff;
    if hx >= 0x4043_687A {
        if hx >= 0x4086_2E42 {
            if hx >= 0x7ff0_0000 {
                if ((hx & 0xfffff) | lo(x)) != 0 {
                    return x + x;
                }
                return if xsb == 0 { x } else { -1.0 };
            }
            if x > O_THRESHOLD {
                return HUGE * HUGE;
            }
        }
        if xsb != 0 {
            return TINY - 1.0;
        }
    }
    let (k, c): (i32, f64);
    if hx > 0x3fd6_2e42 {
        let (h, l, kk);
        if hx < 0x3FF0_A2B2 {
            if xsb == 0 {
                h = x - LN2_HI;
                l = LN2_LO;
                kk = 1;
            } else {
                h = x + LN2_HI;
                l = -LN2_LO;
                kk = -1;
            }
        } else {
            kk = INVLN2.mul_add(x, if xsb == 0 { 0.5 } else { -0.5 }) as i32;
            let t = kk as f64;
            h = (-t).mul_add(LN2_HI, x);
            l = t * LN2_LO;
        }
        x = if hx < 0x3FF0_A2B2 { h - l } else { (-(k_t(kk))).mul_add(LN2_LO, h) };
        c = if hx < 0x3FF0_A2B2 { (h - x) - l } else { (-(k_t(kk))).mul_add(LN2_LO, h - x) };
        k = kk;
    } else if hx < 0x3c90_0000 {
        let t = HUGE + x;
        return x - (t - (HUGE + x));
    } else {
        k = 0;
        c = 0.0;
    }
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r1a = hxs.mul_add(Q[1], 1.0);
    let h2 = hxs * hxs;
    let r2 = hxs.mul_add(Q[3], Q[2]);
    let h4 = h2 * h2;
    let r3 = hxs.mul_add(Q[5], Q[4]);
    let r1 = h4.mul_add(r3, h2.mul_add(r2, r1a));
    let t = (-r1).mul_add(hfx, 3.0);
    let mut e = hxs * ((r1 - t) / (-x).mul_add(t, 6.0));
    if k == 0 {
        return x - x.mul_add(e, -hxs);
    }
    e = x.mul_add(e - c, -c);
    e -= hxs;
    if k == -1 {
        return 0.5f64.mul_add(x - e, -0.5);
    }
    if k == 1 {
        return if x < -0.25 { -2.0 * (e - (x + 0.5)) } else { 2.0f64.mul_add(x - e, 1.0) };
    }
    if k <= -2 || k > 56 {
        let y = 1.0 - (e - x);
        let y = with_hi(y, hi(y).wrapping_add((k << 20) as u32));
        return y - 1.0;
    }
    if k < 20 {
        let t = with_hi(1.0, 0x3ff0_0000 - (0x20_0000 >> k));
        let y = t - (e - x);
        with_hi(y, hi(y).wrapping_add((k << 20) as u32))
    } else {
        let t = with_hi(1.0, ((0x3ff - k) << 20) as u32);
        let y = x - (e + t) + 1.0;
        with_hi(y, hi(y).wrapping_add((k << 20) as u32))
    }
}

fn k_t(k: i32) -> f64 { k as f64 }

pub fn log1p(x: f64) -> f64 {
    const LN2_HI: f64 = 6.93147180369123816490e-01;
    const LN2_LO: f64 = 1.90821492927058770002e-10;
    const TWO54: f64 = 1.80143985094819840000e+16;
    const LP: [f64; 8] = [0.0, 6.666666666666735130e-01, 3.999999999940941908e-01, 2.857142874366239149e-01,
                          2.222219843214978396e-01, 1.818357216161805012e-01, 1.531383769920937332e-01,
                          1.479819860511658591e-01];
    let hx = hi(x) as i32;
    let ax = hx & 0x7fff_ffff;
    let mut k = 1i32;
    let (mut f, mut hu, mut c) = (0.0f64, 0i32, 0.0f64);
    if hx < 0x3FDA_827A {
        if ax >= 0x3ff0_0000 {
            return if x == -1.0 { f64::NEG_INFINITY } else { f64::NAN };
        }
        if ax < 0x3e20_0000 {
            if ax < 0x3c90_0000 {
                return x;
            }
            return x - x * x * 0.5;
        }
        if hx > 0 || hx <= 0xbfd2_bec3u32 as i32 {
            k = 0;
            f = x;
            hu = 1;
        }
    } else if hx >= 0x7ff0_0000 {
        return x + x;
    }
    let _ = TWO54;
    if k != 0 {
        let mut u;
        if hx < 0x4340_0000 {
            u = 1.0 + x;
            hu = hi(u) as i32;
            k = (hu >> 20) - 1023;
            c = if k > 0 { 1.0 - (u - x) } else { x - (u - 1.0) };
            c /= u;
        } else {
            u = x;
            hu = hi(u) as i32;
            k = (hu >> 20) - 1023;
            c = 0.0;
        }
        hu &= 0x000f_ffff;
        if hu < 0x6a09e {
            u = with_hi(u, (hu | 0x3ff0_0000) as u32);
        } else {
            k += 1;
            u = with_hi(u, (hu | 0x3fe0_0000) as u32);
            hu = (0x0010_0000 - hu) >> 2;
        }
        f = u - 1.0;
    }
    let hfsq = 0.5 * f * f;
    if hu == 0 {
        if f == 0.0 {
            if k == 0 {
                return 0.0;
            }
            c += k as f64 * LN2_LO;
            return k as f64 * LN2_HI + c;
        }
        let r = hfsq * (1.0 - 0.66666666666666666 * f);
        if k == 0 {
            return f - r;
        }
        return k as f64 * LN2_HI - ((r - (k as f64 * LN2_LO + c)) - f);
    }
    let s = f / (2.0 + f);
    let z = s * s;
    let r1 = z * LP[1];
    let z2 = z * z;
    let r2 = LP[2] + z * LP[3];
    let z4 = z2 * z2;
    let r3 = LP[4] + z * LP[5];
    let z6 = z4 * z2;
    let r4 = LP[6] + z * LP[7];
    let r = r1 + z2 * r2 + z4 * r3 + z6 * r4;
    if k == 0 {
        return f - (hfsq - s * (hfsq + r));
    }
    k as f64 * LN2_HI - ((hfsq - (s * (hfsq + r) + (k as f64 * LN2_LO + c))) - f)
}

pub fn log10(mut x: f64) -> f64 {
    const TWO54: f64 = 1.80143985094819840000e+16;
    const IVLN10: f64 = 4.34294481903251816668e-01;
    const LOG10_2HI: f64 = 3.01029995663611771306e-01;
    const LOG10_2LO: f64 = 3.69423907715893078616e-13;
    let mut hx = x.to_bits() as i64;
    let mut k: i32 = 0;
    if hx < 0x0010_0000_0000_0000 {
        if (hx & 0x7fff_ffff_ffff_ffff) == 0 {
            return f64::NEG_INFINITY;
        }
        if hx < 0 {
            return f64::NAN;
        }
        k -= 54;
        x *= TWO54;
        hx = x.to_bits() as i64;
    }
    if hx >= 0x7ff0_0000_0000_0000 {
        return x + x;
    }
    k += ((hx >> 52) - 1023) as i32;
    let i = ((k as u64 & 0x8000_0000_0000_0000) >> 63) as i64;
    let hx = (hx & 0x000f_ffff_ffff_ffff) | ((0x3ff - i) << 52);
    let y = (k as i64 + i) as f64;
    let x = f64::from_bits(hx as u64);
    let z = y * LOG10_2LO + IVLN10 * armmath::log(x);
    z + y * LOG10_2HI
}

pub fn sinh(x: f64) -> f64 {
    const SHUGE: f64 = 1.0e307;
    let jx = hi(x) as i32;
    let ix = jx & 0x7fff_ffff;
    if ix >= 0x7ff0_0000 {
        return x + x;
    }
    let h = if jx < 0 { -0.5 } else { 0.5 };
    if ix < 0x4036_0000 {
        if ix < 0x3e30_0000 && SHUGE + x > 1.0 {
            return x;
        }
        let t = expm1(x.abs());
        if ix < 0x3ff0_0000 {
            return h * (2.0 * t - t * t / (t + 1.0));
        }
        return h * (t + t / (t + 1.0));
    }
    if ix < 0x4086_2e42 {
        return h * armmath::exp(x.abs());
    }
    let lx = lo(x);
    if ix < 0x4086_33ce || (ix == 0x4086_33ce && lx <= 0x8fb9_f87d) {
        let w = armmath::exp(0.5 * x.abs());
        let t = h * w;
        return t * w;
    }
    x * SHUGE
}

pub fn cosh(x: f64) -> f64 {
    let ix = (hi(x) & 0x7fff_ffff) as i32;
    if ix < 0x4036_0000 {
        if ix < 0x3fd6_2e43 {
            if ix < 0x3c80_0000 {
                return 1.0;
            }
            let t = expm1(x.abs());
            let w = 1.0 + t;
            return 1.0 + (t * t) / (w + w);
        }
        let t = armmath::exp(x.abs());
        return 0.5 * t + 0.5 / t;
    }
    if ix < 0x4086_2e42 {
        return 0.5 * armmath::exp(x.abs());
    }
    let fix = x.to_bits() & 0x7fff_ffff_ffff_ffff;
    if fix <= 0x4086_33ce_8fb9_f87d {
        let w = armmath::exp(0.5 * x.abs());
        let t = 0.5 * w;
        return t * w;
    }
    if ix >= 0x7ff0_0000 {
        return x * x;
    }
    f64::INFINITY
}

pub fn tanh(x: f64) -> f64 {
    let jx = hi(x) as i32;
    let lx = lo(x) as i32;
    let ix = jx & 0x7fff_ffff;
    if ix >= 0x7ff0_0000 {
        return if jx >= 0 { 1.0 / x + 1.0 } else { 1.0 / x - 1.0 };
    }
    let z;
    if ix < 0x4036_0000 {
        if (ix | lx) == 0 {
            return x;
        }
        if ix < 0x3c80_0000 {
            return x * (1.0 + x);
        }
        if ix >= 0x3ff0_0000 {
            let t = expm1(2.0 * x.abs());
            z = 1.0 - 2.0 / (t + 2.0);
        } else {
            let t = expm1(-2.0 * x.abs());
            z = -t / (t + 2.0);
        }
    } else {
        z = 1.0 - 1.0e-300;
    }
    if jx >= 0 { z } else { -z }
}
