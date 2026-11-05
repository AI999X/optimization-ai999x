//! A deterministic measure of a solve's work: the arithmetic operations and comparisons of the procedures' runtime
//! (py.rs), counted. The figure engine reads it to size its sweeps by what solves actually cost (WebAssembly has no
//! clock here) and may set a limit for one solve: past it every counted operation fails with an error that no
//! `except` clause catches (PyErr::Unsupported), and the solve is marked as stopped, so its answer is discarded
//! whatever the procedure did with the error. Without a limit (every ordinary solve) counting never fails.

use crate::py::PyErr;
use std::cell::Cell;

thread_local! {
    static WORK: Cell<u64> = const { Cell::new(0) };
    static LIMIT: Cell<u64> = const { Cell::new(u64::MAX) };
    static STOPPED: Cell<bool> = const { Cell::new(false) };
}

/// One unit of work; an error once the limit set by `limit_after` is passed.
#[inline]
pub fn tick() -> Result<(), PyErr> {
    let w = WORK.with(|c| {
        let w = c.get().wrapping_add(1);
        c.set(w);
        w
    });
    if w > LIMIT.with(|l| l.get()) {
        STOPPED.with(|s| s.set(true));
        return Err(PyErr::Unsupported("work limit".into()));
    }
    Ok(())
}

/// The work counted so far (a running total: compare two readings).
pub fn work() -> u64 {
    WORK.with(|c| c.get())
}

/// Allow `n` more units of work from now on.
pub fn limit_after(n: u64) {
    let w = work();
    LIMIT.with(|l| l.set(w.saturating_add(n)));
    STOPPED.with(|s| s.set(false));
}

/// Remove the limit; true when it was reached since `limit_after`.
pub fn unlimit() -> bool {
    LIMIT.with(|l| l.set(u64::MAX));
    STOPPED.with(|s| s.replace(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_limit_stops_and_is_reported_once() {
        assert!(tick().is_ok());
        let w0 = work();
        limit_after(3);
        assert!(tick().is_ok() && tick().is_ok() && tick().is_ok());
        assert!(matches!(tick(), Err(PyErr::Unsupported(_))));
        assert!(tick().is_err());                                 // stays stopped until the limit is removed
        assert!(unlimit());
        assert!(!unlimit());
        assert!(tick().is_ok());
        assert_eq!(work(), w0 + 6);
    }
}
