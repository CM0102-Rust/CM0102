//! Game dates (`si/code/Date.cpp`). 8-byte class:
//!   +0 i16 day of the year (0 = January 1st), +2 u16 year, +4 i32 leap year (0/1).
//! thiscall methods (the object comes in ecx); those that return a Data by value get the return
//! address as the first argument and return it in eax.
//!   { 0x54a110 } Data()                    day 40 of 1972, leap year
//!   { 0x549ef0 } Data(day, year)           computes the leap flag (Gregorian rule, 16-bit year)
//!   { 0x549f70 } Data(day_of_month, month, year, weekday)  validates (errors Date.cpp 52 and 59),
//!                converts through the cumulative-days tables and, if asked, adjusts to the weekday
//!   { 0x54a3f0 } dia_da_semana()           0..6, counted from 1600
//!   { 0x54a2d0 } ajustar_semana(target, dest)  dest = this date moved to the nearest `target` day (+-3)
//!   { 0x417c10 } operator=(src) -> copy    (assigns and returns a copy)
//!   { 0x54c5b0 } operator+(n days)         \
//!   { 0x54c8e0 } operator-(n days)          > each uses a local static Data (initialized on the
//!   { 0x54c770 } mais_semanas(n)           /  first call, with an empty destructor in atexit)
//!   { 0x54cdb0 } dias_ate(other)           this - other in days (other by value; uses the static Data
//!                0xae2470, which ends up with this's date and other's year)

use crate::comparison::{side_by_side_light as side_by_side, Scoreboard, TestRng};
use crate::detours::{self, Prologue};
use crate::crt;
use crate::registro::linha;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Data {
    pub day: i16,
    pub year: u16,
    pub is_leap_year: i32,
}

/// Path "E:\dev\CM3\cm3 00-01\si\code\Date.cpp" in the exe (for the error messages).
const SOURCE_PATH: usize = 0x9b8478;
/// Cumulative days before each month (i16 x 12): normal and leap year.
const NORMAL_TABLE: *const i16 = 0x9b8448 as *const i16;
const LEAP_TABLE: *const i16 = 0x9b8460 as *const i16;

/// Local static Data of an operator: value, guard byte (bit 0 = already initialized) and destructor.
struct LocalStatic {
    value: usize,
    guard: usize,
    destructor: usize,
}

const ST_PLUS: LocalStatic = LocalStatic { value: 0xae2420, guard: 0xae2469, destructor: 0x54c760 };
const ST_MINUS: LocalStatic = LocalStatic { value: 0xae24c0, guard: 0xae249e, destructor: 0x54caa0 };
const ST_WEEKS: LocalStatic = LocalStatic { value: 0xae2410, guard: 0xae2430, destructor: 0x54c8d0 };
const ST_DAYS: LocalStatic = LocalStatic { value: 0xae2470, guard: 0xae2418, destructor: 0x54cf60 };

/// Memory the operators touch besides the arguments (for comparison mode).
const STATICS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1)];

const PROLOGUES: [Prologue; 10] = [
    // 0x54a110: mov eax, ecx; mov word ptr [eax], 0x28
    Prologue { address: 0x54a110, bytes: &[0x8b, 0xc1, 0x66, 0xc7, 0x00, 0x28, 0x00] },
    // 0x549ef0: mov ax, word ptr [esp + 4]
    Prologue { address: 0x549ef0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    // 0x54a3f0: push esi; push edi; xor edi, edi; mov eax, 0x51eb851f
    Prologue { address: 0x54a3f0, bytes: &[0x56, 0x57, 0x33, 0xff, 0xb8, 0x1f, 0x85, 0xeb, 0x51] },
    // 0x417c10: push ecx; mov eax, dword ptr [esp + 0xc]
    Prologue { address: 0x417c10, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] },
    // 0x54c5b0: sub esp, 0x10; mov al, byte ptr [0xae2469]
    Prologue { address: 0x54c5b0, bytes: &[0x83, 0xec, 0x10, 0xa0, 0x69, 0x24, 0xae, 0x00] },
    // 0x54c8e0: sub esp, 0x10; push esi; push edi
    Prologue { address: 0x54c8e0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] },
    // 0x54c770: push ecx; mov al, byte ptr [0xae2430]
    Prologue { address: 0x54c770, bytes: &[0x51, 0xa0, 0x30, 0x24, 0xae, 0x00] },
    // 0x54a2d0: mov eax, dword ptr fs:[0]
    Prologue { address: 0x54a2d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    // 0x549f70: sub esp, 0x200
    Prologue { address: 0x549f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    // 0x54cdb0: sub esp, 0xc; mov al, byte ptr [0xae2418]
    Prologue { address: 0x54cdb0, bytes: &[0x83, 0xec, 0x0c, 0xa0, 0x18, 0x24, 0xae, 0x00] },
];

/// Rule of { 0x549ef0 }: odd or not divisible by 4 -> no; not divisible by 100 -> yes;
/// otherwise, only if divisible by 400.
pub fn is_leap_year(year: u16) -> i32 {
    let a = year as i32;
    if a % 2 == 1 || a % 4 != 0 {
        0
    } else if a % 100 != 0 {
        1
    } else {
        (a % 400 == 0) as i32
    }
}

/// { `exe::date_date_54a110` }
pub unsafe extern "thiscall" fn default_date(this: *mut Data) -> *mut Data {
    *this = Data { day: 40, year: 1972, is_leap_year: 1 };
    this
}

/// { 0x549ef0 } The two arguments take 4 bytes on the stack; only the low 16 bits count.
pub unsafe extern "thiscall" fn new_date(this: *mut Data, day: u32, year: u32) -> *mut Data {
    let year = year as u16;
    *this = Data { day: day as u16 as i16, year: year, is_leap_year: is_leap_year(year) };
    this
}

/// { `exe::date_get_weekday` } Days since 1600 (years * 365 + leap years) + day of the year, minus 1 if the year is
/// a leap year, signed modulo 7; 1..6 becomes 0..5, 0 becomes 6 and negative stays as is.
pub unsafe extern "thiscall" fn weekday(this: *const Data) -> i32 {
    let d = *this;
    let y = d.year as i32 - 1600;
    let mut days = y / 400 + y * 365 - y / 100 + y / 4 + d.day as i32;
    if d.is_leap_year != 0 {
        days -= 1;
    }
    let r = days % 7;
    if r > 0 {
        r - 1
    } else if r == 0 {
        6
    } else {
        r
    }
}

/// { `exe::date_operator_3d` } this = *src (year, day, leap flag, in that order); *ret = *this; returns ret.
pub unsafe extern "thiscall" fn copia(this: *mut Data, ret: *mut Data, src: *const Data) -> *mut Data {
    (*this).year = (*src).year;
    (*this).day = (*src).day;
    (*this).is_leap_year = (*src).is_leap_year;
    *ret = *this;
    ret
}

/// Initialization of the local static on the first call: guard |= 1, Data(), atexit(destructor).
unsafe fn prepare(e: &LocalStatic) -> *mut Data {
    let g = e.guard as *mut u8;
    if *g & 1 == 0 {
        *g |= 1;
        *(e.value as *mut Data) = Data { day: 40, year: 1972, is_leap_year: 1 };
        crt::atexit(e.destructor);
    }
    e.value as *mut Data
}

/// Moves `day` (which may pass 365) forward into the following years. Comparisons in signed 16 bits.
fn normalize_forward(mut day: i16, mut year: u16) -> (i16, u16) {
    loop {
        let limit = if is_leap_year(year) == 1 { 365 } else { 364 };
        if day <= limit {
            return (day, year);
        }
        day = day.wrapping_sub(limit + 1);
        year = year.wrapping_add(1);
    }
}

/// Moves `day` (which may be negative) back into the previous years, adding the length of the previous year.
fn normalize_backward(mut day: i16, mut year: u16) -> (i16, u16) {
    loop {
        if day >= 0 {
            return (day, year);
        }
        let previous = year.wrapping_sub(1);
        day = day.wrapping_add(if is_leap_year(previous) == 1 { 366 } else { 365 });
        year = previous;
    }
}

unsafe fn store(st: *mut Data, ret: *mut Data, day: i16, year: u16) -> *mut Data {
    (*st).year = year;
    (*st).day = day;
    (*st).is_leap_year = is_leap_year(year);
    *ret = *st;
    ret
}

/// { `exe::date_add_days` } n (16 bits) negative: static = *this and returns static - (-n).
/// Quirk preserved: with n = -32768, -n stays -32768 and plus/minus call each other until the stack
/// overflows (same in the original; the game never asks for that).
pub unsafe extern "thiscall" fn mais(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let st = prepare(&ST_PLUS);
    if (n as i16) < 0 {
        let (mut t1, mut t2, mut t3) = (Data::default(), Data::default(), Data::default());
        copia(st, &mut t1, this);
        let r = menos(st, &mut t2, (n as i32).wrapping_neg() as u32);
        copia(st, &mut t3, r);
        *ret = *st;
        return ret;
    }
    let (day, year) = normalize_forward((*this).day.wrapping_add(n as i16), (*this).year);
    store(st, ret, day, year)
}

/// { 0x54c8e0 } n (16 bits) negative: static = *this and returns static + (-n).
pub unsafe extern "thiscall" fn menos(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let st = prepare(&ST_MINUS);
    if (n as i16) < 0 {
        let (mut t1, mut t2, mut t3) = (Data::default(), Data::default(), Data::default());
        copia(st, &mut t1, this);
        let r = mais(st, &mut t2, (n as i32).wrapping_neg() as u32);
        copia(st, &mut t3, r);
        *ret = *st;
        return ret;
    }
    let (day, year) = normalize_backward((*this).day.wrapping_sub(n as i16), (*this).year);
    store(st, ret, day, year)
}

/// { `exe::date_add_weeks` } n (8 bits) weeks.
pub unsafe extern "thiscall" fn add_weeks(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let st = prepare(&ST_WEEKS);
    let days = ((n as u8) as u16).wrapping_mul(7) as i16;
    let (day, year) = normalize_forward(days.wrapping_add((*this).day), (*this).year);
    store(st, ret, day, year)
}

/// { `exe::date_get_nearest_weekday` } *dest = *this and, if the weekday is not `target`, moves dest to the nearest `target`
/// (difference brought to -3..3). The return value is not used by the callers.
pub unsafe extern "thiscall" fn adjust_week(this: *const Data, target: u32, dest: *mut Data) -> u32 {
    let weekday = weekday(this);
    let mut t = Data::default();
    copia(dest, &mut t, this);
    if weekday as i16 == target as i16 {
        return target;
    }
    let mut diff = weekday.wrapping_sub(target as i32);
    if (diff as i16) < 0 {
        if (diff as i16) < -3 {
            diff += 7;
        }
    } else if (diff as i16) > 3 {
        diff -= 7;
    }
    let (mut t2, mut t3) = (Data::default(), Data::default());
    let r = menos(dest, &mut t2, diff as u32);
    copia(dest, &mut t3, r) as u32
}

/// Day of month and month out of range: the game shows the error (Date.cpp 52 / 59) and corrects it.
fn valid_input(day: u32, month: u32) -> bool {
    (1..=31).contains(&(day as i16)) && (0..=11).contains(&(month as u8 as i8))
}

/// { `exe::date_date_549f70` } Data(day_of_month, month 0..11, year, weekday or 0xffff).
pub unsafe extern "thiscall" fn from_dmy(this: *mut Data, day: u32, month: u32, year: u32, weekday: u32) -> *mut Data {
    let mut di = day as i16;
    if !(1..=31).contains(&di) {
        crt::error_box(SOURCE_PATH, 0x34);
        di = 1;
    }
    let mut m = month as u8 as i8;
    if !(0..=11).contains(&m) {
        crt::error_box(SOURCE_PATH, 0x3b);
        m = 0;
    }
    let year = year as u16;
    (*this).year = year;
    if is_leap_year(year) == 1 {
        (*this).is_leap_year = 1;
        (*this).day = (*LEAP_TABLE.add(m as usize)).wrapping_add(di);
    } else {
        (*this).is_leap_year = 0;
        (*this).day = (*NORMAL_TABLE.add(m as usize)).wrapping_add(di);
    }
    if weekday as u16 != 0xffff {
        adjust_week(this, weekday, this);
    }
    this
}

/// Days of year `year` by the rule of { `exe::date_days_since` } (the same Gregorian one, on the year as unsigned 32 bits).
fn days_in_year(year: u16) -> i32 {
    let a = year as i32;
    if a % 2 != 0 || a % 4 != 0 {
        365
    } else if a % 100 != 0 || a % 400 == 0 {
        366
    } else {
        365
    }
}

/// { `exe::date_days_since` } (ret 8) this - other in days, with `other` by value (8 bytes on the stack). In the same
/// year it is just the difference of the days. With different years, the static 0xae2470 gets *this (`exe::date_operator_3d`)
/// and walks year by year to `other`'s, adding the days of each year: going back, those of the year
/// reached itself, going forward, those of the year before each one (as in the original); at the end the static
/// holds `other`'s year.
pub unsafe extern "thiscall" fn days_until(this: *const Data, other: Data) -> i32 {
    let st = prepare(&ST_DAYS);
    let a = *this;
    let mut sum = 0i32;
    if other.year != a.year {
        *st = a;
        let mut year = a.year;
        if other.year > a.year {
            while year < other.year {
                sum -= days_in_year(year.wrapping_sub(1));
                year += 1;
            }
        } else {
            while year > other.year {
                year -= 1;
                sum += days_in_year(year);
            }
        }
        (*st).year = year;
    }
    sum + a.day as i32 - other.day as i32
}

// ---------- comparison with the originals ----------

static mut ORIGINALS: [usize; 10] = [0; 10];
static SCOREBOARD: Scoreboard = Scoreboard::new("date");

unsafe fn original_fn<F>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINALS[i])
}

/// The dates (8 bytes each, up to 3) and the statics, in a fixed array on the stack (no allocation: dates are
/// compared billions of times in a career with all the leagues).
struct Regions([(usize, usize); 9], usize);

impl core::ops::Deref for Regions {
    type Target = [(usize, usize)];
    fn deref(&self) -> &Self::Target {
        &self.0[..self.1]
    }
}

fn regions(objects: &[usize]) -> Regions {
    let mut r = Regions([(0, 0); 9], 0);
    for (i, &o) in objects.iter().enumerate() {
        r.0[i] = (o, 8);
    }
    for (i, &e) in STATICS.iter().enumerate() {
        r.0[objects.len() + i] = e;
    }
    r.1 = objects.len() + STATICS.len();
    r
}

unsafe extern "thiscall" fn compare_default(this: *mut Data) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data) -> *mut Data = original_fn(0);
    let (r, rr, equal, mo, mr) = side_by_side(&regions(&[this as usize]), || f(this), || default_date(this));
    SCOREBOARD.record(equal && r == rr, || format!("default original={:x?} rust={:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn compare_new(this: *mut Data, day: u32, year: u32) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, u32, u32) -> *mut Data = original_fn(1);
    let (r, rr, equal, mo, mr) = side_by_side(&regions(&[this as usize]), || f(this, day, year), || new_date(this, day, year));
    SCOREBOARD.record(equal && r == rr, || format!("new({:#x}, {:#x}) original={:x?} rust={:x?}", day, year, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_weekday(this: *const Data) -> i32 {
    let f: unsafe extern "thiscall" fn(*const Data) -> i32 = original_fn(2);
    let r = f(this);
    let ours = weekday(this);
    SCOREBOARD.record(r == ours, || format!("week({:?}) original={} rust={}", *this, r, ours));
    r
}

unsafe extern "thiscall" fn compare_copy(this: *mut Data, ret: *mut Data, src: *const Data) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, *mut Data, *const Data) -> *mut Data = original_fn(3);
    let reg = regions(&[this as usize, ret as usize, src as usize]);
    let (r, rr, equal, mo, mr) = side_by_side(&reg, || f(this, ret, src), || copia(this, ret, src));
    SCOREBOARD.record(equal && r == rr, || format!("copy original={:x?} rust={:x?}", mo, mr));
    r
}

macro_rules! compare_operator {
    ($name:ident, $i:expr, $rust:ident, $text:literal) => {
        unsafe extern "thiscall" fn $name(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
            let f: unsafe extern "thiscall" fn(*const Data, *mut Data, u32) -> *mut Data = original_fn($i);
            let reg = regions(&[this as usize, ret as usize]);
            let (r, rr, equal, mo, mr) = side_by_side(&reg, || f(this, ret, n), || $rust(this, ret, n));
            SCOREBOARD.record(equal && r == rr, || format!("{}({:?}, {:#x}) original={:x?} rust={:x?}", $text, *this, n, mo, mr));
            r
        }
    };
}

compare_operator!(compare_plus, 4, mais, "mais");
compare_operator!(compare_minus, 5, menos, "menos");
compare_operator!(compare_weeks, 6, add_weeks, "mais_semanas");

unsafe extern "thiscall" fn compare_adjust(this: *const Data, target: u32, dest: *mut Data) -> u32 {
    let f: unsafe extern "thiscall" fn(*const Data, u32, *mut Data) -> u32 = original_fn(7);
    let reg = regions(&[this as usize, dest as usize]);
    // The return value is the address of a temporary on the stack: it is not compared.
    let (r, _, equal, mo, mr) = side_by_side(&reg, || f(this, target, dest), || adjust_week(this, target, dest));
    SCOREBOARD.record(equal, || format!("adjust_week({:#x}) original={:x?} rust={:x?}", target, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_dma(this: *mut Data, day: u32, month: u32, year: u32, weekday: u32) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, u32, u32, u32, u32) -> *mut Data = original_fn(8);
    if !valid_input(day, month) {
        // Error path: it would show the box twice. Only the original runs.
        return f(this, day, month, year, weekday);
    }
    let reg = regions(&[this as usize]);
    let (r, rr, equal, mo, mr) = side_by_side(&reg, || f(this, day, month, year, weekday), || from_dmy(this, day, month, year, weekday));
    SCOREBOARD.record(equal && r == rr, || format!("dma({:#x}, {:#x}, {:#x}, {:#x}) original={:x?} rust={:x?}", day, month, year, weekday, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_days(this: *const Data, other: Data) -> i32 {
    let f: unsafe extern "thiscall" fn(*const Data, Data) -> i32 = original_fn(9);
    let reg = [(this as usize, 8), (ST_DAYS.value, 8), (ST_DAYS.guard, 1)];
    let (r, rr, equal, mo, mr) = side_by_side(&reg, || f(this, other), || days_until(this, other));
    SCOREBOARD.record(equal && r == rr, || format!("dias_ate({:?}, {:?}) original={} {:x?} rust={} {:x?}", *this, other, r, mo, rr, mr));
    r
}

/// Self-test (runs after the game's CRT is initialized: the operators call atexit).
pub unsafe fn self_test() {
    let mut s = TestRng(0x1234_5678_9abc_def1);
    SCOREBOARD.self_test(|| {
        // Constructors: every 16-bit year, with junk in the upper 16 bits of the arguments.
        for year in 0..=0xffffu32 {
            let junk = s.next_u64() as u32;
            let mut d = Data::default();
            compare_new(&mut d, (junk & 0xffff_0000) | (junk >> 7 & 0xffff), (junk << 16) | year);
        }
        let mut d = Data { day: -1, year: 0, is_leap_year: 7 };
        compare_default(&mut d);
        // Weekday: every year x days -5..370 x leap flag 0/1 and some odd values.
        for year in 0..=0xffffu16 {
            for day in (-5i16..=370).step_by(if (1590..=2100).contains(&year) { 1 } else { 23 }) {
                for b in [0, 1] {
                    compare_weekday(&Data { day: day, year: year, is_leap_year: b });
                }
            }
        }
        for _ in 0..200_000 {
            let x = s.next_u64();
            compare_weekday(&Data { day: x as i16, year: (x >> 16) as u16, is_leap_year: (x >> 32) as i32 });
        }
        // Operators and co.: plausible and odd dates, small, large, negative n and edges.
        // Out: n with low 16 bits = 0x8000 (-32768). Negated it stays -32768, and plus/minus
        // call each other forever until the stack overflows; the original does the same.
        let edges: [u32; 10] = [0, 1, 7, 365, 366, 0x7fff, 0x8001, 0xffff, 0x1_0005, 0xffff_fff9];
        for i in 0..400_000u32 {
            let x = s.next_u64();
            let year = if i % 4 == 0 { (x >> 48) as u16 } else { 1900 + ((x >> 48) % 250) as u16 };
            let day = if i % 5 == 0 { (x >> 32) as i16 } else { ((x >> 32) % 400) as i16 - 20 };
            let a = Data { day: day, year: year, is_leap_year: if i % 7 == 0 { (x >> 20) as i32 } else { is_leap_year(year) } };
            let n = if i % 3 == 0 { edges[(x as usize >> 4) % edges.len()] } else { (x >> 8) as u32 % 2000 };
            let n = if i % 2 == 0 { n } else { (n as i32).wrapping_neg() as u32 };
            if n as u16 == 0x8000 {
                continue;
            }
            let mut ret = Data::default();
            compare_plus(&a, &mut ret, n);
            compare_minus(&a, &mut ret, n);
            compare_weeks(&a, &mut ret, n);
            let mut dest = Data { day: 99, year: 1, is_leap_year: 3 };
            compare_adjust(&a, (x >> 12) as u32 % 9, &mut dest);
            let mut target = a;
            let src = Data { day: (x >> 16) as i16, year: (x >> 24) as u16, is_leap_year: x as i32 };
            compare_copy(&mut target, &mut ret, &src);
            compare_copy(&mut target, &mut ret, &target as *const Data); // src == this
            let (dm, m) = (1 + (x >> 40) as u32 % 31, (x >> 45) as u32 % 12);
            let weekday = if i % 2 == 0 { 0xffff } else { (x >> 50) as u32 % 7 };
            let mut c = Data::default();
            compare_dma(&mut c, dm, m, year as u32, weekday);
        }
    });
}

pub unsafe fn install(comparing: bool) {
    let rust: [usize; 10] = [
        default_date as *const () as usize,
        new_date as *const () as usize,
        weekday as *const () as usize,
        copia as *const () as usize,
        mais as *const () as usize,
        menos as *const () as usize,
        add_weeks as *const () as usize,
        adjust_week as *const () as usize,
        from_dmy as *const () as usize,
        days_until as *const () as usize,
    ];
    let comp: [usize; 10] = [
        compare_default as *const () as usize,
        compare_new as *const () as usize,
        compare_weekday as *const () as usize,
        compare_copy as *const () as usize,
        compare_plus as *const () as usize,
        compare_minus as *const () as usize,
        compare_weeks as *const () as usize,
        compare_adjust as *const () as usize,
        compare_dma as *const () as usize,
        compare_days as *const () as usize,
    ];
    let names = ["data_padrao", "data_nova", "dia_da_semana", "data_copia", "data_mais", "data_menos", "data_mais_semanas", "data_ajustar_semana", "data_dma", "data_dias_ate"];
    for i in 0..10 {
        if let Some(t) = detours::hook(names[i], &PROLOGUES[i], if comparing { comp[i] } else { rust[i] }) {
            ORIGINALS[i] = t;
        }
    }
    if comparing {
        linha("date: comparison mode (original x Rust)");
    }
}

/// Simulation in original mode: the same routines detoured to the original itself, which then runs
/// on the private stack like the Rust version (see `detours::isolar`).
pub unsafe fn install_passthrough() {
    for (i, p) in PROLOGUES.iter().enumerate() {
        detours::hook(&format!("date {} (original, isolated)", i), p, detours::PASSTHROUGH);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_per_year() {
        assert_eq!(days_in_year(2000), 366);
        assert_eq!(days_in_year(1900), 365);
        assert_eq!(days_in_year(2004), 366);
        assert_eq!(days_in_year(2001), 365);
    }

    #[test]
    fn leap_years() {
        assert_eq!(is_leap_year(2000), 1);
        assert_eq!(is_leap_year(1900), 0);
        assert_eq!(is_leap_year(2001), 0);
        assert_eq!(is_leap_year(2004), 1);
        assert_eq!(is_leap_year(0), 1);
    }

    #[test]
    fn weekday_range() {
        unsafe {
            // After 1600 the result stays in 0..6 and moves one day at a time.
            for year in [1601u16, 1999, 2000, 2001] {
                let mut previous = weekday(&Data { day: 0, year: year, is_leap_year: is_leap_year(year) });
                for day in 1..366 {
                    let r = weekday(&Data { day: day, year: year, is_leap_year: is_leap_year(year) });
                    assert!((0..=6).contains(&r));
                    assert_eq!(r, (previous + 1) % 7);
                    previous = r;
                }
            }
            // In 1600, day 0, leap year: the game's arithmetic gives -1 (negative remainder returned as is).
            assert_eq!(weekday(&Data { day: 0, year: 1600, is_leap_year: 1 }), -1);
        }
    }

    #[test]
    fn normalization() {
        assert_eq!(normalize_forward(365, 2001), (0, 2002));
        assert_eq!(normalize_forward(365, 2000), (365, 2000)); // leap year: 366 days
        assert_eq!(normalize_forward(366 + 365, 2000), (0, 2002));
        assert_eq!(normalize_backward(-1, 2001), (365, 2000));
        assert_eq!(normalize_backward(-1, 2002), (364, 2001));
    }
}
