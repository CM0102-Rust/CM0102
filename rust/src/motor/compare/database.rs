use super::*;

// The comparers of the routines in ../database.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../database.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `favourite_of` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_favourite_of(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_FAVOURITE_OF)(a, b);
    let n = favourite_of(a, b);
    PLACAR.registrar(r == n, || format!("favourite_of original={} rust={}", r, n));
    r
}

/// Comparer for `rejected_by` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_rejected_by(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_REJECTED_BY)(a, b);
    let n = rejected_by(a, b);
    PLACAR.registrar(r == n, || format!("rejected_by original={} rust={}", r, n));
    r
}

/// Comparer for `database_player_initialisation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador de 0x535a80: original e cópia lado a lado sobre o banco que ela escreve (pessoas, jogadores e não
/// jogadores inteiros, a tabela de 79 bytes por pessoa em [0xae2d0c]), [0x9b59b4], as bandeiras 0x9b59b8/9 e os quatro
/// vetores globais dos moldes, o texto de 0xde1f64, a data guardada da 0x54cab0 e o estado do sorteador (a cópia sem
/// contar sorteios). Sem pessoas (caixa e fim): só a original.
pub(crate) unsafe extern "C" fn comparar_database_player_initialisation(data: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_DATABASE_PLAYER_INITIALISATION);
    let gi = |a: usize| (*(a as *const i32)).max(0) as usize;
    let rd = |a: usize| *(a as *const usize);
    if gi(0xae2364) == 0 || rd(G_STAFF_TABLE) == 0 {
        return o(data);
    }
    let mut regions = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
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
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regions, || o(data),
        || crate::aleatorio::sem_contar(|| database_player_initialisation(data)));
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])));
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("database_player_initialisation: diverge em {:x} original={:02x} rust={:02x}", a, x, y),
        None => "database_player_initialisation: estado do sorteador diferente".to_string(),
    });
    // simplification: dezenas de MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_format_name_from_strings` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_name_from_strings(name: usize, sobrenome: usize, common: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_STRINGS);
    if text == 0 || (size as u16 as i16) < 0x69 || !(1..=7).contains(&(form as u8)) {
        return o(name, sobrenome, common, text, size, form); // caixa: só a original
    }
    // o texto produzido (até o tamanho dado, no máximo 128 bytes)
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(name, sobrenome, common, text, size, form),
        || database_format_name_from_strings(name, sobrenome, common, text, size, form));
    PLACAR.registrar(igual && r == x, || format!("database_format_name_from_strings(forma {}) original={} rust={} {:x?} x {:x?}", form as u8, r, x, mo, mr));
    r
}

/// Comparer for `database_get_job_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_job_name(job: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, usize) -> usize>(IDX_DATABASE_GET_JOB_NAME);
    // o texto [0xae19b4] e o byte do gênero, se houver
    let regions = [(G_JOB_NAME_BUFFER, 100usize), (genero, 1)];
    let reg = if genero != 0 { &regions[..] } else { &regions[..1] };
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(job, curto, maiuscula, genero),
        || database_get_job_name(job, curto, maiuscula, genero));
    PLACAR.registrar(igual && r == x, || format!("database_get_job_name({}, {}, {}) original={:x} rust={:x} {:x?} x {:x?}", job as u8 as i8, curto, maiuscula, r, x, mo, mr));
    r
}

/// Comparer for `database_get_language_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_language_name(language: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_DATABASE_GET_LANGUAGE_NAME);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae21d4, 100)], || o(language), || database_get_language_name(language));
    PLACAR.registrar(igual && r == x, || format!("database_get_language_name({}) original={:x} rust={:x} {:x?} x {:x?}", language as u8 as i8, r, x, mo, mr));
    r
}

/// Comparer for `club_get_national_team_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_get_national_team_name(clube: usize, variante: u32, text: usize, size: i32, form: u32, aspas: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(IDX_CLUB_GET_NATIONAL_TEAM_NAME);
    let f = form as u8 as i8;
    if size < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return o(clube, variante, text, size, form, aspas); // caixa: só a original
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(clube, variante, text, size, form, aspas),
        || club_get_national_team_name(clube, variante, text, size, form, aspas));
    PLACAR.registrar(igual && r == x, || format!("club_get_national_team_name({:x}, {}, forma {}, {}) original={} rust={} {:x?} x {:x?}", clube, variante, f, aspas, r, x, mo, mr));
    r
}

/// Comparer for `database_get_player_attribute_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_player_attribute_name(attribute: u32, text: usize, size: u32, abreviado: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_PLAYER_ATTRIBUTE_NAME);
    let x = attribute as u8 as i8;
    if !(1..=0x1f).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return o(attribute, text, size, abreviado); // caixa: só a original
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, 100)], || o(attribute, text, size, abreviado),
        || database_get_player_attribute_name(attribute, text, size, abreviado));
    PLACAR.registrar(igual && r == n, || format!("database_get_player_attribute_name({}, {}) original={} rust={} {:x?} x {:x?}", x, abreviado, r, n, mo, mr));
    r
}

/// Comparer for `database_format_player_positions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_player_positions(player: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_PLAYER_POSITIONS);
    if text == 0 || player == 0 || (size as u16 as i16) < 0x1e {
        return o(player, text, size, form); // caixa: só a original
    }
    // o texto, as notas do jogador (a original pode gravar +0x13) e o começo do texto global [0xde1f64]
    let n = (size as u16 as usize).min(128);
    let reg = [(text, n), (player + 0xf, 12), (0xde1f64, 64)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(player, text, size, form),
        || database_format_player_positions(player, text, size, form));
    PLACAR.registrar(igual && r == x, || format!("database_format_player_positions({:x}, forma {}) original={} rust={} {:x?} x {:x?}", player, form as u8, r, x, mo, mr));
    r
}

/// Comparer for `database_format_name_from_ids` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_name_from_ids(name: i32, sobrenome: i32, common: i32, text: usize, size: u32, form: u32, person: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_IDS);
    if text == 0 || (size as i16) < 0x69 {
        return o(name, sobrenome, common, text, size, form, person); // caixa: só a original
    }
    // o texto produzido (até o tamanho dado, no máximo 128 bytes)
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(name, sobrenome, common, text, size, form, person),
        || database_format_name_from_ids(name, sobrenome, common, text, size, form, person));
    PLACAR.registrar(igual && r == x, || format!("database_format_name_from_ids({}, {}, {}, forma {}, pessoa {}) original={} rust={} {:x?} x {:x?}", name, sobrenome, common, form, person, r, x, mo, mr));
    r
}

/// Comparer for `database_format_official_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_official_name(oficial: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_OFFICIAL_NAME);
    if oficial == 0 || text == 0 || (size as i16) < 0x69 {
        return o(oficial, text, size, form); // caixa: só a original
    }
    let n = (size as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(oficial, text, size, form),
        || database_format_official_name(oficial, text, size, form));
    PLACAR.registrar(igual && r == x, || format!("database_format_official_name({:x}, forma {}) original={} rust={} {:x?} x {:x?}", oficial, form as u8, r, x, mo, mr));
    r
}

/// Comparer for `position_mask_get_text` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_position_mask_get_text(mask: u32, text: usize, maiuscula: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32) -> u8>(IDX_POSITION_MASK_GET_TEXT);
    if text == 0 {
        return o(mask, text, maiuscula);
    }
    // sem tamanho: os nomes traduzidos cabem em 128
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, 128)], || o(mask, text, maiuscula),
        || position_mask_get_text(mask, text, maiuscula));
    PLACAR.registrar(igual && r == x, || format!("position_mask_get_text({:x}, {}) original={} rust={} {:x?} x {:x?}", mask, maiuscula as u8, r, x, mo, mr));
    r
}

/// Comparer for `write_position_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_write_position_name(mask: u32, text: usize, size: i32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(IDX_WRITE_POSITION_NAME);
    if size < 0x1e {
        return o(mask, text, size); // caixa: só a original
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(mask, text, size),
        || write_position_name(mask, text, size));
    PLACAR.registrar(igual && r == x, || format!("write_position_name({:x}) original={} rust={} {:x?} x {:x?}", mask, r, x, mo, mr));
    r
}

/// Comparer for `write_player_description` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_write_player_description(person: i32, position: u32, qualidades: u32, text: usize, size: i32, genero: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(IDX_WRITE_PLAYER_DESCRIPTION);
    if size < 0x1e {
        return o(person, position, qualidades, text, size, genero); // caixa: só a original
    }
    let n = (size as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, n)], || o(person, position, qualidades, text, size, genero),
        || write_player_description(person, position, qualidades, text, size, genero));
    PLACAR.registrar(igual && r == x, || format!("write_player_description({}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, position, qualidades, r, x, mo, mr));
    r
}

/// Comparer for `staff_is_best_favoured_candidate` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_best_favoured_candidate(person: usize, clube: usize, candidato: usize) -> u8 {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u8>(IDX_STAFF_IS_BEST_FAVOURED_CANDIDATE);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(person, clube, candidato),
        || crate::aleatorio::sem_contar(|| staff_is_best_favoured_candidate(person, clube, candidato)));
    PLACAR.registrar(igual && r == x, || format!("staff_is_best_favoured_candidate({:x}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, clube, candidato, r, x, mo, mr));
    r
}

/// Comparer for `staff_get_player_strength_flags` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_player_strength_flags(person: usize, other: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_GET_PLAYER_STRENGTH_FLAGS);
    let (r, _, igual, d) = compare_with_rng_retry(|| o(person, other), || staff_get_player_strength_flags(person, other));
    PLACAR.registrar(igual, || format!("staff_get_player_strength_flags({:x}, {:x}) {}", person, other, d));
    r
}

/// Comparer for `staff_get_player_weakness_flags` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_player_weakness_flags(person: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_GET_PLAYER_WEAKNESS_FLAGS);
    let (r, _, igual, d) = compare_with_rng_retry(|| o(person, clube), || staff_get_player_weakness_flags(person, clube));
    PLACAR.registrar(igual, || format!("staff_get_player_weakness_flags({:x}, {:x}) {}", person, clube, d));
    r
}

/// Comparer for `database_count_club_physios` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_count_club_physios(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_PHYSIOS)(clube);
    if clube == 0 {
        return r; // caixa: só a original
    }
    let n = database_count_club_physios(clube);
    PLACAR.registrar(r == n, || format!("database_count_club_physios({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_is_b_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_is_b_team(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_B_TEAM)(clube);
    if clube == 0 {
        return r;
    }
    let n = club_is_b_team(clube);
    PLACAR.registrar(r == n, || format!("club_is_b_team({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_collect_non_playing_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_collect_non_playing_staff(clube: usize, out: *mut usize, max: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(IDX_CLUB_COLLECT_NON_PLAYING_STAFF);
    if clube == 0 {
        return o(clube, out, max);
    }
    let reg = [(out as usize, 4 * (max as i8).max(0) as usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, out, max), || club_collect_non_playing_staff(clube, out, max));
    PLACAR.registrar(igual && r == n, || format!("club_collect_non_playing_staff({:x}, {:x}, {}) original={} rust={} {:x?} x {:x?}", clube, out as usize, max as i8, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_player_attribute_visible` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_player_attribute_visible(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_IS_PLAYER_ATTRIBUTE_VISIBLE)(who_ve, person, attribute);
    let n = staff_is_player_attribute_visible(who_ve, person, attribute);
    PLACAR.registrar(r == n, || format!("staff_is_player_attribute_visible({:x}, {:x}, {}) original={} rust={}", who_ve, person, attribute as u8 as i8, r, n));
    r
}

/// Comparer for `staff_get_player_attribute_for_viewer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_player_attribute_for_viewer(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE_FOR_VIEWER)(who_ve, person, attribute);
    let x = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&x) {
        return r; // caixa: só a original
    }
    let n = staff_get_player_attribute_for_viewer(who_ve, person, attribute);
    PLACAR.registrar(r == n, || format!("staff_get_player_attribute_for_viewer({:x}, {:x}, {}) original={} rust={}", who_ve, person, x, r, n));
    r
}

/// Comparer for `database_get_non_player_attribute_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_non_player_attribute_name(attribute: u32, text: usize, size: u32, abreviado: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_NON_PLAYER_ATTRIBUTE_NAME);
    let x = attribute as u8 as i8;
    if !(1..=12).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return o(attribute, text, size, abreviado);
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let reg = [(text, 100)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(attribute, text, size, abreviado),
        || database_get_non_player_attribute_name(attribute, text, size, abreviado));
    PLACAR.registrar(igual && r == n, || format!("database_get_non_player_attribute_name({}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}", x, text, size as u16 as i16, abreviado, r, n, mo, mr));
    r
}

/// Comparer for `database_compare_staff_by_surname` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_STAFF_BY_SURNAME)(a, b);
    let n = database_compare_staff_by_surname(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_staff_by_surname({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_REPUTATION)(a, b);
    let n = database_compare_clubs_by_reputation(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_club_nation_is_eec` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_club_nation_is_eec(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_NATION_IS_EEC)(clube);
    let n = database_club_nation_is_eec(clube);
    PLACAR.registrar(r == n, || format!("database_club_nation_is_eec({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `get_regions_sorted_by_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_get_regions_sorted_by_name(language: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_GET_REGIONS_SORTED_BY_NAME);
    let reg = [(G_REGIONS_SORTED_BY_NAME, 14), (0xae232b, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(language), || get_regions_sorted_by_name(language));
    PLACAR.registrar(igual && r == n, || format!("get_regions_sorted_by_name({}) original={:x} rust={:x} {:x?} x {:x?}", language as u8, r, n, mo, mr));
    r
}

/// Comparer for `compare_region_names` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_region_names(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_REGION_NAMES)(a, b);
    let n = compare_region_names(a, b);
    PLACAR.registrar(r == n, || format!("compare_region_names({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `staff_get_opinion_link_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_opinion_link_staff(person: usize, target: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_STAFF_GET_OPINION_LINK_STAFF)(person, target);
    let n = staff_get_opinion_link_staff(person, target);
    PLACAR.registrar(r == n, || format!("staff_get_opinion_link_staff({:x}, {:x}) original={:x} rust={:x}", person, target, r, n));
    r
}

/// Comparer for `club_add_favoured_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_add_favoured_staff(clube: usize, person: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_CLUB_ADD_FAVOURED_STAFF);
    let reg = preference_regions(if clube != 0 { clube + club::FAV_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, person, forcar),
        || crate::aleatorio::sem_contar(|| club_add_favoured_staff(clube, person, forcar)));
    PLACAR.registrar(igual, || format!("club_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", clube, person, forcar, mo, mr));
}

/// Comparer for `club_add_disliked_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_add_disliked_staff(clube: usize, person: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_CLUB_ADD_DISLIKED_STAFF);
    let reg = preference_regions(if clube != 0 { clube + club::FAV_STAFF1 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, person, forcar),
        || crate::aleatorio::sem_contar(|| club_add_disliked_staff(clube, person, forcar)));
    PLACAR.registrar(igual, || format!("club_add_disliked_staff({:x}, {:x}, {}) {:x?} x {:x?}", clube, person, forcar, mo, mr));
}

/// Comparer for `staff_add_favoured_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_add_favoured_staff(person: usize, other: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_FAVOURED_STAFF);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(person, other, forcar),
        || crate::aleatorio::sem_contar(|| staff_add_favoured_staff(person, other, forcar)));
    PLACAR.registrar(igual, || format!("staff_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", person, other, forcar, mo, mr));
}

/// Comparer for `staff_add_disliked_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_add_disliked_club(person: usize, clube: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_DISLIKED_CLUB);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + 4 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(person, clube, forcar),
        || crate::aleatorio::sem_contar(|| staff_add_disliked_club(person, clube, forcar)));
    PLACAR.registrar(igual, || format!("staff_add_disliked_club({:x}, {:x}, {}) {:x?} x {:x?}", person, clube, forcar, mo, mr));
}

/// Comparer for `staff_fade_staff_relationships` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_fade_staff_relationships(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_FADE_STAFF_RELATIONSHIPS);
    let pref = if person != 0 { read_usize(person + staff::STAFF_PREFERENCES) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(person),
        || crate::aleatorio::sem_contar(|| staff_fade_staff_relationships(person)));
    PLACAR.registrar(igual, || format!("staff_fade_staff_relationships({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `staff_get_disliked_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_disliked_staff(person: usize, out: usize, n: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> usize>(IDX_STAFF_GET_DISLIKED_STAFF);
    let mut reg = vec![(n, 1)];
    if out != 0 {
        reg.push((out, 12));
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(person, out, n), || staff_get_disliked_staff(person, out, n));
    PLACAR.registrar(igual && r == m, || format!("staff_get_disliked_staff({:x}, {:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", person, out, n, r, m, mo, mr));
    r
}

/// Comparer for `staff_get_natural_position_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_natural_position_mask(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_STAFF_GET_NATURAL_POSITION_MASK)(person);
    let n = staff_get_natural_position_mask(person);
    PLACAR.registrar(r == n, || format!("staff_get_natural_position_mask({:x}) original={:x} rust={:x}", person, r, n));
    r
}

/// Comparer for `club_count_squad_with_position_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_count_squad_with_position_mask(clube: usize, mask: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_CLUB_COUNT_SQUAD_WITH_POSITION_MASK)(clube, mask);
    let n = club_count_squad_with_position_mask(clube, mask);
    PLACAR.registrar(r == n, || format!("club_count_squad_with_position_mask({:x}, {:x}) original={} rust={}", clube, mask, r, n));
    r
}

/// Comparer for `staff_pick_favoured_team_mate` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_pick_favoured_team_mate(person: usize, clube: usize) -> usize {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_PICK_FAVOURED_TEAM_MATE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(person, clube),
        || crate::aleatorio::sem_contar(|| staff_pick_favoured_team_mate(person, clube)));
    PLACAR.registrar(igual && r == n, || format!("staff_pick_favoured_team_mate({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `club_nation_is_not_argentina` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_nation_is_not_argentina(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_NATION_IS_NOT_ARGENTINA)(clube);
    let n = club_nation_is_not_argentina(clube);
    PLACAR.registrar(r == n, || format!("club_nation_is_not_argentina({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// Comparer for `club_nation_not_argentina_france_croatia` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_nation_not_argentina_france_croatia(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_NATION_NOT_ARGENTINA_FRANCE_CROATIA)(clube);
    let n = club_nation_not_argentina_france_croatia(clube);
    PLACAR.registrar(r == n, || format!("club_nation_not_argentina_france_croatia({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// Comparer for `database_is_melanesian_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_is_melanesian_nation(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_MELANESIAN_NATION)(nacao);
    let n = database_is_melanesian_nation(nacao);
    PLACAR.registrar(r == n, || format!("database_is_melanesian_nation({:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `database_is_polynesian_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_is_polynesian_nation(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_POLYNESIAN_NATION)(nacao);
    let n = database_is_polynesian_nation(nacao);
    PLACAR.registrar(r == n, || format!("database_is_polynesian_nation({:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `database_get_coaching_job_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_coaching_job_name(job: u32, genero: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, usize) -> usize>(IDX_DATABASE_GET_COACHING_JOB_NAME);
    // o texto (os nomes traduzidos cabem em 64) e o byte do gênero
    let com = [(G_COACHING_JOB_NAME_BUFFER, 64usize), (genero, 1)];
    let regions = if genero != 0 { &com[..] } else { &com[..1] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regions, || o(job, genero), || database_get_coaching_job_name(job, genero));
    PLACAR.registrar(igual && r == n, || format!("database_get_coaching_job_name({}) original={:x} rust={:x} {:x?} x {:x?}", job as i8, r, n, mo, mr));
    r
}

/// Comparer for `database_format_years_between_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_years_between_dates(end: usize, start: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(IDX_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
    if text == 0 || size < 5 || end == 0 || start == 0 {
        return o(end, start, text, size); // caixa: só a original
    }
    // "%d" de uma palavra: até 6 bytes com o zero
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(text, 6)], || o(end, start, text, size),
        || database_format_years_between_dates(end, start, text, size));
    PLACAR.registrar(igual && r == n, || format!("database_format_years_between_dates({:x}, {:x}) original={} rust={} {:x?} x {:x?}", end, start, r, n, mo, mr));
    r
}

/// Comparer for `database_compare_clubs_by_short_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_clubs_by_short_name(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_SHORT_NAME)(a, b);
    let n = database_compare_clubs_by_short_name(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_short_name({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_staff_by_full_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_staff_by_full_name(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_STAFF_BY_FULL_NAME)(a, b);
    let n = database_compare_staff_by_full_name(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_staff_by_full_name({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_staff_matches_names` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_staff_matches_names(person: usize, name: u32, sobrenome: u32, common: u32, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(IDX_DATABASE_STAFF_MATCHES_NAMES);
    if person == 0 {
        return o(person, name, sobrenome, common, data); // caixa: só a original
    }
    let r = o(person, name, sobrenome, common, data);
    let n = database_staff_matches_names(person, name, sobrenome, common, data);
    PLACAR.registrar(r == n, || format!("database_staff_matches_names({:x}, {:x}, {:x}, {:x}, {:x}) original={} rust={}", person, name, sobrenome, common, data, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_latitude` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LATITUDE);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_latitude(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_latitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_compare_clubs_by_longitude` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_clubs_by_longitude(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LONGITUDE);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = database_compare_clubs_by_longitude(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_clubs_by_longitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `club_has_enough_defenders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_enough_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_DEFENDERS)(clube);
    let n = club_has_enough_defenders(clube);
    PLACAR.registrar(r == n, || format!("club_has_enough_defenders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_enough_midfielders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_enough_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_MIDFIELDERS)(clube);
    let n = club_has_enough_midfielders(clube);
    PLACAR.registrar(r == n, || format!("club_has_enough_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_enough_strikers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_enough_strikers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_STRIKERS)(clube);
    let n = club_has_enough_strikers(clube);
    PLACAR.registrar(r == n, || format!("club_has_enough_strikers({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_enough_hard_workers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_enough_hard_workers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_ENOUGH_HARD_WORKERS)(clube);
    let n = club_has_enough_hard_workers(clube);
    PLACAR.registrar(r == n, || format!("club_has_enough_hard_workers({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_fit_goalkeeper` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_fit_goalkeeper(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_FIT_GOALKEEPER)(clube);
    let n = club_has_fit_goalkeeper(clube);
    PLACAR.registrar(r == n, || format!("club_has_fit_goalkeeper({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_ball_winning_midfielders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_ball_winning_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_BALL_WINNING_MIDFIELDERS)(clube);
    let n = club_has_ball_winning_midfielders(clube);
    PLACAR.registrar(r == n, || format!("club_has_ball_winning_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_lacks_aerial_strikers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_lacks_aerial_strikers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_AERIAL_STRIKERS)(clube);
    let n = club_lacks_aerial_strikers(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_aerial_strikers({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_lacks_aerial_defenders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_lacks_aerial_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_AERIAL_DEFENDERS)(clube);
    let n = club_lacks_aerial_defenders(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_aerial_defenders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_lacks_fast_defenders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_lacks_fast_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_LACKS_FAST_DEFENDERS)(clube);
    let n = club_lacks_fast_defenders(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_fast_defenders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_fast_striker` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_fast_striker(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_FAST_STRIKER)(clube);
    let n = club_has_fast_striker(clube);
    PLACAR.registrar(r == n, || format!("club_has_fast_striker({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_has_two_creative_midfielders` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_has_two_creative_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_HAS_TWO_CREATIVE_MIDFIELDERS)(clube);
    let n = club_has_two_creative_midfielders(clube);
    PLACAR.registrar(r == n, || format!("club_has_two_creative_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_squad_strength_band` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_squad_strength_band(clube: usize, other: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_CLUB_SQUAD_STRENGTH_BAND)(clube, other);
    let n = club_squad_strength_band(clube, other);
    PLACAR.registrar(r == n, || format!("club_squad_strength_band({:x}, {:x}) original={} rust={}", clube, other, r, n));
    r
}

/// Comparer for `club_reputation_for_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_reputation_for_staff(clube: usize, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_CLUB_REPUTATION_FOR_STAFF);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(clube, person),
        || club_reputation_for_staff(clube, person));
    // os chamadores leem só o ax (movsx)
    PLACAR.registrar(igual && r as u16 == n as u16, || format!("club_reputation_for_staff({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, person, r, n, mo, mr));
    r
}

/// Comparer for `staff_count_disliked_staff_at_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_count_disliked_staff_at_club(person: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB)(person, clube);
    let n = staff_count_disliked_staff_at_club(person, clube);
    PLACAR.registrar(r == n, || format!("staff_count_disliked_staff_at_club({:x}, {:x}) original={} rust={}", person, clube, r, n));
    r
}

/// Comparer for `staff_count_favoured_staff_at_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_count_favoured_staff_at_club(person: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB)(person, clube);
    let n = staff_count_favoured_staff_at_club(person, clube);
    PLACAR.registrar(r == n, || format!("staff_count_favoured_staff_at_club({:x}, {:x}) original={} rust={}", person, clube, r, n));
    r
}

/// Comparer for `staff_remove_favoured_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_remove_favoured_club(person: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_FAVOURED_CLUB);
    let pref = staff_preferences(person);
    if pref == 0 {
        return o(person, clube); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 4, 12)], || o(person, clube), || staff_remove_favoured_club(person, clube));
    PLACAR.registrar(igual && r == n, || format!("staff_remove_favoured_club({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `club_remove_favoured_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_remove_favoured_staff(clube: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_CLUB_REMOVE_FAVOURED_STAFF);
    if clube == 0 {
        return o(clube, person);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::FAV_STAFF1, 12)], || o(clube, person), || club_remove_favoured_staff(clube, person));
    PLACAR.registrar(igual && r == n, || format!("club_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", clube, person, r, n, mo, mr));
    r
}

/// Comparer for `staff_remove_favoured_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_remove_favoured_staff(person: usize, target: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_FAVOURED_STAFF);
    let pref = staff_preferences(person);
    if pref == 0 {
        return o(person, target);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 0x1c, 12)], || o(person, target), || staff_remove_favoured_staff(person, target));
    PLACAR.registrar(igual && r == n, || format!("staff_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, target, r, n, mo, mr));
    r
}

/// Comparer for `staff_remove_disliked_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_remove_disliked_staff(person: usize, target: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_STAFF_REMOVE_DISLIKED_STAFF);
    let pref = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::STAFF_PREFERENCES) as *const usize) };
    if pref == 0 {
        return o(person, target); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 0x28, 12)], || o(person, target), || staff_remove_disliked_staff(person, target));
    PLACAR.registrar(igual && r == n, || format!("staff_remove_disliked_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", person, target, r, n, mo, mr));
    r
}

/// Comparer for `staff_get_favoured_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_favoured_staff(person: usize, array: usize, n: *mut i8) -> *mut i8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut i8) -> *mut i8>(IDX_STAFF_GET_FAVOURED_STAFF);
    let with_array = [(array, 12), (n as usize, 1)];
    let regions: &[(usize, usize)] = if array == 0 { &with_array[1..] } else { &with_array };
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regions, || o(person, array, n), || staff_get_favoured_staff(person, array, n));
    PLACAR.registrar(igual && r == x, || format!("staff_get_favoured_staff({:x}, {:x}) {:x?} x {:x?}", person, array, mo, mr));
    r
}

/// Comparer for `club_is_in_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_is_in_season(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_IN_SEASON)(clube);
    let n = club_is_in_season(clube);
    PLACAR.registrar(r == n, || format!("club_is_in_season({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `club_is_in_major_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_is_in_major_nation(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_IS_IN_MAJOR_NATION)(clube);
    let n = club_is_in_major_nation(clube);
    PLACAR.registrar(r == n, || format!("club_is_in_major_nation({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `nation_is_major_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_nation_is_major_nation(nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_NATION_IS_MAJOR_NATION)(nacao);
    let n = nation_is_major_nation(nacao);
    PLACAR.registrar(r == n, || format!("nation_is_major_nation({:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `staff_clear_opinion_links` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_clear_opinion_links(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_CLEAR_OPINION_LINKS);
    let Some(c) = database_staff_state_record(person) else { return o(person) }; // não escreve nada
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(person), || staff_clear_opinion_links(person));
    PLACAR.registrar(igual, || format!("staff_clear_opinion_links({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `staff_add_opinion_link` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_add_opinion_link(person: usize, other: usize, value: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32)>(IDX_STAFF_ADD_OPINION_LINK);
    let Some(c) = database_staff_state_record(person) else { return o(person, other, value) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(person, other, value),
        || staff_add_opinion_link(person, other, value));
    PLACAR.registrar(igual, || format!("staff_add_opinion_link({:x}, {:x}, {}) {:x?} x {:x?}", person, other, value, mo, mr));
}

/// Comparer for `staff_remove_opinion_link` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_remove_opinion_link(person: usize, value: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(IDX_STAFF_REMOVE_OPINION_LINK);
    let Some(c) = database_staff_state_record(person) else { return o(person, value) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(person, value), || staff_remove_opinion_link(person, value));
    PLACAR.registrar(igual, || format!("staff_remove_opinion_link({:x}, {}) {:x?} x {:x?}", person, value, mo, mr));
}

/// Comparer for `database_get_club_assistant` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_club_assistant(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_ASSISTANT);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::HAS_LINKED_CLUB, 1)], || o(clube), || database_get_club_assistant(clube));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_assistant({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_coaches` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_club_coaches(clube: usize, array: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_COACHES);
    if clube == 0 {
        return o(clube, array); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(array as usize, 20), (clube + club::HAS_LINKED_CLUB, 1)],
        || o(clube, array), || database_get_club_coaches(clube, array));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `database_count_club_coaches` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_count_club_coaches(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_COACHES);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::HAS_LINKED_CLUB, 1)], || o(clube), || database_count_club_coaches(clube));
    PLACAR.registrar(igual && r == n, || format!("database_count_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `nome_em_ingles` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador dos nomes em inglês (0x5454c0, 0x5455a0, 0x5456a0): com o idioma dos nomes ([0x9b5984]) ≠ 0 e ≠ 1 e
/// a tabela do eng.lng ([0xae2408]) ainda nula, o primeiro uso aloca e carrega sob o quadro SEH: só a original.
/// Senão as duas, comparando o ponteiro devolvido (sem efeito na memória).
pub(crate) unsafe fn comparar_nome_em_ingles(name: &str, x: usize, o: unsafe extern "C" fn(usize) -> usize, r: unsafe extern "C" fn(usize) -> usize) -> usize {
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(x);
    }
    let a = o(x);
    let b = r(x);
    PLACAR.registrar(a == b, || format!("{}({:x}) idioma {} original={:x} rust={:x}", name, x, language, a, b));
    a
}

/// Comparer for `nation_get_english_three_letter_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_nation_get_english_three_letter_name(nacao: usize) -> usize {
    comparar_nome_em_ingles("nation_get_english_three_letter_name", nacao, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_THREE_LETTER_NAME), nation_get_english_three_letter_name)
}

/// Comparer for `comp_get_english_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_get_english_name(comp: usize) -> usize {
    comparar_nome_em_ingles("comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_COMP_GET_ENGLISH_NAME), comp_get_english_name)
}

/// Comparer for `staff_comp_get_english_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_comp_get_english_name(comp: usize) -> usize {
    comparar_nome_em_ingles("staff_comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STAFF_COMP_GET_ENGLISH_NAME), staff_comp_get_english_name)
}

/// Comparer for `stadium_get_english_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_stadium_get_english_name(estadio: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STADIUM_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(estadio); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(estadio);
    let n = stadium_get_english_name(estadio);
    PLACAR.registrar(r == n, || format!("stadium_get_english_name({:x}) idioma {} original={:x} rust={:x}", estadio, language, r, n));
    r
}

/// Comparador: as duas a partir do mesmo buffer do caminho (0xde279c, 0x104 bytes); compara a resposta e o caminho
/// montado (acusa o nome trocado mesmo sem o arquivo na pasta).
pub(crate) unsafe extern "C" fn comparar_database_language_file_exists(language: i32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32) -> i32>(IDX_DATABASE_LANGUAGE_FILE_EXISTS);
    let (r, x, igual, _, _) = crate::comparacao::lado_a_lado(&[(0xde279c, 0x104)], || o(language), || database_language_file_exists(language));
    PLACAR.registrar(igual && r == x, || format!("database_language_file_exists({}) original={} rust={} caminho igual {}", language, r, x, igual));
    r
}

/// Comparer for `eng_lng_free_tables` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_lng_free_tables(t: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_LNG_FREE_TABLES)(t)
}

/// Comparer for `database_generate_extra_non_playing_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador de 0x527240: original e cópia lado a lado sobre as tabelas de clubes e de pessoas inteiras, [0x9b59b4], o
/// texto de 0xde1f64 e o estado do sorteador (a cópia sem contar sorteios). Com a pessoa de cargo 8 fora das casas
/// (caixa), só a original. O estouro das 23.000 (caixa) não é previsto aqui: a base de fábrica gera bem menos.
pub(crate) unsafe extern "C" fn comparar_database_generate_extra_non_playing_staff() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_DATABASE_GENERATE_EXTRA_NON_PLAYING_STAFF);
    if any_coach_missing_from_club() {
        return o();
    }
    let mut regions = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regions.push((0x9b59b4, 4));
    regions.push((0xde1f64, 0x40));
    regions.push((read_usize(G_CLUB_TABLE), read_i32(G_CLUB_COUNT).max(0) as usize * 581));
    regions.push((read_usize(G_STAFF_TABLE), read_i32(G_STAFF_COUNT).max(0) as usize * 110));
    let (r, m, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regions, || o(),
        || crate::aleatorio::sem_contar(|| database_generate_extra_non_playing_staff()));
    let onde = first_memory_difference(&mo, &mr);
    PLACAR.registrar(igual && r == m, || match onde {
        Some((a, x, y)) => format!("database_generate_extra_non_playing_staff: original={} rust={}, diverge em {:x} original={:02x} rust={:02x}", r, m, a, x, y),
        None => format!("database_generate_extra_non_playing_staff: original={} rust={}", r, m),
    });
    // simplification: 29 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
    r
}

/// Comparer for `database_trim_club_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_trim_club_table(size: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_TRIM_CLUB_TABLE)(size)
}

/// Comparer for `database_prune_comp_history_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_prune_comp_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_PRUNE_COMP_HISTORY_TABLE)()
}

/// Comparer for `resolve_database_indexes` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador de 0x5315a0: original e cópia lado a lado sobre as 12 tabelas inteiras (~50 MB na carga) e o texto de
/// 0xde1f64. Pessoa com jogador, preferências ou não jogador fora da faixa (caixa): só a original.
pub(crate) unsafe extern "C" fn comparar_resolve_database_indexes(leniente: u32) {
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
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regions, || o(leniente), || resolve_database_indexes(leniente));
    let onde = first_memory_difference(&mo, &mr);
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("resolve_database_indexes({}): diverge em {:x} original={:02x} rust={:02x}", leniente, a, x, y),
        None => format!("resolve_database_indexes({}): diverge", leniente),
    });
    // simplification: ~50 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_sort_clubs_by_reputation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador de 0x53db40: original e cópia lado a lado sobre o vetor de ids [0xae2d1c].
pub(crate) unsafe extern "C" fn comparar_database_sort_clubs_by_reputation(refazer: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_SORT_CLUBS_BY_REPUTATION);
    let v = *(G_CLUB_REPUTATION_ORDER as *const usize);
    let n = core::ptr::read_unaligned(G_CLUB_COUNT as *const i32);
    if v == 0 || n <= 0 {
        return o(refazer);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(v, n as usize * 4)], || o(refazer), || database_sort_clubs_by_reputation(refazer));
    let onde = first_memory_difference(&mo, &mr);
    PLACAR.registrar(igual, || format!("database_sort_clubs_by_reputation({}): diverge em {:x?}", refazer as u8, onde));
}

/// Comparer for `database_free_staff_history_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_free_staff_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_FREE_STAFF_HISTORY_TABLE)()
}

/// Comparer for `database_free_staff_comp_history_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_free_staff_comp_history_table() {
    original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_FREE_STAFF_COMP_HISTORY_TABLE)()
}

/// Comparer for `database_grow_comp_history_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_grow_comp_history_table(n: i32) -> u32 {
    original_fn::<unsafe extern "C" fn(i32) -> u32>(IDX_DATABASE_GROW_COMP_HISTORY_TABLE)(n)
}

/// Comparer for `qsort_comparator_club_ids_by_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_QSORT_COMPARATOR_CLUB_IDS_BY_REPUTATION)(a, b);
    let n = qsort_comparator_club_ids_by_reputation(a, b);
    PLACAR.registrar(r == n, || format!("qsort_comparator_club_ids_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `qsort_comparator_squad_by_player_0xb` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_QSORT_COMPARATOR_SQUAD_BY_PLAYER_0XB)(a, b);
    let n = qsort_comparator_squad_by_player_0xb(a, b);
    PLACAR.registrar(r == n, || format!("qsort_comparator_squad_by_player_0xb({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_staff_by_player_ability_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_STAFF_BY_PLAYER_ABILITY_DESC)(a, b);
    let n = compare_staff_by_player_ability_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_staff_by_player_ability_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `normalizacao_do_jogador` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador de 0x538bc0 e 0x538ec0: o jogador inteiro (70 bytes) e o byte +0x58 da pessoa, com o gerente
/// do treino ([0xde1f44]) zerado nos dois lados (o tirar e devolver do progresso mexe nas tabelas do treino,
/// fora das regiões); com gerente, a original roda depois de verdade a partir do mesmo estado.
pub(crate) unsafe fn comparar_normalizacao_do_jogador(o: unsafe extern "C" fn(usize), rust: unsafe extern "C" fn(usize), person: usize,
                                           name: &'static str) {
    let player = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::PLAYER) as *const usize) };
    if player == 0 {
        return o(person); // não faz nada: só a original
    }
    let regions = [(player, 70), (person + staff::DETERMINATION, 1)];
    let manager = *(G_TRAINING_MANAGER as *const usize);
    let before = crate::comparacao::Memoria::ler(&regions);
    *(G_TRAINING_MANAGER as *mut usize) = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions, || o(person), || rust(person));
    *(G_TRAINING_MANAGER as *mut usize) = manager;
    PLACAR.registrar(igual, || format!("{}({:x}) {:x?} x {:x?}", name, person, mo, mr));
    if manager != 0 {
        before.escrever();
        o(person);
    }
}

/// Comparer for `staff_normalise_player_physical_attributes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_normalise_player_physical_attributes(person: usize) {
    comparar_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_PLAYER_PHYSICAL_ATTRIBUTES), staff_normalise_player_physical_attributes, person,
        "staff_normalise_player_physical_attributes");
}

/// Comparer for `database_pick_second_name_by_weight` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_pick_second_name_by_weight(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_SECOND_NAME_BY_WEIGHT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_second_name_by_weight(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_second_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparer for `staff_normalise_player_attributes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_normalise_player_attributes(person: usize) {
    comparar_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_PLAYER_ATTRIBUTES), staff_normalise_player_attributes, person,
        "staff_normalise_player_attributes");
}

/// Comparer for `nation_get_english_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_nation_get_english_name(nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(nacao); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(nacao);
    let n = nation_get_english_name(nacao);
    PLACAR.registrar(r == n, || format!("nation_get_english_name({:x}) idioma {} original={:x} rust={:x}", nacao, language, r, n));
    r
}

/// Comparer for `club_get_english_short_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_get_english_short_name(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_SHORT_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = club_get_english_short_name(clube);
    PLACAR.registrar(r == n, || format!("club_get_english_short_name({:x}) idioma {} original={:x} rust={:x}", clube, language, r, n));
    r
}

/// Comparer for `times_a_e_b` (comparison mode: see the header of motor.rs).
///
/// [pt] Original e cópia lado a lado sobre as regiões dos times A e B; a mensagem dá o primeiro byte que difere (a
/// tabela de clubes tem 8 MB: as fotos não vão para a mensagem nem voltam às sobras).
pub(crate) unsafe fn comparar_times_a_e_b(name: &'static str, original: impl FnOnce(), rust: impl FnOnce()) {
    let regions = ab_team_regions();
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regions, original, rust);
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| {
        x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k]))
    });
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("{}: diverge em {:x} original={:02x} rust={:02x}", name, a, x, y),
        None => format!("{}: diverge", name),
    });
    mo.0.clear();
    mr.0.clear();
}

/// Comparer for `database_trim_senior_and_b_team_squads` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_trim_senior_and_b_team_squads() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS);
    comparar_times_a_e_b("database_trim_senior_and_b_team_squads", || o(), || database_trim_senior_and_b_team_squads());
}

/// Comparer for `database_merge_b_team_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_merge_b_team_staff() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_MERGE_B_TEAM_STAFF);
    comparar_times_a_e_b("database_merge_b_team_staff", || o(), || database_merge_b_team_staff());
}

/// Comparer for `database_build_nation_name_index` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original monta o índice; a cópia monta outro (novo malloc), os dois são comparados byte a byte, o
/// da cópia é liberado (free 0x9452ca) e o da original volta a [0xae2338]. Muda a heap do modo de comparação (a
/// impressão dele não é critério).
pub(crate) unsafe extern "C" fn comparar_database_build_nation_name_index() {
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
    PLACAR.registrar(onde.is_none(), || {
        let k = onde.unwrap_or(0) & !3;
        format!("database_build_nation_name_index: diverge na nação {} +{:#x} original={} rust={}", k / 0x24, k % 0x24,
            core::ptr::read_unaligned((t + k) as *const i32), core::ptr::read_unaligned((r + k) as *const i32))
    });
    *(G_NATION_NAME_INDEX_TABLE as *mut usize) = t;
}

/// Comparer for `database_remove_staff_from_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_remove_staff_from_club(clube: usize, person: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_REMOVE_STAFF_FROM_CLUB);
    if clube == 0 || person == 0 {
        return o(clube, person, data, desligar, como_jogador); // 0 ou caixa: só a original
    }
    // o clube, a pessoa e, na seleção principal, a sub-21 (a única outra casa que a rotina zera)
    let mut regions = vec![(clube, 581), (person, 110)];
    let (id, n, nacoes) = (*(clube as *const i32), *(G_CLUB_COUNT as *const i32), *(G_NATION_COUNT as *const i32));
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    if id >= n.wrapping_sub(nacoes.wrapping_mul(2)) && id < n.wrapping_sub(nacoes) && nacao != 0 {
        let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
        let s = sub21(nacao);
        if s != clube {
            regions.push((s, 581));
        }
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(clube, person, data, desligar, como_jogador),
        || database_remove_staff_from_club(clube, person, data, desligar, como_jogador));
    PLACAR.registrar(igual && r == m, || format!("database_remove_staff_from_club({:x}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, person, data != 0, desligar, r, m, mo, mr));
    r
}

/// Comparer for `database_get_club_scouts` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_club_scouts(clube: usize, array: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_SCOUTS);
    if clube == 0 {
        return o(clube, array); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(array as usize, 28), (clube + club::HAS_LINKED_CLUB, 1)],
        || o(clube, array), || database_get_club_scouts(clube, array));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_scouts({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `get_adjective_form_index` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_get_adjective_form_index(indice: u32, language: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u8>(IDX_GET_ADJECTIVE_FORM_INDEX);
    let r = o(indice, language);
    let n = get_adjective_form_index(indice, language);
    PLACAR.registrar(r == n, || format!("get_adjective_form_index({}, idioma {}) original={} rust={}", indice as u8 as i8, language, r, n));
    r
}

/// Comparer for `staff_get_non_player_attribute` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_non_player_attribute(person: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_NON_PLAYER_ATTRIBUTE);
    let r = o(person, attribute);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=12).contains(&a) {
        return r; // caixa: só a original
    }
    let n = staff_get_non_player_attribute(person, attribute);
    PLACAR.registrar(r == n, || format!("staff_get_non_player_attribute({:x}, {}) original={} rust={}", person, a, r, n));
    r
}

/// Comparer for `player_attribute_rating_with_scaled_ability` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_player_attribute_rating_with_scaled_ability(player: usize, value: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_PLAYER_ATTRIBUTE_RATING_WITH_SCALED_ABILITY);
    let r = o(player, value);
    let n = player_attribute_rating_with_scaled_ability(player, value);
    PLACAR.registrar(r == n, || format!("player_attribute_rating_with_scaled_ability({:x}, {}) original={} rust={}", player, value as u8 as i8, r, n));
    r
}

/// Comparer for `database_pick_first_name_by_weight` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_pick_first_name_by_weight(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_FIRST_NAME_BY_WEIGHT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_first_name_by_weight(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_first_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparer for `database_get_comp_colours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_comp_colours(competicao: usize, frente: *mut u16, bottom: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_COMP_COLOURS);
    if competicao == 0 {
        return o(competicao, frente, bottom); // caixa: só a original
    }
    let regions = [(frente as usize, 2), (bottom as usize, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions, || o(competicao, frente, bottom),
        || database_get_comp_colours(competicao, frente, bottom));
    PLACAR.registrar(igual && r == n, || format!("database_get_comp_colours({:x}) original={} rust={} {:x?} x {:x?}", competicao, r, n, mo, mr));
    r
}

/// Comparer for `database_club_has_room_for_job` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_club_has_room_for_job(clube: usize, job: u32, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    let r = o(clube, job, person);
    let c = job as u8 as i8;
    if clube == 0 || person == 0 || !(1..=15).contains(&c) || c == 14 {
        return r; // caixa: só a original
    }
    let n = database_club_has_room_for_job(clube, job, person);
    PLACAR.registrar(r == n, || format!("database_club_has_room_for_job({:x}, {}, {:x}) original={} rust={}", clube, c, person, r, n));
    r
}

/// Comparer for `database_add_staff_to_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_add_staff_to_club(clube: usize, person: usize, job: u32, data: usize, write: u32, reduzir: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(IDX_DATABASE_ADD_STAFF_TO_CLUB);
    if clube == 0 || person == 0 {
        return o(clube, person, job, data, write, reduzir); // caixa: só a original
    }
    // grava só no clube (581 bytes) e na pessoa (110); 0x600a50 só consulta
    let regions = [(clube, 0x245), (person, 0x6e)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(clube, person, job, data, write, reduzir),
        || database_add_staff_to_club(clube, person, job, data, write, reduzir));
    PLACAR.registrar(igual && r == n, || format!("database_add_staff_to_club({:x}, {:x}, {}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, person, job as u8, data, write, reduzir, r, n, mo, mr));
    r
}

/// Comparer for `database_count_club_squad` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_count_club_squad(clube: usize, mode: u32, filtro: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(IDX_DATABASE_COUNT_CLUB_SQUAD);
    if clube == 0 {
        return o(clube, mode, filtro); // caixa: só a original
    }
    // 0x540a50 pode zerar +0x244 do clube; o filtro grava o último contrato em [0xae1080]
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::HAS_LINKED_CLUB, 1), (G_CONTRACT_LOOKUP_RESULT, 4)],
        || o(clube, mode, filtro), || database_count_club_squad(clube, mode, filtro));
    PLACAR.registrar(igual && r == n, || format!("database_count_club_squad({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", clube, mode as u8, filtro, r, n, mo, mr));
    r
}

/// Comparer for `database_collect_club_squad` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_collect_club_squad(clube: usize, array: *mut usize, dobro: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(IDX_DATABASE_COLLECT_CLUB_SQUAD);
    if clube == 0 {
        return o(clube, array, dobro); // caixa: só a original
    }
    let regions = [(array as usize, if dobro != 0 { 400 } else { 200 }), (clube + club::HAS_LINKED_CLUB, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(clube, array, dobro),
        || database_collect_club_squad(clube, array, dobro));
    PLACAR.registrar(igual && r == n, || format!("database_collect_club_squad({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, dobro, r, n, mo, mr));
    r
}

/// Comparer for `club_can_be_managed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_can_be_managed(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_CLUB_CAN_BE_MANAGED);
    let mark = [(clube.wrapping_add(0x244), 1)];
    let regions = if clube == 0 { &mark[..0] } else { &mark[..] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regions, || o(clube), || club_can_be_managed(clube));
    PLACAR.registrar(igual && r == n, || format!("club_can_be_managed({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `database_vector_to_non_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_vector_to_non_player(not_player: usize, person: usize, array: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_DATABASE_VECTOR_TO_NON_PLAYER);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(not_player + 0x0e, 21)],
        || o(not_player, person, array), || database_vector_to_non_player(not_player, person, array));
    PLACAR.registrar(igual && r == n, || format!("database_vector_to_non_player({:x}) original={:x} rust={:x} {:x?} x {:x?}", not_player, r, n, mo, mr));
    r
}

/// Comparer for `database_vector_to_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_vector_to_player(player: usize, person: usize, array: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_DATABASE_VECTOR_TO_PLAYER);
    // os deslocamentos de ATTRIBUTE_VECTOR_CELLS ficam em +0x0f..+0x44
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(player + 0x0f, 0x44 - 0x0f + 1), (person + staff::DETERMINATION, 1)],
        || o(player, person, array), || database_vector_to_player(player, person, array));
    PLACAR.registrar(igual && r == n, || format!("database_vector_to_player({:x}) original={:x} rust={:x} {:x?} x {:x?}", player, r, n, mo, mr));
    r
}

/// Comparer for `staff_normalise_non_player_attributes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_normalise_non_player_attributes(person: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_NORMALISE_NON_PLAYER_ATTRIBUTES);
    let nj = if person == 0 { 0 } else { core::ptr::read_unaligned((person + staff::NON_PLAYER) as *const usize) };
    if nj == 0 {
        return o(person); // não faz nada: só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(nj + 0x0e, 21)], || o(person),
        || staff_normalise_non_player_attributes(person));
    PLACAR.registrar(igual, || format!("staff_normalise_non_player_attributes({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `database_get_colour_family` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_colour_family(colour: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_GET_COLOUR_FAMILY);
    let r = o(colour);
    if colour_family(colour).is_none() {
        return r; // caixa: só a original
    }
    let n = database_get_colour_family(colour);
    PLACAR.registrar(r == n, || format!("database_get_colour_family({:x}: id {}) original={} rust={}", colour, *(colour as *const u32), r, n));
    r
}

/// Comparer for `database_colours_clash` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_colours_clash(a: usize, b: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_DATABASE_COLOURS_CLASH);
    let r = o(a, b);
    if colour_family(a).is_none() || colour_family(b).is_none() {
        return r; // caixa: só a original
    }
    let n = database_colours_clash(a, b);
    PLACAR.registrar(r == n, || format!("database_colours_clash({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `staff_compute_non_player_attribute` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_compute_non_player_attribute(person: usize, not_player: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_NON_PLAYER_ATTRIBUTE);
    let r = o(person, not_player, attribute);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=12).contains(&a) || not_player == 0 {
        return r; // caixa: só a original
    }
    let n = staff_compute_non_player_attribute(person, not_player, attribute);
    PLACAR.registrar(r == n, || format!("staff_compute_non_player_attribute({:x}, {:x}, {}) original={} rust={}", person, not_player, a, r, n));
    r
}

/// Comparer for `database_pick_first_name_for_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_pick_first_name_for_nation(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_first_name_for_nation(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_first_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparer for `database_pick_second_name_for_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_pick_second_name_for_nation(nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_second_name_for_nation(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_second_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparer for `database_format_staff_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_format_staff_name(person: usize, target: usize, size: u32, estilo: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_STAFF_NAME);
    if person == 0 || target == 0 || (size as i16) < 0x69 || !(1..=7).contains(&(estilo as u8)) {
        return o(person, target, size, estilo); // caixa: só a original
    }
    // o destino tem pelo menos 0x69 bytes (a rotina exige); 0x600740 e 0x6005e0 só leem
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(target, 0x69)], || o(person, target, size, estilo),
        || database_format_staff_name(person, target, size, estilo));
    PLACAR.registrar(igual && r == n, || format!("database_format_staff_name({:x}, estilo {}) original={} rust={} {:x?} x {:x?}", person, estilo as u8, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_kit_colours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_club_kit_colours(clube: usize, kit: u32, frente: *mut u16, bottom: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_CLUB_KIT_COLOURS);
    if clube == 0 {
        return o(clube, kit, frente, bottom); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(frente as usize, 2), (bottom as usize, 2)],
        || o(clube, kit, frente, bottom), || database_get_club_kit_colours(clube, kit, frente, bottom));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_kit_colours({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, kit as u8, r, n, mo, mr));
    r
}

/// Comparer for `database_get_club_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_club_manager(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_MANAGER);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro (e a recusa se repete com ela zerada):
    // a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::HAS_LINKED_CLUB, 1)], || o(clube),
        || database_get_club_manager(clube));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_manager({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `club_get_english_name` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_club_get_english_name(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_NAME);
    let language = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if language != 0 && language != 1 && *(G_ENGLISH_NAME_TABLES as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = club_get_english_name(clube);
    PLACAR.registrar(r == n, || format!("club_get_english_name({:x}) idioma {} original={:x} rust={:x}", clube, language, r, n));
    r
}

/// Comparer for `staff_adjust_morale` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_adjust_morale(person: usize, delta: u32, limitar: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(IDX_STAFF_ADJUST_MORALE);
    if delta as u8 == 0 {
        let r = o(person, delta, limitar);
        PLACAR.registrar(r == 0, || format!("staff_adjust_morale({:x}, 0) original={}", person, r));
        return r;
    }
    // sorteia (0x90cfc0): o estado do gerador volta entre os lados e a cópia não conta os sorteios
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((core::ptr::read_unaligned((person + staff::PLAYER) as *const usize) + 0x45, 1));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(person, delta, limitar),
        || crate::aleatorio::sem_contar(|| staff_adjust_morale(person, delta, limitar)));
    PLACAR.registrar(igual && r == n, || format!("staff_adjust_morale({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", person, delta as u8 as i8, limitar, r, n, mo, mr));
    r
}

/// Comparer for `database_non_player_to_attribute_vector` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_non_player_to_attribute_vector(not_player: usize, person: usize, array: usize, end: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32>(IDX_DATABASE_NON_PLAYER_TO_ATTRIBUTE_VECTOR);
    let mut regions = vec![(array, 21 * 3)];
    if !end.is_null() {
        regions.push((end as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions, || o(not_player, person, array, end),
        || database_non_player_to_attribute_vector(not_player, person, array, end));
    PLACAR.registrar(igual && r == n, || format!("database_non_player_to_attribute_vector({:x}) original={:x} rust={:x} {:x?} x {:x?}", not_player, r, n, mo, mr));
    r
}

/// Comparer for `staff_set_squad_flags` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_set_squad_flags(person: usize, squad: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_STAFF_SET_SQUAD_FLAGS);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(person + staff::PLAYING_SQUAD, 1)], || o(person, squad),
        || staff_set_squad_flags(person, squad));
    PLACAR.registrar(igual && r == n, || format!("staff_set_squad_flags({:x}, {:x}) {:x?} x {:x?}", person, squad as u8, mo, mr));
    r
}

/// Comparer for `database_compare_comp_history_rows` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_COMP_HISTORY_ROWS)(a, b);
    let n = database_compare_comp_history_rows(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_comp_history_rows({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `database_player_to_attribute_vector` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_player_to_attribute_vector(player: usize, person: usize, array: usize, fim_dos_lados: *mut u8,
                                                            fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize>(IDX_DATABASE_PLAYER_TO_ATTRIBUTE_VECTOR);
    let mut regions = vec![(array, 0x3f * 3)];
    for p in [fim_dos_lados, fim_das_posicoes, fim_dos_ponderados] {
        if !p.is_null() {
            regions.push((p as usize, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions,
        || o(player, person, array, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados),
        || database_player_to_attribute_vector(player, person, array, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados));
    PLACAR.registrar(igual && r == n, || format!("database_player_to_attribute_vector({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", player, person, r, n, mo, mr));
    r
}

/// Comparer for `nation_is_uk_or_ireland` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_nation_is_uk_or_ireland(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_NATION_IS_UK_OR_IRELAND)(nacao);
    let n = nation_is_uk_or_ireland(nacao);
    PLACAR.registrar(r == n, || format!("nation_is_uk_or_ireland({:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `database_is_basque_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_is_basque_club(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_IS_BASQUE_CLUB)(clube);
    let n = database_is_basque_club(clube);
    PLACAR.registrar(r == n, || format!("database_is_basque_club({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `staff_is_transfer_listed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_transfer_listed(person: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0, 0x4cf5f0 e 0x600a50 (clube não
    // nulo) só leem
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_TRANSFER_LISTED);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, clube),
        || staff_is_transfer_listed(person, clube));
    PLACAR.registrar(igual && r == n, || format!("staff_is_transfer_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_loan_listed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_loan_listed(person: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_LOAN_LISTED);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, clube),
        || staff_is_loan_listed(person, clube));
    PLACAR.registrar(igual && r == n, || format!("staff_is_loan_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `staff_is_listed_by_request` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_listed_by_request(person: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_LISTED_BY_REQUEST);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, clube),
        || staff_is_listed_by_request(person, clube));
    PLACAR.registrar(igual && r == n, || format!("staff_is_listed_by_request({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `player_attribute_rating` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_player_attribute_rating(player: usize, value: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_PLAYER_ATTRIBUTE_RATING)(player, value);
    let n = player_attribute_rating(player, value);
    PLACAR.registrar(r == n, || format!("player_attribute_rating({:x}, {}) original={} rust={}", player, value as u8 as i8, r, n));
    r
}

/// Comparer for `database_club_has_reserve_squad` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_club_has_reserve_squad(clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro: a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + club::HAS_LINKED_CLUB, 1)], || o(clube),
        || database_club_has_reserve_squad(clube));
    PLACAR.registrar(igual && r == n, || format!("database_club_has_reserve_squad({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `database_get_reputation_scope` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_reputation_scope(person: usize, who_ve: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_DATABASE_GET_REPUTATION_SCOPE)(person, who_ve);
    let n = database_get_reputation_scope(person, who_ve);
    PLACAR.registrar(r == n, || format!("database_get_reputation_scope({:x}, {:x}) original={} rust={}", person, who_ve, r, n));
    r
}

/// Comparer for `database_get_staff_reputation_for_viewer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_staff_reputation_for_viewer(person: usize, who_ve: usize, not_player: u32) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u16>(IDX_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER)(person, who_ve, not_player);
    let n = database_get_staff_reputation_for_viewer(person, who_ve, not_player);
    PLACAR.registrar(r == n, || format!("database_get_staff_reputation_for_viewer({:x}, {:x}, {}) original={} rust={}", person, who_ve, not_player, r, n));
    r
}

/// Comparer for `database_is_staff_foreign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_is_staff_foreign(person: usize, nacao: usize, groups: u32) -> u32 {
    // a 0x4cf5a0 e o Date (num Date local) só leem: os dois lados podem rodar
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(IDX_DATABASE_IS_STAFF_FOREIGN)(person, nacao, groups);
    let n = database_is_staff_foreign(person, nacao, groups);
    PLACAR.registrar(r == n, || format!("database_is_staff_foreign({:x}, {:x}, {}) original={} rust={}", person, nacao, groups, r, n));
    r
}

/// Comparer for `database_get_staff_age` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_get_staff_age(person: usize, data: *const u16) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(IDX_DATABASE_GET_STAFF_AGE);
    if person == 0 {
        return o(person, data); // caixa: só a original
    }
    let r = o(person, data);
    let n = database_get_staff_age(person, data);
    PLACAR.registrar(r == n, || format!("database_get_staff_age({:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `nation_allows_free_transfers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_nation_allows_free_transfers(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_NATION_ALLOWS_FREE_TRANSFERS)(nacao);
    let n = nation_allows_free_transfers(nacao);
    PLACAR.registrar(r == n, || format!("nation_allows_free_transfers({:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `database_is_staff_eu_national` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_is_staff_eu_national(person: usize, nation_of_club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DATABASE_IS_STAFF_EU_NATIONAL)(person, nation_of_club);
    let n = database_is_staff_eu_national(person, nation_of_club);
    PLACAR.registrar(r == n, || format!("database_is_staff_eu_national({:x}, {:x}) original={} rust={}", person, nation_of_club, r, n));
    r
}

/// Comparer for `staff_is_transfer_listed_or_requested` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_is_transfer_listed_or_requested(person: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person, clube),
        || staff_is_transfer_listed_or_requested(person, clube));
    PLACAR.registrar(igual && r == n, || format!("staff_is_transfer_listed_or_requested({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, clube, r, n, mo, mr));
    r
}

/// Comparer for `staff_compute_player_attribute` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_compute_player_attribute(person: usize, player: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_PLAYER_ATTRIBUTE);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&a) || player == 0 {
        return o(person, player, attribute); // caixa: só a original
    }
    let r = o(person, player, attribute);
    let n = staff_compute_player_attribute(person, player, attribute);
    PLACAR.registrar(r == n, || format!("staff_compute_player_attribute({:x}, {}) original={} rust={}", player, a, r, n));
    r
}

/// Comparer for `staff_get_player_attribute` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_get_player_attribute(person: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE);
    let a = attribute as u8 as i8;
    if person == 0 || !(1..=0x1f).contains(&a) {
        return o(person, attribute); // caixa: só a original
    }
    let r = o(person, attribute);
    let n = staff_get_player_attribute(person, attribute);
    PLACAR.registrar(r == n, || format!("staff_get_player_attribute({:x}, {}) original={} rust={}", person, a, r, n));
    r
}

/// Comparer for `save_database_for_game` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_save_database_for_game(save: usize, a4: u32, kind: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(IDX_SAVE_DATABASE_FOR_GAME)(save, a4, kind)
}

/// Comparer for `load_database_from_stream` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_load_database_from_stream(save: usize, compartilhado: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_LOAD_DATABASE_FROM_STREAM)(save, compartilhado)
}

/// Comparer for `ball_out` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ball_out(this: usize, l: u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> u8>(IDX_BALL_OUT);
    // chamada de dentro de outra comparacao que grava os comentarios: so a original (ver porta)
    if GRAVANDO_6D6F70.load(std::sync::atomic::Ordering::Relaxed) {
        return o(this, l);
    }
    if is_recording() {
        return call_recording(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, this, [l as u32, 0, 0, 0, 0], || o(this, l) as u32) as u8;
    }
    let mut extra = vec![(this + 0x8ea0, 0x20), (this + 0x4780, 0x10), (this + 0xf570, 0x20), (this + 0x1a0, 0x10)];
    let owner = *((this + 0xf582) as *const usize);
    if owner != 0 {
        extra.extend([(owner + 0x29, 2), (owner + 0x19c, 2)]);
    }
    let (r, n) = (core::cell::Cell::new(0u8), core::cell::Cell::new(0u8));
    let (igual, k) = comparar_com_comentarios_em(&extra, || r.set(o(this, l)), |c| n.set(ball_out_with(this, l, c)));
    PLACAR.registrar(igual && r.get() == n.get(), || format!("ball_out({}) original={} rust={} ({} comentarios)", l, r.get(), n.get(), k));
    r.get()
}

/// Comparer for `likes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_likes(target: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_LIKES)(target, person);
    let n = likes(target, person);
    PLACAR.registrar(r == n, || format!("likes original={} rust={}", r, n));
    r
}

/// Comparer for `dislikes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_dislikes(target: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DISLIKES)(target, person);
    let n = dislikes(target, person);
    PLACAR.registrar(r == n, || format!("dislikes original={} rust={}", r, n));
    r
}

/// Comparer for `mark_position` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_mark_position(this: usize, position: i8, sheet: *const u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, *const u32) -> u32>(IDX_MARK_POSITION);
    let reg = [((this as isize + 0x5bf + 4 * position as isize) as usize, 4), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, position, sheet), || mark_position(this, position, sheet));
    PLACAR.registrar(igual && r == n, || format!("mark_position({}) original={:x?} rust={:x?}", position, mo, mr));
    r
}

/// Comparer for `foul_in_engine` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_foul_in_engine(
    m: usize, j: usize, other: usize, kind: i8, who: i8, cartao: *mut u8, grav: *mut i8, advantage: *mut u8,
) -> u8 {
    // a decisao em si (0x6e9820) ja e comparada no proprio desvio; aqui so o repasse
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(IDX_FOUL_IN_ENGINE);
    let mut regions = vec![(m + 0xf5b2 + 8, 2), (cartao as usize, 1), (grav as usize, 1), (advantage as usize, 1)];
    for p in [j, other] {
        if p != 0 {
            regions.push((p, 0x1be));
        }
    }
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, _, _) = crate::comparacao::lado_a_lado(
        &regions,
        || o(m, j, other, kind, who, cartao, grav, advantage),
        || crate::aleatorio::sem_contar(|| foul_in_engine(m, j, other, kind, who, cartao, grav, advantage)),
    );
    PLACAR.registrar(igual && r == v, || format!("foul_in_engine original={} rust={}", r, v));
    r
}

/// Comparer for `type_not_11` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_type_not_11(kind: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i8) -> u32>(IDX_TYPE_NOT_11)(kind);
    let v = type_not_11(kind);
    PLACAR.registrar(r == v, || format!("type_not_11({}) original={} rust={}", kind, r, v));
    r
}

/// Comparer for `type_11_to_15` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_type_11_to_15(kind: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i8) -> u32>(IDX_TYPE_11_TO_15)(kind);
    let v = type_11_to_15(kind);
    PLACAR.registrar(r == v, || format!("type_11_to_15({}) original={} rust={}", kind, r, v));
    r
}

/// Comparer for `national_side_class` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_national_side_class(clube: usize, flag: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(IDX_NATIONAL_SIDE_CLASS)(clube, flag);
    let v = national_side_class(clube, flag);
    PLACAR.registrar(r == v, || format!("national_side_class({:#x}, {}) original={} rust={}", clube, flag, r, v));
    r
}

/// Comparer for `clear_person_mark` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_person_mark(this: usize, person: usize, k: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_CLEAR_PERSON_MARK);
    let reg = [(person_flag_byte(this, person, k) as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, person, k), || clear_person_mark(this, person, k));
    PLACAR.registrar(igual, || format!("clear_person_mark original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `free_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_free_record() -> usize {
    let o = original_fn::<unsafe extern "C" fn() -> usize>(IDX_FREE_RECORD);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0x9b59b4, 4));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| free_record()));
    PLACAR.registrar(igual && r == v, || format!("free_record original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `database_global_9b5984` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_database_global_9b5984() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_DATABASE_GLOBAL_9B5984)();
    let v = database_global_9b5984();
    PLACAR.registrar(r == v, || format!("database_global_9b5984 original={:#x} rust={:#x}", r, v));
    r
}

/// Comparer for `partner_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_partner_club(clube: usize, eh_a: *mut u32, active: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(IDX_PARTNER_CLUB);
    let mut reg: Vec<(usize, usize)> = vec![(clube + club::HAS_LINKED_CLUB, 1)];
    if !eh_a.is_null() {
        reg.push((eh_a as usize, 4));
    }
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(clube, eh_a, active), || partner_club(clube, eh_a, active));
    PLACAR.registrar(igual && r == v, || format!("partner_club original={:x} {:x?} rust={:x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `likes_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_likes_club(person: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_LIKES_CLUB)(person, clube);
    let n = likes_club(person, clube);
    PLACAR.registrar(r == n, || format!("likes_club original={} rust={}", r, n));
    r
}

/// Comparer for `dislikes_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_dislikes_club(person: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_DISLIKES_CLUB)(person, clube);
    let n = dislikes_club(person, clube);
    PLACAR.registrar(r == n, || format!("dislikes_club original={} rust={}", r, n));
    r
}

/// Comparer for `main_national_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_main_national_team(nacao: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_MAIN_NATIONAL_TEAM)(nacao);
    let v = main_national_team(nacao);
    PLACAR.registrar(r == v, || format!("main_national_team original={:x} rust={:x}", r, v));
    r
}

/// Comparer for `b_national_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_b_national_team(nacao: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_B_NATIONAL_TEAM)(nacao);
    let v = b_national_team(nacao);
    PLACAR.registrar(r == v, || format!("b_national_team original={:x} rust={:x}", r, v));
    r
}

/// Comparer for `is_second_national_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_is_second_national_team(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_SECOND_NATIONAL_TEAM)(clube);
    let v = is_second_national_team(clube);
    PLACAR.registrar(r == v, || format!("is_second_national_team original={} rust={}", r, v));
    r
}

/// Comparer for `aggregate_score` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_aggregate_score(this: usize, l: i8, home: *mut i8, away: *mut i8, cl: *mut u8, first_leg: *mut u8, f3: *mut u8, bonus: u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> u8>(IDX_AGGREGATE_SCORE);
    let reg = [(home as usize, 1), (away as usize, 1), (cl as usize, 1), (first_leg as usize, 1), (f3 as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, home, away, cl, first_leg, f3, bonus), || aggregate_score(this, l, home, away, cl, first_leg, f3, bonus));
    PLACAR.registrar(igual && r == n, || format!("aggregate_score({}, {}) original={} {:x?} rust={} {:x?}", l, bonus, r, mo, n, mr));
    r
}
