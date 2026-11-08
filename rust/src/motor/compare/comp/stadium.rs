use super::*;

// The comparers of the routines in ../comp/stadium.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/stadium.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `find_available_venue` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado (a cópia sem contar os sorteios) pelas regiões de `stadium_choice_regions` e pela devolução; com reserva que pode alocar, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_find_available_venue(this: usize, marks: u32, cont: usize, nacao: usize, data: usize, home: usize, away: usize, partida: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize>(IDX_FIND_AVAILABLE_VENUE);
    let Some((r, m)) = stadium_choice_regions(this, None) else {
        // simplification: the reserve may allocate the list; original only
        return o(this, marks, cont, nacao, data, home, away, partida);
    };
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&r[..m], || o(this, marks, cont, nacao, data, home, away, partida), || {
        crate::aleatorio::sem_contar(|| find_available_venue(this, marks, cont, nacao, data, home, away, partida))
    });
    PLACAR.registrar(igual && ro == rr, || format!("find_available_venue(marcas {:x}, nacao {:x}, cont {:x}): original {:x} rust {:x} {:x?} x {:x?}", marks, nacao, cont, ro, rr, mo, mr));
    ro
}

/// Comparer for `set_fixture_venue` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado pela partida (0x44 bytes) e pelas regiões de `stadium_choice_regions`; partida nula ou reserva que pode alocar:
/// só a original.
pub(crate) unsafe extern "thiscall" fn comparar_set_fixture_venue(this: usize, marks: u32, partida: usize, estadio: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(IDX_SET_FIXTURE_VENUE);
    if partida == 0 {
        return o(this, marks, partida, estadio);
    }
    let Some((r, m)) = stadium_choice_regions(this, Some((partida, 0x44))) else {
        // simplification: the reserve may allocate the list; original only
        return o(this, marks, partida, estadio);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&r[..m], || o(this, marks, partida, estadio), || crate::aleatorio::sem_contar(|| set_fixture_venue(this, marks, partida, estadio)));
    PLACAR.registrar(igual, || format!("set_fixture_venue(marcas {:x}, comp {:x}): {:x?} x {:x?}", marks, read_u32_as_usize(partida), mo, mr));
}

/// Comparer for `calculate_match_attendance` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado (a cópia sem contar os sorteios) pelo estado do sorteador, pela saída, pelos lugares do estádio (que
/// ela pode subir para 1.500) e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_calculate_match_attendance(this: usize, jogo: usize, out: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32>(IDX_CALCULATE_MATCH_ATTENDANCE);
    if jogo == 0 {
        return o(this, jogo, out);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !out.is_null() {
        reg.push((out as usize, 12));
    }
    if read_u32_as_usize(jogo + 0x18) != 0 {
        reg.push((read_u32_as_usize(jogo + 0x18) + 0x3c, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, out), || crate::aleatorio::sem_contar(|| calculate_match_attendance(this, jogo, out)));
    PLACAR.registrar(igual && r == n, || format!("calculate_match_attendance(comp {:x}, tipo {}): original {} rust {} {:x?} x {:x?}", read_u32_as_usize(jogo), read_u8(jogo + 0x3a), r, n, mo, mr));
    r
}

/// Comparer for `score_venue_for_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_score_venue_for_fixture(this: usize, est: usize, marks: u32, home: usize, away: usize, jogo: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32>(IDX_SCORE_VENUE_FOR_FIXTURE)(this, est, marks, home, away, jogo)
}

/// Comparer for `get_travel_distance_factor` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (os bits do double).
pub(crate) unsafe extern "thiscall" fn comparar_get_travel_distance_factor(this: usize, jogo: usize, cid_est: usize, cid_club: usize, base: f64) -> f64 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64>(IDX_GET_TRAVEL_DISTANCE_FACTOR)(this, jogo, cid_est, cid_club, base);
    let n = get_travel_distance_factor(this, jogo, cid_est, cid_club, base);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("get_travel_distance_factor(jogo {:x}, base {}) {} x {}", jogo, base, r, n));
    r
}

/// Comparer for `stadium_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: no dia 0 do ano (refaz a lista dos estádios, aloca) só a original. Nos outros, lado a lado pelo cabeçalho
/// (0x1c bytes), pelas reservas, por [0xa835a4], pelo estádio nacional da Inglaterra, pela casa da seleção e pelos
/// lugares do estádio fechado.
pub(crate) unsafe extern "thiscall" fn comparar_stadium_daily_update(this: usize, today: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_STADIUM_DAILY_UPDATE);
    if read_u16(today) == 0 {
        return o(this, today);
    }
    let mut regions = [(0usize, 0usize); 6];
    let mut m = 0;
    regions[m] = (this, 0x1c);
    m += 1;
    let n = read_usize(this + 4) as i32;
    if read_usize(this) != 0 && n > 0 {
        regions[m] = (read_usize(this), n as usize * 12);
        m += 1;
    }
    regions[m] = (0xa835a4, 4);
    m += 1;
    let nacao = england_nation();
    regions[m] = (nacao + nation::NATIONAL_STADIUM, 4);
    m += 1;
    let s = nation_national_team(nacao);
    if s != 0 {
        regions[m] = (s + 0x69, 4);
        m += 1;
    }
    if read_usize(G_STADIUM_STASHED_VENUE_ID) as i32 != -1 {
        regions[m] = (stadium_by_id(read_usize(G_STADIUM_STASHED_VENUE_ID)) + 0x3c, 8);
        m += 1;
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions[..m], || o(this, today), || stadium_daily_update(this, today));
    PLACAR.registrar(igual, || format!("stadium_daily_update(dia {}, ano {}) {:x?} x {:x?}", read_u16(today) as i16, read_u16(today + 2), mo, mr));
}

/// Comparer for `add_booking` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: com lista e vaga (não aloca), lado a lado pelo cabeçalho (0xc bytes) e pelas reservas com a nova; senão
/// (aloca, realoca ou a caixa) só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_booking(this: usize, estadio: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_BOOKING);
    let n = read_usize(this + 4);
    if estadio == 0 || data == 0 || read_usize(this) == 0 || n == read_usize(this + 8) {
        return o(this, estadio, data);
    }
    let regions = [(this, 0xc), (read_usize(this), (n + 1) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(this, estadio, data), || add_booking(this, estadio, data));
    PLACAR.registrar(igual, || format!("add_booking(estadio {:x}, reservas {}) {:x?} x {:x?}", estadio, n, mo, mr));
}

/// Comparer for `is_stadium_free_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia (as duas só leem), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_is_stadium_free_on_date(this: usize, estadio: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_STADIUM_FREE_ON_DATE)(this, estadio, data);
    let n = is_stadium_free_on_date(this, estadio, data);
    PLACAR.registrar(r == n, || format!("is_stadium_free_on_date(estadio {:x}) original={} rust={}", estadio, r, n));
    r
}

/// Comparer for `stadium_compare_bookings` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_stadium_compare_bookings(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STADIUM_COMPARE_BOOKINGS)(a, b);
    let n = stadium_compare_bookings(a, b);
    PLACAR.registrar(r == n, || format!("stadium_compare_bookings({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `build_venue_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado à mão: a original no objeto de quem chama, a cópia num rascunho com os mesmos 0x20 bytes de antes e a
/// lista velha zerada (a original já a soltou); iguais os campos (fora o ponteiro +0xc) e as listas (a do rascunho é
/// solta). Sem o HostCountry (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_build_venue_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_VENUE_LIST);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32(G_HOST_COUNTRY) == 0 {
        return o(this);
    }
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x20);
    s[0xc..0x18].fill(0);
    o(this);
    let r = s.as_mut_ptr() as usize;
    build_venue_list(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x20);
    let campos = a[..0xc] == s[..0xc] && a[0x10..0x20] == s[0x10..0x20];
    let (la, lb) = (read_u32(this + 0xc) as usize, read_u32(r + 0xc) as usize);
    let n = (read_u32(this + 0x10) as i32).max(0) as usize * 16;
    let lists = if la != 0 && lb != 0 {
        core::slice::from_raw_parts(la as *const u8, n) == core::slice::from_raw_parts(lb as *const u8, n)
    } else {
        la == lb
    };
    if lb != 0 {
        soltar(lb);
    }
    PLACAR.registrar(campos && lists, || format!("build_venue_list: campos {} listas {} (contas {} e {})", campos, lists, read_u32(this + 0x10) as i32, read_u32(r + 0x10) as i32));
}

/// Comparer for `stadium_compare_venues_by_capacity` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_stadium_compare_venues_by_capacity(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STADIUM_COMPARE_VENUES_BY_CAPACITY)(a, b);
    let n = stadium_compare_venues_by_capacity(a, b);
    PLACAR.registrar(r == n, || format!("stadium_compare_venues_by_capacity({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
