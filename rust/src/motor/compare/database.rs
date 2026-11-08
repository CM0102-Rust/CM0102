use super::*;

// The comparers of the routines in ../database.rs.
// [pt] Os comparadores das rotinas de ../database.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `favourite_of`.
pub(crate) unsafe extern "C" fn compare_favourite_of(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_FAVOURITE_OF)(a, b);
    let n = favourite_of(a, b);
    SCOREBOARD.record(r == n, || format!("favourite_of original={} rust={}", r, n));
    r
}

/// Comparer for `rejected_by`.
pub(crate) unsafe extern "C" fn compare_rejected_by(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_REJECTED_BY)(a, b);
    let n = rejected_by(a, b);
    SCOREBOARD.record(r == n, || format!("rejected_by original={} rust={}", r, n));
    r
}

/// Comparer for `database_player_initialisation`.
///
/// Comparer of 0x535a80: original routine and copy side by side over the database it writes (whole persons, players and
/// non-players, the 79-byte-per-person table at [0xae2d0c]), [0x9b59b4], the flags 0x9b59b8/9 and the four global
/// template vectors, the text at 0xde1f64, the stored date of 0x54cab0 and the random generator's state (the copy
/// without counting draws).
///
/// Without persons (error box and end): only the original.
///
/// [pt] Comparador de 0x535a80: original e cópia lado a lado sobre o banco que ela escreve (pessoas, jogadores e não
/// jogadores inteiros, a tabela de 79 bytes por pessoa em [0xae2d0c]), [0x9b59b4], as bandeiras 0x9b59b8/9 e os quatro
/// vetores globais dos moldes, o texto de 0xde1f64, a data guardada da 0x54cab0 e o estado do sorteador (a cópia sem
/// contar sorteios). Sem pessoas (caixa e fim): só a original.
pub(crate) unsafe extern "C" fn compare_database_player_initialisation(data: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_DATABASE_PLAYER_INITIALISATION);
    let gi = |a: usize| (*(a as *const i32)).max(0) as usize;
    let rd = |a: usize| *(a as *const usize);
    if gi(0xae2364) == 0 || rd(G_STAFF_TABLE) == 0 {
        return o(data);
    }
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.extend_from_slice(&[(0x9b59b4, 4), (0x9b59b8, 2), (0xae2044, 0xbd), (0xae2114, 0xbd), (0xae18bc, 0xbd),
        (0xae2238, 0xbd), (0xde1f64, 0x100), (0xae24aa, 1), (G_SUB_WEEKS_RESULT_DATE, 8)]);
    regions.push((rd(G_STAFF_TABLE), gi(0xae2364) * 110));
    regions.push((rd(G_STAFF_STATE_TABLE), gi(0xae2364) * 0x4f));
    if rd(G_PLAYER_DATA_TABLE) != 0 {
        regions.push((rd(G_PLAYER_DATA_TABLE), gi(0xae236c) * 70));
    }
    if rd(G_NON_PLAYER_TABLE) != 0 {
        regions.push((rd(G_NON_PLAYER_TABLE), gi(0xae2368) * 68));
    }
    let (_, _, igual, mut mo, mut mr) = crate::comparison::side_by_side(&regions, || o(data),
        || crate::rng::without_counting(|| database_player_initialisation(data)));
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])));
    SCOREBOARD.record(igual, || match onde {
        Some((a, x, y)) => format!("database_player_initialisation: diverge em {:x} original={:02x} rust={:02x}", a, x, y),
        None => "database_player_initialisation: estado do sorteador diferente".to_string(),
    });
    // simplification: tens of MB per snapshot; they do not go back to the leftovers
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_format_name_from_strings`.
pub(crate) unsafe extern "C" fn compare_database_format_name_from_strings(name: usize, sobrenome: usize, common: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_STRINGS);
    if text == 0 || (size as u16 as i16) < 0x69 || !(1..=7).contains(&(form as u8)) {
        return o(name, sobrenome, common, text, size, form); // message box: original only
    }
    // the produced text (up to the given size, at most 128 bytes)
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(name, sobrenome, common, text, size, form),
        || database_format_name_from_strings(name, sobrenome, common, text, size, form));
    SCOREBOARD.record(igual && r == x, || format!("database_format_name_from_strings(forma {}) original={} rust={} {:x?} x {:x?}", form as u8, r, x, mo, mr));
    r
}

/// Comparer for `database_get_job_name`.
pub(crate) unsafe extern "C" fn compare_database_get_job_name(job: u32, curto: u32, maiuscula: u32, gender: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, usize) -> usize>(IDX_DATABASE_GET_JOB_NAME);
    // the text [0xae19b4] and the gender byte, if any
    let regions = [(G_JOB_NAME_BUFFER, 100usize), (gender, 1)];
    let reg = if gender != 0 { &regions[..] } else { &regions[..1] };
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(reg, || o(job, curto, maiuscula, gender),
        || database_get_job_name(job, curto, maiuscula, gender));
    SCOREBOARD.record(igual && r == x, || format!("database_get_job_name({}, {}, {}) original={:x} rust={:x} {:x?} x {:x?}", job as u8 as i8, curto, maiuscula, r, x, mo, mr));
    r
}

/// Comparer for `database_get_language_name`.
pub(crate) unsafe extern "C" fn compare_database_get_language_name(language: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_DATABASE_GET_LANGUAGE_NAME);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae21d4, 100)], || o(language), || database_get_language_name(language));
    SCOREBOARD.record(igual && r == x, || format!("database_get_language_name({}) original={:x} rust={:x} {:x?} x {:x?}", language as u8 as i8, r, x, mo, mr));
    r
}

/// Comparer for `club_get_national_team_name`.
pub(crate) unsafe extern "C" fn compare_club_get_national_team_name(club: usize, variant: u32, text: usize, size: i32, form: u32, aspas: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(IDX_CLUB_GET_NATIONAL_TEAM_NAME);
    let f = form as u8 as i8;
    if size < 0x47 || ((1..=4).contains(&f) && club == 0) {
        return o(club, variant, text, size, form, aspas); // message box: original only
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(club, variant, text, size, form, aspas),
        || club_get_national_team_name(club, variant, text, size, form, aspas));
    SCOREBOARD.record(igual && r == x, || format!("club_get_national_team_name({:x}, {}, forma {}, {}) original={} rust={} {:x?} x {:x?}", club, variant, f, aspas, r, x, mo, mr));
    r
}

/// Comparer for `database_get_player_attribute_name`.
pub(crate) unsafe extern "C" fn compare_database_get_player_attribute_name(attribute: u32, text: usize, size: u32, abreviado: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_PLAYER_ATTRIBUTE_NAME);
    let x = attribute as u8 as i8;
    if !(1..=0x1f).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return o(attribute, text, size, abreviado); // message box: original only
    }
    // the first 100 bytes (the minimum accepted size): the names fit in them
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, 100)], || o(attribute, text, size, abreviado),
        || database_get_player_attribute_name(attribute, text, size, abreviado));
    SCOREBOARD.record(igual && r == n, || format!("database_get_player_attribute_name({}, {}) original={} rust={} {:x?} x {:x?}", x, abreviado, r, n, mo, mr));
    r
}

/// Comparer for `database_format_player_positions`.
pub(crate) unsafe extern "C" fn compare_database_format_player_positions(player: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_PLAYER_POSITIONS);
    if text == 0 || player == 0 || (size as u16 as i16) < 0x1e {
        return o(player, text, size, form); // message box: original only
    }
    // the text, the player’s ratings (the original may write +0x13) and the start of the global text [0xde1f64]
    let n = (size as u16 as usize).min(128);
    let reg = [(text, n), (player + player_sheet::GOALKEEPER, 12), (0xde1f64, 64)];
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(player, text, size, form),
        || database_format_player_positions(player, text, size, form));
    SCOREBOARD.record(igual && r == x, || format!("database_format_player_positions({:x}, forma {}) original={} rust={} {:x?} x {:x?}", player, form as u8, r, x, mo, mr));
    r
}

/// Comparer for `database_format_name_from_ids`.
pub(crate) unsafe extern "C" fn compare_database_format_name_from_ids(name: i32, sobrenome: i32, common: i32, text: usize, size: u32, form: u32, person: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_IDS);
    if text == 0 || (size as i16) < 0x69 {
        return o(name, sobrenome, common, text, size, form, person); // message box: original only
    }
    // the produced text (up to the given size, at most 128 bytes)
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(name, sobrenome, common, text, size, form, person),
        || database_format_name_from_ids(name, sobrenome, common, text, size, form, person));
    SCOREBOARD.record(igual && r == x, || format!("database_format_name_from_ids({}, {}, {}, forma {}, pessoa {}) original={} rust={} {:x?} x {:x?}", name, sobrenome, common, form, person, r, x, mo, mr));
    r
}

/// Comparer for `database_format_official_name`.
pub(crate) unsafe extern "C" fn compare_database_format_official_name(oficial: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_OFFICIAL_NAME);
    if oficial == 0 || text == 0 || (size as i16) < 0x69 {
        return o(oficial, text, size, form); // message box: original only
    }
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(oficial, text, size, form),
        || database_format_official_name(oficial, text, size, form));
    SCOREBOARD.record(igual && r == x, || format!("database_format_official_name({:x}, forma {}) original={} rust={} {:x?} x {:x?}", oficial, form as u8, r, x, mo, mr));
    r
}

/// Comparer for `position_mask_get_text`.
pub(crate) unsafe extern "C" fn compare_position_mask_get_text(mask: u32, text: usize, maiuscula: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32) -> u8>(IDX_POSITION_MASK_GET_TEXT);
    if text == 0 {
        return o(mask, text, maiuscula);
    }
    // no size: the translated names fit in 128
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, 128)], || o(mask, text, maiuscula),
        || position_mask_get_text(mask, text, maiuscula));
    SCOREBOARD.record(igual && r == x, || format!("position_mask_get_text({:x}, {}) original={} rust={} {:x?} x {:x?}", mask, maiuscula as u8, r, x, mo, mr));
    r
}

/// Comparer for `write_position_name`.
pub(crate) unsafe extern "C" fn compare_write_position_name(mask: u32, text: usize, size: i32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(IDX_WRITE_POSITION_NAME);
    if size < 0x1e {
        return o(mask, text, size); // message box: original only
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(mask, text, size),
        || write_position_name(mask, text, size));
    SCOREBOARD.record(igual && r == x, || format!("write_position_name({:x}) original={} rust={} {:x?} x {:x?}", mask, r, x, mo, mr));
    r
}

/// Comparer for `write_player_description`.
pub(crate) unsafe extern "C" fn compare_write_player_description(person: i32, position: u32, qualidades: u32, text: usize, size: i32, gender: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(IDX_WRITE_PLAYER_DESCRIPTION);
    if size < 0x1e {
        return o(person, position, qualidades, text, size, gender); // message box: original only
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, n)], || o(person, position, qualidades, text, size, gender),
        || write_player_description(person, position, qualidades, text, size, gender));
    SCOREBOARD.record(igual && r == x, || format!("write_player_description({}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, position, qualidades, r, x, mo, mr));
    r
}

/// Comparer for `staff_is_best_favoured_candidate`.
pub(crate) unsafe extern "C" fn compare_staff_is_best_favoured_candidate(person: usize, club: usize, candidate: usize) -> u8 {
    // the calls (contract, average, 0x8cca10) go through the match’s address: the random state goes back between the sides and the
    // the copy does not count, if any of them draws
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u8>(IDX_STAFF_IS_BEST_FAVOURED_CANDIDATE);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(person, club, candidate),
        || crate::rng::without_counting(|| staff_is_best_favoured_candidate(person, club, candidate)));
    SCOREBOARD.record(igual && r == x, || format!("staff_is_best_favoured_candidate({:x}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, candidate, r, x, mo, mr));
    r
}

/// Comparer for `staff_get_player_strength_flags`.
pub(crate) unsafe extern "C" fn compare_staff_get_player_strength_flags(person: usize, other: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_GET_PLAYER_STRENGTH_FLAGS);
    let (r, _, igual, d) = compare_with_rng_retry(|| o(person, other), || staff_get_player_strength_flags(person, other));
    SCOREBOARD.record(igual, || format!("staff_get_player_strength_flags({:x}, {:x}) {}", person, other, d));
    r
}

/// Comparer for `staff_get_player_weakness_flags`.
pub(crate) unsafe extern "C" fn compare_staff_get_player_weakness_flags(person: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_GET_PLAYER_WEAKNESS_FLAGS);
    let (r, _, igual, d) = compare_with_rng_retry(|| o(person, club), || staff_get_player_weakness_flags(person, club));
    SCOREBOARD.record(igual, || format!("staff_get_player_weakness_flags({:x}, {:x}) {}", person, club, d));
    r
}

/// Comparer for `database_count_club_physios`.
pub(crate) unsafe extern "C" fn compare_database_count_club_physios(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_PHYSIOS)(club);
    if club == 0 {
        return r; // message box: original only
    }
    let n = database_count_club_physios(club);
    SCOREBOARD.record(r == n, || format!("database_count_club_physios({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_is_b_team`.
pub(crate) unsafe extern "C" fn compare_club_is_b_team(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_B_TEAM)(club);
    if club == 0 {
        return r;
    }
    let n = club_is_b_team(club);
    SCOREBOARD.record(r == n, || format!("club_is_b_team({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_collect_non_playing_staff`.
pub(crate) unsafe extern "C" fn compare_club_collect_non_playing_staff(club: usize, out: *mut usize, max: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(IDX_CLUB_COLLECT_NON_PLAYING_STAFF);
    if club == 0 {
        return o(club, out, max);
    }
    let reg = [(out as usize, 4 * (max as i8).max(0) as usize)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(club, out, max), || club_collect_non_playing_staff(club, out, max));
    SCOREBOARD.record(igual && r == n, || format!("club_collect_non_playing_staff({:x}, {:x}, {}) original={} rust={} {:x?} x {:x?}", club, out as usize, max as i8, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_player_attribute_visible`.
pub(crate) unsafe extern "C" fn compare_staff_is_player_attribute_visible(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_IS_PLAYER_ATTRIBUTE_VISIBLE)(who_ve, person, attribute);
    let n = staff_is_player_attribute_visible(who_ve, person, attribute);
    SCOREBOARD.record(r == n, || format!("staff_is_player_attribute_visible({:x}, {:x}, {}) original={} rust={}", who_ve, person, attribute as u8 as i8, r, n));
    r
}

/// Comparer for `staff_get_player_attribute_for_viewer`.
pub(crate) unsafe extern "C" fn compare_staff_get_player_attribute_for_viewer(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE_FOR_VIEWER)(who_ve, person, attribute);
    let x = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&x) {
        return r; // message box: original only
    }
    let n = staff_get_player_attribute_for_viewer(who_ve, person, attribute);
    SCOREBOARD.record(r == n, || format!("staff_get_player_attribute_for_viewer({:x}, {:x}, {}) original={} rust={}", who_ve, person, x, r, n));
    r
}

/// Comparer for `database_get_non_player_attribute_name`.
pub(crate) unsafe extern "C" fn compare_database_get_non_player_attribute_name(attribute: u32, text: usize, size: u32, abreviado: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_NON_PLAYER_ATTRIBUTE_NAME);
    let x = attribute as u8 as i8;
    if !(1..=12).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return o(attribute, text, size, abreviado);
    }
    // the first 100 bytes (the minimum accepted size): the names fit in them
    let reg = [(text, 100)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(attribute, text, size, abreviado),
        || database_get_non_player_attribute_name(attribute, text, size, abreviado));
    SCOREBOARD.record(igual && r == n, || format!("database_get_non_player_attribute_name({}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}", x, text, size as u16 as i16, abreviado, r, n, mo, mr));
    r
}

/// Comparer for `database_compare_staff_by_surname`.
pub(crate) unsafe extern "C" fn compare_database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_STAFF_BY_SURNAME)(a, b);
    let n = database_compare_staff_by_surname(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_staff_by_surname({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_reputation`.
pub(crate) unsafe extern "C" fn compare_database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_REPUTATION)(a, b);
    let n = database_compare_clubs_by_reputation(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_clubs_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_club_nation_is_eec`.
pub(crate) unsafe extern "C" fn compare_database_club_nation_is_eec(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_NATION_IS_EEC)(club);
    let n = database_club_nation_is_eec(club);
    SCOREBOARD.record(r == n, || format!("database_club_nation_is_eec({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `get_regions_sorted_by_name`.
pub(crate) unsafe extern "C" fn compare_get_regions_sorted_by_name(language: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_GET_REGIONS_SORTED_BY_NAME);
    let reg = [(G_REGIONS_SORTED_BY_NAME, 14), (0xae232b, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(language), || get_regions_sorted_by_name(language));
    SCOREBOARD.record(igual && r == n, || format!("get_regions_sorted_by_name({}) original={:x} rust={:x} {:x?} x {:x?}", language as u8, r, n, mo, mr));
    r
}

/// Comparer for `compare_region_names`.
pub(crate) unsafe extern "C" fn compare_compare_region_names(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_REGION_NAMES)(a, b);
    let n = compare_region_names(a, b);
    SCOREBOARD.record(r == n, || format!("compare_region_names({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `staff_get_opinion_link_staff`.
pub(crate) unsafe extern "C" fn compare_staff_get_opinion_link_staff(person: usize, target: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_STAFF_GET_OPINION_LINK_STAFF)(person, target);
    let n = staff_get_opinion_link_staff(person, target);
    SCOREBOARD.record(r == n, || format!("staff_get_opinion_link_staff({:x}, {:x}) original={:x} rust={:x}", person, target, r, n));
    r
}

/// Comparer for `club_add_favoured_staff`.
pub(crate) unsafe extern "C" fn compare_club_add_favoured_staff(club: usize, person: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_CLUB_ADD_FAVOURED_STAFF);
    let reg = preference_regions(if club != 0 { club + club::FAV_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(club, person, force),
        || crate::rng::without_counting(|| club_add_favoured_staff(club, person, force)));
    SCOREBOARD.record(igual, || format!("club_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", club, person, force, mo, mr));
}

/// Comparer for `club_add_disliked_staff`.
pub(crate) unsafe extern "C" fn compare_club_add_disliked_staff(club: usize, person: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_CLUB_ADD_DISLIKED_STAFF);
    let reg = preference_regions(if club != 0 { club + club::FAV_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(club, person, force),
        || crate::rng::without_counting(|| club_add_disliked_staff(club, person, force)));
    SCOREBOARD.record(igual, || format!("club_add_disliked_staff({:x}, {:x}, {}) {:x?} x {:x?}", club, person, force, mo, mr));
}

/// Comparer for `staff_add_favoured_staff`.
pub(crate) unsafe extern "C" fn compare_staff_add_favoured_staff(person: usize, other: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_FAVOURED_STAFF);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, other, force),
        || crate::rng::without_counting(|| staff_add_favoured_staff(person, other, force)));
    SCOREBOARD.record(igual, || format!("staff_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", person, other, force, mo, mr));
}

/// Comparer for `staff_add_disliked_club`.
pub(crate) unsafe extern "C" fn compare_staff_add_disliked_club(person: usize, club: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_DISLIKED_CLUB);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + 4 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, club, force),
        || crate::rng::without_counting(|| staff_add_disliked_club(person, club, force)));
    SCOREBOARD.record(igual, || format!("staff_add_disliked_club({:x}, {:x}, {}) {:x?} x {:x?}", person, club, force, mo, mr));
}

/// Comparer for `staff_fade_staff_relationships`.
pub(crate) unsafe extern "C" fn compare_staff_fade_staff_relationships(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_FADE_STAFF_RELATIONSHIPS);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person),
        || crate::rng::without_counting(|| staff_fade_staff_relationships(person)));
    SCOREBOARD.record(igual, || format!("staff_fade_staff_relationships({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `staff_get_disliked_staff`.
pub(crate) unsafe extern "C" fn compare_staff_get_disliked_staff(person: usize, out: usize, n: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> usize>(IDX_STAFF_GET_DISLIKED_STAFF);
    let mut reg = vec![(n, 1)];
    if out != 0 {
        reg.push((out, 12));
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, out, n), || staff_get_disliked_staff(person, out, n));
    SCOREBOARD.record(igual && r == m, || format!("staff_get_disliked_staff({:x}, {:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", person, out, n, r, m, mo, mr));
    r
}

/// Comparer for `staff_get_natural_position_mask`.
pub(crate) unsafe extern "C" fn compare_staff_get_natural_position_mask(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_STAFF_GET_NATURAL_POSITION_MASK)(person);
    let n = staff_get_natural_position_mask(person);
    SCOREBOARD.record(r == n, || format!("staff_get_natural_position_mask({:x}) original={:x} rust={:x}", person, r, n));
    r
}

/// Comparer for `club_count_squad_with_position_mask`.
pub(crate) unsafe extern "C" fn compare_club_count_squad_with_position_mask(club: usize, mask: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_CLUB_COUNT_SQUAD_WITH_POSITION_MASK)(club, mask);
    let n = club_count_squad_with_position_mask(club, mask);
    SCOREBOARD.record(r == n, || format!("club_count_squad_with_position_mask({:x}, {:x}) original={} rust={}", club, mask, r, n));
    r
}

/// Comparer for `staff_pick_favoured_team_mate`.
pub(crate) unsafe extern "C" fn compare_staff_pick_favoured_team_mate(person: usize, club: usize) -> usize {
    // the calls (contract, average, 0x8cca10) go through the match’s address: the random state goes back between the sides and the
    // the copy does not count, if any of them draws
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_PICK_FAVOURED_TEAM_MATE);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(person, club),
        || crate::rng::without_counting(|| staff_pick_favoured_team_mate(person, club)));
    SCOREBOARD.record(igual && r == n, || format!("staff_pick_favoured_team_mate({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `club_nation_is_not_argentina`.
pub(crate) unsafe extern "C" fn compare_club_nation_is_not_argentina(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_NATION_IS_NOT_ARGENTINA)(club);
    let n = club_nation_is_not_argentina(club);
    SCOREBOARD.record(r == n, || format!("club_nation_is_not_argentina({:x}) original={:x} rust={:x}", club, r, n));
    r
}

/// Comparer for `club_nation_not_argentina_france_croatia`.
pub(crate) unsafe extern "C" fn compare_club_nation_not_argentina_france_croatia(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_NATION_NOT_ARGENTINA_FRANCE_CROATIA)(club);
    let n = club_nation_not_argentina_france_croatia(club);
    SCOREBOARD.record(r == n, || format!("club_nation_not_argentina_france_croatia({:x}) original={:x} rust={:x}", club, r, n));
    r
}

/// Comparer for `database_is_melanesian_nation`.
pub(crate) unsafe extern "C" fn compare_database_is_melanesian_nation(nation: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_MELANESIAN_NATION)(nation);
    let n = database_is_melanesian_nation(nation);
    SCOREBOARD.record(r == n, || format!("database_is_melanesian_nation({:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `database_is_polynesian_nation`.
pub(crate) unsafe extern "C" fn compare_database_is_polynesian_nation(nation: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_POLYNESIAN_NATION)(nation);
    let n = database_is_polynesian_nation(nation);
    SCOREBOARD.record(r == n, || format!("database_is_polynesian_nation({:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `database_get_coaching_job_name`.
pub(crate) unsafe extern "C" fn compare_database_get_coaching_job_name(job: u32, gender: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, usize) -> usize>(IDX_DATABASE_GET_COACHING_JOB_NAME);
    // the text (the translated names fit in 64) and the gender byte
    let com = [(G_COACHING_JOB_NAME_BUFFER, 64usize), (gender, 1)];
    let regions = if gender != 0 { &com[..] } else { &com[..1] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regions, || o(job, gender), || database_get_coaching_job_name(job, gender));
    SCOREBOARD.record(igual && r == n, || format!("database_get_coaching_job_name({}) original={:x} rust={:x} {:x?} x {:x?}", job as i8, r, n, mo, mr));
    r
}

/// Comparer for `database_format_years_between_dates`.
pub(crate) unsafe extern "C" fn compare_database_format_years_between_dates(end: usize, start: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(IDX_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
    if text == 0 || size < 5 || end == 0 || start == 0 {
        return o(end, start, text, size); // message box: original only
    }
    // ”%d” of a word: up to 6 bytes including the zero
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(text, 6)], || o(end, start, text, size),
        || database_format_years_between_dates(end, start, text, size));
    SCOREBOARD.record(igual && r == n, || format!("database_format_years_between_dates({:x}, {:x}) original={} rust={} {:x?} x {:x?}", end, start, r, n, mo, mr));
    r
}

/// Comparer for `database_compare_clubs_by_short_name`.
pub(crate) unsafe extern "C" fn compare_database_compare_clubs_by_short_name(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_SHORT_NAME)(a, b);
    let n = database_compare_clubs_by_short_name(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_clubs_by_short_name({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_staff_by_full_name`.
pub(crate) unsafe extern "C" fn compare_database_compare_staff_by_full_name(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_STAFF_BY_FULL_NAME)(a, b);
    let n = database_compare_staff_by_full_name(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_staff_by_full_name({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_staff_matches_names`.
pub(crate) unsafe extern "C" fn compare_database_staff_matches_names(person: usize, name: u32, sobrenome: u32, common: u32, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(IDX_DATABASE_STAFF_MATCHES_NAMES);
    if person == 0 {
        return o(person, name, sobrenome, common, data); // message box: original only
    }
    let r = o(person, name, sobrenome, common, data);
    let n = database_staff_matches_names(person, name, sobrenome, common, data);
    SCOREBOARD.record(r == n, || format!("database_staff_matches_names({:x}, {:x}, {:x}, {:x}, {:x}) original={} rust={}", person, name, sobrenome, common, data, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_latitude`.
pub(crate) unsafe extern "C" fn compare_database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LATITUDE);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // message box: original only
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_latitude(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_clubs_by_latitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_longitude`.
pub(crate) unsafe extern "C" fn compare_database_compare_clubs_by_longitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LONGITUDE);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // message box: original only
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_longitude(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_clubs_by_longitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `club_has_enough_defenders`.
pub(crate) unsafe extern "C" fn compare_club_has_enough_defenders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_DEFENDERS)(club);
    let n = club_has_enough_defenders(club);
    SCOREBOARD.record(r == n, || format!("club_has_enough_defenders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_enough_midfielders`.
pub(crate) unsafe extern "C" fn compare_club_has_enough_midfielders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_MIDFIELDERS)(club);
    let n = club_has_enough_midfielders(club);
    SCOREBOARD.record(r == n, || format!("club_has_enough_midfielders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_enough_strikers`.
pub(crate) unsafe extern "C" fn compare_club_has_enough_strikers(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_STRIKERS)(club);
    let n = club_has_enough_strikers(club);
    SCOREBOARD.record(r == n, || format!("club_has_enough_strikers({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_enough_hard_workers`.
pub(crate) unsafe extern "C" fn compare_club_has_enough_hard_workers(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_HARD_WORKERS)(club);
    let n = club_has_enough_hard_workers(club);
    SCOREBOARD.record(r == n, || format!("club_has_enough_hard_workers({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_fit_goalkeeper`.
pub(crate) unsafe extern "C" fn compare_club_has_fit_goalkeeper(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_FIT_GOALKEEPER)(club);
    let n = club_has_fit_goalkeeper(club);
    SCOREBOARD.record(r == n, || format!("club_has_fit_goalkeeper({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_ball_winning_midfielders`.
pub(crate) unsafe extern "C" fn compare_club_has_ball_winning_midfielders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_BALL_WINNING_MIDFIELDERS)(club);
    let n = club_has_ball_winning_midfielders(club);
    SCOREBOARD.record(r == n, || format!("club_has_ball_winning_midfielders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_lacks_aerial_strikers`.
pub(crate) unsafe extern "C" fn compare_club_lacks_aerial_strikers(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_AERIAL_STRIKERS)(club);
    let n = club_lacks_aerial_strikers(club);
    SCOREBOARD.record(r == n, || format!("club_lacks_aerial_strikers({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_lacks_aerial_defenders`.
pub(crate) unsafe extern "C" fn compare_club_lacks_aerial_defenders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_AERIAL_DEFENDERS)(club);
    let n = club_lacks_aerial_defenders(club);
    SCOREBOARD.record(r == n, || format!("club_lacks_aerial_defenders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_lacks_fast_defenders`.
pub(crate) unsafe extern "C" fn compare_club_lacks_fast_defenders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_FAST_DEFENDERS)(club);
    let n = club_lacks_fast_defenders(club);
    SCOREBOARD.record(r == n, || format!("club_lacks_fast_defenders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_fast_striker`.
pub(crate) unsafe extern "C" fn compare_club_has_fast_striker(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_FAST_STRIKER)(club);
    let n = club_has_fast_striker(club);
    SCOREBOARD.record(r == n, || format!("club_has_fast_striker({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_has_two_creative_midfielders`.
pub(crate) unsafe extern "C" fn compare_club_has_two_creative_midfielders(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_TWO_CREATIVE_MIDFIELDERS)(club);
    let n = club_has_two_creative_midfielders(club);
    SCOREBOARD.record(r == n, || format!("club_has_two_creative_midfielders({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_squad_strength_band`.
pub(crate) unsafe extern "C" fn compare_club_squad_strength_band(club: usize, other: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_CLUB_SQUAD_STRENGTH_BAND)(club, other);
    let n = club_squad_strength_band(club, other);
    SCOREBOARD.record(r == n, || format!("club_squad_strength_band({:x}, {:x}) original={} rust={}", club, other, r, n));
    r
}

/// Comparer for `club_reputation_for_staff`.
pub(crate) unsafe extern "C" fn compare_club_reputation_for_staff(club: usize, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_CLUB_REPUTATION_FOR_STAFF);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(club, person),
        || club_reputation_for_staff(club, person));
    // the callers read only ax (movsx)
    SCOREBOARD.record(igual && r as u16 == n as u16, || format!("club_reputation_for_staff({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", club, person, r, n, mo, mr));
    r
}

/// Comparer for `staff_count_disliked_staff_at_club`.
pub(crate) unsafe extern "C" fn compare_staff_count_disliked_staff_at_club(person: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB)(person, club);
    let n = staff_count_disliked_staff_at_club(person, club);
    SCOREBOARD.record(r == n, || format!("staff_count_disliked_staff_at_club({:x}, {:x}) original={} rust={}", person, club, r, n));
    r
}

/// Comparer for `staff_count_favoured_staff_at_club`.
pub(crate) unsafe extern "C" fn compare_staff_count_favoured_staff_at_club(person: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB)(person, club);
    let n = staff_count_favoured_staff_at_club(person, club);
    SCOREBOARD.record(r == n, || format!("staff_count_favoured_staff_at_club({:x}, {:x}) original={} rust={}", person, club, r, n));
    r
}

/// Comparer for `staff_remove_favoured_club`.
pub(crate) unsafe extern "C" fn compare_staff_remove_favoured_club(person: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_FAVOURED_CLUB);
    let pref = staff_preferences(person);
    if pref == 0 {
        return o(person, club); // writes nothing
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + 4, 12)], || o(person, club), || staff_remove_favoured_club(person, club));
    SCOREBOARD.record(igual && r == n, || format!("staff_remove_favoured_club({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `club_remove_favoured_staff`.
pub(crate) unsafe extern "C" fn compare_club_remove_favoured_staff(club: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_CLUB_REMOVE_FAVOURED_STAFF);
    if club == 0 {
        return o(club, person);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::FAV_STAFF1, 12)], || o(club, person), || club_remove_favoured_staff(club, person));
    SCOREBOARD.record(igual && r == n, || format!("club_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", club, person, r, n, mo, mr));
    r
}

/// Comparer for `staff_remove_favoured_staff`.
pub(crate) unsafe extern "C" fn compare_staff_remove_favoured_staff(person: usize, target: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_FAVOURED_STAFF);
    let pref = staff_preferences(person);
    if pref == 0 {
        return o(person, target);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + preferences::STAFF_FAVOURITE_STAFF1, 12)], || o(person, target), || staff_remove_favoured_staff(person, target));
    SCOREBOARD.record(igual && r == n, || format!("staff_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, target, r, n, mo, mr));
    r
}

/// Comparer for `staff_remove_disliked_staff`.
pub(crate) unsafe extern "C" fn compare_staff_remove_disliked_staff(person: usize, target: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_DISLIKED_STAFF);
    let pref = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::STAFF_PREFERENCES) as *const usize) };
    if pref == 0 {
        return o(person, target); // writes nothing
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + preferences::STAFF_DISLIKED_STAFF1, 12)], || o(person, target), || staff_remove_disliked_staff(person, target));
    SCOREBOARD.record(igual && r == n, || format!("staff_remove_disliked_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, target, r, n, mo, mr));
    r
}

/// Comparer for `staff_get_favoured_staff`.
pub(crate) unsafe extern "C" fn compare_staff_get_favoured_staff(person: usize, array: usize, n: *mut i8) -> *mut i8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut i8) -> *mut i8>(IDX_STAFF_GET_FAVOURED_STAFF);
    let with_array = [(array, 12), (n as usize, 1)];
    let regions: &[(usize, usize)] = if array == 0 { &with_array[1..] } else { &with_array };
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(regions, || o(person, array, n), || staff_get_favoured_staff(person, array, n));
    SCOREBOARD.record(igual && r == x, || format!("staff_get_favoured_staff({:x}, {:x}) {:x?} x {:x?}", person, array, mo, mr));
    r
}

/// Comparer for `club_is_in_season`.
pub(crate) unsafe extern "C" fn compare_club_is_in_season(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_IN_SEASON)(club);
    let n = club_is_in_season(club);
    SCOREBOARD.record(r == n, || format!("club_is_in_season({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `club_is_in_major_nation`.
pub(crate) unsafe extern "C" fn compare_club_is_in_major_nation(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_IS_IN_MAJOR_NATION)(club);
    let n = club_is_in_major_nation(club);
    SCOREBOARD.record(r == n, || format!("club_is_in_major_nation({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `nation_is_major_nation`.
pub(crate) unsafe extern "C" fn compare_nation_is_major_nation(nation: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_NATION_IS_MAJOR_NATION)(nation);
    let n = nation_is_major_nation(nation);
    SCOREBOARD.record(r == n, || format!("nation_is_major_nation({:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `staff_clear_opinion_links`.
pub(crate) unsafe extern "C" fn compare_staff_clear_opinion_links(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_CLEAR_OPINION_LINKS);
    let Some(c) = database_staff_state_record(person) else { return o(person) }; // writes nothing
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(c, 79)], || o(person), || staff_clear_opinion_links(person));
    SCOREBOARD.record(igual, || format!("staff_clear_opinion_links({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `staff_add_opinion_link`.
pub(crate) unsafe extern "C" fn compare_staff_add_opinion_link(person: usize, other: usize, value: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32)>(IDX_STAFF_ADD_OPINION_LINK);
    let Some(c) = database_staff_state_record(person) else { return o(person, other, value) };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(c, 79)], || o(person, other, value),
        || staff_add_opinion_link(person, other, value));
    SCOREBOARD.record(igual, || format!("staff_add_opinion_link({:x}, {:x}, {}) {:x?} x {:x?}", person, other, value, mo, mr));
}

/// Comparer for `staff_remove_opinion_link`.
pub(crate) unsafe extern "C" fn compare_staff_remove_opinion_link(person: usize, value: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(IDX_STAFF_REMOVE_OPINION_LINK);
    let Some(c) = database_staff_state_record(person) else { return o(person, value) };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(c, 79)], || o(person, value), || staff_remove_opinion_link(person, value));
    SCOREBOARD.record(igual, || format!("staff_remove_opinion_link({:x}, {}) {:x?} x {:x?}", person, value, mo, mr));
}

/// Comparer for `database_get_club_assistant`.
pub(crate) unsafe extern "C" fn compare_database_get_club_assistant(club: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_ASSISTANT);
    if club == 0 {
        return o(club); // message box: original only
    }
    // the club’s +0x244 flag, which 0x540a50 clears when it rejects the partner
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::HAS_LINKED_CLUB, 1)], || o(club), || database_get_club_assistant(club));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_assistant({:x}) original={:x} rust={:x} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_coaches`.
pub(crate) unsafe extern "C" fn compare_database_get_club_coaches(club: usize, array: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_COACHES);
    if club == 0 {
        return o(club, array); // message box: original only
    }
    // the club’s vector and flag +0x244, which 0x540a50 zeroes when it rejects the partner
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(array as usize, 20), (club + club::HAS_LINKED_CLUB, 1)],
        || o(club, array), || database_get_club_coaches(club, array));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `database_count_club_coaches`.
pub(crate) unsafe extern "C" fn compare_database_count_club_coaches(club: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_COACHES);
    if club == 0 {
        return o(club); // message box: original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::HAS_LINKED_CLUB, 1)], || o(club), || database_count_club_coaches(club));
    SCOREBOARD.record(igual && r == n, || format!("database_count_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `nome_em_ingles`, the English names (0x5454c0, 0x5455a0, 0x5456a0) (comparison mode: see the header of
/// motor.rs).
///
/// With the names language ([0x9b5984]) ≠ 0 and ≠ 1 and the eng.lng table ([0xae2408]) still null, the first use
/// allocates and loads under the SEH frame: only the original routine.
///
/// Otherwise both run, comparing the returned pointer (no effect on memory).
///
/// [pt] Comparador dos nomes em inglês (0x5454c0, 0x5455a0, 0x5456a0): com o idioma dos nomes ([0x9b5984]) ≠ 0 e ≠ 1 e
/// a tabela do eng.lng ([0xae2408]) ainda nula, o primeiro uso aloca e carrega sob o quadro SEH: só a original.
/// Senão as duas, comparando o ponteiro devolvido (sem efeito na memória).
pub(crate) unsafe fn compare_nome_em_ingles(name: &str, x: usize, o: unsafe extern "C" fn(usize) -> usize, r: unsafe extern "C" fn(usize) -> usize) -> usize {
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(x);
    }
    let a = o(x);
    let b = r(x);
    SCOREBOARD.record(a == b, || format!("{}({:x}) idioma {} original={:x} rust={:x}", name, x, language, a, b));
    a
}

/// Comparer for `nation_get_english_three_letter_name`.
pub(crate) unsafe extern "C" fn compare_nation_get_english_three_letter_name(nation: usize) -> usize {
    compare_nome_em_ingles("nation_get_english_three_letter_name", nation, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_THREE_LETTER_NAME), nation_get_english_three_letter_name)
}

/// Comparer for `comp_get_english_name`.
pub(crate) unsafe extern "C" fn compare_comp_get_english_name(comp: usize) -> usize {
    compare_nome_em_ingles("comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_COMP_GET_ENGLISH_NAME), comp_get_english_name)
}

/// Comparer for `staff_comp_get_english_name`.
pub(crate) unsafe extern "C" fn compare_staff_comp_get_english_name(comp: usize) -> usize {
    compare_nome_em_ingles("staff_comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STAFF_COMP_GET_ENGLISH_NAME), staff_comp_get_english_name)
}

/// Comparer for `stadium_get_english_name`.
pub(crate) unsafe extern "C" fn compare_stadium_get_english_name(stadium: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STADIUM_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(stadium); // first use (allocates and loads): original only
    }
    let r = o(stadium);
    let n = stadium_get_english_name(stadium);
    SCOREBOARD.record(r == n, || format!("stadium_get_english_name({:x}) idioma {} original={:x} rust={:x}", stadium, language, r, n));
    r
}

/// Comparador: as duas a partir do mesmo buffer do caminho (0xde279c, 0x104 bytes); compara a resposta e o caminho
/// montado (acusa o nome trocado mesmo sem o arquivo na pasta).
pub(crate) unsafe extern "C" fn compare_database_language_file_exists(language: i32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32) -> i32>(IDX_DATABASE_LANGUAGE_FILE_EXISTS);
    let (r, x, igual, _, _) = crate::comparison::side_by_side(&[(0xde279c, 0x104)], || o(language), || database_language_file_exists(language));
    SCOREBOARD.record(igual && r == x, || format!("database_language_file_exists({}) original={} rust={} caminho igual {}", language, r, x, igual));
    r
}

/// Comparer for `eng_lng_free_tables`.
pub(crate) unsafe extern "thiscall" fn compare_eng_lng_free_tables(t: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_LNG_FREE_TABLES)(t)
}

/// Comparer for `database_generate_extra_non_playing_staff` (0x527240; comparison mode: see the header of motor.rs).
///
/// Original and copy side by side over the whole club and person tables, [0x9b59b4], the text at 0xde1f64 and the
/// random generator state (the copy without counting draws). With a person of job 8 outside the slots (error box), only
/// the original runs. Overflowing the 23,000 (error box) is not anticipated here: the factory database generates far
/// fewer.
///
/// [pt] Comparador de 0x527240: original e cópia lado a lado sobre as tabelas de clubes e de pessoas inteiras, [0x9b59b4], o
/// texto de 0xde1f64 e o estado do sorteador (a cópia sem contar sorteios). Com a pessoa de cargo 8 fora das casas
/// (caixa), só a original. O estouro das 23.000 (caixa) não é previsto aqui: a base de fábrica gera bem menos.
pub(crate) unsafe extern "C" fn compare_database_generate_extra_non_playing_staff() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_DATABASE_GENERATE_EXTRA_NON_PLAYING_STAFF);
    if any_coach_missing_from_club() {
        return o();
    }
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((0x9b59b4, 4));
    regions.push((0xde1f64, 0x40));
    regions.push((read_usize(G_CLUB_TABLE), read_i32(G_CLUB_COUNT).max(0) as usize * 581));
    regions.push((read_usize(G_STAFF_TABLE), read_i32(G_STAFF_COUNT).max(0) as usize * 110));
    let (r, m, igual, mut mo, mut mr) = crate::comparison::side_by_side(&regions, || o(),
        || crate::rng::without_counting(|| database_generate_extra_non_playing_staff()));
    let onde = first_memory_difference(&mo, &mr);
    SCOREBOARD.record(igual && r == m, || match onde {
        Some((a, x, y)) => format!("database_generate_extra_non_playing_staff: original={} rust={}, diverge em {:x} original={:02x} rust={:02x}", r, m, a, x, y),
        None => format!("database_generate_extra_non_playing_staff: original={} rust={}", r, m),
    });
    // simplification: 29 MB per snapshot; they do not go back to the leftovers
    mo.0.clear();
    mr.0.clear();
    r
}

/// Comparer for `database_trim_club_table`.
pub(crate) unsafe extern "C" fn compare_database_trim_club_table(size: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_TRIM_CLUB_TABLE)(size)
}

/// Comparer for `database_prune_comp_history_table`.
pub(crate) unsafe extern "C" fn compare_database_prune_comp_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_PRUNE_COMP_HISTORY_TABLE)()
}

/// Comparer for `resolve_database_indexes`.
///
/// [pt] Comparador de 0x5315a0: original e cópia lado a lado sobre as 12 tabelas inteiras (~50 MB na carga) e o texto de
/// 0xde1f64. Pessoa com jogador, preferências ou não jogador fora da faixa (caixa): só a original.
pub(crate) unsafe extern "C" fn compare_resolve_database_indexes(leniente: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_RESOLVE_DATABASE_INDEXES);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    let (pessoas, n) = (rd(G_STAFF_TABLE), gi(0xae2364));
    let caixa = (0..n.max(0) as usize).any(|i| PERSON_RANGE_CHECK_FIELDS.iter().any(|&(d, teto, ..)| gi(pessoas + i * staff::SIZE + d) >= gi(teto)));
    if caixa {
        return o(leniente);
    }
    let mut regions = vec![(0xde1f64usize, 0x80usize)];
    for &(p, cont, tam) in DATABASE_TABLES.iter() {
        if rd(p) != 0 && gi(cont) > 0 {
            regions.push((rd(p), gi(cont) as usize * tam));
        }
    }
    let (_, _, igual, mut mo, mut mr) = crate::comparison::side_by_side(&regions, || o(leniente), || resolve_database_indexes(leniente));
    let onde = first_memory_difference(&mo, &mr);
    SCOREBOARD.record(igual, || match onde {
        Some((a, x, y)) => format!("resolve_database_indexes({}): diverge em {:x} original={:02x} rust={:02x}", leniente, a, x, y),
        None => format!("resolve_database_indexes({}): diverge", leniente),
    });
    // simplification: ~50 MB per photo; they do not return to the leftovers
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_sort_clubs_by_reputation`.
///
/// [pt] Comparador de 0x53db40: original e cópia lado a lado sobre o vetor de ids [0xae2d1c].
pub(crate) unsafe extern "C" fn compare_database_sort_clubs_by_reputation(redo: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_SORT_CLUBS_BY_REPUTATION);
    let v = *(G_CLUB_REPUTATION_ORDER as *const usize);
    let n = core::ptr::read_unaligned(G_CLUB_COUNT as *const i32);
    if v == 0 || n <= 0 {
        return o(redo);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(v, n as usize * 4)], || o(redo), || database_sort_clubs_by_reputation(redo));
    let onde = first_memory_difference(&mo, &mr);
    SCOREBOARD.record(igual, || format!("database_sort_clubs_by_reputation({}): diverge em {:x?}", redo as u8, onde));
}

/// Comparer for `database_free_staff_history_table`.
pub(crate) unsafe extern "C" fn compare_database_free_staff_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_FREE_STAFF_HISTORY_TABLE)()
}

/// Comparer for `database_free_staff_comp_history_table`.
pub(crate) unsafe extern "C" fn compare_database_free_staff_comp_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_FREE_STAFF_COMP_HISTORY_TABLE)()
}

/// Comparer for `database_grow_comp_history_table`.
pub(crate) unsafe extern "C" fn compare_database_grow_comp_history_table(n: i32) -> u32 {
    original_fn::<unsafe extern "C" fn(i32) -> u32>(IDX_DATABASE_GROW_COMP_HISTORY_TABLE)(n)
}

/// Comparer for `qsort_comparator_club_ids_by_reputation`.
pub(crate) unsafe extern "C" fn compare_qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_QSORT_COMPARATOR_CLUB_IDS_BY_REPUTATION)(a, b);
    let n = qsort_comparator_club_ids_by_reputation(a, b);
    SCOREBOARD.record(r == n, || format!("qsort_comparator_club_ids_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `qsort_comparator_squad_by_player_0xb`.
pub(crate) unsafe extern "C" fn compare_qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_QSORT_COMPARATOR_SQUAD_BY_PLAYER_0XB)(a, b);
    let n = qsort_comparator_squad_by_player_0xb(a, b);
    SCOREBOARD.record(r == n, || format!("qsort_comparator_squad_by_player_0xb({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_staff_by_player_ability_desc`.
pub(crate) unsafe extern "C" fn compare_compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_STAFF_BY_PLAYER_ABILITY_DESC)(a, b);
    let n = compare_staff_by_player_ability_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_staff_by_player_ability_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `normalizacao_do_jogador` (comparison mode: see the header of motor.rs): the comparer of `0x538bc0` and
/// `0x538ec0`.
///
/// Compares the whole player (70 bytes) and byte `+0x58` of the person, with the training manager ([`0xde1f44`]) zeroed
/// on both sides (taking away and giving back the progress touches the training tables, outside the regions); with a
/// training manager, the original routine then runs for real from the same state.
///
/// [pt] Comparador de 0x538bc0 e 0x538ec0: o jogador inteiro (70 bytes) e o byte +0x58 da pessoa, com o gerente
/// do treino ([0xde1f44]) zerado nos dois lados (o tirar e devolver do progresso mexe nas tabelas do treino,
/// fora das regiões); com gerente, a original roda depois de verdade a partir do mesmo estado.
pub(crate) unsafe fn compare_normalizacao_do_jogador(o: unsafe extern "C" fn(usize), rust: unsafe extern "C" fn(usize), person: usize,
                                           name: &'static str) {
    let player = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::PLAYER) as *const usize) };
    if player == 0 {
        return o(person); // does nothing: original only
    }
    let regions = [(player, 70), (person + staff::DETERMINATION, 1)];
    let manager = *(G_TRAINING_MANAGER as *const usize);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    *(G_TRAINING_MANAGER as *mut usize) = 0;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(person), || rust(person));
    *(G_TRAINING_MANAGER as *mut usize) = manager;
    SCOREBOARD.record(igual, || format!("{}({:x}) {:x?} x {:x?}", name, person, mo, mr));
    if manager != 0 {
        before.write_back();
        o(person);
    }
}

/// Comparer for `staff_normalise_player_physical_attributes`.
pub(crate) unsafe extern "C" fn compare_staff_normalise_player_physical_attributes(person: usize) {
    compare_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_PLAYER_PHYSICAL_ATTRIBUTES), staff_normalise_player_physical_attributes, person,
        "staff_normalise_player_physical_attributes");
}

/// Comparer for `database_pick_second_name_by_weight`.
pub(crate) unsafe extern "C" fn compare_database_pick_second_name_by_weight(nation: usize) -> usize {
    // only the CRT rand() changes ([0xad6c30]): the state is restored between the sides and the copy does not count
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_SECOND_NAME_BY_WEIGHT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(nation),
        || crate::rng::without_counting(|| database_pick_second_name_by_weight(nation)));
    SCOREBOARD.record(igual && r == n, || format!("database_pick_second_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}

/// Comparer for `staff_normalise_player_attributes`.
pub(crate) unsafe extern "C" fn compare_staff_normalise_player_attributes(person: usize) {
    compare_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_PLAYER_ATTRIBUTES), staff_normalise_player_attributes, person,
        "staff_normalise_player_attributes");
}

/// Comparer for `nation_get_english_name`.
pub(crate) unsafe extern "C" fn compare_nation_get_english_name(nation: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(nation); // first use (allocates and loads): original only
    }
    let r = o(nation);
    let n = nation_get_english_name(nation);
    SCOREBOARD.record(r == n, || format!("nation_get_english_name({:x}) idioma {} original={:x} rust={:x}", nation, language, r, n));
    r
}

/// Comparer for `club_get_english_short_name`.
pub(crate) unsafe extern "C" fn compare_club_get_english_short_name(club: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_SHORT_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(club); // first use (allocates and loads): original only
    }
    let r = o(club);
    let n = club_get_english_short_name(club);
    SCOREBOARD.record(r == n, || format!("club_get_english_short_name({:x}) idioma {} original={:x} rust={:x}", club, language, r, n));
    r
}

/// Comparer for `times_a_e_b`.
///
/// [pt] Original e cópia lado a lado sobre as regiões dos times A e B; a mensagem dá o primeiro byte que difere (a
/// tabela de clubes tem 8 MB: as fotos não vão para a mensagem nem voltam às sobras).
pub(crate) unsafe fn compare_times_a_e_b(name: &'static str, original: impl FnOnce(), rust: impl FnOnce()) {
    let regions = ab_team_regions();
    let (_, _, igual, mut mo, mut mr) = crate::comparison::side_by_side(&regions, original, rust);
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| {
        x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k]))
    });
    SCOREBOARD.record(igual, || match onde {
        Some((a, x, y)) => format!("{}: diverge em {:x} original={:02x} rust={:02x}", name, a, x, y),
        None => format!("{}: diverge", name),
    });
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_trim_senior_and_b_team_squads`.
pub(crate) unsafe extern "C" fn compare_database_trim_senior_and_b_team_squads() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS);
    compare_times_a_e_b("database_trim_senior_and_b_team_squads", || o(), || database_trim_senior_and_b_team_squads());
}

/// Comparer for `database_merge_b_team_staff`.
pub(crate) unsafe extern "C" fn compare_database_merge_b_team_staff() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_MERGE_B_TEAM_STAFF);
    compare_times_a_e_b("database_merge_b_team_staff", || o(), || database_merge_b_team_staff());
}

/// Comparer for `database_build_nation_name_index`.
///
/// The original routine builds the index; the copy builds another (new malloc), the two are compared byte by byte, the
/// copy's is freed (free 0x9452ca) and the original's goes back to [0xae2338].
///
/// It changes the heap in comparison mode (its fingerprint is not a criterion).
///
/// [pt] Comparador: a original monta o índice; a cópia monta outro (novo malloc), os dois são comparados byte a byte, o
/// da cópia é liberado (free 0x9452ca) e o da original volta a [0xae2338]. Muda a heap do modo de comparação (a
/// impressão dele não é critério).
pub(crate) unsafe extern "C" fn compare_database_build_nation_name_index() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_BUILD_NATION_NAME_INDEX)();
    let t = *(G_NATION_NAME_INDEX_TABLE as *const usize);
    let n = (*(G_NATION_COUNT as *const i32)).max(0) as usize * 0x24;
    if t == 0 || n == 0 {
        return;
    }
    let mut buffer = vec![0u8; n];
    let r = buffer.as_mut_ptr() as usize;
    *(G_NATION_NAME_INDEX_TABLE as *mut usize) = r;
    fill_nation_name_index(r);
    let original = core::slice::from_raw_parts(t as *const u8, n);
    let copy = core::slice::from_raw_parts(r as *const u8, n);
    let onde = original.iter().zip(copy).position(|(a, b)| a != b);
    SCOREBOARD.record(onde.is_none(), || {
        let k = onde.unwrap_or(0) & !3;
        format!("database_build_nation_name_index: diverge na nação {} +{:#x} original={} rust={}", k / 0x24, k % 0x24,
            core::ptr::read_unaligned((t + k) as *const i32), core::ptr::read_unaligned((r + k) as *const i32))
    });
    *(G_NATION_NAME_INDEX_TABLE as *mut usize) = t;
}

/// Comparer for `database_remove_staff_from_club`.
pub(crate) unsafe extern "C" fn compare_database_remove_staff_from_club(club: usize, person: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_REMOVE_STAFF_FROM_CLUB);
    if club == 0 || person == 0 {
        return o(club, person, data, desligar, como_jogador); // 0 or box: original only
    }
    // the club, the person and, for the senior national team, the U21 (the only other place the routine zeroes)
    let mut regions = vec![(club, 581), (person, 110)];
    let (id, n, nacoes) = (*(club as *const i32), *(G_CLUB_COUNT as *const i32), *(G_NATION_COUNT as *const i32));
    let nation = core::ptr::read_unaligned((club + club::NATION) as *const usize);
    if id >= n.wrapping_sub(nacoes.wrapping_mul(2)) && id < n.wrapping_sub(nacoes) && nation != 0 {
        let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
        let s = sub21(nation);
        if s != club {
            regions.push((s, 581));
        }
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(club, person, data, desligar, como_jogador),
        || database_remove_staff_from_club(club, person, data, desligar, como_jogador));
    SCOREBOARD.record(igual && r == m, || format!("database_remove_staff_from_club({:x}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        club, person, data != 0, desligar, r, m, mo, mr));
    r
}

/// Comparer for `database_get_club_scouts`.
pub(crate) unsafe extern "C" fn compare_database_get_club_scouts(club: usize, array: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_SCOUTS);
    if club == 0 {
        return o(club, array); // message box: original only
    }
    // the club’s vector and flag +0x244, which 0x540a50 zeroes when it rejects the partner
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(array as usize, 28), (club + club::HAS_LINKED_CLUB, 1)],
        || o(club, array), || database_get_club_scouts(club, array));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_scouts({:x}) original={} rust={} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `get_adjective_form_index`.
pub(crate) unsafe extern "C" fn compare_get_adjective_form_index(index: u32, language: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u8>(IDX_GET_ADJECTIVE_FORM_INDEX);
    let r = o(index, language);
    let n = get_adjective_form_index(index, language);
    SCOREBOARD.record(r == n, || format!("get_adjective_form_index({}, idioma {}) original={} rust={}", index as u8 as i8, language, r, n));
    r
}

/// Comparer for `staff_get_non_player_attribute`.
pub(crate) unsafe extern "C" fn compare_staff_get_non_player_attribute(person: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_NON_PLAYER_ATTRIBUTE);
    let r = o(person, attribute);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=12).contains(&a) {
        return r; // message box: original only
    }
    let n = staff_get_non_player_attribute(person, attribute);
    SCOREBOARD.record(r == n, || format!("staff_get_non_player_attribute({:x}, {}) original={} rust={}", person, a, r, n));
    r
}

/// Comparer for `player_attribute_rating_with_scaled_ability`.
pub(crate) unsafe extern "C" fn compare_player_attribute_rating_with_scaled_ability(player: usize, value: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_PLAYER_ATTRIBUTE_RATING_WITH_SCALED_ABILITY);
    let r = o(player, value);
    let n = player_attribute_rating_with_scaled_ability(player, value);
    SCOREBOARD.record(r == n, || format!("player_attribute_rating_with_scaled_ability({:x}, {}) original={} rust={}", player, value as u8 as i8, r, n));
    r
}

/// Comparer for `database_pick_first_name_by_weight`.
pub(crate) unsafe extern "C" fn compare_database_pick_first_name_by_weight(nation: usize) -> usize {
    // only the CRT rand() changes ([0xad6c30]): the state is restored between the sides and the copy does not count
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_FIRST_NAME_BY_WEIGHT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(nation),
        || crate::rng::without_counting(|| database_pick_first_name_by_weight(nation)));
    SCOREBOARD.record(igual && r == n, || format!("database_pick_first_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}

/// Comparer for `database_get_comp_colours`.
pub(crate) unsafe extern "C" fn compare_database_get_comp_colours(competition: usize, front: *mut u16, bottom: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_COMP_COLOURS);
    if competition == 0 {
        return o(competition, front, bottom); // message box: original only
    }
    let regions = [(front as usize, 2), (bottom as usize, 2)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(competition, front, bottom),
        || database_get_comp_colours(competition, front, bottom));
    SCOREBOARD.record(igual && r == n, || format!("database_get_comp_colours({:x}) original={} rust={} {:x?} x {:x?}", competition, r, n, mo, mr));
    r
}

/// Comparer for `database_club_has_room_for_job`.
pub(crate) unsafe extern "C" fn compare_database_club_has_room_for_job(club: usize, job: u32, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    let r = o(club, job, person);
    let c = job as u8 as i8;
    if club == 0 || person == 0 || !(1..=15).contains(&c) || c == 14 {
        return r; // message box: original only
    }
    let n = database_club_has_room_for_job(club, job, person);
    SCOREBOARD.record(r == n, || format!("database_club_has_room_for_job({:x}, {}, {:x}) original={} rust={}", club, c, person, r, n));
    r
}

/// Comparer for `database_add_staff_to_club`.
pub(crate) unsafe extern "C" fn compare_database_add_staff_to_club(club: usize, person: usize, job: u32, data: usize, write: u32, reduzir: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(IDX_DATABASE_ADD_STAFF_TO_CLUB);
    if club == 0 || person == 0 {
        return o(club, person, job, data, write, reduzir); // message box: original only
    }
    // writes only to the club (581 bytes) and the person (110); 0x600a50 only queries
    let regions = [(club, 0x245), (person, 0x6e)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(club, person, job, data, write, reduzir),
        || database_add_staff_to_club(club, person, job, data, write, reduzir));
    SCOREBOARD.record(igual && r == n, || format!("database_add_staff_to_club({:x}, {:x}, {}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        club, person, job as u8, data, write, reduzir, r, n, mo, mr));
    r
}

/// Comparer for `database_count_club_squad`.
pub(crate) unsafe extern "C" fn compare_database_count_club_squad(club: usize, mode: u32, filtro: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(IDX_DATABASE_COUNT_CLUB_SQUAD);
    if club == 0 {
        return o(club, mode, filtro); // message box: original only
    }
    // 0x540a50 may zero the club’s +0x244; the filter writes the last contract to [0xae1080]
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::HAS_LINKED_CLUB, 1), (G_CONTRACT_LOOKUP_RESULT, 4)],
        || o(club, mode, filtro), || database_count_club_squad(club, mode, filtro));
    SCOREBOARD.record(igual && r == n, || format!("database_count_club_squad({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", club, mode as u8, filtro, r, n, mo, mr));
    r
}

/// Comparer for `database_collect_club_squad`.
pub(crate) unsafe extern "C" fn compare_database_collect_club_squad(club: usize, array: *mut usize, dobro: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(IDX_DATABASE_COLLECT_CLUB_SQUAD);
    if club == 0 {
        return o(club, array, dobro); // message box: original only
    }
    let regions = [(array as usize, if dobro != 0 { 400 } else { 200 }), (club + club::HAS_LINKED_CLUB, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(club, array, dobro),
        || database_collect_club_squad(club, array, dobro));
    SCOREBOARD.record(igual && r == n, || format!("database_collect_club_squad({:x}, {}) original={} rust={} {:x?} x {:x?}", club, dobro, r, n, mo, mr));
    r
}

/// Comparer for `club_can_be_managed`.
pub(crate) unsafe extern "C" fn compare_club_can_be_managed(club: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_CAN_BE_MANAGED);
    let mark = [(club.wrapping_add(0x244), 1)];
    let regions = if club == 0 { &mark[..0] } else { &mark[..] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regions, || o(club), || club_can_be_managed(club));
    SCOREBOARD.record(igual && r == n, || format!("club_can_be_managed({:x}) original={} rust={} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `database_vector_to_non_player`.
pub(crate) unsafe extern "C" fn compare_database_vector_to_non_player(not_player: usize, person: usize, array: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_DATABASE_VECTOR_TO_NON_PLAYER);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(not_player + non_player::ATTACKING, 21)],
        || o(not_player, person, array), || database_vector_to_non_player(not_player, person, array));
    SCOREBOARD.record(igual && r == n, || format!("database_vector_to_non_player({:x}) original={:x} rust={:x} {:x?} x {:x?}", not_player, r, n, mo, mr));
    r
}

/// Comparer for `database_vector_to_player`.
pub(crate) unsafe extern "C" fn compare_database_vector_to_player(player: usize, person: usize, array: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_DATABASE_VECTOR_TO_PLAYER);
    // the ATTRIBUTE_VECTOR_CELLS offsets are at +0x0f..+0x44
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(player + player_sheet::GOALKEEPER, 0x44 - 0x0f + 1), (person + staff::DETERMINATION, 1)],
        || o(player, person, array), || database_vector_to_player(player, person, array));
    SCOREBOARD.record(igual && r == n, || format!("database_vector_to_player({:x}) original={:x} rust={:x} {:x?} x {:x?}", player, r, n, mo, mr));
    r
}

/// Comparer for `staff_normalise_non_player_attributes`.
pub(crate) unsafe extern "C" fn compare_staff_normalise_non_player_attributes(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_NON_PLAYER_ATTRIBUTES);
    let nj = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::NON_PLAYER) as *const usize) };
    if nj == 0 {
        return o(person); // does nothing: original only
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(nj + non_player::ATTACKING, 21)], || o(person),
        || staff_normalise_non_player_attributes(person));
    SCOREBOARD.record(igual, || format!("staff_normalise_non_player_attributes({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `database_get_colour_family`.
pub(crate) unsafe extern "C" fn compare_database_get_colour_family(colour: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_GET_COLOUR_FAMILY);
    let r = o(colour);
    if colour_family(colour).is_none() {
        return r; // message box: original only
    }
    let n = database_get_colour_family(colour);
    SCOREBOARD.record(r == n, || format!("database_get_colour_family({:x}: id {}) original={} rust={}", colour, *(colour as *const u32), r, n));
    r
}

/// Comparer for `database_colours_clash`.
pub(crate) unsafe extern "C" fn compare_database_colours_clash(a: usize, b: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_DATABASE_COLOURS_CLASH);
    let r = o(a, b);
    if colour_family(a).is_none() || colour_family(b).is_none() {
        return r; // message box: original only
    }
    let n = database_colours_clash(a, b);
    SCOREBOARD.record(r == n, || format!("database_colours_clash({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `staff_compute_non_player_attribute`.
pub(crate) unsafe extern "C" fn compare_staff_compute_non_player_attribute(person: usize, not_player: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_NON_PLAYER_ATTRIBUTE);
    let r = o(person, not_player, attribute);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=12).contains(&a) || not_player == 0 {
        return r; // message box: original only
    }
    let n = staff_compute_non_player_attribute(person, not_player, attribute);
    SCOREBOARD.record(r == n, || format!("staff_compute_non_player_attribute({:x}, {:x}, {}) original={} rust={}", person, not_player, a, r, n));
    r
}

/// Comparer for `database_pick_first_name_for_nation`.
pub(crate) unsafe extern "C" fn compare_database_pick_first_name_for_nation(nation: usize) -> usize {
    // only the CRT’s rand() changes ([0xad6c30]): the state goes back between the sides
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(nation),
        || crate::rng::without_counting(|| database_pick_first_name_for_nation(nation)));
    SCOREBOARD.record(igual && r == n, || format!("database_pick_first_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}

/// Comparer for `database_pick_second_name_for_nation`.
pub(crate) unsafe extern "C" fn compare_database_pick_second_name_for_nation(nation: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(nation),
        || crate::rng::without_counting(|| database_pick_second_name_for_nation(nation)));
    SCOREBOARD.record(igual && r == n, || format!("database_pick_second_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}

/// Comparer for `database_format_staff_name`.
pub(crate) unsafe extern "C" fn compare_database_format_staff_name(person: usize, target: usize, size: u32, estilo: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_STAFF_NAME);
    if person == 0 || target == 0 || (size as i16) < 0x69 || !(1..=7).contains(&(estilo as u8)) {
        return o(person, target, size, estilo); // message box: original only
    }
    // the destination has at least 0x69 bytes (the routine requires it); 0x600740 and 0x6005e0 only read
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(target, 0x69)], || o(person, target, size, estilo),
        || database_format_staff_name(person, target, size, estilo));
    SCOREBOARD.record(igual && r == n, || format!("database_format_staff_name({:x}, estilo {}) original={} rust={} {:x?} x {:x?}", person, estilo as u8, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_kit_colours`.
pub(crate) unsafe extern "C" fn compare_database_get_club_kit_colours(club: usize, kit: u32, front: *mut u16, bottom: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_CLUB_KIT_COLOURS);
    if club == 0 {
        return o(club, kit, front, bottom); // message box: original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(front as usize, 2), (bottom as usize, 2)],
        || o(club, kit, front, bottom), || database_get_club_kit_colours(club, kit, front, bottom));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_kit_colours({:x}, {}) original={} rust={} {:x?} x {:x?}", club, kit as u8, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_manager`.
pub(crate) unsafe extern "C" fn compare_database_get_club_manager(club: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_MANAGER);
    if club == 0 {
        return o(club); // message box: original only
    }
    // 0x540a50 clears the club’s +0x244 flag when it rejects the partner (and the rejection repeats with it cleared):
    // the region is restored between the sides
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::HAS_LINKED_CLUB, 1)], || o(club),
        || database_get_club_manager(club));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_manager({:x}) original={:x} rust={:x} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `club_get_english_name`.
pub(crate) unsafe extern "C" fn compare_club_get_english_name(club: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(club); // first use (allocates and loads): original only
    }
    let r = o(club);
    let n = club_get_english_name(club);
    SCOREBOARD.record(r == n, || format!("club_get_english_name({:x}) idioma {} original={:x} rust={:x}", club, language, r, n));
    r
}

/// Comparer for `staff_adjust_morale`.
pub(crate) unsafe extern "C" fn compare_staff_adjust_morale(person: usize, delta: u32, limitar: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(IDX_STAFF_ADJUST_MORALE);
    if delta as u8 == 0 {
        let r = o(person, delta, limitar);
        SCOREBOARD.record(r == 0, || format!("staff_adjust_morale({:x}, 0) original={}", person, r));
        return r;
    }
    // rolls (0x90cfc0): the generator state goes back between the sides and the copy does not count the rolls
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((core::ptr::read_unaligned((person + staff::PLAYER) as *const usize) + 0x45, 1));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, delta, limitar),
        || crate::rng::without_counting(|| staff_adjust_morale(person, delta, limitar)));
    SCOREBOARD.record(igual && r == n, || format!("staff_adjust_morale({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", person, delta as u8 as i8, limitar, r, n, mo, mr));
    r
}

/// Comparer for `database_non_player_to_attribute_vector`.
pub(crate) unsafe extern "C" fn compare_database_non_player_to_attribute_vector(not_player: usize, person: usize, array: usize, end: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32>(IDX_DATABASE_NON_PLAYER_TO_ATTRIBUTE_VECTOR);
    let mut regions = vec![(array, 21 * 3)];
    if !end.is_null() {
        regions.push((end as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(not_player, person, array, end),
        || database_non_player_to_attribute_vector(not_player, person, array, end));
    SCOREBOARD.record(igual && r == n, || format!("database_non_player_to_attribute_vector({:x}) original={:x} rust={:x} {:x?} x {:x?}", not_player, r, n, mo, mr));
    r
}

/// Comparer for `staff_set_squad_flags`.
pub(crate) unsafe extern "C" fn compare_staff_set_squad_flags(person: usize, squad: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_STAFF_SET_SQUAD_FLAGS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(person + staff::PLAYING_SQUAD, 1)], || o(person, squad),
        || staff_set_squad_flags(person, squad));
    SCOREBOARD.record(igual && r == n, || format!("staff_set_squad_flags({:x}, {:x}) {:x?} x {:x?}", person, squad as u8, mo, mr));
    r
}

/// Comparer for `database_compare_comp_history_rows`.
pub(crate) unsafe extern "C" fn compare_database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_COMP_HISTORY_ROWS)(a, b);
    let n = database_compare_comp_history_rows(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_comp_history_rows({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_player_to_attribute_vector`.
pub(crate) unsafe extern "C" fn compare_database_player_to_attribute_vector(player: usize, person: usize, array: usize, end_of_sides: *mut u8,
                                                            end_of_positions: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize>(IDX_DATABASE_PLAYER_TO_ATTRIBUTE_VECTOR);
    let mut regions = vec![(array, 0x3f * 3)];
    for p in [end_of_sides, end_of_positions, fim_dos_ponderados] {
        if !p.is_null() {
            regions.push((p as usize, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(player, person, array, end_of_sides, end_of_positions, fim_dos_ponderados),
        || database_player_to_attribute_vector(player, person, array, end_of_sides, end_of_positions, fim_dos_ponderados));
    SCOREBOARD.record(igual && r == n, || format!("database_player_to_attribute_vector({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", player, person, r, n, mo, mr));
    r
}

/// Comparer for `nation_is_uk_or_ireland`.
pub(crate) unsafe extern "C" fn compare_nation_is_uk_or_ireland(nation: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_NATION_IS_UK_OR_IRELAND)(nation);
    let n = nation_is_uk_or_ireland(nation);
    SCOREBOARD.record(r == n, || format!("nation_is_uk_or_ireland({:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `database_is_basque_club`.
pub(crate) unsafe extern "C" fn compare_database_is_basque_club(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_BASQUE_CLUB)(club);
    let n = database_is_basque_club(club);
    SCOREBOARD.record(r == n, || format!("database_is_basque_club({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `staff_is_transfer_listed`.
pub(crate) unsafe extern "C" fn compare_staff_is_transfer_listed(person: usize, club: usize) -> u32 {
    // without a club, both write the contract to [0xae1080]; 0x4cf5a0, 0x4cf5f0 and 0x600a50 (club not
    // null) only read
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_TRANSFER_LISTED);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, club),
        || staff_is_transfer_listed(person, club));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_transfer_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_loan_listed`.
pub(crate) unsafe extern "C" fn compare_staff_is_loan_listed(person: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_LOAN_LISTED);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, club),
        || staff_is_loan_listed(person, club));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_loan_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_listed_by_request`.
pub(crate) unsafe extern "C" fn compare_staff_is_listed_by_request(person: usize, club: usize) -> u32 {
    // with no club, both write the contract in [0xae1080]; 0x4cf5a0 and 0x4cf5f0 only read
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_LISTED_BY_REQUEST);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, club),
        || staff_is_listed_by_request(person, club));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_listed_by_request({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `player_attribute_rating`.
pub(crate) unsafe extern "C" fn compare_player_attribute_rating(player: usize, value: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_PLAYER_ATTRIBUTE_RATING)(player, value);
    let n = player_attribute_rating(player, value);
    SCOREBOARD.record(r == n, || format!("player_attribute_rating({:x}, {}) original={} rust={}", player, value as u8 as i8, r, n));
    r
}

/// Comparer for `database_club_has_reserve_squad`.
pub(crate) unsafe extern "C" fn compare_database_club_has_reserve_squad(club: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    if club == 0 {
        return o(club); // message box: original only
    }
    // 0x540a50 clears the club’s +0x244 mark when it refuses the partner: the region goes back between the sides
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(club + club::HAS_LINKED_CLUB, 1)], || o(club),
        || database_club_has_reserve_squad(club));
    SCOREBOARD.record(igual && r == n, || format!("database_club_has_reserve_squad({:x}) original={} rust={} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `database_get_reputation_scope`.
pub(crate) unsafe extern "C" fn compare_database_get_reputation_scope(person: usize, who_ve: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_DATABASE_GET_REPUTATION_SCOPE)(person, who_ve);
    let n = database_get_reputation_scope(person, who_ve);
    SCOREBOARD.record(r == n, || format!("database_get_reputation_scope({:x}, {:x}) original={} rust={}", person, who_ve, r, n));
    r
}

/// Comparer for `database_get_staff_reputation_for_viewer`.
pub(crate) unsafe extern "C" fn compare_database_get_staff_reputation_for_viewer(person: usize, who_ve: usize, not_player: u32) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u16>(IDX_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER)(person, who_ve, not_player);
    let n = database_get_staff_reputation_for_viewer(person, who_ve, not_player);
    SCOREBOARD.record(r == n, || format!("database_get_staff_reputation_for_viewer({:x}, {:x}, {}) original={} rust={}", person, who_ve, not_player, r, n));
    r
}

/// Comparer for `database_is_staff_foreign`.
pub(crate) unsafe extern "C" fn compare_database_is_staff_foreign(person: usize, nation: usize, groups: u32) -> u32 {
    // 0x4cf5a0 and the Date (in a local Date) only read: both sides can run
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(IDX_DATABASE_IS_STAFF_FOREIGN)(person, nation, groups);
    let n = database_is_staff_foreign(person, nation, groups);
    SCOREBOARD.record(r == n, || format!("database_is_staff_foreign({:x}, {:x}, {}) original={} rust={}", person, nation, groups, r, n));
    r
}

/// Comparer for `database_get_staff_age`.
pub(crate) unsafe extern "C" fn compare_database_get_staff_age(person: usize, data: *const u16) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(IDX_DATABASE_GET_STAFF_AGE);
    if person == 0 {
        return o(person, data); // message box: original only
    }
    let r = o(person, data);
    let n = database_get_staff_age(person, data);
    SCOREBOARD.record(r == n, || format!("database_get_staff_age({:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `nation_allows_free_transfers`.
pub(crate) unsafe extern "C" fn compare_nation_allows_free_transfers(nation: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_NATION_ALLOWS_FREE_TRANSFERS)(nation);
    let n = nation_allows_free_transfers(nation);
    SCOREBOARD.record(r == n, || format!("nation_allows_free_transfers({:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `database_is_staff_eu_national`.
pub(crate) unsafe extern "C" fn compare_database_is_staff_eu_national(person: usize, nation_of_club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DATABASE_IS_STAFF_EU_NATIONAL)(person, nation_of_club);
    let n = database_is_staff_eu_national(person, nation_of_club);
    SCOREBOARD.record(r == n, || format!("database_is_staff_eu_national({:x}, {:x}) original={} rust={}", person, nation_of_club, r, n));
    r
}

/// Comparer for `staff_is_transfer_listed_or_requested`.
pub(crate) unsafe extern "C" fn compare_staff_is_transfer_listed_or_requested(person: usize, club: usize) -> u32 {
    // with no club, both write the contract in [0xae1080]; 0x4cf5a0 and 0x4cf5f0 only read
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, club),
        || staff_is_transfer_listed_or_requested(person, club));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_transfer_listed_or_requested({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `staff_compute_player_attribute`.
pub(crate) unsafe extern "C" fn compare_staff_compute_player_attribute(person: usize, player: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_PLAYER_ATTRIBUTE);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&a) || player == 0 {
        return o(person, player, attribute); // message box: original only
    }
    let r = o(person, player, attribute);
    let n = staff_compute_player_attribute(person, player, attribute);
    SCOREBOARD.record(r == n, || format!("staff_compute_player_attribute({:x}, {}) original={} rust={}", player, a, r, n));
    r
}

/// Comparer for `staff_get_player_attribute`.
pub(crate) unsafe extern "C" fn compare_staff_get_player_attribute(person: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&a) {
        return o(person, attribute); // message box: original only
    }
    let r = o(person, attribute);
    let n = staff_get_player_attribute(person, attribute);
    SCOREBOARD.record(r == n, || format!("staff_get_player_attribute({:x}, {}) original={} rust={}", person, a, r, n));
    r
}

/// Comparer for `save_database_for_game`.
pub(crate) unsafe extern "C" fn compare_save_database_for_game(save: usize, a4: u32, kind: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(IDX_SAVE_DATABASE_FOR_GAME)(save, a4, kind)
}

/// Comparer for `load_database_from_stream`.
pub(crate) unsafe extern "C" fn compare_load_database_from_stream(save: usize, compartilhado: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_LOAD_DATABASE_FROM_STREAM)(save, compartilhado)
}

/// Comparer for `ball_out`.
pub(crate) unsafe extern "thiscall" fn compare_ball_out(this: usize, l: u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> u8>(IDX_BALL_OUT);
    // call from inside another comparison that records the commentary: original only (see port)
    if RECORDING_6D6F70.load(std::sync::atomic::Ordering::Relaxed) {
        return o(this, l);
    }
    if is_recording() {
        return call_recording(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, this, [l as u32, 0, 0, 0, 0], || o(this, l) as u32) as u8;
    }
    let mut extra = vec![(this + 0x8ea0, 0x20), (this + 0x4780, 0x10), (this + 0xf570, 0x20), (this + 0x1a0, 0x10)];
    let owner = *((this + engine::BALL_HOLDER) as *const usize);
    if owner != 0 {
        extra.extend([(owner + 0x29, 2), (owner + 0x19c, 2)]);
    }
    let (r, n) = (core::cell::Cell::new(0u8), core::cell::Cell::new(0u8));
    let (igual, k) = compare_with_commentary_at(&extra, || r.set(o(this, l)), |c| n.set(ball_out_with(this, l, c)));
    SCOREBOARD.record(igual && r.get() == n.get(), || format!("ball_out({}) original={} rust={} ({} comentarios)", l, r.get(), n.get(), k));
    r.get()
}

/// Comparer for `likes`.
pub(crate) unsafe extern "C" fn compare_likes(target: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_LIKES)(target, person);
    let n = likes(target, person);
    SCOREBOARD.record(r == n, || format!("likes original={} rust={}", r, n));
    r
}

/// Comparer for `dislikes`.
pub(crate) unsafe extern "C" fn compare_dislikes(target: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DISLIKES)(target, person);
    let n = dislikes(target, person);
    SCOREBOARD.record(r == n, || format!("dislikes original={} rust={}", r, n));
    r
}

/// Comparer for `mark_position`.
pub(crate) unsafe extern "thiscall" fn compare_mark_position(this: usize, position: i8, sheet: *const u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, *const u32) -> u32>(IDX_MARK_POSITION);
    let reg = [((this as isize + 0x5bf + 4 * position as isize) as usize, 4), (this + formation_fields::CHANGED_FLAG, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, position, sheet), || mark_position(this, position, sheet));
    SCOREBOARD.record(igual && r == n, || format!("mark_position({}) original={:x?} rust={:x?}", position, mo, mr));
    r
}

/// Comparer for `foul_in_engine`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_foul_in_engine(
    m: usize, j: usize, other: usize, kind: i8, who: i8, card: *mut u8, grav: *mut i8, advantage: *mut u8,
) -> u8 {
    // the decision itself (0x6e9820) is already compared in its own hook; here only the pass-through
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(IDX_FOUL_IN_ENGINE);
    let mut regions = vec![(m + engine::REFEREE + 8, 2), (card as usize, 1), (grav as usize, 1), (advantage as usize, 1)];
    for p in [j, other] {
        if p != 0 {
            regions.push((p, 0x1be));
        }
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (r, v, igual, _, _) = crate::comparison::side_by_side(
        &regions,
        || o(m, j, other, kind, who, card, grav, advantage),
        || crate::rng::without_counting(|| foul_in_engine(m, j, other, kind, who, card, grav, advantage)),
    );
    SCOREBOARD.record(igual && r == v, || format!("foul_in_engine original={} rust={}", r, v));
    r
}

/// Comparer for `type_not_11`.
pub(crate) unsafe extern "C" fn compare_type_not_11(kind: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i8) -> u32>(IDX_TYPE_NOT_11)(kind);
    let v = type_not_11(kind);
    SCOREBOARD.record(r == v, || format!("type_not_11({}) original={} rust={}", kind, r, v));
    r
}

/// Comparer for `type_11_to_15`.
pub(crate) unsafe extern "C" fn compare_type_11_to_15(kind: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i8) -> u32>(IDX_TYPE_11_TO_15)(kind);
    let v = type_11_to_15(kind);
    SCOREBOARD.record(r == v, || format!("type_11_to_15({}) original={} rust={}", kind, r, v));
    r
}

/// Comparer for `national_side_class`.
pub(crate) unsafe extern "stdcall" fn compare_national_side_class(club: usize, flag: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(IDX_NATIONAL_SIDE_CLASS)(club, flag);
    let v = national_side_class(club, flag);
    SCOREBOARD.record(r == v, || format!("national_side_class({:#x}, {}) original={} rust={}", club, flag, r, v));
    r
}

/// Comparer for `clear_person_mark`.
pub(crate) unsafe extern "thiscall" fn compare_clear_person_mark(this: usize, person: usize, k: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_CLEAR_PERSON_MARK);
    let reg = [(person_flag_byte(this, person, k) as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, k), || clear_person_mark(this, person, k));
    SCOREBOARD.record(igual, || format!("clear_person_mark original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `free_record`.
pub(crate) unsafe extern "C" fn compare_free_record() -> usize {
    let o = original_fn::<unsafe extern "C" fn() -> usize>(IDX_FREE_RECORD);
    let mut reg: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    reg.push((0x9b59b4, 4));
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| free_record()));
    SCOREBOARD.record(igual && r == v, || format!("free_record original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `database_global_9b5984`.
pub(crate) unsafe extern "C" fn compare_database_global_9b5984() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_DATABASE_GLOBAL_9B5984)();
    let v = database_global_9b5984();
    SCOREBOARD.record(r == v, || format!("database_global_9b5984 original={:#x} rust={:#x}", r, v));
    r
}

/// Comparer for `partner_club`.
pub(crate) unsafe extern "C" fn compare_partner_club(club: usize, eh_a: *mut u32, active: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(IDX_PARTNER_CLUB);
    let mut reg: Vec<(usize, usize)> = vec![(club + club::HAS_LINKED_CLUB, 1)];
    if !eh_a.is_null() {
        reg.push((eh_a as usize, 4));
    }
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(club, eh_a, active), || partner_club(club, eh_a, active));
    SCOREBOARD.record(igual && r == v, || format!("partner_club original={:x} {:x?} rust={:x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `likes_club`.
pub(crate) unsafe extern "C" fn compare_likes_club(person: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_LIKES_CLUB)(person, club);
    let n = likes_club(person, club);
    SCOREBOARD.record(r == n, || format!("likes_club original={} rust={}", r, n));
    r
}

/// Comparer for `dislikes_club`.
pub(crate) unsafe extern "C" fn compare_dislikes_club(person: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DISLIKES_CLUB)(person, club);
    let n = dislikes_club(person, club);
    SCOREBOARD.record(r == n, || format!("dislikes_club original={} rust={}", r, n));
    r
}

/// Comparer for `main_national_team`.
pub(crate) unsafe extern "C" fn compare_main_national_team(nation: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_MAIN_NATIONAL_TEAM)(nation);
    let v = main_national_team(nation);
    SCOREBOARD.record(r == v, || format!("main_national_team original={:x} rust={:x}", r, v));
    r
}

/// Comparer for `b_national_team`.
pub(crate) unsafe extern "C" fn compare_b_national_team(nation: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_B_NATIONAL_TEAM)(nation);
    let v = b_national_team(nation);
    SCOREBOARD.record(r == v, || format!("b_national_team original={:x} rust={:x}", r, v));
    r
}

/// Comparer for `is_second_national_team`.
pub(crate) unsafe extern "C" fn compare_is_second_national_team(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_SECOND_NATIONAL_TEAM)(club);
    let v = is_second_national_team(club);
    SCOREBOARD.record(r == v, || format!("is_second_national_team original={} rust={}", r, v));
    r
}

/// Comparer for `aggregate_score`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_aggregate_score(this: usize, l: i8, home: *mut i8, away: *mut i8, cl: *mut u8, first_leg: *mut u8, f3: *mut u8, bonus: u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> u8>(IDX_AGGREGATE_SCORE);
    let reg = [(home as usize, 1), (away as usize, 1), (cl as usize, 1), (first_leg as usize, 1), (f3 as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, l, home, away, cl, first_leg, f3, bonus), || aggregate_score(this, l, home, away, cl, first_leg, f3, bonus));
    SCOREBOARD.record(igual && r == n, || format!("aggregate_score({}, {}) original={} {:x?} rust={} {:x?}", l, bonus, r, mo, n, mr));
    r
}
