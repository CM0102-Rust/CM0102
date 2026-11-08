// ---------- %%LEVA%%a leva: os amistosos internacionais: o par de nações que não joga amistoso, a ordem das nações pela reputação e a dos amistosos pela data (friendly.cpp) ----------

unsafe fn cpp11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x5cc3e0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_banned_nation_pair(a, b)`: percorre os pares proibidos
/// (vetor de 8 bytes em +0x3502, conta em byte com sinal em +0x3506) e devolve 1 quando (a, b), (a, −1) ou (−1, b) está
/// lá. Senão 0: o resto da original (Austrália, Nova Zelândia e Oceania, 0x9cf224/0x9cf3f4/0x9cfa18, pela tabela das
/// nações) só lê e cai sempre no `xor eax, eax`.
unsafe extern "thiscall" fn is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let n = *((this + 0x3506) as *const i8);
    let mut k: i8 = 0;
    while k < n {
        let p = cpp11_d(this + 0x3502).wrapping_add(k as isize as usize * 8);
        let (x, y) = (cpp11_i(p), cpp11_i(p + 4));
        if (x == a && (y == b || y == -1)) || (x == -1 && y == b) {
            crate::comparacao::ramo("cpp_11 par proibido");
            return 1; // o par proibido
        }
        k = k.wrapping_add(1);
    }
    // ponytail: as leituras da Oceania não mudam a devolução (sempre 0); a cópia não as repete.
    0
}

/// { 0x5cc5f0 } (cdecl, friendly.cpp): `compare_nations_by_reputation_desc(a, b)`: o qsort das listas de nações
/// (ponteiros) pela reputação (+0x8e, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let ra = *((cpp11_d(a) + 0x8e) as *const i16) as i32;
    let rb = *((cpp11_d(b) + 0x8e) as *const i16) as i32;
    rb - ra // a maior reputação primeiro
}

/// { 0x5cc610 } (cdecl, friendly.cpp): `compare_fixture_records_by_date(a, b)`: o qsort dos registros de 20 bytes dos
/// amistosos pela data em +3: −1 se a de `a` é antes (0x456770), 1 se é depois (0x456740), senão 0. Quente (qsort):
/// sem contador.
unsafe extern "C" fn compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpp11_d(b + 3) as u32, cpp11_d(b + 7) as u32);
    let r = if antes(a + 3, d0, d1) != 0 { -1 } else { (depois(a + 3, d0, d1) != 0) as i32 };
    r // a data mais cedo primeiro
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(%%I0%%)(this, a, b);
    let n = is_banned_nation_pair(this, a, b);
    PLACAR.registrar(r == n, || format!("is_banned_nation_pair({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_nations_by_reputation_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_nations_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_fixture_records_by_date(a, b);
    PLACAR.registrar(r == n, || format!("compare_fixture_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
