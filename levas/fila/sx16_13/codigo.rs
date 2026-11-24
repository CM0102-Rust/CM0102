// --- sx16_13: physio.cpp (part 1): the static object and the physio's judgements of a player ---

/// Registers the empty at-exit destructor of physio.cpp's static object (`atexit(0x7a1a80)`).
/// Original: `physio_static_init` (0x7a1a70, physio.cpp, cdecl; a CRT start-up initializer).
pub(crate) unsafe extern "C" fn physio_static_init() {
    exe::crt_atexit(0x7a1a80usize); // physio_static_object_destructor
}

/// Comparer for `physio_static_init`: it registers a destructor once, so only the original runs.
pub(crate) unsafe extern "C" fn compare_physio_static_init() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// The empty at-exit destructor of physio.cpp's static object: nothing.
/// Original: `physio_static_object_destructor` (0x7a1a80, physio.cpp, cdecl).
pub(crate) unsafe extern "C" fn physio_static_object_destructor() {}

/// Comparer for `physio_static_object_destructor`: the original (nothing to compare).
pub(crate) unsafe extern "C" fn compare_physio_static_object_destructor() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// The career stage of a player for the physio's report, from his age (`staff::AGE`, a signed byte) against the peak
/// age of his main position: 33 for a 20-rated goalkeeper, 31 for a defender or defensive midfielder, 29 for a
/// midfielder or attacking midfielder, otherwise 28. Past 1.1 × the peak and older than 32: 4; under 18: 1; 18..23: 2;
/// otherwise 8. Without a person or a player sheet: 0.
/// Original: `physio_get_age_stage` (0x7a25c0, physio.cpp, thiscall(physio, person), ret 4; `this` unused).
pub(crate) unsafe extern "thiscall" fn physio_get_age_stage(_this: usize, person: usize) -> u8 {
    if person == 0 {
        return 0;
    }
    let sheet = Staff(person).player();
    if sheet.is_null() {
        return 0;
    }
    let age = read_i8(person + staff::AGE);
    let rated = |field: usize| read_i8(sheet.addr() + field) == 0x14;
    let peak: i8 = if rated(player_sheet::GOALKEEPER) {
        0x21
    } else if rated(player_sheet::DEFENDER) || rated(player_sheet::DEFENSIVE_MIDFIELDER) {
        0x1f
    } else if rated(player_sheet::MIDFIELDER) || rated(player_sheet::ATTACKING_MIDFIELDER) {
        0x1d
    } else {
        0x1c
    };
    if peak as f64 * read_f64(rdata::F64_1_1) < age as f64 && age > 0x20 {
        covered!("sx16_13 veteran stage");
        4
    } else if age < 0x12 {
        1
    } else if age >= 0x18 {
        8
    } else {
        2
    }
}

/// Comparer for `physio_get_age_stage`: the same stage.
pub(crate) unsafe extern "thiscall" fn compare_physio_get_age_stage(this: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, person);
    let n = physio_get_age_stage(this, person);
    SCOREBOARD.record(r == n, || format!("physio_get_age_stage: original={} rust={}", r, n));
    r
}

/// The days a physio needs with a player before he can judge him: 0 for a top player (current ability above 160, or
/// potential above 160 with current above 100); otherwise (the two staff ids summed, modulo 35) + 30 + 25, 40 or 55 by
/// the physio's own current ability (above 140, above 80, else). Without the physio or his non-player sheet (box line
/// 0x355), or without the player or his player sheet (0x35a): the physio.cpp "Error" box and 180.
/// Original: `physio_get_days_to_judge` (0x7a2900, physio.cpp, thiscall(physio, physio staff, player), ret 8; `this`
/// unused).
pub(crate) unsafe extern "thiscall" fn physio_get_days_to_judge(_this: usize, physio: usize, player: usize) -> i32 {
    const SOURCE: usize = 0xa6b1e0; // physio.cpp
    if physio == 0 || Staff(physio).non_player().is_null() {
        show_error_box(SOURCE, 0x355);
        return 0xb4;
    }
    if player == 0 || Staff(player).player().is_null() {
        show_error_box(SOURCE, 0x35a);
        return 0xb4;
    }
    let sheet = Staff(player).player().addr();
    let current = read_i16(sheet + player_sheet::CURRENT_ABILITY);
    if current > 0xa0 || (read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0xa0 && current > 0x64) {
        covered!("sx16_13 top player judged at once");
        return 0;
    }
    let skill = Staff(physio).non_player().current_ability();
    let base = if skill > 0x8c { 0x19 } else if skill > 0x50 { 0x28 } else { 0x37 };
    read_i32(physio).wrapping_add(read_i32(player)) % 0x23 + base + 0x1e
}

/// Comparer for `physio_get_days_to_judge`: the same days (the error boxes only through the original).
pub(crate) unsafe extern "thiscall" fn compare_physio_get_days_to_judge(this: usize, physio: usize, player: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I3%%)(this, physio, player);
    if physio == 0 || player == 0 || Staff(physio).non_player().is_null() || Staff(player).player().is_null() {
        return r;
    }
    let n = physio_get_days_to_judge(this, physio, player);
    SCOREBOARD.record(r == n, || format!("physio_get_days_to_judge: original={} rust={}", r, n));
    r
}
