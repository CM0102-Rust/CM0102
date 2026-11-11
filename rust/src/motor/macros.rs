// The match-engine macros (textual scope: included first).

/// Test coverage marker, not game logic: counts how often this path ran (ddraw.log, `branch NAME: N`), so the mutation
/// tests (engenharia-reversa/mutacoes.py) skip a mutation whose path never ran. Compiled out without the `coverage`
/// feature (on by default; `cargo build --release --no-default-features` builds the DLL without the markers).
macro_rules! covered {
    ($name:expr) => {
        if cfg!(feature = "coverage") {
            crate::comparison::branch($name)
        }
    };
}

macro_rules! reader {
    ($nome:ident, $campo:expr) => {
        /// Leitor de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            *((this + $campo) as *const u8)
        }
    };
}

macro_rules! recorder {
    ($nome:ident, $campo:expr) => {
        /// Gravador de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, v: u8) {
            *((this + $campo) as *mut u8) = v;
        }
    };
}

macro_rules! compare_reader {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>($i)(this);
            let n = $rust(this);
            SCOREBOARD.record(r == n, || format!("{} original={} rust={}", stringify!($rust), r, n));
            r
        }
    };
}

macro_rules! compare_recorder {
    ($nome:ident, $rust:ident, $i:expr, $campo:expr, $t:ty) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, v: $t) {
            let f = original_fn::<unsafe extern "thiscall" fn(usize, $t)>($i);
            let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + $campo, core::mem::size_of::<$t>())], || f(this, v), || $rust(this, v));
            SCOREBOARD.record(igual, || format!("{} original={:x?} rust={:x?}", stringify!($rust), mo, mr));
        }
    };
}

macro_rules! compare_split {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, l: i8, delta: i16) {
            let f = original_fn::<unsafe extern "thiscall" fn(usize, i8, i16)>($i);
            let reg: Vec<(usize, usize)> = (0..20).map(|k| (Engine(this).player(l, k).addr() + 0x35, 2)).collect();
            let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this, l, delta), || $rust(this, l, delta));
            SCOREBOARD.record(igual, || format!("{}({}, {}) original={:x?} rust={:x?}", stringify!($rust), l, delta, mo, mr));
        }
    };
}

macro_rules! compare_value {
    ($nome:ident, $rust:ident, $i:expr, ($($a:ident: $t:ty),*) -> $r:ty) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize $(, $a: $t)*) -> $r {
            let r = original_fn::<unsafe extern "thiscall" fn(usize $(, $t)*) -> $r>($i)(this $(, $a)*);
            let n = $rust(this $(, $a)*);
            SCOREBOARD.record(r == n, || format!("{}({:#x}{}) original={:#x} rust={:#x}", stringify!($rust), this, String::new() $(+ &format!(", {:?}", $a))*, r, n));
            r
        }
    };
}

macro_rules! compare_order {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "C" fn $nome(a: *const usize, b: *const usize) -> i32 {
            let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>($i)(a, b);
            let v = $rust(a, b);
            SCOREBOARD.record(r == v, || format!("{} original={} rust={}", stringify!($rust), r, v));
            r
        }
    };
}
