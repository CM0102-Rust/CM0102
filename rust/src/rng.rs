//! The game's random number generator: `aleatorio(n)` { 0x90cfc0 }, cdecl, called from thousands
//! of places (matches, regens, transfers, injuries...).
//!
//! How it works (original listing):
//! - there is a fixed table of numbers in the exe (0xaa2454..=0xad4130, dwords); `[0xde1f60]` points to the
//!   current entry and `[0xde1f5c]` holds a 16-bit offset;
//! - each call advances the pointer one entry; past the end (`[0xde2798]`), it goes back to the start and
//!   draws a new offset with the CRT's `rand()` (0x946a04) & 0xffff;
//! - |n| <= 0xffff: (offset + entry) % n (signed division; negative n flips the sign);
//! - |n| > 0xffff: a floating-point part ((s % 0xffff) * (n * 1/65535), truncated) plus a
//!   recursive call with n*10/65535 for the low digits, capped at n-1.
//! The seed (0x90d0a0, option `-seed`) calls srand and picks the starting point in the table.

use core::cell::Cell;

use crate::comparison::{Scoreboard, TestRng};
use crate::detours::{self, Prologue};
use crate::registro::linha;
use crate::simulation;

const POINTER: *mut u32 = 0xde1f60 as *mut u32;
const OFFSET: *mut i32 = 0xde1f5c as *mut i32;
const TABLE_END: *const u32 = 0xde2798 as *const u32;
const TABLE_START: u32 = 0xaa2454;
/// State of the CRT's rand() (single-threaded Visual C++ 6 version).
const RAND_STATE: *mut u32 = 0xad6c30 as *mut u32;
/// 1/65535, double at 0x970a70.
const ONE_OVER_65535: f64 = 1.0 / 65535.0;

// 0x90cfc0: push ecx; push esi; mov esi, dword ptr [esp + 0xc]
const PROLOGUE: Prologue = Prologue { address: 0x90cfc0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] };

static mut ORIGINAL: usize = 0;

/// The game's own CRT rand() (same state the game uses).
unsafe fn rand_crt() -> i32 {
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    f()
}

thread_local! {
    /// Off during the Rust replica of comparison mode, so the simulation's draw count is
    /// the same in all three modes (entries into the routine, recursion included).
    static COUNTING: Cell<bool> = const { Cell::new(true) };
}

/// Runs `f` without counting its draws (Rust replica of routines that draw, in comparison mode).
pub fn without_counting<R>(f: impl FnOnce() -> R) -> R {
    let before = COUNTING.with(|c| c.replace(false));
    let r = f();
    COUNTING.with(|c| c.set(before));
    r
}

/// Generator state (table offset and pointer, CRT rand): regions for `side_by_side`.
pub const STATE_REGIONS: [(usize, usize); 2] = [(0xde1f5c, 8), (0xad6c30, 4)];

unsafe fn count_draw(n: i32) {
    detours::recheck_on_day();
    if COUNTING.with(|c| c.get()) {
        simulation::record_draw(n, *POINTER, *OFFSET, *RAND_STATE);
    }
}

/// Rust version of { `exe::utils_random` }.
pub unsafe extern "C" fn aleatorio(n: i32) -> i32 {
    count_draw(n);
    if n == 0 {
        return 0;
    }
    let mut p = (*POINTER).wrapping_add(4);
    *POINTER = p;
    let offset = if p > *TABLE_END {
        *POINTER = TABLE_START;
        p = TABLE_START;
        let d = rand_crt() & 0xffff;
        *OFFSET = d;
        d
    } else {
        *OFFSET
    };
    let entry = *(p as *const i32);
    if (-0xffff..=0xffff).contains(&n) {
        let r = offset.wrapping_add(entry).wrapping_rem(n);
        return if n > 0 { r } else { r.wrapping_neg() };
    }
    // n outside 16 bits: high part in floating point (x87 in double precision, as Windows
    // configures it), truncated like _ftol; low part by recursion.
    let s = offset.wrapping_add(entry).wrapping_rem(0xffff);
    let high = (s as f64 * (n as f64 * ONE_OVER_65535)) as i64 as i32;
    let ten_n = n.wrapping_mul(10);
    let low = aleatorio(div_by_65535(ten_n));
    let mut r = high.wrapping_add(low.wrapping_add(5) / 10);
    if r > n - 1 {
        r = n - 1;
    }
    r
}

/// What the compiler generated for `x / 65535` (signed): imul 0x80008001, add, sar 15, + sign.
fn div_by_65535(x: i32) -> i32 {
    let high = ((x as i64 * 0x8000_8001u32 as i32 as i64) >> 32) as i32;
    let t = high.wrapping_add(x) >> 15;
    t + ((t as u32) >> 31) as i32
}

// ---------- comparison with the original ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    pointer: u32,
    offset: i32,
    rand: u32,
}

impl State {
    unsafe fn read() -> State {
        State { pointer: *POINTER, offset: *OFFSET, rand: *RAND_STATE }
    }
    unsafe fn write_back(self) {
        *POINTER = self.pointer;
        *OFFSET = self.offset;
        *RAND_STATE = self.rand;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Running {
    Nothing,
    Original,
}

thread_local! {
    static RUNNING: Cell<Running> = const { Cell::new(Running::Nothing) };
}

static SCOREBOARD: Scoreboard = Scoreboard::new("random");

thread_local! {
    /// Draw trace (diagnosis): (n, answer) of each outside call, when on.
    static TRACE: core::cell::RefCell<Option<Vec<(i32, i32)>>> = const { core::cell::RefCell::new(None) };
}

/// Runs `f` keeping the outside draws it makes (n and answer).
pub fn trace<R>(f: impl FnOnce() -> R) -> (R, Vec<(i32, i32)>) {
    let before = TRACE.with(|t| t.borrow_mut().replace(Vec::new()));
    let r = f();
    let v = TRACE.with(|t| core::mem::replace(&mut *t.borrow_mut(), before)).unwrap_or_default();
    (r, v)
}

/// Runs `f` without keeping draws in the trace (those of a recorded unit: the Rust copy gets the result
/// from the tape and does not redo them, so the original's list only has to hold what the routine itself draws).
pub fn pause_trace<R>(f: impl FnOnce() -> R) -> R {
    let saved = TRACE.with(|t| t.borrow_mut().take());
    let r = f();
    TRACE.with(|t| *t.borrow_mut() = saved);
    r
}

unsafe fn original(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(ORIGINAL);
    f(n)
}

/// Runs the original and the Rust version from the same state and compares result and final state.
/// The game goes on with the original's result and state.
unsafe extern "C" fn compare(n: i32) -> i32 {
    count_draw(n);
    // The original calls itself (recursion for large n) through this detour.
    if RUNNING.with(|r| r.get()) == Running::Original {
        return original(n);
    }
    let tracing = TRACE.with(|t| t.borrow().is_some());
    let before = State::read();
    RUNNING.with(|r| r.set(Running::Original));
    let r_orig = original(n);
    RUNNING.with(|r| r.set(Running::Nothing));
    let after_orig = State::read();

    before.write_back();
    // restore (do not switch on): inside a Rust replica (`without_counting`) that draws several times, switching on
    // here made the replica's following draws enter the count
    let was_counting = COUNTING.with(|c| c.replace(false));
    let r_rust = aleatorio(n);
    COUNTING.with(|c| c.set(was_counting));
    let after_rust = State::read();
    after_orig.write_back();

    SCOREBOARD.record(r_orig == r_rust && after_orig == after_rust, || {
        format!(
            "n={} original={} rust={} state before={:x?} original={:x?} rust={:x?}",
            n, r_orig, r_rust, before, after_orig, after_rust
        )
    });
    if tracing {
        TRACE.with(|t| {
            if let Some(v) = t.borrow_mut().as_mut() {
                v.push((n, r_orig));
            }
        });
    }
    r_orig
}

/// Observer mode (`CM_ORIGINAL=1`): runs the original and only counts/checks the simulation stop.
unsafe extern "C" fn observe(n: i32) -> i32 {
    count_draw(n);
    original(n)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Rust,
    Compare,
    Observe,
}

pub unsafe fn install(mode: Mode) {
    let target = match mode {
        Mode::Rust => aleatorio as *const () as usize,
        Mode::Compare => compare as *const () as usize,
        Mode::Observe => observe as *const () as usize,
    };
    if let Some(t) = detours::hook("random", &PROLOGUE, target) {
        ORIGINAL = t;
        match mode {
            Mode::Compare => linha("random: comparison mode (original x Rust)"),
            Mode::Observe => linha("aleatorio: modo observador (so a original roda)"),
            Mode::Rust => {}
        }
    }
}

/// Compares original and Rust on `cases` synthetic inputs (state and n drawn, covering small,
/// large, negative n, edges and the table wrap). Requires comparison mode installed
/// (the original's recursion has to land in the original).
pub unsafe fn self_test(cases: u64) {
    if ORIGINAL == 0 {
        return;
    }
    const REAL_END: u32 = 0xad4130; // what the seed (0x90d0a0) writes to [0xde2798]
    let end = TABLE_END as *mut u32;
    let (saved, saved_end) = (State::read(), *end);
    let mut s = TestRng(0x9e37_79b9_7f4a_7c15);
    let mut draw = || s.next_u64();
    let entries = ((REAL_END - TABLE_START) / 4 + 1) as u64;
    let edges = [1, 2, 3, 7, 10, 100, 0xfffe, 0xffff, 0x10000, 0x10001, 100_000, 1_000_000, i32::MAX];
    SCOREBOARD.self_test(|| for i in 0..cases {
        let a = draw();
        // Pointer: any entry; one in eight near the end, to exercise the wrap.
        let entry = if a % 8 == 0 { entries - 1 - (a >> 3) % 3 } else { (a >> 3) % entries };
        *end = if a % 97 == 0 { 0 } else { REAL_END }; // before the seed the end is still 0
        State {
            pointer: TABLE_START + entry as u32 * 4,
            offset: ((a >> 20) & 0xffff) as i32,
            rand: (a >> 32) as u32,
        }
        .write_back();
        let b = draw();
        let n = match b % 6 {
            0 => (b >> 8) as i32 % 100 + 1,
            1 => ((b >> 8) % 0xffff) as i32 + 1,
            2 => -(((b >> 8) % 0xffff) as i32 + 1),
            3 => (b >> 8) as u32 as i32, // any, including huge and negative ones
            4 => edges[(b >> 8) as usize % edges.len()],
            _ => -edges[(b >> 8) as usize % (edges.len() - 1)],
        };
        compare(n);
        if i % 1_000_000 == 999_999 {
            linha(&format!("aleatorio: autoteste {} casos...", i + 1));
        }
    });
    saved.write_back();
    *end = saved_end;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_by_65535_matches_compiler() {
        for x in [0, 1, 65534, 65535, 65536, 655350, -1, -65535, -65536, i32::MAX, i32::MIN, 123456789, -987654321] {
            assert_eq!(div_by_65535(x), x / 65535, "x={}", x);
        }
    }
}
