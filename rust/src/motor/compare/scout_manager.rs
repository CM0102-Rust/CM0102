use super::*;

// The comparers of the routines in ../scout_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../scout_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `scout_manager_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7f8750: os relatórios sorteiam e alocam e as buscas mexem nas listas dos humanos: só a original
/// roda. Antes, a cópia em ensaio poda uma cópia dos pedidos (sem os relatórios) e os põe na ordem da original; os
/// pedidos que a original deixa (conta e os 0x11 bytes de cada) têm de ser esses.
pub(crate) unsafe extern "thiscall" fn comparar_scout_manager_daily_update(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCOUT_MANAGER_DAILY_UPDATE);
    if read_u8(G_CURRENT_DAY_SLICE) != 2 {
        return o(this);
    }
    let n0 = read_i32(this + 0x64).max(0) as usize;
    let base = read_usize(this + 0x5c);
    let mut copy: Vec<u8> = (0..n0 * 0x11).map(|k| read_u8(base + k)).collect();
    copy.push(0);
    // the trial run prunes the copy: swaps the vector and the count of `this` for the copy’s and restores them afterwards
    let (b0, c0) = (read_usize(this + 0x5c), read_i32(this + 0x64));
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, copy.as_mut_ptr() as usize);
    let state = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let a1080 = read_usize(G_CONTRACT_LOOKUP_RESULT);
    SCOUT_DAILY_DRY_RUN = true;
    crate::aleatorio::sem_contar(|| scout_manager_daily_update(this));
    SCOUT_DAILY_DRY_RUN = false;
    state.escrever();
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, a1080);
    let n1 = read_i32(this + 0x64).max(0) as usize;
    let sort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    sort(copy.as_mut_ptr() as usize, n1 as i32, 0x11, 0x7fc270);
    let plan: Vec<u8> = copy[..n1 * 0x11].to_vec();
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, b0);
    core::ptr::write_unaligned((this + 0x64) as *mut i32, c0);
    let r = o(this);
    let n2 = read_i32(this + 0x64).max(0) as usize;
    let now: Vec<u8> = (0..n2 * 0x11).map(|k| read_u8(read_usize(this + 0x5c) + k)).collect();
    PLACAR.registrar(r == 1 && now == plan, || format!("scout_manager_daily_update: pedidos {} -> original {} ensaio {}; {:x?} x {:x?}", n0, n2, n1, now, plan));
    r
}

/// Comparer for `send_scout_reports` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7f9890: a original entrega notícias (aloca nas caixas): só ela roda, com o registro das chamadas à
/// 0x7f9a60 ligado; antes, a cópia em ensaio (sem a 0x7f9a60) diz os grupos (jogador, clube, quantos) em ordem, e as
/// chamadas da original têm de ser essas.
pub(crate) unsafe extern "thiscall" fn comparar_send_scout_reports(this: usize, list: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_SCOUT_REPORTS);
    let cursor = read_usize(list + 0xa);
    *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) = Some(Vec::new());
    SCOUT_DAILY_DRY_RUN = true;
    send_scout_reports(this, list);
    SCOUT_DAILY_DRY_RUN = false;
    let plan = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).replace(Vec::new()).unwrap_or_default();
    core::ptr::write_unaligned((list + 0xa) as *mut usize, cursor);
    o(this, list);
    let done = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).take().unwrap_or_default();
    PLACAR.registrar(done == plan, || format!("send_scout_reports(lista {:#x}) original {:x?} ensaio {:x?}", list, done, plan));
}

/// Comparer for `send_scout_report_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7f9a60: a notícia é entregue (aloca na caixa do humano): só a original roda (simplification: o efeito é a
/// caixa de entrada; os grupos que chegam aqui são conferidos pela 0x7f9890). Com o registro ligado, anota a chamada.
pub(crate) unsafe extern "thiscall" fn comparar_send_scout_report_news(this: usize, r: usize, how_many: u32) {
    if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
        g.push((read_usize(r + 0x2b) as u32, read_usize(r + 0x50) as u32, how_many as u8));
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SEND_SCOUT_REPORT_NEWS)(this, r, how_many);
}

/// Comparer for `current_search_has_no_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê); com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_current_search_has_no_club(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CURRENT_SEARCH_HAS_NO_CLUB);
    if matches!(current_human_index(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = current_search_has_no_club(this);
    PLACAR.registrar(r == n, || format!("current_search_has_no_club() original={} rust={}", r, n));
    r
}

/// Comparer for `is_club_in_session_searches` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê); com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_is_club_in_session_searches(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_CLUB_IN_SESSION_SEARCHES);
    if matches!(current_human_index(), Ok(None)) {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = is_club_in_session_searches(this, clube);
    PLACAR.registrar(r == n, || format!("is_club_in_session_searches(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `scout_manager_scout_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a observação (0x7b5be0, original) grava o que os olheiros sabem e os relatórios, a 0x7ae9c0 começa
/// buscas e a 0x7f9890 entrega as notícias: não pode rodar duas vezes. Só a original roda (simplification: ela mesma só grava
/// o vetor e a lista locais, que saem); confere a devolução (1).
pub(crate) unsafe extern "thiscall" fn comparar_scout_manager_scout_match(this: usize, p: usize, result: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SCOUT_MANAGER_SCOUT_MATCH)(this, p, result);
    PLACAR.registrar(r == 1, || format!("scout_manager_scout_match(partida {:x}) original={}", p, r));
    r
}

/// Comparer for `get_edit_session` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê); com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_edit_session(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_EDIT_SESSION);
    if scout_manager_human_manager_index(manager).is_none() {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_edit_session(this, manager);
    PLACAR.registrar(r == n, || format!("get_edit_session(tecnico {:x}) original={:x} rust={:x}", manager, r, n));
    r
}

/// Comparer for `set_edit_session` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas 16 sessões e a devolução; com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_set_edit_session(this: usize, sessao: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SET_EDIT_SESSION);
    if scout_manager_human_manager_index(manager).is_none() {
        return o(this, sessao, manager);
    }
    let reg = [(this + 0xc, 0x40)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, sessao, manager),
        || set_edit_session(this, sessao, manager));
    PLACAR.registrar(igual && r == n, || format!("set_edit_session(sessao {:x}, tecnico {:x}) original={} rust={} {:x?} x {:x?}",
        sessao, manager, r, n, mo, mr));
    r
}

/// Comparer for `get_selected_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura); nos casos de erro, só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_get_selected_list(this: usize, manager: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_SELECTED_LIST);
    if manager_human_index(manager).is_none() {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_selected_list(this, manager);
    PLACAR.registrar(r == n, || format!("get_selected_list(tecnico {:x}) original={} rust={}", manager, r, n));
    r
}

/// Comparer for `set_selected_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos 16 bytes das listas escolhidas, e a devolução; nos casos de erro, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_set_selected_list(this: usize, list: u32, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_SET_SELECTED_LIST);
    if manager_human_index(manager).is_none() {
        return o(this, list, manager);
    }
    let reg = [(this + 0x4c, 0x10)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, list, manager), || set_selected_list(this, list, manager));
    PLACAR.registrar(igual && r == n, || format!("set_selected_list(lista {}, tecnico {:x}) original={} rust={} {:x?} x {:x?}", list as u8, manager, r, n, mo, mr));
    r
}

/// Comparer for `fixture_has_scouted_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fixture_has_scouted_player(this: usize, partida: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FIXTURE_HAS_SCOUTED_PLAYER)(this, partida);
    let n = fixture_has_scouted_player(this, partida);
    PLACAR.registrar(r == n, || format!("fixture_has_scouted_player(partida {:x}) original={} rust={}", partida, r, n));
    r
}

/// Comparer for `is_player_shortlisted` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_player_shortlisted(this: usize, person: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_PLAYER_SHORTLISTED);
    if person == 0 || (manager == 0 && scout_manager_current_human_manager() == 0) {
        return o(this, person, manager);
    }
    let r = o(this, person, manager);
    let n = is_player_shortlisted(this, person, manager);
    PLACAR.registrar(r == n, || format!("is_player_shortlisted(pessoa {:x}, gerente {:x}) original={} rust={}", person, manager, r, n));
    r
}

/// Comparer for `scout_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_scout_record(t: usize, which: i8, manager: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    let n = scout_record(t, which, manager);
    PLACAR.registrar(r == n, || format!("scout_record({:#x}, {}, {:p}) original={:#x} rust={:#x}", t, which, manager, r, n));
    r
}

/// Comparer for `collect_club_scout_targets` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os alvos da cópia antes; a original enche a lista; confere o valor e a lista (cada item, os 0x6e bytes
/// da pessoa).
pub(crate) unsafe extern "thiscall" fn comparar_collect_club_scout_targets(this: usize, clube: *const u32, list: usize) -> u32 {
    let alvos = if clube.is_null() { Vec::new() } else { scouting_targets_of_club(this, *clube) };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, usize) -> u32>(IDX_COLLECT_CLUB_SCOUT_TARGETS)(this, clube, list);
    let mut same = r == (!alvos.is_empty()) as u32;
    if list != 0 {
        let itens = list_items_capped(list);
        same &= itens.len() == alvos.len()
            && itens.iter().zip(&alvos).all(|(&i, &p)| core::slice::from_raw_parts(i as *const u8, 0x6e) == core::slice::from_raw_parts(p as *const u8, 0x6e));
    }
    PLACAR.registrar(same, || format!("collect_club_scout_targets original={} alvos {}", r, alvos.len()));
    r
}

/// Comparer for `remove_scout_request` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_scout_request(this: usize, person: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_REMOVE_SCOUT_REQUEST);
    let n = (*((this + 0x64) as *const i32)).max(0) as usize;
    let reg = [(this + 0x5c, 0xc), (read_u32(this + 0x5c) as usize, n * 17)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, person, clube), || remove_scout_request(this, person, clube));
    PLACAR.registrar(igual, || format!("remove_scout_request({:x}, {:x}) {:x?} x {:x?}", person, clube, mo, mr));
}

/// Comparer for `can_club_scout_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x5a0590 só procura).
pub(crate) unsafe extern "thiscall" fn comparar_can_club_scout_nation(this: usize, nacao: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_CLUB_SCOUT_NATION)(this, nacao, clube);
    if club_or_current_human(clube) == 0 {
        return r;
    }
    let n = can_club_scout_nation(this, nacao, clube);
    PLACAR.registrar(r == n, || format!("can_club_scout_nation({:x}, {:x}) original={} rust={}", nacao, clube, r, n));
    r
}

/// Comparer for `sync_scout_lists` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7f7270: lado a lado nas 10 buscas do técnico (0x2b0 bytes cada, achadas antes pela 0x7f6cd0, que
/// só lê) e na devolução. Sem técnico ou com a edição aberta (a 0x8043c0 mexe na edição): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_sync_scout_lists(this: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SYNC_SCOUT_LISTS);
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        return o(this, manager);
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) || read_usize(this + slot as usize * 4 + 0xc) != 0 {
        return o(this, manager);
    }
    let mut regions = Vec::new();
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l != 0 {
            regions.push((l, 0x2b0usize));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(this, manager), || sync_scout_lists(this, manager));
    PLACAR.registrar(igual && r == n, || format!("sync_scout_lists(técnico {:x}) original={} rust={} {:x?} x {:x?}", m, r, n, mo, mr));
    r
}

/// Comparer for `find_club_search_entry` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7f8610: pela devolução (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_club_search_entry(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_CLUB_SEARCH_ENTRY);
    let r = o(this, p);
    let n = find_club_search_entry(this, p);
    PLACAR.registrar(r == n, || format!("find_club_search_entry(pessoa {:x}) original={:x} rust={:x}", p, r, n));
    r
}

/// Comparer for `scout_manager_write_list_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo texto (até 0x100 bytes do tamanho dado), e a devolução; nos casos de erro, só a
/// original (caixa).
pub(crate) unsafe extern "C" fn comparar_scout_manager_write_list_name(indice: u32, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(IDX_SCOUT_MANAGER_WRITE_LIST_NAME);
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || text == 0 || size < 0x32 {
        return o(indice, text, size);
    }
    let reg = [(text, (size as usize).min(0x100))];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(indice, text, size), || scout_manager_write_list_name(indice, text, size));
    PLACAR.registrar(igual && r == n, || format!("scout_manager_write_list_name(indice {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// Comparer for `has_scout_request` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: consulta pura (a busca binária e a data local): pelo valor; jogador nulo só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_has_scout_request(this: usize, player: usize, clube: usize, so_vencidos: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_HAS_SCOUT_REQUEST);
    if player == 0 {
        return o(this, player, clube, so_vencidos);
    }
    let r = o(this, player, clube, so_vencidos);
    let n = has_scout_request(this, player, clube, so_vencidos);
    PLACAR.registrar(r == n, || format!("has_scout_request(jogador {:x}, clube {:x}, {}) original={} rust={}", player, clube, so_vencidos, r, n));
    r
}
