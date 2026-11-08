use super::*;

// The comparers of the routines in ../match_screens.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_screens.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_match_stats` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_match_stats(stats: usize, zip: usize, n: i32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> usize>(IDX_CONSTRUCT_MATCH_STATS)(stats, zip, n)
}

/// Comparer for `destroy_match_stats` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_destroy_match_stats(stats: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_MATCH_STATS)(stats)
}

/// Comparer for `fill_match_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fill_match_record(stats: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_FILL_MATCH_RECORD);
    // simplification: the first call (static data and atexit) and the errors only through the original
    if m == 0 || reg == 0 || *(G_MATCH_STATS_FILL_RECORD_STATIC_DATE_GUARD as *const u8) & 1 == 0 {
        return o(stats, m, reg, sem_renda);
    }
    // the shirt number given by the original (0x85a470) stays outside the regions: the copy finds it already given
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(reg, 0xd7a)], || o(stats, m, reg, sem_renda),
        || fill_match_record(stats, m, reg, sem_renda));
    SCOREBOARD.record(igual && r == n, || {
        let difs: Vec<usize> = mo.0[0].1.iter().zip(mr.0[0].1.iter()).enumerate().filter(|(_, (a, b))| a != b).map(|(i, _)| i).take(16).collect();
        format!("fill_match_record(sem_renda={}) original={} rust={} bytes diferentes em {:x?}", sem_renda, r, n, difs)
    });
    r
}

/// Comparer for `match_stats_compare_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> i32>(IDX_MATCH_STATS_COMPARE_EVENTS)(a, b);
    let n = match_stats_compare_events(a, b);
    SCOREBOARD.record(r == n, || format!("match_stats_compare_events({:x?}, {:x?}) original={} rust={}",
        core::slice::from_raw_parts(a, 4), core::slice::from_raw_parts(b, 4), r, n));
    r
}

/// Comparer for `montagem_da_partida`, the match screen builds (comparison mode: see the header of motor.rs).
///
/// They are compared through the display list: the screen regions, the parameter block without 0x14 (the time that the
/// redraw of the latest scores 0x724520 writes in tab 8), the marks 0xb6b7a8/ac/b0, the text 0xde1f64 and the network
/// buffer position ([0xae290c]), plus `extra`.
///
/// [pt] As montagens da tela da partida são comparadas pela lista de exibição: as regiões da tela, o bloco
/// de parâmetros sem o 0x14 (a hora que o redesenho dos últimos placares 0x724520 grava na aba 8), as
/// marcas 0xb6b7a8/ac/b0, o texto 0xde1f64 e a posição do buffer de rede ([0xae290c]), mais `extra`.
pub(crate) unsafe fn comparar_montagem_da_partida(name: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = screen_build_regions();
    reg.push((0xb6b7a8, 12));
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7e2 + slot * 0x300) as *const usize);
    if block != 0 {
        reg.push((block, 0xb4));
        reg.push((block + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // the background photo (0x41c410) rolls: the generator state goes back for the second round (without it the
    // generator advanced twice in comparison mode and the career shifted; 204th)
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, || crate::rng::without_counting(rust));
    SCOREBOARD.record(igual, || format!("{}: diferencas em {:?}", name, first_differences(&mo, &mr)));
}

/// Comparer for `build_match_screen` (comparison mode: see the header of motor.rs).
///
/// The match screen is compared by its display list, plus what it writes outside the screen: the human's player sheet
/// (+0 and +0xea), his mark among the spectators and the count, the speed focus in the human's `Display` (+0x12f4fe)
/// and the commentator volume (+0x2db0). Only the original runs when there is something that cannot run twice: the
/// photo of parameter 0x1d to be freed, the highlight of parameter 7 to be commentated (sound, 0x6d7b00), or tabs 1
/// (the summary also commentates), 3 (the zones allocate the photo) and 10 (the live table).
///
/// [pt] A tela da partida é comparada pela lista de exibição, com o que ela grava fora da tela: a ficha do
/// humano (+0 e +0xea), a marca dele nos espectadores e a contagem, o foco das velocidades no `Display`
/// do humano (+0x12f4fe) e o volume do narrador (+0x2db0). Só a original quando há o que não roda duas
/// vezes: a foto do parâmetro 0x1d a liberar, o lance do parâmetro 7 a narrar (som, 0x6d7b00) ou as
/// abas 1 (o resumo também narra), 3 (as zonas alocam a foto) e 10 (a tabela ao vivo).
pub(crate) unsafe extern "C" fn comparar_build_match_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BUILD_MATCH_SCREEN);
    let aba = get_screen_param(2) as u8;
    // tabs 2 (statistics) and 7 (ratings) allocate the copy of state 0x1d when it is missing: also original only
    if get_screen_param(0x1d) != 0 || get_screen_param(7) != 0 || matches!(aba, 1 | 2 | 3 | 7 | 10) {
        // simplification: freeing, commentary with sound and allocating do not run twice; these cases are proven in Rust mode
        return o();
    }
    let sheet: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let f = sheet(0);
    let mut extra = Vec::new();
    if current_human_display() != 0 {
        extra.push((current_human_display() + 0x12f4fe, 3));
    }
    if f != 0 {
        extra.push((f, 1));
        extra.push((f + 0xea, 4));
    }
    let p = get_screen_param(0);
    if p != 0 && get_screen_param(1) as u8 != 1 {
        let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
        let espectadores = rd(p + 0x5b);
        extra.push(((espectadores as isize + 0xc5 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as usize, 4));
        extra.push((espectadores + 0x108, 1));
        let pm = rd(p + 0x57);
        if pm != 0 && rd(pm + engine::COMMENTATOR) != 0 {
            extra.push((rd(pm + engine::COMMENTATOR) + 0x2db0, 1));
        }
    }
    comparar_montagem_da_partida("build_match_screen", &extra, || o(), || build_match_screen());
}

/// Comparer for `handle_match_screen_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_handle_match_screen_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_MATCH_SCREEN_INPUT)(item)
}

/// Comparer for `build_match_details_screen` (comparison mode: see the header of motor.rs).
///
/// The details screen is compared through the display list when the match's record and picture already exist
/// (parameters `0x1f` and `0x20`) and there is no picture `0x1d` to free. Only the original routine when they are
/// created here (allocation) or on tabs 1 (the summary narrates with sound), 3 (the zones allocate the picture) and 4
/// (the report is created).
///
/// [pt] A tela de detalhes é comparada pela lista de exibição quando o registro e a foto da partida já
/// existem (parâmetros 0x1f e 0x20) e não há foto 0x1d a liberar. Só a original quando eles nascem aqui
/// (alocação) ou nas abas 1 (o resumo narra com som), 3 (as zonas alocam a foto) e 4 (o relatório nasce).
pub(crate) unsafe extern "C" fn comparar_build_match_details_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BUILD_MATCH_DETAILS_SCREEN);
    let aba = get_screen_param(2) as u8;
    // tabs 2 (statistics) and 6 (ratings) allocate the copy of state 0x1d: also original only
    if get_screen_param(0x1d) != 0 || get_screen_param(0x1f) == 0 || get_screen_param(0x20) == 0 || matches!(aba, 1 | 2 | 3 | 4 | 6) {
        // simplification: allocating, freeing and narrating do not run twice; those cases are proven in Rust mode
        return o();
    }
    comparar_montagem_da_partida("build_match_details_screen", &[], || o(), || build_match_details_screen());
}

/// Comparer for `refresh_possession_bar` (comparison mode: see the header of motor.rs).
///
/// [pt] A barra é comparada de verdade quando já existe a cópia do estado (parâmetro 0x1d) e a atualização
/// é local: o `Display` do gerente, os parâmetros, a cópia (0xd7a bytes) e o texto 0xde1f64. Sem a cópia
/// (nasce aqui) ou pela rede: só a original.
pub(crate) unsafe extern "C" fn comparar_refresh_possession_bar() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_REFRESH_POSSESSION_BAR);
    let p0 = get_screen_param(0);
    let copy = get_screen_param(0x1d);
    let mut reg = match screen_update_regions(false) {
        Some(r) if p0 == 0 || copy != 0 => r,
        // simplification: the copy’s allocation and the network packet do not run twice; proven in Rust mode
        _ => return o(),
    };
    reg.extend(screen_param_regions());
    if copy != 0 {
        reg.push((copy, 0xd7a));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || refresh_possession_bar());
    SCOREBOARD.record(igual, || format!("refresh_possession_bar: diferencas em {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `draw_match_overview_panel` (comparison mode: see the header of motor.rs).
///
/// The overview is truly compared through the display list, with `*topo`, `*base` and the possession bar at the end
/// (the manager's `Display` and the state copy).
///
/// Only the original routine: with the error warning, with the commentator (parameter 7: writes the move and plays the
/// sound), in a match without the state copy (the bar allocates it) and with the network update.
///
/// [pt] A visão geral é comparada de verdade pela lista de exibição, com `*topo`, `*base` e a barra da posse
/// do fim (o `Display` do gerente e a cópia do estado). Só a original: com o aviso de erro, com o
/// narrador (parâmetro 7: escreve o lance e toca o som), na partida sem a cópia do estado (a barra a
/// aloca) e com a atualização pela rede.
pub(crate) unsafe extern "C" fn comparar_draw_match_overview_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_OVERVIEW_PANEL);
    let p0 = get_screen_param(0);
    let foto = get_screen_param(0x1d);
    let commentator = p0 != 0
        && core::ptr::read_unaligned((core::ptr::read_unaligned((p0 + 0x5f) as *const usize) + 0x53) as *const u32) == 0
        && get_screen_param(7) != 0;
    let reg = screen_update_regions(false);
    if overview_would_error() || commentator || (p0 != 0 && foto == 0) || reg.is_none() {
        // simplification: warning, sound, allocation and network do not run twice; those paths are proved in Rust mode
        return o(top, base);
    }
    let mut reg = reg.unwrap_or_default();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((top as usize, 4));
    reg.push((base as usize, 4));
    if foto != 0 {
        reg.push((foto, 0xd7a));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(top, base), || draw_match_overview_panel(top, base));
    SCOREBOARD.record(igual, || format!("draw_match_overview_panel: diferencas em {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `match_screens_draw_live_league_table` (comparison mode: see the header of motor.rs).
///
/// The live table is genuinely compared through the display list, together with the league (0xee bytes: [+0xad]) and
/// its table ([+0xb1], 59 bytes per club), which the routine reorders. The local copies are born and die inside it.
///
/// Without a league or without parameter 0xe ("Error" box): only the original routine.
///
/// [pt] A tabela ao vivo é comparada de verdade pela lista de exibição, com a liga (0xee bytes: [+0xad]) e a
/// tabela dela ([+0xb1], 59 bytes por clube), que a rotina reordena. As cópias locais nascem e morrem
/// dentro dela. Sem liga ou sem o parâmetro 0xe (caixa "Error"): só a original.
pub(crate) unsafe extern "C" fn comparar_match_screens_draw_live_league_table(league: usize, top: i32, base: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_MATCH_SCREENS_DRAW_LIVE_LEAGUE_TABLE);
    if league == 0 || get_screen_param(0xe) == 0 {
        return o(league, top, base);
    }
    let mut extra = vec![(league, 0xee)];
    let table = core::ptr::read_unaligned((league + league::TABLE) as *const usize);
    let n = core::ptr::read_unaligned((league + league::CLUB_COUNT) as *const i16);
    if table != 0 && n > 0 {
        extra.push((table, n as usize * 59));
    }
    comparar_pela_lista_com("match_screens_draw_live_league_table", &extra, || o(league, top, base), || match_screens_draw_live_league_table(league, top, base));
}

/// Comparer for `match_screens_draw_team_selection` (comparison mode: see the header of motor.rs).
///
/// [pt] A conferência é comparada de verdade pela lista de exibição (o Text local é alocado e liberado
/// dentro de cada execução).
pub(crate) unsafe extern "C" fn comparar_match_screens_draw_team_selection() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_DRAW_TEAM_SELECTION);
    comparar_painel_pela_lista("match_screens_draw_team_selection", || o(), || match_screens_draw_team_selection());
}

/// Comparer for `match_screens_load` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_screens_load(stats: usize, zip: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_LOAD)(stats, zip)
}

/// Comparer for `match_screens_save` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_screens_save(stats: usize, zip: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_SAVE)(stats, zip)
}

/// Comparer for `read_match_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_match_record(stats: usize, k: i32, out: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_READ_MATCH_RECORD)(stats, k, out)
}

/// Comparer for `match_screens_record_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_screens_record_match(stats: usize, m: usize, r: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_MATCH_SCREENS_RECORD_MATCH)(stats, m, r)
}

/// Comparer for `write_match_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_write_match_record(stats: usize, game: usize, r: usize, events: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD)(stats, game, r, events)
}

/// Comparer for `fill_team_stats_rows` (comparison mode: see the header of motor.rs).
///
/// The cells are really compared: building (`modo` ≠ 0) through the display list, local update through the manager's
/// `Display`; the parameters (`0x1a`) and the text `0xde1f64` too. An update that goes over the network or parameter
/// `0x1c` other than 1 and 2: only the original routine.
///
/// [pt] As células são comparadas de verdade: montagem (`modo` ≠ 0) pela lista de exibição, atualização
/// local pelo `Display` do gerente; os parâmetros (o 0x1a) e o texto 0xde1f64 também. Atualização que
/// vai pela rede ou parâmetro 0x1c fora de 1 e 2: só a original.
pub(crate) unsafe extern "C" fn comparar_fill_team_stats_rows(mode: u32, area: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_FILL_TEAM_STATS_ROWS);
    let column = get_screen_param(0x1c) as u8;
    let mut reg = match screen_update_regions(mode != 0) {
        Some(r) if column == 1 || column == 2 => r,
        _ => return o(mode, area),
    };
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(mode, area), || fill_team_stats_rows(mode, area));
    SCOREBOARD.record(igual, || format!("fill_team_stats_rows({}, {:#x}): diferencas em {:?}", mode, area, first_differences(&mo, &mr)));
}

/// Comparer for `refresh_latest_scores_rows` (comparison mode: see the header of motor.rs).
///
/// [pt] O redesenho é comparado de verdade (atualização local): o `Display` do gerente, os parâmetros (menos
/// o 0x14, a hora que a rotina grava no fim) e o texto 0xde1f64. Pela rede: só a original.
pub(crate) unsafe extern "C" fn comparar_refresh_latest_scores_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_REFRESH_LATEST_SCORES_ROWS);
    let mut reg = match screen_update_regions(false) {
        Some(r) => r,
        None => return o(),
    };
    reg.push((0xb6b7a8, 12));
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7e2 + slot * 0x300) as *const usize);
    if block != 0 {
        reg.push((block, 0xb4));
        reg.push((block + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || refresh_latest_scores_rows());
    SCOREBOARD.record(igual, || format!("refresh_latest_scores_rows: diferencas em {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `match_screens_handle_waiting_screen` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_waiting_screen() -> i32 {
    original_fn::<unsafe extern "C" fn() -> i32>(IDX_MATCH_SCREENS_HANDLE_WAITING_SCREEN)()
}

/// Comparer for `pela_lista_com` (comparison mode: see the header of motor.rs).
///
/// [pt] Como `comparar_painel_pela_lista`, com mais regiões (o que a rotina grava fora da tela).
pub(crate) unsafe fn comparar_pela_lista_com(name: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // the background photo (0x41c410) rolls: the generator state goes back for the second round (without it the
    // generator advanced twice in comparison mode and the career shifted; 204th)
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, || crate::rng::without_counting(rust));
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", name, dif)
    });
}

/// Comparer for `draw_action_zones_panel` (comparison mode: see the header of motor.rs).
///
/// The action zones are compared for real, including `*topo` (+0x19), the fixed colour 0xb6b7a4, the guard 0xb6b7b4 and
/// the match photo that the values routine (0x720320) compares and rewrites (parameter 0x1d, 0xd7a bytes). Without the
/// photo yet (the first draw), the values routine allocates it: only the original runs.
///
/// [pt] As zonas de ação são comparadas de verdade, com o `*topo` (+0x19), a cor fixa 0xb6b7a4 e a guarda
/// 0xb6b7b4 e a foto da partida que os valores (0x720320) comparam e regravam (parâmetro 0x1d, 0xd7a
/// bytes). Sem a foto ainda (o primeiro desenho), os valores a alocam: só a original roda.
pub(crate) unsafe extern "C" fn comparar_draw_action_zones_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_ACTION_ZONES_PANEL);
    let foto = get_screen_param(0x1d);
    if foto == 0 {
        // simplification: the allocation does not run twice; this design is proven in Rust mode
        return o(top, base);
    }
    let extra = [(top as usize, 4), (0xb6b7a4, 2), (0xb6b7b4, 1), (foto, 0xd7a)];
    comparar_pela_lista_com("draw_action_zones_panel", &extra, || o(top, base), || draw_action_zones_panel(top, base));
}

/// Comparer for `draw_match_report_panel` (comparison mode: see the header of motor.rs).
///
/// [pt] O relatório é comparado de verdade quando já existe (na partida ou no parâmetro 0x2d); quando nasce
/// aqui (alocação e construção) só a original roda.
pub(crate) unsafe extern "C" fn comparar_draw_match_report_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_REPORT_PANEL);
    if get_screen_param(0) == 0 && *((get_screen_param(0x20) + 0x4c) as *const u8) == 0 && get_screen_param(0x2d) == 0 {
        // simplification: the report allocation does not run twice; this path is proven in Rust mode
        return o(top, base);
    }
    comparar_painel_pela_lista("draw_match_report_panel", || o(top, base), || draw_match_report_panel(top, base));
}

/// Comparer for `build_day_fixtures_screen` (comparison mode: see the header of motor.rs).
///
/// [pt] A montagem é comparada de verdade, com a marca de vista do humano no grupo (+0x14 + 4h).
pub(crate) unsafe extern "C" fn comparar_build_day_fixtures_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BUILD_DAY_FIXTURES_SCREEN);
    let mark = (get_screen_param(1) as isize + 0x14 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as usize;
    comparar_pela_lista_com("build_day_fixtures_screen", &[(mark, 4)], || o(), || build_day_fixtures_screen());
}

/// Comparer for `valores_da_partida`, the match values (comparison mode: see the header of motor.rs).
///
/// The three routines are truly compared when the state copy (parameter 0x1d) already exists: both update from the same
/// state, and the screen manager (with the message buffer position in +0x3802, which each item update zeroes and
/// fills), the current screen's `Display`, the parameter block, the state copy (0xd7a bytes) and the text 0xde1f64 must
/// come out equal.
///
/// The content of the message buffer ([manager + 0x37fe]) is not included (as in the lists batch).
///
/// [pt] As três rotinas são comparadas de verdade quando já existe a cópia do estado (parâmetro 0x1d): as
/// duas atualizam a partir do mesmo estado e o gerente de telas (com a posição do buffer de mensagens
/// em +0x3802, que cada atualização de item zera e preenche), o `Display` da tela atual, o bloco de
/// parâmetros, a cópia do estado (0xd7a bytes) e o texto 0xde1f64 têm de sair iguais. O conteúdo do
/// buffer de mensagens ([gerente + 0x37fe]) não entra (como na leva das listas).
pub(crate) unsafe fn comparar_valores_da_partida(name: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let copy = get_screen_param(0x1d);
    if copy == 0 {
        // simplification: without a copy the routine allocates the copy (0x944cf1) and writes it to parameter 0x1d; it cannot
        // to run twice. Original only; the Rust copy proves this branch in Rust mode.
        return original();
    }
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((copy, 0xd7a));
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", name, dif)
    });
}

/// Comparer for `refresh_match_stats_values` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_refresh_match_stats_values() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_REFRESH_MATCH_STATS_VALUES);
    comparar_valores_da_partida("refresh_match_stats_values", || o(), || refresh_match_stats_values());
}

/// Comparer for `refresh_action_zones_values` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_refresh_action_zones_values() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_REFRESH_ACTION_ZONES_VALUES);
    comparar_valores_da_partida("refresh_action_zones_values", || o(), || refresh_action_zones_values());
}

/// Comparer for `refresh_player_ratings_rows` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_refresh_player_ratings_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_REFRESH_PLAYER_RATINGS_ROWS);
    comparar_valores_da_partida("refresh_player_ratings_rows", || o(), || refresh_player_ratings_rows());
}

/// Comparer for `painel_pela_lista` (comparison mode: see the header of motor.rs).
///
/// The three panels are genuinely compared: both draw from the same state, and the screen manager, the current screen's
/// `Display`, the parameter block with the marks 0xb6b7a8/ac/b0 and the text 0xde1f64 must come out equal.
///
/// The list (0x55fba0) also writes into a network buffer ([0xae2908] at position [0xae290c]): the position is included
/// in the regions (it is reset so the second run writes to the same place, and the size written is compared); the
/// contents of that buffer are not.
///
/// [pt] Os três painéis são comparados de verdade: as duas desenham a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual, o bloco de parâmetros com as marcas 0xb6b7a8/ac/b0 e o texto
/// 0xde1f64 têm de sair iguais. A lista (0x55fba0) também escreve num buffer de rede ([0xae2908] na
/// posição [0xae290c]): a posição entra nas regiões (volta para a segunda rodada escrever no mesmo
/// lugar e o tamanho escrito é comparado); o conteúdo desse buffer não.
pub(crate) unsafe fn comparar_painel_pela_lista(name: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", name, dif)
    });
}

/// Comparer for `draw_team_stats_panel` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_draw_team_stats_panel(side: u32, top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32, *mut i32)>(IDX_DRAW_TEAM_STATS_PANEL);
    comparar_painel_pela_lista("draw_team_stats_panel", || o(side, top, base), || draw_team_stats_panel(side, top, base));
}

/// Comparer for `draw_player_ratings_panel` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_draw_player_ratings_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_PLAYER_RATINGS_PANEL);
    // simplification: without the state copy (parameter 0x1d) the update inside the panel allocates it, and each
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if get_screen_param(0x1d) == 0 {
        return o(top, base);
    }
    comparar_painel_pela_lista("draw_player_ratings_panel", || o(top, base), || draw_player_ratings_panel(top, base));
}

/// Comparer for `draw_match_stats_panel` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_draw_match_stats_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_STATS_PANEL);
    // simplification: without the state copy (parameter 0x1d) the update inside the panel allocates it, and each
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if get_screen_param(0x1d) == 0 {
        return o(top, base);
    }
    comparar_painel_pela_lista("draw_match_stats_panel", || o(top, base), || draw_match_stats_panel(top, base));
}

/// Comparer for `match_screens_handle_continue_command` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_continue_command(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_SCREENS_HANDLE_CONTINUE_COMMAND)(item)
}

/// Comparer for `match_screens_open_team_selection` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_open_team_selection() {
    original_fn::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_OPEN_TEAM_SELECTION)()
}

/// Comparer for `match_screens_handle_team_selection_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_team_selection_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_SCREENS_HANDLE_TEAM_SELECTION_INPUT)(item)
}

/// Comparer for `match_team_block_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_team_block_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_TEAM_BLOCK_ASSIGN);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(this, 0x625)], || o(this, src), || match_team_block_assign(this, src));
    SCOREBOARD.record(igual && r == n, || format!("match_team_block_assign({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// Comparer for `match_player_row_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_player_row_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_PLAYER_ROW_ASSIGN);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(this, 0x49)], || o(this, src), || match_player_row_assign(this, src));
    SCOREBOARD.record(igual && r == n, || format!("match_player_row_assign({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// Comparer for `match_screens_tally_live_score` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_tally_live_score(partida: usize, index: u32, res: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(IDX_MATCH_SCREENS_TALLY_LIVE_SCORE);
    if res == 0 {
        return o(partida, index, res);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(res + 0x43, 0xa)], || o(partida, index, res), || match_screens_tally_live_score(partida, index, res));
    SCOREBOARD.record(igual, || format!("match_screens_tally_live_score(partida {:#x}, indice {}): {:02x?} / {:02x?}", partida, index, mo.0, mr.0));
}

/// Comparer for `fixture_entry_matches` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fixture_entry_matches(stats: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(IDX_FIXTURE_ENTRY_MATCHES);
    if k < -1 || k > (*((stats + 0x1c) as *const i32)).wrapping_sub(1) {
        return o(stats, k, a, b, data_a, data_b);
    }
    let r = o(stats, k, a, b, data_a, data_b);
    let n = fixture_entry_matches(stats, k, a, b, data_a, data_b);
    SCOREBOARD.record(r == n, || format!("fixture_entry_matches({}, {}, {}, {}, {}) original={} rust={}", k, a, b, data_a, data_b, r, n));
    r
}

/// Comparer for `match_stats_get_record_bucket` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_match_stats_get_record_bucket(game: usize, record: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_MATCH_STATS_GET_RECORD_BUCKET)(game, record);
    let n = match_stats_get_record_bucket(game, record);
    SCOREBOARD.record(r == n, || format!("match_stats_get_record_bucket original={} rust={}", r, n));
    r
}

/// Comparer for `open_day_fixtures_screen` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_open_day_fixtures_screen(slot: u32, group: usize) {
    original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_DAY_FIXTURES_SCREEN)(slot, group)
}

/// Comparer for `handle_day_fixtures_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_handle_day_fixtures_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_DAY_FIXTURES_INPUT)(item)
}

/// Comparer for `match_screens_find_best_rated_starter_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Só lê: compara a vaga devolvida (tática nula: só a original, pela caixa de erro).
pub(crate) unsafe extern "C" fn comparar_match_screens_find_best_rated_starter_slot(tactic: usize, contexto: i32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, i32) -> u8>(IDX_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT)(tactic, contexto);
    if tactic == 0 {
        return r;
    }
    let n = match_screens_find_best_rated_starter_slot(tactic, contexto);
    SCOREBOARD.record(r == n, || format!("match_screens_find_best_rated_starter_slot({:#x}, {}) {} x {}", tactic, contexto, r as i8, n as i8));
    r
}

/// Comparer for `open_match_screen` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_open_match_screen(slot: u32, partida: usize) {
    original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_MATCH_SCREEN)(slot, partida)
}

/// Comparer for `resposta` (comparison mode: see the header of motor.rs).
///
/// Both responses are really compared on clicks (the screen's parameter block and the marks `0xb6b7a8`/`ac`/`b0`, plus
/// the return value); when there is a redraw (the 5-second switch of the scores, new statistics data) only the original
/// routine runs, because the redraw writes the time into parameter `0x14` and touches the display list.
///
/// [pt] As duas respostas são comparadas de verdade nos cliques (o bloco de parâmetros da tela e as marcas
/// 0xb6b7a8/ac/b0, mais a devolução); quando há redesenho (a troca de 5 s dos placares, os dados novos
/// das estatísticas) só a original roda, porque o redesenho grava a hora no parâmetro 0x14 e mexe na
/// lista de exibição.
pub(crate) unsafe fn comparar_resposta(name: &str, item: u32, o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32, redesenha: bool) -> i32 {
    if item as u16 == 0xffff || redesenha {
        return o(item);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || r(item));
    SCOREBOARD.record(igual && ro == rr, || format!("{}(item {:#x}) original={} rust={} {:?} x {:?}", name, item, ro, rr, mo, mr));
    ro
}

/// Comparer for `match_screens_handle_latest_scores_cycle` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_latest_scores_cycle(item: u32) -> i32 {
    let now: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    // 1 s of slack: the time may roll over between the two sides
    let redesenha = now().wrapping_add(1000) >= (get_screen_param(0x14) as u32).wrapping_add(5000);
    comparar_resposta("match_screens_handle_latest_scores_cycle", item, original_fn(431), match_screens_handle_latest_scores_cycle, redesenha)
}

/// Comparer for `handle_team_stats_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_handle_team_stats_input(item: u32) -> i32 {
    let p0 = get_screen_param(0);
    let redesenha = *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *const u32) != 0 && *(*((p0 + 0x5b) as *const usize) as *const u8) & 4 != 0;
    comparar_resposta("handle_team_stats_input", item, original_fn(432), handle_team_stats_input, redesenha)
}

/// Comparer for `open_match_details` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_open_match_details(partida: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_MATCH_DETAILS)(partida)
}

/// Comparer for `match_details_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_details_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_DETAILS_INPUT)(item)
}

/// Comparer for `close_match_details` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_close_match_details() {
    original_fn::<unsafe extern "C" fn()>(IDX_CLOSE_MATCH_DETAILS)()
}

/// Comparer for `open_latest_scores` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_open_latest_scores(slot: u32, entry: usize) {
    original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_LATEST_SCORES)(slot, entry)
}

/// Comparer for `build_latest_scores` (comparison mode: see the header of motor.rs).
///
/// [pt] A montagem é comparada de verdade: as duas montam a tela a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual (a lista de exibição) e o título (0xde1f64) têm de sair iguais.
pub(crate) unsafe extern "C" fn comparar_build_latest_scores() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BUILD_LATEST_SCORES);
    let mut reg = screen_build_regions();
    reg.push((0xde1f64, 0x100));
    // the list (0x55fba0) writes into a network buffer ([0xae2908] at position [0xae290c]): the position is restored
    // for the second round to write in the same place
    reg.push((0xae290c, 4));
    // the background photo (0x41c410) rolls: the generator state goes back for the second round (without it the
    // generator advanced twice in comparison mode and the career shifted; 204th)
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| build_latest_scores()));
    SCOREBOARD.record(igual, || {
        // only the differences (the list is large): where they start and how many bytes
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("build_latest_scores: diferencas em {:?}", dif)
    });
}

/// Comparer for `latest_scores_input` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_latest_scores_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_LATEST_SCORES_INPUT)(item)
}

/// Comparer for `match_control_label` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_control_label(state: u8, m: usize, out: *mut u8, size: i32, marks: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(IDX_MATCH_CONTROL_LABEL);
    if m == 0 || out.is_null() || size < 100 || marks.is_null() {
        return o(state, m, out, size, marks);
    }
    let reg = [(out as usize, size.min(0x100) as usize), (marks as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(state, m, out, size, marks), || match_control_label(state, m, out, size, marks));
    SCOREBOARD.record(igual && r == n, || format!("match_control_label(estado {}) original={} rust={} {:x?} x {:x?}", state, r, n, mo, mr));
    r
}

/// Comparer for `close_match_screen` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_close_match_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_CLOSE_MATCH_SCREEN)()
}

/// Comparer for `open_waiting_screen` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_open_waiting_screen(slot: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_OPEN_WAITING_SCREEN)(slot)
}

/// Comparer for `draw_latest_scores_panel` (comparison mode: see the header of motor.rs).
///
/// [pt] O painel é comparado de verdade: as duas desenham a partir do mesmo estado e o gerente de telas, o
/// `Display` da tela atual, os parâmetros (menos o 0x14, a hora que o redesenho 0x724520 grava), o
/// `*topo` e o texto 0xde1f64 têm de sair iguais.
pub(crate) unsafe extern "C" fn comparar_draw_latest_scores_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_LATEST_SCORES_PANEL);
    let mut reg = screen_build_regions();
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7e2 + slot * 0x300) as *const usize);
    if block != 0 {
        reg.push((block, 0xb4));
        reg.push((block + 0xb8, 0x200 - 0xb8));
    }
    reg.push((top as usize, 4));
    reg.push((0xde1f64, 0x100));
    // the list (0x55fba0) writes into a network buffer ([0xae2908] at position [0xae290c]): the position is restored
    // for the second round to write in the same place
    reg.push((0xae290c, 4));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(top, base), || draw_latest_scores_panel(top, base));
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("draw_latest_scores_panel: diferencas em {:?}", dif)
    });
}
