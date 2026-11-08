// ---------- %%LEVA%%a leva: a riqueza do clube e duas ordens: a faixa das finanças, a ordem dos jogadores pela habilidade e a dos estádios pela capacidade (finance.cpp, game.cpp, stadium.cpp) ----------

/// { 0x5986b0 } (thiscall, finance.cpp): `Finance::get_wealth_band()`, a faixa do saldo (64 bits em +0/+4) do clube (+8,
/// vetor 0xae23b4) pela reputação R (+0x80), usada pelas transferências, pela diretoria, pelos técnicos, pelas finanças e
/// pelo adiamento. Seleção (0x53b900): 0. Acima de 2500·R + 5.000.000: 2; acima de 1000·R + 3.500.000: 1; abaixo de
/// −2.500.000 − 1500·R: −2; abaixo de −250·R (R < 5000) ou de (3750 − R)·1000 (R ≥ 5000): −1; senão 0.
unsafe extern "thiscall" fn get_wealth_band(this: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let id = core::ptr::read_unaligned((this + 8) as *const u32);
    let clube = core::ptr::read_unaligned(0xae23b4 as *const usize).wrapping_add(id.wrapping_mul(0x245) as usize);
    if selecao(clube) != 0 {
        return 0;
    }
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    let r = rep as i32;
    let saldo = core::ptr::read_unaligned(this as *const i64);
    if saldo > r.wrapping_mul(2500).wrapping_add(5_000_000) as i64 {
        crate::comparacao::ramo("cpu_7 rica");
        return 2; // cpu7: a faixa rica
    }
    if saldo > r.wrapping_mul(1000).wrapping_add(3_500_000) as i64 {
        crate::comparacao::ramo("cpu_7 confortavel");
        return 1; // cpu7: a faixa confortável
    }
    if saldo < (-2_500_000i32).wrapping_sub(r.wrapping_mul(1500)) as i64 {
        return 0xfe;
    }
    let devendo = if rep < 0x1388 { r.wrapping_mul(-250) } else { (0xea6 - r).wrapping_mul(1000) };
    if saldo < devendo as i64 {
        crate::comparacao::ramo("cpu_7 devendo");
        return 0xff; // cpu7: a faixa devendo
    }
    0
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_get_wealth_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(%%I0%%)(this);
    let n = get_wealth_band(this);
    PLACAR.registrar(r == n, || format!("get_wealth_band(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}

/// { 0x5d6220 } (cdecl, game.cpp): `compare_players_by_ability(a, b)`, o qsort dos dados de jogador em
/// game_daily_staff_update (0x5d588f): pela habilidade atual (+5), depois pela potencial (+7), crescente.
unsafe extern "C" fn compare_players_by_ability(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let h = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if h(pa, 5) == h(pb, 5) {
        return h(pa, 7) - h(pb, 7);
    }
    h(pa, 5) - h(pb, 5)
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_compare_players_by_ability(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I1%%)(a, b);
    let n = compare_players_by_ability(a, b);
    PLACAR.registrar(r == n, || format!("compare_players_by_ability({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// { 0x85e350 } (cdecl, stadium.cpp): `stadium_compare_venues_by_capacity(a, b)`, o qsort dos locais na busca do
/// estádio neutro (0x85c8de): pelos lugares (+0x3c) do estádio (+0xc), do maior para o menor; sem um dos estádios, 0.
unsafe extern "C" fn stadium_compare_venues_by_capacity(a: usize, b: usize) -> i32 {
    let ea = core::ptr::read_unaligned((a + 0xc) as *const usize);
    if ea == 0 {
        return 0;
    }
    let eb = core::ptr::read_unaligned((b + 0xc) as *const usize);
    if eb == 0 {
        return 0;
    }
    core::ptr::read_unaligned((eb + 0x3c) as *const i32).wrapping_sub(core::ptr::read_unaligned((ea + 0x3c) as *const i32))
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_stadium_compare_venues_by_capacity(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = stadium_compare_venues_by_capacity(a, b);
    PLACAR.registrar(r == n, || format!("stadium_compare_venues_by_capacity({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
