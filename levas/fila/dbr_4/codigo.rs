// ---------- %%LEVA%%a leva: as ordens dos clubes pela latitude e pela longitude da cidade do estádio (Database.cpp) ----------

/// A ordem dos clubes `ca` e `cb` (não nulos) pela coordenada f64 em `d` da cidade (+0x38) do estádio (+0x69); sem
/// estádio ou sem cidade, a de `ca` vale [0x9689a0] e a de `cb` vale 0. −1 se a de `ca` é menor (ou NaN: o `fcom`
/// com C0), 1 se maior, 0 se igual.
unsafe fn order_by_city_coordinate(ca: usize, cb: usize, d: usize) -> i32 {
    let cidade = |c: usize| {
        let e = core::ptr::read_unaligned((c + 0x69) as *const usize);
        if e == 0 { 0 } else { core::ptr::read_unaligned((e + 0x38) as *const usize) }
    };
    let (xa, xb) = (cidade(ca), cidade(cb));
    let x = if xa != 0 { core::ptr::read_unaligned((xa + d) as *const f64) } else { read_f64_at(0x9689a0) };
    let y = if xb != 0 { core::ptr::read_unaligned((xb + d) as *const f64) } else { 0.0 };
    if !(x >= y) {
        -1
    } else if x > y {
        1
    } else {
        0
    }
}

/// { 0x53fd40 } (cdecl, Database.cpp): `database_compare_clubs_by_latitude(a, b)`: comparação do qsort de ponteiros
/// de clube pela latitude (+0x23, f64) da cidade do estádio, a menor primeiro. Clube nulo: a original (caixa e 0).
/// Nas copas sorteadas por região (eng_auto_cup 0x56b730, cro_a1, fin_prm, ita_ser_a, nor_prm, por_prm, spa_first,
/// swe_cup).
unsafe extern "C" fn database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
    if ca == 0 || cb == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    }
    order_by_city_coordinate(ca, cb, 0x23) // a latitude
}

unsafe extern "C" fn comparar_database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_latitude(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_latitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53fe50 } (cdecl, Database.cpp): `database_compare_clubs_by_longitude(a, b)`: a mesma comparação pela
/// longitude (+0x2b, f64). Clube nulo: a original (caixa e 0). Nas copas sorteadas por região (bel_first, hol_cup,
/// ita_c_cup, spa_first, tur_first).
unsafe extern "C" fn database_compare_clubs_by_longitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
    if ca == 0 || cb == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    }
    order_by_city_coordinate(ca, cb, 0x2b) // a longitude
}

unsafe extern "C" fn comparar_database_compare_clubs_by_longitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_longitude(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_longitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
