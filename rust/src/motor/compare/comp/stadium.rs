use super::*;

// The comparers of the routines in ../comp/stadium.rs.
// [pt] Os comparadores das rotinas de ../comp/stadium.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `find_available_venue`.
///
/// [pt] Lado a lado (a cópia sem contar os sorteios) pelas regiões de `stadium_choice_regions` e pela devolução; com reserva que pode alocar, só a original.
pub(crate) unsafe extern "thiscall" fn compare_find_available_venue(this: usize, marks: u32, count: usize, nation: usize, data: usize, home: usize, away: usize, fixture: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize>(IDX_FIND_AVAILABLE_VENUE);
    let Some((r, m)) = stadium_choice_regions(this, None) else {
        // simplification: the reserve may allocate the list; original only
        return o(this, marks, count, nation, data, home, away, fixture);
    };
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&r[..m], || o(this, marks, count, nation, data, home, away, fixture), || {
        crate::rng::without_counting(|| find_available_venue(this, marks, count, nation, data, home, away, fixture))
    });
    SCOREBOARD.record(igual && ro == rr, || format!("find_available_venue(marks {:x}, nation {:x}, count {:x}): original {:x} rust {:x} {:x?} x {:x?}", marks, nation, count, ro, rr, mo, mr));
    ro
}

/// Comparer for `set_fixture_venue`.
///
/// [pt] Lado a lado pela partida (0x44 bytes) e pelas regiões de `stadium_choice_regions`; partida nula ou reserva que pode alocar:
/// só a original.
pub(crate) unsafe extern "thiscall" fn compare_set_fixture_venue(this: usize, marks: u32, fixture: usize, stadium: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(IDX_SET_FIXTURE_VENUE);
    if fixture == 0 {
        return o(this, marks, fixture, stadium);
    }
    let Some((r, m)) = stadium_choice_regions(this, Some((fixture, 0x44))) else {
        // simplification: the reserve may allocate the list; original only
        return o(this, marks, fixture, stadium);
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&r[..m], || o(this, marks, fixture, stadium), || crate::rng::without_counting(|| set_fixture_venue(this, marks, fixture, stadium)));
    SCOREBOARD.record(igual, || format!("set_fixture_venue(marks {:x}, comp {:x}): {:x?} x {:x?}", marks, read_usize(fixture), mo, mr));
}

/// Comparer for `calculate_match_attendance`.
///
/// [pt] Lado a lado (a cópia sem contar os sorteios) pelo estado do sorteador, pela saída, pelos lugares do estádio (que
/// ela pode subir para 1.500) e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_calculate_match_attendance(this: usize, game: usize, out: *mut i32) -> i32 {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32>(IDX_CALCULATE_MATCH_ATTENDANCE);
    if game.is_null() {
        return o(this, game.addr(), out);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if !out.is_null() {
        reg.push((out as usize, 12));
    }
    if game.stadium() != 0 {
        reg.push((game.stadium() + stadium::CAPACITY, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, game.addr(), out), || crate::rng::without_counting(|| calculate_match_attendance(this, game.addr(), out)));
    SCOREBOARD.record(igual && r == n, || format!("calculate_match_attendance(comp {:x}, kind {}): original {} rust {} {:x?} x {:x?}", read_usize(game.addr()), game.category(), r, n, mo, mr));
    r
}

/// Comparer for `score_venue_for_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_score_venue_for_fixture(this: usize, est: usize, marks: u32, home: usize, away: usize, game: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32>(IDX_SCORE_VENUE_FOR_FIXTURE)(this, est, marks, home, away, game)
}

/// Comparer for `get_travel_distance_factor`.
///
/// [pt] Comparador: pelo valor (os bits do double).
pub(crate) unsafe extern "thiscall" fn compare_get_travel_distance_factor(this: usize, game: usize, cid_est: usize, cid_club: usize, base: f64) -> f64 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64>(IDX_GET_TRAVEL_DISTANCE_FACTOR)(this, game, cid_est, cid_club, base);
    let n = get_travel_distance_factor(this, game, cid_est, cid_club, base);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("get_travel_distance_factor(game {:x}, base {}) {} x {}", game, base, r, n));
    r
}

/// Comparer for `stadium_daily_update`.
///
/// On day 0 of the year (it rebuilds the stadium list, which allocates) only the original routine runs.
///
/// On the other days, side by side on the header (0x1c bytes), the reservations, [0xa835a4], England's national
/// stadium, the national team's home and the seats of the closed stadium.
///
/// [pt] Comparador: no dia 0 do ano (refaz a lista dos estádios, aloca) só a original. Nos outros, lado a lado pelo cabeçalho
/// (0x1c bytes), pelas reservas, por [0xa835a4], pelo estádio nacional da Inglaterra, pela casa da seleção e pelos
/// lugares do estádio fechado.
pub(crate) unsafe extern "thiscall" fn compare_stadium_daily_update(this: usize, today: usize) {
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
    regions[m] = (G_STADIUM_STASHED_VENUE_ID, 4);
    m += 1;
    let nation = Nation(england_nation());
    regions[m] = (nation.addr() + nation::NATIONAL_STADIUM, 4);
    m += 1;
    let s = Club(nation_national_team(nation.addr()));
    if !s.is_null() {
        regions[m] = (s.addr() + club::STADIUM, 4);
        m += 1;
    }
    if read_usize(G_STADIUM_STASHED_VENUE_ID) as i32 != -1 {
        regions[m] = (stadium_by_id(read_usize(G_STADIUM_STASHED_VENUE_ID)) + stadium::CAPACITY, 8);
        m += 1;
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions[..m], || o(this, today), || stadium_daily_update(this, today));
    SCOREBOARD.record(igual, || format!("stadium_daily_update(day {}, year {}) {:x?} x {:x?}", read_u16(today) as i16, read_u16(today + 2), mo, mr));
}

/// Comparer for `add_booking`.
///
/// [pt] Comparador: com lista e vaga (não aloca), lado a lado pelo cabeçalho (0xc bytes) e pelas reservas com a nova; senão
/// (aloca, realoca ou a caixa) só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_booking(this: usize, stadium: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_BOOKING);
    let n = read_usize(this + 4);
    if stadium == 0 || data == 0 || read_usize(this) == 0 || n == read_usize(this + 8) {
        return o(this, stadium, data);
    }
    let regions = [(this, 0xc), (read_usize(this), (n + 1) * 12)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, stadium, data), || add_booking(this, stadium, data));
    SCOREBOARD.record(igual, || format!("add_booking(stadium {:x}, reservas {}) {:x?} x {:x?}", stadium, n, mo, mr));
}

/// Comparer for `is_stadium_free_on_date`.
///
/// [pt] Comparador: a original e a cópia (as duas só leem), pelo valor.
pub(crate) unsafe extern "thiscall" fn compare_is_stadium_free_on_date(this: usize, stadium: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_STADIUM_FREE_ON_DATE)(this, stadium, data);
    let n = is_stadium_free_on_date(this, stadium, data);
    SCOREBOARD.record(r == n, || format!("is_stadium_free_on_date(stadium {:x}) original={} rust={}", stadium, r, n));
    r
}

/// Comparer for `stadium_compare_bookings`.
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn compare_stadium_compare_bookings(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STADIUM_COMPARE_BOOKINGS)(a, b);
    let n = stadium_compare_bookings(a, b);
    SCOREBOARD.record(r == n, || format!("stadium_compare_bookings({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `build_venue_list`.
///
/// Side by side by hand: the original routine on the caller's object, the copy on a scratch object with the same 0x20
/// bytes as before and the old list zeroed (the original has already released it); the fields (except the pointer +0xc)
/// and the lists must be equal (the scratch object's list is released).
///
/// Without the HostCountry (the error box): only the original.
///
/// [pt] Lado a lado à mão: a original no objeto de quem chama, a cópia num rascunho com os mesmos 0x20 bytes de antes e a
/// lista velha zerada (a original já a soltou); iguais os campos (fora o ponteiro +0xc) e as listas (a do rascunho é
/// solta). Sem o HostCountry (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn compare_build_venue_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_VENUE_LIST);
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
    let fields = a[..0xc] == s[..0xc] && a[0x10..0x20] == s[0x10..0x20];
    let (la, lb) = (read_u32(this + stadium_obj::VENUES) as usize, read_u32(r + stadium_obj::VENUES) as usize);
    let n = (read_u32(this + stadium_obj::VENUE_COUNT) as i32).max(0) as usize * 16;
    let lists = if la != 0 && lb != 0 {
        core::slice::from_raw_parts(la as *const u8, n) == core::slice::from_raw_parts(lb as *const u8, n)
    } else {
        la == lb
    };
    if lb != 0 {
        exe::crt_free(lb);
    }
    SCOREBOARD.record(fields && lists, || format!("build_venue_list: fields {} lists {} (contas {} e {})", fields, lists, read_u32(this + stadium_obj::VENUE_COUNT) as i32, read_u32(r + stadium_obj::VENUE_COUNT) as i32));
}

/// Comparer for `stadium_compare_venues_by_capacity`.
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn compare_stadium_compare_venues_by_capacity(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STADIUM_COMPARE_VENUES_BY_CAPACITY)(a, b);
    let n = stadium_compare_venues_by_capacity(a, b);
    SCOREBOARD.record(r == n, || format!("stadium_compare_venues_by_capacity({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
