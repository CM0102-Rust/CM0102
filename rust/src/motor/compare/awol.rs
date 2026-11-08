use super::*;

// The comparers of the routines in ../awol.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../awol.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `clear_awol` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_awol(this: usize, pessoa: usize) {
    compare_toggle_mark_and_unlist(this, pessoa, 1, 0, original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_AWOL), "clear_awol");
}

/// Comparer for `clear_missed_training` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_missed_training(this: usize, pessoa: usize) {
    compare_toggle_mark_and_unlist(this, pessoa, 2, 4, original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_MISSED_TRAINING), "clear_missed_training");
}

/// Comparer for `awol_post_missed_training_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original monta a notícia e a entrega (aloca na caixa): só ela roda, com os sorteios rastreados; depois
/// a cópia em ensaio refaz o caminho com os sorteios da fita, sem entregar. Confere que os sorteios do ensaio são o
/// começo da fita da original (o resto é da entrega) com os mesmos n, e anota o destino (a original não o expõe).
pub(crate) unsafe extern "thiscall" fn comparar_awol_post_missed_training_news(this: usize, jogador: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_AWOL_POST_MISSED_TRAINING_NEWS);
    let ((), tape_lock) = crate::aleatorio::rastrear(|| o(this, jogador, clube));
    let n = tape_lock.len();
    *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE) = Some((tape_lock, 0, false));
    AWOL_NEWS_DESTINATION = (0, 0);
    crate::aleatorio::sem_contar(|| awol_post_missed_training_news(this, jogador, clube));
    let (k, falhou) = match &*core::ptr::addr_of!(AWOL_NEWS_TAPE) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE) = None;
    let destino = AWOL_NEWS_DESTINATION;
    PLACAR.registrar(!falhou && k <= n, || format!("awol_post_missed_training_news(jogador {:#x}, clube {:#x}) tape_lock de {} sorteios, o ensaio usou {} (n diferente: {}), destino {:x?}",
        jogador, clube, n, k, falhou, destino));
}

/// Comparer for `awol_daily_update` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_awol_daily_update(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AWOL_DAILY_UPDATE)(this)
}

/// Comparer for `has_missed_training` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da falta ao treino: lado a lado pela devolução (byte baixo) e pelo [0xae1080]; sem contrato com o bit 2,
/// só a original (a 0x4197c0 tira e libera o nó da lista).
pub(crate) unsafe extern "thiscall" fn comparar_has_missed_training(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HAS_MISSED_TRAINING);
    if pessoa != 0 && read_i32(pessoa) < read_i32(G_STAFF_COUNT) && read_u8(read_usize(this + 0xc).wrapping_add(read_i32(pessoa) as usize)) & 2 != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let k = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
        if k == 0 || read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(club::SIZE)) == 0 {
            // simplification: tira a falta da lista (libera o nó): só a original
            return o(this, pessoa);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, pessoa), || has_missed_training(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("has_missed_training(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// Comparer for `has_fitness_incident` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do atraso: lado a lado pela devolução (byte baixo) e pelo cursor da lista.
pub(crate) unsafe extern "thiscall" fn comparar_has_fitness_incident(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HAS_FITNESS_INCIDENT);
    let l = read_usize(this + 8);
    let regioes: Vec<(usize, usize)> = if l != 0 { vec![(l + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa), || has_fitness_incident(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("has_fitness_incident(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// Comparer for `miss_training` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da falta marcada, em ensaio: a cópia para na decisão (0 nada, 1 some, 2 marca a falta) e a original roda
/// a partir do mesmo estado (sorteio, marca da pessoa, [0xae1080]); a decisão da original sai da marca (bit 1 novo:
/// sumiu; bit 2 novo: faltou). Sem decisão, o estado inteiro tem de bater (os mesmos sorteios). O byte do caso (+0xc do
/// registro) não entra: na original é resíduo da pilha.
pub(crate) unsafe extern "thiscall" fn comparar_miss_training(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MISS_TRAINING);
    if pessoa == 0 || read_usize(pessoa + staff::PLAYER) == 0 || read_i32(pessoa) >= read_i32(G_STAFF_COUNT) {
        return o(this, pessoa);
    }
    let m = read_usize(this + 0xc).wrapping_add(read_i32(pessoa) as usize);
    let mut regioes = vec![(m, 1), (G_CONTRACT_LOOKUP_RESULT, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    MISS_TRAINING_DRY_RUN = true;
    MISS_TRAINING_DECISION = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| miss_training(this, pessoa)));
    MISS_TRAINING_DRY_RUN = false;
    let n = MISS_TRAINING_DECISION;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let b0 = read_u8(m);
    o(this, pessoa);
    let novo = read_u8(m) & !b0;
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let r = if novo & 2 != 0 { 2 } else if novo & 1 != 0 { 1 } else { 0 };
    let igual = (r == n || (n == 1 && b0 & 1 != 0)) && (n != 0 || mo == mr);
    PLACAR.registrar(igual, || format!("miss_training(pessoa {:#x}) original={} {:x?} rust={} {:x?}", pessoa, r, mo, n, mr));
}

/// Comparer for `end_absence` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do fim da ausência, em ensaio: a cópia sem a notícia e sem tirar da lista; a marca da pessoa e, sem
/// notícia, o [0xae1080] têm de bater com os da original.
pub(crate) unsafe extern "thiscall" fn comparar_end_absence(this: usize, r: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_END_ABSENCE);
    if r == 0 {
        return o(this, r);
    }
    let m = read_usize(this + 0xc).wrapping_add(read_i32(r) as usize);
    let regioes = [(m, 1), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    MISS_TRAINING_DRY_RUN = true;
    MISS_TRAINING_DECISION = 0;
    crate::aleatorio::pausar_rastro(|| end_absence(this, r));
    MISS_TRAINING_DRY_RUN = false;
    let noticia = MISS_TRAINING_DECISION != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, r);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = mo.0[0] == mr.0[0] && (noticia || mo.0[1] == mr.0[1]);
    PLACAR.registrar(igual, || format!("end_absence(marca {:#x}) original={:x?} rust={:x?}", m, mo, mr));
}

/// Comparer for `report_poor_fitness` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do incidente: a original roda sozinha (põe o registro na lista, dá a notícia e multa) e confere o
/// plano: jogador e clube não nulos põem logo depois da cauda de antes (ou na cabeça, lista vazia) o registro com o
/// jogador, o clube e a data de hoje (16 bytes; o byte +0x10 é sobra de pilha).
pub(crate) unsafe extern "thiscall" fn comparar_report_poor_fitness(this: usize, jogador: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_REPORT_POOR_FITNESS);
    let l = read_usize(this + 8);
    if jogador == 0 || clube == 0 || l == 0 {
        return o(this, jogador, clube);
    }
    let cauda = read_usize(l + 6);
    let mut plano = [0u8; 0x10];
    plano[..4].copy_from_slice(&read_u32(jogador).to_le_bytes());
    plano[4..8].copy_from_slice(&read_u32(clube).to_le_bytes());
    plano[8..].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    o(this, jogador, clube);
    let no = if cauda != 0 { read_usize(cauda + 6) } else { read_usize(l + 2) };
    let dados = if no != 0 { read_usize(no) } else { 0 };
    let igual = dados != 0 && core::slice::from_raw_parts(dados as *const u8, 0x10) == plano;
    PLACAR.registrar(igual, || format!("report_poor_fitness(jogador {:#x}, clube {:#x}) plano={:x?} lista: no {:#x} dados {:#x}", jogador, clube, plano, no, dados));
}

/// Comparer for `is_player_awol` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x418f60 (quente: sem alocar): lado a lado em [0xae1080] e no al devolvido; com o bit, sem `só_ler` e
/// sem contrato ou clube (a 0x4196a0 tira e libera o nó da lista: não pode rodar duas vezes), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_is_player_awol(this: usize, p: usize, so_ler: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_IS_PLAYER_AWOL);
    if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && so_ler == 0 && read_u8(read_usize(this + 0xc) + read_usize(p)) & 1 != 0 {
        let c = injury_main_contract_stored(p); // a original o refaz igual (e regrava [0xae1080])
        if c == 0 || injury_club_of_contract(c) == 0 {
            return o(this, p, so_ler);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, p, so_ler), || is_player_awol(this, p, so_ler));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("is_player_awol(pessoa {:#x}, so_ler {}) original={} {:x?} rust={} {:x?}", p, so_ler, r as u8, mo, n as u8, mr));
    r
}
