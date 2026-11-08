// The match-engine macros (textual scope: included first).

macro_rules! leitor {
    ($nome:ident, $campo:expr) => {
        /// Leitor de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            *((this + $campo) as *const u8)
        }
    };
}

macro_rules! gravador {
    ($nome:ident, $campo:expr) => {
        /// Gravador de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, v: u8) {
            *((this + $campo) as *mut u8) = v;
        }
    };
}

macro_rules! comparar_leitor {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>($i)(this);
            let n = $rust(this);
            PLACAR.record(r == n, || format!("{} original={} rust={}", stringify!($rust), r, n));
            r
        }
    };
}

macro_rules! comparar_gravador {
    ($nome:ident, $rust:ident, $i:expr, $campo:expr, $t:ty) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, v: $t) {
            let f = original_fn::<unsafe extern "thiscall" fn(usize, $t)>($i);
            let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + $campo, core::mem::size_of::<$t>())], || f(this, v), || $rust(this, v));
            PLACAR.record(igual, || format!("{} original={:x?} rust={:x?}", stringify!($rust), mo, mr));
        }
    };
}

macro_rules! comparar_repartir {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize, l: i8, delta: i16) {
            let f = original_fn::<unsafe extern "thiscall" fn(usize, i8, i16)>($i);
            let reg: Vec<(usize, usize)> = (0..20).map(|k| (side_player_slot(this, l, k) + 0x35, 2)).collect();
            let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this, l, delta), || $rust(this, l, delta));
            PLACAR.record(igual, || format!("{}({}, {}) original={:x?} rust={:x?}", stringify!($rust), l, delta, mo, mr));
        }
    };
}

macro_rules! comparar_valor {
    ($nome:ident, $rust:ident, $i:expr, ($($a:ident: $t:ty),*) -> $r:ty) => {
        pub(crate) unsafe extern "thiscall" fn $nome(this: usize $(, $a: $t)*) -> $r {
            let r = original_fn::<unsafe extern "thiscall" fn(usize $(, $t)*) -> $r>($i)(this $(, $a)*);
            let n = $rust(this $(, $a)*);
            PLACAR.record(r == n, || format!("{}({:#x}{}) original={:#x} rust={:#x}", stringify!($rust), this, String::new() $(+ &format!(", {:?}", $a))*, r, n));
            r
        }
    };
}

macro_rules! comparar_ordem {
    ($nome:ident, $rust:ident, $i:expr) => {
        pub(crate) unsafe extern "C" fn $nome(a: *const usize, b: *const usize) -> i32 {
            let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>($i)(a, b);
            let v = $rust(a, b);
            PLACAR.record(r == v, || format!("{} original={} rust={}", stringify!($rust), r, v));
            r
        }
    };
}
