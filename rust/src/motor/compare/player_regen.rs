use super::*;

// The comparers of the routines in ../player_regen.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../player_regen.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `regenerate_as_youth_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: a rotina tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
/// contrato e manda notícia; não pode rodar duas vezes). A cópia roda no modo Rust (contadores "cqg_7 ...") e é provada
/// pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_regenerate_as_youth_player(this: usize, p: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REGENERATE_AS_YOUTH_PLAYER)(this, p)
}

/// Comparer for `remove_dispute` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem a marca (nada a fazer), lado a lado na marca e no cursor da lista. Com a marca, só a original com o
/// plano: a marca zerada e a conta da lista uma a menos quando a pessoa está nela (a procura feita antes, com o cursor
/// devolvido).
pub(crate) unsafe extern "thiscall" fn comparar_remove_dispute(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_DISPUTE);
    if p == 0 || read_u32_as_usize(this + 4) == 0 {
        return o(this, p);
    }
    let m = read_u32_as_usize(this + 4).wrapping_add(read_i32(p) as isize as usize);
    let lista = read_u32_as_usize(this);
    if *(m as *const u8) == 0 {
        let mut reg = vec![(m, 1)];
        if lista != 0 {
            reg.push((lista, 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p), || remove_dispute(this, p));
        PLACAR.registrar(igual, || format!("remove_dispute(pessoa {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut achou = 0i16;
    let conta_antes = if lista != 0 { read_i16(lista) } else { 0 };
    if lista != 0 {
        let cursor = read_u32_as_usize(lista + 0xa);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32_as_usize(lista + 2) as u32);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        for _ in 0..conta_antes.max(0) {
            let no = proximo(lista);
            if no == 0 {
                break;
            }
            if read_i32(no) == read_i32(p) {
                achou = 1;
                break;
            }
        }
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor as u32);
    }
    o(this, p);
    let conta_depois = if lista != 0 { read_i16(lista) } else { 0 };
    PLACAR.registrar(*(m as *const u8) == 0 && conta_depois == conta_antes - achou,
        || format!("remove_dispute(pessoa {:x}) conta {} -> {} (achou {})", p, conta_antes, conta_depois, achou));
}

/// Comparer for `player_setup_find_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_find_staff(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32) -> usize>(IDX_PLAYER_SETUP_FIND_STAFF)(primeiro, comum, segundo, clube, no_elenco);
    let n = player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco);
    PLACAR.registrar(r == n, || format!("player_setup_find_staff(clube {:x}, elenco {}) original={:x} rust={:x}", clube, no_elenco as u8, r, n));
    r
}

/// Comparer for `player_setup_read_quoted_token` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no destino (até 0x100 bytes) e na devolução.
pub(crate) unsafe extern "C" fn comparar_player_setup_read_quoted_token(origem: usize, destino: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_PLAYER_SETUP_READ_QUOTED_TOKEN);
    if origem == 0 || destino == 0 {
        return o(origem, destino);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(destino, 0x100)], || o(origem, destino), || player_setup_read_quoted_token(origem, destino));
    PLACAR.registrar(igual && r == n, || format!("player_setup_read_quoted_token({:x}) original={:x} rust={:x}", origem, r, n));
    r
}

/// Comparer for `player_setup_type_from_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_type_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_TYPE_FROM_NAME)(nome);
    let n = player_setup_type_from_name(nome);
    PLACAR.registrar(r == n, || format!("player_setup_type_from_name({:x}) original={:x} rust={:x}", nome, r, n));
    r
}

/// Comparer for `process_retirements` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_RETIREMENTS)(this)
}

/// Comparer for `player_regen_post_retirement_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_retirement_news(p: usize, motivo: u32, anunciada: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(IDX_PLAYER_REGEN_POST_RETIREMENT_NEWS)(p, motivo, anunciada)
}

/// Comparer for `player_regen_post_youth_promotion_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_youth_promotion_news(j: usize, c: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize)>(IDX_PLAYER_REGEN_POST_YOUTH_PROMOTION_NEWS)(j, c)
}

/// Comparer for `player_regen_sign_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o contrato novo aloca e o valor sorteia: só a original, conferindo o plano (a vaga, o clube da pessoa, a
/// devolução e o +0x3a do contrato).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_sign_for_club(pessoa: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_SIGN_FOR_CLUB);
    let k = first_free_squad_slot(clube);
    let r = o(pessoa, clube);
    if k == 0x32 {
        PLACAR.registrar(r == 0, || format!("player_regen_sign_for_club({:x}, {:x}) sem vaga original={}", pessoa, clube, r));
        return r;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
    let ok = r == 1 && read_i32(clube + club::SQUAD + 4 * k) == pessoa as i32 && read_i32(pessoa + staff::CLUB_JOB) == clube as i32 && (c == 0 || *((c + 0x3a) as *const u8) == 0);
    PLACAR.registrar(ok, || format!("player_regen_sign_for_club({:x}, {:x}) vaga {} original={}", pessoa, clube, k, r));
    r
}

/// Comparer for `schedule_injury_retirement` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no estado do sorteio, na tabela (ponteiro, conta e capacidade, e as entradas até a
/// capacidade). Com a tabela cheia (conta = capacidade − 1, a 0x7a8510 realoca): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_injury_retirement(this: usize, pessoa: usize, dias: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i16)>(IDX_SCHEDULE_INJURY_RETIREMENT);
    let (n, cap) = (read_i32(this + 8), read_i32(this + 0xc));
    if pessoa == 0 || n >= cap.wrapping_sub(1) || cap <= 0 {
        return o(this, pessoa, dias);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 4, 12));
    reg.push((read_i32(this + 4) as usize, cap as usize * 0x1a));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, dias), || crate::aleatorio::sem_contar(|| schedule_injury_retirement(this, pessoa, dias)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("schedule_injury_retirement({:x}, {}) diferencas em {:x?}", pessoa, dias, dif)
    });
}

/// Comparer for `player_regen_post_retirement_announcement_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a notícia vai para as caixas dos técnicos: só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_retirement_announcement_news(pessoa: usize, motivo: i8) {
    original_fn::<unsafe extern "stdcall" fn(usize, i8)>(IDX_PLAYER_REGEN_POST_RETIREMENT_ANNOUNCEMENT_NEWS)(pessoa, motivo)
}

/// Comparador: o id da ficha antes; a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn comparar_add_player_data_to_pool(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    let n = read_usize(this + 0x40);
    let id = read_usize(read_usize(read_usize(G_STAFF_TABLE).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize) + 0x61)) as u32;
    original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32)>(IDX_ADD_PLAYER_DATA_TO_POOL)(this, pessoa, nacao, segunda);
    let item = read_usize(this + 0x3c).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + 4) as u32, read_usize(item + 8) as u32];
    PLACAR.registrar(read_usize(this + 0x40) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_player_data_to_pool({}, {}, {}) conta {} -> {}, item {:?} (id {})", pessoa, nacao, segunda, n, read_usize(this + 0x40), g, id)
    });
}

/// Comparer for `has_spare_non_player_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_has_spare_non_player_record(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_SPARE_NON_PLAYER_RECORD)(this);
    let n = has_spare_non_player_record(this);
    PLACAR.registrar(r == n, || format!("has_spare_non_player_record({:x}) original={:x} rust={:x}", this, r, n));
    r
}

/// Comparer for `player_regen_check_initial_retirement` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo sorteio, [0xae1080], a saída e a data estática (a cópia sem contar), e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_check_initial_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_INITIAL_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&retirement_decision_regions(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_initial_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_initial_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: como o da 0x7a79c0.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_check_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&retirement_decision_regions(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `build_initial_retirement_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_initial_retirement_list(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INITIAL_RETIREMENT_LIST)(this)
}

/// Comparer for `find_new_retirements` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_new_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIND_NEW_RETIREMENTS)(this)
}

/// Comparer for `player_regen_retire_from_non_playing_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem rotinas com efeito (job_exit_is_memory_only), lado a lado pela pessoa (0x6e bytes) e pelos dois clubes
/// (0x245 bytes cada); senão (vaga do técnico, cancelamento de ofertas, listas do humano) só a original.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_retire_from_non_playing_staff(pessoa: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_PLAYER_REGEN_RETIRE_FROM_NON_PLAYING_STAFF);
    if !job_exit_is_memory_only(pessoa) {
        return o(pessoa);
    }
    let mut reg = job_exit_regions(pessoa);
    reg.retain(|r| r.0 != 0);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || player_regen_retire_from_non_playing_staff(pessoa));
    PLACAR.registrar(igual, || format!("player_regen_retire_from_non_playing_staff(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// Comparer for `player_regen_score_non_player_candidate` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo estado do sorteio (a cópia sem contar), e a nota.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_score_non_player_candidate(pessoa: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> i32>(IDX_PLAYER_REGEN_SCORE_NON_PLAYER_CANDIDATE);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa),
        || crate::aleatorio::sem_contar(|| player_regen_score_non_player_candidate(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_score_non_player_candidate({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `player_regen_compare_retirements` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_RETIREMENTS)(a, b);
    let n = player_regen_compare_retirements(a, b);
    PLACAR.registrar(r == n, || format!("player_regen_compare_retirements({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_regen_compare_queue_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_QUEUE_ENTRIES)(a, b);
    let n = player_regen_compare_queue_entries(a, b);
    PLACAR.registrar(r == n, || format!("player_regen_compare_queue_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `retire_from_playing_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela pessoa (+0x24..+0x3d), pelas 50 vagas do clube e da seleção e pelo aviso em
/// [0xde1f64]. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha, então a tática fica fora.
pub(crate) unsafe extern "stdcall" fn comparar_retire_from_playing_staff(p: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_RETIRE_FROM_PLAYING_STAFF);
    let mut reg = vec![(p + 0x24, 0x1a), (0xde1f64, 0x80)];
    for campo in [0x39usize, 0x24] {
        let time = read_usize(p + campo);
        if time != 0 {
            reg.push((time + 0xd7, 200));
        }
    }
    let (antes_clube, antes_cargo) = (read_usize(p + 0x39), read_i8(p + 0x3d));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p), || retire_from_playing_staff(p));
    PLACAR.registrar(igual, || format!("retire_from_playing_staff(id {}, clube {:x}, cargo {}) {:x?} x {:x?}", read_usize(p), antes_clube, antes_cargo, mo, mr));
}

/// Comparer for `pick_best_club_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x7ad220 sorteia), pelo clube devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_pick_best_club_for_player(this: usize, jogador: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_BEST_CLUB_FOR_PLAYER);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| pick_best_club_for_player(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("pick_best_club_for_player(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(jogador), r, n, mo, mr));
    r
}

/// Comparer for `pick_club_from_nation_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio, pelo clube devolvido e pelo cursor (+0xa) das listas
/// que podem ser percorridas (a da nação do jovem e a de [0x9cf2e4]).
pub(crate) unsafe extern "thiscall" fn comparar_pick_club_from_nation_list(this: usize, jogador: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_CLUB_FROM_NATION_LIST);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for n in [read_usize(read_usize(jogador + 0x1a)), read_usize(KEY_NATION_ENGLAND)] {
        let l = nation_club_list(this, n);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| pick_club_from_nation_list(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("pick_club_from_nation_list(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(jogador), r, n, mo, mr));
    r
}

/// Comparador: a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn comparar_add_spare_non_player_record(this: usize, id: u32, nacao: u32, segunda: u32) {
    let n = read_usize(this + 0x20);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_ADD_SPARE_NON_PLAYER_RECORD)(this, id, nacao, segunda);
    let item = read_usize(this + 0x1c).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + 4) as u32, read_usize(item + 8) as u32];
    PLACAR.registrar(read_usize(this + 0x20) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_spare_non_player_record({}, {}, {}) conta {} -> {}, item {:?}", id, nacao, segunda, n, read_usize(this + 0x20), g)
    });
}

/// Comparador: o registro copiado antes; a original acrescenta (pode realocar); confere a conta e os campos.
pub(crate) unsafe extern "thiscall" fn comparar_add_retirement(this: usize, r: usize) {
    let n = read_usize(this + 8);
    let esperado = retirement_entry_fields(r);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_RETIREMENT)(this, r);
    let item = read_usize(this + 4).wrapping_add(n.wrapping_mul(26));
    let g = retirement_entry_fields(item);
    PLACAR.registrar(read_usize(this + 8) == n.wrapping_add(1) && g == esperado, || {
        format!("add_retirement conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + 8), g, esperado)
    });
}

/// Comparer for `queue_staff_for_regen` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original sorteia e realoca (não pode rodar duas vezes): o plano antes (o que entra em cada lista);
/// confere as contas e os itens (a nação sorteada só pela faixa).
pub(crate) unsafe extern "thiscall" fn comparar_queue_staff_for_regen(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN);
    let id = read_usize(entrada) as i32;
    if id < 0 || id > *(G_STAFF_COUNT as *const i32) {
        return o(this, entrada);
    }
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let (n1, n2, n3) = (read_usize(this + 0x14), read_usize(this + 0x20), read_usize(this + 0x40));
    let datas = read_usize(entrada + 4) as u32;
    let np = read_usize(pessoa + staff::NON_PLAYER);
    let jp = read_usize(pessoa + staff::PLAYER);
    let nac = read_usize(pessoa + staff::NATION);
    let reserva = (np != 0).then(|| (read_usize(np) as u32, (nac != 0).then(|| read_usize(nac) as u32), read_usize(pessoa) as u32));
    let banco = (jp != 0).then(|| (read_usize(jp) as u32, read_usize(nac) as u32, read_usize(pessoa) as u32));
    o(this, entrada);
    let fila = read_usize(this + 0x10).wrapping_add(n1.wrapping_mul(12));
    let mut ok = read_usize(this + 0x14) == n1 + 1 && read_usize(fila) as i32 == id && read_usize(fila + 4) as u32 == datas;
    ok &= read_usize(this + 0x20) == n2 + reserva.is_some() as usize;
    if let Some((rid, rnac, rp)) = reserva {
        let i = read_usize(this + 0x1c).wrapping_add(n2 << 4);
        let gn = read_usize(i + 4) as u32;
        ok &= read_usize(i) as u32 == rid && read_usize(i + 0xc) as u32 == rp
            && rnac.map_or(gn < (*(G_NATION_COUNT as *const i32)).max(1) as u32, |v| gn == v);
    }
    ok &= read_usize(this + 0x40) == n3 + banco.is_some() as usize;
    if let Some((bid, bnac, bp)) = banco {
        let i = read_usize(this + 0x3c).wrapping_add(n3 << 4);
        ok &= read_usize(i) as u32 == bid && read_usize(i + 4) as u32 == bnac && read_usize(i + 0xc) as u32 == bp;
    }
    PLACAR.registrar(ok, || {
        format!("queue_staff_for_regen({}) contas {} {} {} -> {} {} {}, reserva {:?}, banco {:?}", id, n1, n2, n3,
            read_usize(this + 0x14), read_usize(this + 0x20), read_usize(this + 0x40), reserva, banco)
    });
}

/// Comparer for `remove_retirement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_retirement(this: usize, chave: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_RETIREMENT);
    let n = (*((this + 8) as *const i32)).max(0) as usize;
    let reg = [(this + 4, 8), (read_u32(this + 4) as usize, n * 26)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, chave), || remove_retirement(this, chave));
    PLACAR.registrar(igual, || format!("remove_retirement {:x?} x {:x?}", mo, mr));
}

/// Comparer for `player_regen_check_unemployed_retirement` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pela saída (26 bytes), a cópia em sem_contar.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_check_unemployed_retirement(pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_UNEMPLOYED_RETIREMENT);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 0x1a));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_unemployed_retirement(pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_unemployed_retirement({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `regen_queue_entry_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_regen_queue_entry_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_REGEN_QUEUE_ENTRY_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || regen_queue_entry_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("regen_queue_entry_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `player_regen_pick_staff_for_non_player_data` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x7a7150 sorteia (0x90cfc0) a cada pessoa; lado a lado pelo estado do sorteio, a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_pick_staff_for_non_player_data(this: usize, nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_REGEN_PICK_STAFF_FOR_NON_PLAYER_DATA);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao),
        || crate::aleatorio::sem_contar(|| player_regen_pick_staff_for_non_player_data(this, nacao)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_pick_staff_for_non_player_data({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparer for `remove_spare_non_player_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_spare_non_player_record(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_SPARE_NON_PLAYER_RECORD);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&list16_regions(regen, 0x1c, 0x20), || o(regen, id), || remove_spare_non_player_record(regen, id));
    PLACAR.registrar(igual, || format!("remove_spare_non_player_record({}) {:x?} x {:x?}", id, mo, mr));
}

/// Comparer for `remove_player_data_from_pool` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_player_data_from_pool(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_PLAYER_DATA_FROM_POOL);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&list16_regions(regen, 0x3c, 0x40), || o(regen, id), || remove_player_data_from_pool(regen, id));
    PLACAR.registrar(igual, || format!("remove_player_data_from_pool({}) {:x?} x {:x?}", id, mo, mr));
}

/// Comparer for `find_retirement_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_retirement_entry(regen: usize, id: *const i32, por_lesao: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u32) -> usize>(IDX_FIND_RETIREMENT_ENTRY)(regen, id, por_lesao);
    let n = find_retirement_entry(regen, id, por_lesao);
    PLACAR.registrar(r == n, || format!("find_retirement_entry({}, {}) original={:#x} rust={:#x}", *id, por_lesao, r, n));
    r
}

/// Comparer for `player_regen_score_club_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_score_club_for_player(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(IDX_PLAYER_REGEN_SCORE_CLUB_FOR_PLAYER);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(jogador, clube, elenco), || crate::aleatorio::sem_contar(|| player_regen_score_club_for_player(jogador, clube, elenco)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_score_club_for_player(elenco {}) original={} rust={} sorteio igual={}", elenco, r, n, igual));
    r
}

/// Comparer for `retire_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_retire_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RETIRE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || retire_record_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("retire_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `player_setup_injury_from_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (o al; só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_injury_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_INJURY_FROM_NAME)(nome);
    let n = player_setup_injury_from_name(nome);
    PLACAR.registrar(r as u8 == n as u8, || format!("player_setup_injury_from_name({:x}) original={:x} rust={:x}", nome, r as u8, n as u8));
    r
}

/// Comparer for `build_nation_club_lists` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca as listas: o plano antes (os ids dos clubes de cada nação, na ordem); a original roda;
/// confere os itens de cada lista.
pub(crate) unsafe extern "thiscall" fn comparar_build_nation_club_lists(this: usize) {
    let n = read_i32(G_NATION_COUNT).max(0) as usize;
    let mut plano: Vec<Vec<usize>> = vec![Vec::new(); n];
    let base = read_usize(G_CLUB_TABLE);
    for j in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let clube = base + j * club::SIZE;
        let nacao = injury_youth_list_nation(clube);
        if nacao != 0 && (read_i32(nacao) as usize) < n {
            plano[read_i32(nacao) as usize].push(read_usize(clube));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_CLUB_LISTS)(this);
    let errada = (0..n).find(|&k| {
        let l = read_usize(read_usize(this) + 4 * k);
        let ids: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().map(|e| read_usize(e)).collect() } else { Vec::new() };
        l != 0 && ids != plano[k]
    });
    PLACAR.registrar(errada.is_none(), || format!("build_nation_club_lists: a lista da nação {:?} difere do plano", errada));
}

/// Comparer for `process_regen_queue` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: a 0x7abac0 (o regen jovem) aloca o contrato e manda notícia e as reservas de técnico refazem pessoas
/// achadas pelo caminho: só a original, conferindo o plano da fila. Sem entrada vencida e sem reservas a refazer, os dois
/// lado a lado no cabeçalho (nada muda). Com vencidas (o regen pode pôr entradas novas na fila), reservas (mais de 2)
/// ou id inválido: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_regen_queue(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_REGEN_QUEUE);
    if read_i32(this + 0x20) > 2 {
        crate::comparacao::ramo("cpw_10 reservas so na original");
        return o(this);
    }
    let c0 = read_i32(this + 0x14);
    let base = read_usize(this + 0x10);
    let mut k = 0i32;
    if read_i32(this + 0x40) != 0 {
        while k < c0 {
            let e = base + k as usize * 0xc;
            if !is_regen_entry_due(e) {
                break;
            }
            let id = read_i32(e);
            if id < 0 || id > read_i32(G_STAFF_COUNT) {
                return o(this); // simplification: a caixa de erro só na original
            }
            k += 1;
        }
    }
    if k == 0 {
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x44)], || o(this), || process_regen_queue(this));
        PLACAR.registrar(igual, || format!("process_regen_queue({:x}) sem vencidas {:x?} x {:x?}", this, mo, mr));
        return;
    }
    // o regen (0x7abac0) pode pôr entradas novas na fila (na carga do save, 4 vencidas de 10 e a conta caiu só 1)
    crate::comparacao::ramo("cpw_10 vencidas so na original");
    o(this);
}

/// Comparer for `take_spare_non_player_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_take_spare_non_player_record(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32) -> u32>(IDX_TAKE_SPARE_NON_PLAYER_RECORD)(this, id, nacao, segunda)
}

/// Comparer for `record_bits` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_record_bits(p: *const i32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(*const i32) -> u8>(IDX_RECORD_BITS)(p);
    let n = record_bits(p);
    PLACAR.registrar(r == n, || format!("record_bits({:p}) original={} rust={}", p, r, n));
    r
}
