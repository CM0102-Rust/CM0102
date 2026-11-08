use super::*;

// The comparers of the routines in ../comp/fix_man.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/fix_man.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fix_man_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do dia do calendário: só a original (simplification: as notícias vão aos técnicos, os nós dos amistosos e
/// das filas são liberados e os jogos de reservas entram no calendário: não roda duas vezes). Confere o que a rotina
/// garante: a fila das notícias some ([+0x1128] = 0 se havia) e a lista das conferências fica vazia.
pub(crate) unsafe extern "thiscall" fn comparar_fix_man_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIX_MAN_DAILY_UPDATE);
    let havia = read_u32_as_usize(this + 0x1128);
    let antes = read_i16(G_FIXTURE_LEGS_LIST);
    o(this);
    let fila = read_u32_as_usize(this + 0x1128);
    let depois = read_i16(G_FIXTURE_LEGS_LIST);
    PLACAR.registrar((havia == 0 || fila == 0) && (antes == 0 || depois == 0),
        || format!("fix_man_daily_update: fila das notícias {:x} -> {:x}, conferências {} -> {}", havia, fila, antes, depois));
}

/// Comparer for `get_fixture_list` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da lista do dia: com o ano no bloco ou no outro bloco, as duas (só leem) e a mesma lista; senão só a
/// original (simplification: troca o bloco do outro ano, que grava e lê o arquivo, ou aloca um bloco novo).
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_list(this: usize, ano: u32, dia: u32, fatia: u32, tipo: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize>(IDX_GET_FIXTURE_LIST);
    let outro = read_u32_as_usize(this + 0x112c);
    let direto = read_i16(this + 0x1130) == ano as i16 || (outro != 0 && read_i16(outro + 0x1130) == ano as i16);
    if !direto {
        return o(this, ano, dia, fatia, tipo);
    }
    let r = o(this, ano, dia, fatia, tipo);
    let n = get_fixture_list(this, ano, dia, fatia, tipo);
    PLACAR.registrar(r == n, || format!("get_fixture_list(ano {} dia {} fatia {} tipo {}): original={:x} rust={:x}",
        ano as u16, dia as u16, fatia as u8, tipo as u8 as i8, r, n));
    r
}

/// Comparer for `load_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da carga dos jogos do ano: só a original (simplification: lê o arquivo e aloca os nós das listas; não roda
/// duas vezes). Confere o nome gravado em +0x1133.
pub(crate) unsafe extern "thiscall" fn comparar_load_fixtures(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_FIXTURES)(this);
    let esperado = format!("fixtures_{}.tmp", read_i16(this + 0x1130) as u16);
    let s = core::slice::from_raw_parts((this + 0x1133) as *const u8, esperado.len() + 1);
    PLACAR.registrar(&s[..esperado.len()] == esperado.as_bytes() && s[esperado.len()] == 0,
        || format!("load_fixtures: nome {:?} (esperado {})", s, esperado));
}

/// Comparer for `collect_day_clashes` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador dos conflitos de um dia: a cópia em ensaio (guarda os jogos em vez de acrescentar, com uma gravidade
/// própria) e a original a partir do mesmo estado; confere a gravidade e os nós novos no fim da lista (os 0x4f bytes de
/// cada jogo, na ordem). As listas dos dias (0x5abf10) têm o cursor refeito nas duas.
pub(crate) unsafe extern "thiscall" fn comparar_collect_day_clashes(this: usize, jogo: usize, grav: *mut i32, lista: usize, nacoes: usize, n: u32, data: usize, desloc: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(IDX_COLLECT_DAY_CLASHES);
    if jogo == 0 || lista == 0 || grav.is_null() || read_usize(jogo + 0x1c) == 0 || read_usize(jogo + 0x20) == 0 {
        return o(this, jogo, grav, lista, nacoes, n, data, desloc);
    }
    let mut g = *grav;
    DAY_CLASHES_APPENDED.clear();
    DAY_CLASHES_DRY_RUN = true;
    collect_day_clashes(this, jogo, &mut g, lista, nacoes, n, data, desloc);
    DAY_CLASHES_DRY_RUN = false;
    let esperado = fixtures_bytes(&DAY_CLASHES_APPENDED);
    let ne = DAY_CLASHES_APPENDED.len();
    let antes = fix_man_list_items(lista);
    o(this, jogo, grav, lista, nacoes, n, data, desloc);
    let depois = fix_man_list_items(lista);
    let igual = *grav == g && depois.len() == antes.len() + ne && depois[..antes.len()] == antes[..] && fixtures_bytes(&depois[antes.len()..]) == esperado;
    PLACAR.registrar(igual, || format!("collect_day_clashes(jogo {:x}, desloc {}) gravidade original={} rust={}, jogos novos original={} rust={}", jogo, desloc as u8 as i8, *grav, g, depois.len().saturating_sub(antes.len()), ne));
}

/// Comparer for `find_fixture_clashes` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador dos conflitos de um jogo: a original e a cópia, cada uma com a sua lista nova, a partir do mesmo estado
/// (a original não muda o jogo: só cria a lista e mexe nos cursores das listas dos dias, que as duas refazem); confere a
/// devolução (lista ou 0), a saída e os jogos das duas listas (0x4f bytes cada, na ordem), e destrói e libera a lista
/// da cópia como o calendário faz (0x68d040, 0x944c94).
pub(crate) unsafe extern "thiscall" fn comparar_find_fixture_clashes(this: usize, jogo: usize, saida: *mut i32, dois_dias: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_FIND_FIXTURE_CLASHES);
    if jogo == 0 || saida.is_null() {
        return o(this, jogo, saida, dois_dias);
    }
    let r = o(this, jogo, saida, dois_dias);
    let go = *saida;
    let mut gr: i32 = 0;
    let nr = find_fixture_clashes(this, jogo, &mut gr, dois_dias);
    let (lo, lr) = (fixtures_bytes(&fix_man_list_items(r)), fixtures_bytes(&fix_man_list_items(nr)));
    let igual = (r == 0) == (nr == 0) && go == gr && lo == lr;
    PLACAR.registrar(igual, || format!("find_fixture_clashes(jogo {:x}) original={:x} saida={} jogos={} rust={:x} saida={} jogos={}", jogo, r, go, lo.len() / 0x4f, nr, gr, lr.len() / 0x4f));
    if nr != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destruir(nr);
        liberar(nr);
    }
    r
}

/// Comparer for `fix_man_load_fixture_leg_checks` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da carga das conferências: só a original (simplification: lê o arquivo e aloca os nós da lista; não pode rodar
/// duas vezes).
pub(crate) unsafe extern "C" fn comparar_fix_man_load_fixture_leg_checks() {
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_LOAD_FIXTURE_LEG_CHECKS)()
}

/// Comparer for `find_new_fixture_date` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da nova data: lado a lado no jogo (0x4f bytes); os conflitos de dentro (0x5ad780) criam e destroem a
/// lista deles. Só a original quando o jogo entra na lista de conferência das pernas (aloca o nó).
pub(crate) unsafe extern "thiscall" fn comparar_find_new_fixture_date(this: usize, jogo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIND_NEW_FIXTURE_DATE);
    let units_digit: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_ROUND_VARIANT);
    // a caixa da janela sem data (+0x24, tipo 2) apareceria duas vezes: só a original
    if jogo == 0 || matches!(units_digit(read_u16(jogo + 0x30) as i16 as i32), 3 | 4) || (read_u8(jogo + 0x3a) == 2 && read_u16(jogo + 0x24) == 0) {
        return o(this, jogo);
    }
    let reg = [(jogo, 0x4f)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || find_new_fixture_date(this, jogo));
    PLACAR.registrar(igual, || format!("find_new_fixture_date(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// Comparer for `add_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da entrada no calendário: só a original (simplification: grava nas listas do calendário, aloca nós e o
/// bloco do outro ano, remarca e entrega notícias; não pode rodar duas vezes). A cópia segue no modo Rust, com os
/// contadores.
pub(crate) unsafe extern "thiscall" fn comparar_add_fixture(this: usize, jogo: usize, zerar: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_FIXTURE)(this, jogo, zerar)
}

/// Comparer for `set_year` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do ano do calendário: só a original (simplification: grava e lê os arquivos dos jogos do ano e refaz todas
/// as listas; não pode rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_set_year(this: usize, ano: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_YEAR)(this, ano)
}

/// Comparer for `fix_man_save_club_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5af040: só a original.
pub(crate) unsafe extern "C" fn comparar_fix_man_save_club_dates() {
    // simplification: grava o arquivo do save (não pode rodar duas vezes); a cópia é provada pela prova do save (o .qst
    // gravado no modo rust com e sem a leva, item a item: club_dates.tmp)
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_SAVE_CLUB_DATES)()
}

/// Comparer for `fix_man_load_club_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5aee90: só a original.
pub(crate) unsafe extern "C" fn comparar_fix_man_load_club_dates() {
    // simplification: aloca a tabela global e lê o arquivo (não pode rodar duas vezes); a cópia é provada pela carga do
    // save de férias na prova do save (levas/N/CARGA) e, sem o arquivo, pela carreira nova
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_LOAD_CLUB_DATES)()
}

/// Comparer for `fix_man_save_fixture_leg_checks` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5af4a0: só a original.
pub(crate) unsafe extern "C" fn comparar_fix_man_save_fixture_leg_checks() {
    // simplification: grava o arquivo do save e anda o cursor da lista; a cópia é provada pela prova do save
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_SAVE_FIXTURE_LEG_CHECKS)()
}

/// Comparer for `resolve_same_day_clash` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador em ensaio: a rotina só grava o cursor da lista (+0xa, lido pela 0x68d1b0) e o resto vai pelas
/// chamadas do calendário (tirar, juntar, conflitos que alocam a lista, a notícia), que não podem rodar duas vezes.
/// A cópia roda em ensaio (para na decisão, sem a caixa) e a original depois, a partir do mesmo estado: lado a lado no
/// cabeçalho da lista (+2 começo, +0xa cursor). Sem o jogo ou o primeiro da lista: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_resolve_same_day_clash(this: usize, jogo: usize, conflitos: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_RESOLVE_SAME_DAY_CLASH);
    let cabeca = read_usize(conflitos + 2);
    if jogo == 0 || cabeca == 0 || read_usize(cabeca) == 0 {
        return o(this, jogo, conflitos);
    }
    let reg = [(conflitos + 2, 0xc)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    SAME_DAY_CLASH_DRY_RUN = true;
    resolve_same_day_clash(this, jogo, conflitos);
    SAME_DAY_CLASH_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    let r = o(this, jogo, conflitos);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr && (r == 0 || r == 1), || format!("resolve_same_day_clash(jogo {:#x}) devolveu {}, cursor da lista {:x?} x {:x?}", jogo, r, mo, mr));
    r
}

/// Comparer for `move_nearby_club_fixtures` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_nearby_club_fixtures(this: usize, jogo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MOVE_NEARBY_CLUB_FIXTURES)(this, jogo)
}

/// Comparer for `reschedule_club_fixtures_near_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_reschedule_club_fixtures_near_date(this: usize, clube: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE)(this, clube, data)
}

/// Comparer for `queue_fixture_moved_news` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia, em ensaio. Já na fila: lado a lado no cabeçalho da fila e na notícia achada (a data nova).
/// Nova: a cópia monta a notícia sem pô-la na fila e a original roda; a fila tem de crescer um e o último item (a cauda,
/// +6, nó +0) ser a notícia da cópia nos 0xde bytes que o construtor (0x77bbe0) grava; o último dos 0xdf copiados é
/// sobra de pilha na original (a cópia leva zero).
pub(crate) unsafe extern "thiscall" fn comparar_queue_fixture_moved_news(this: usize, f: usize, causa: usize, velha: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_QUEUE_FIXTURE_MOVED_NEWS);
    let l = read_usize(this + 0x1128);
    let alvo = if l != 0 { find_queued_fixture_news(l, f) } else { 0 };
    if alvo != 0 {
        let regioes = [(this + 0x1128, 4), (l, 0xe), (alvo, 0xdf)];
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, f, causa, velha), || queue_fixture_moved_news(this, f, causa, velha));
        PLACAR.registrar(igual, || format!("queue_fixture_moved_news(jogo {:#x}, na fila) original={:x?} rust={:x?}", f, mo, mr));
        return;
    }
    let mut regioes = vec![(this + 0x1128, 4)];
    if l != 0 {
        regioes.push((l, 0xe));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let conta = if l != 0 { read_i16_as_i32(l) } else { 0 };
    FIXTURE_MOVED_NEWS_DRY_RUN = true;
    FIXTURE_MOVED_NEWS_WAS_BUILT = false;
    crate::aleatorio::pausar_rastro(|| queue_fixture_moved_news(this, f, causa, velha));
    FIXTURE_MOVED_NEWS_DRY_RUN = false;
    antes.escrever();
    o(this, f, causa, velha);
    let l2 = read_usize(this + 0x1128);
    let montada = FIXTURE_MOVED_NEWS_WAS_BUILT;
    let igual = montada
        && l2 != 0
        && read_i16_as_i32(l2) == conta + 1
        && read_usize(read_usize(l2 + 6)) != 0
        && core::slice::from_raw_parts(read_usize(read_usize(l2 + 6)) as *const u8, 0xde)
            == core::slice::from_raw_parts(core::ptr::addr_of!(FIXTURE_MOVED_NEWS_BUILT) as *const u8, 0xde);
    PLACAR.registrar(igual, || format!("queue_fixture_moved_news(jogo {:#x}, nova) montada={} conta {} -> {}", f, montada, conta, if l2 != 0 { read_i16_as_i32(l2) } else { -1 }));
}

/// Comparer for `find_stored_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia só andam o cursor da lista (e a primeira pode criar o bloco do ano, que a segunda
/// acha pronto): pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_stored_fixture(this: usize, f: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_STORED_FIXTURE)(this, f);
    let n = find_stored_fixture(this, f);
    PLACAR.registrar(r == n, || format!("find_stored_fixture(jogo {:x}) original={:x} rust={:x}", f, r, n));
    r
}

/// Comparer for `insert_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: acrescenta à lista (não roda duas vezes): a lista é achada antes pela cópia, a original guarda e a conta
/// tem de subir 1 com a cauda nova igual aos 0x4f bytes do jogo.
pub(crate) unsafe extern "thiscall" fn comparar_insert_fixture(this: usize, f: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_INSERT_FIXTURE);
    let l = find_fixture_list(this, f);
    if l == 0 {
        return o(this, f); // simplification: sem memória para o bloco, só a original (a caixa)
    }
    let conta = read_u16(l);
    o(this, f);
    let cauda = read_u32_as_usize(l + 6);
    let igual = read_u16(l) == conta.wrapping_add(1) && cauda != 0
        && core::slice::from_raw_parts(read_u32_as_usize(cauda) as *const u8, 0x4f) == core::slice::from_raw_parts(f as *const u8, 0x4f);
    PLACAR.registrar(igual, || format!("insert_fixture(jogo {:x}) conta antes={} depois={}", f, conta, read_u16(l)));
}

/// Comparer for `remove_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: tira da lista (não roda duas vezes): antes da original, a cópia acha a lista e vê se algum item tem os
/// bytes do jogo (o tamanho guardado em +4, como a 0x68d1f0); a original tira e devolve: a devolução e a conta.
pub(crate) unsafe extern "thiscall" fn comparar_remove_fixture(this: usize, f: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_REMOVE_FIXTURE);
    let l = find_fixture_list(this, f);
    if l == 0 {
        return o(this, f); // simplification: sem memória para o bloco, só a original (a caixa)
    }
    let conta = read_u16(l);
    let mut no = read_u32_as_usize(l + 2);
    let mut tem = false;
    while no != 0 && !tem {
        let n = *((no + 4) as *const i16) as isize as usize;
        tem = core::slice::from_raw_parts(read_u32_as_usize(no) as *const u8, n) == core::slice::from_raw_parts(f as *const u8, n);
        no = read_u32_as_usize(no + 6);
    }
    let r = o(this, f);
    let esperado = tem as u32;
    let igual = r == esperado && read_u16(l) == conta.wrapping_sub(esperado as u16);
    PLACAR.registrar(igual, || format!("remove_fixture(jogo {:x}) original={} esperado={} conta antes={} depois={}",
        f, r, esperado, conta, read_u16(l)));
    r
}

/// Comparer for `find_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem lista, pela devolução (a original e a cópia só andam os cursores das listas do calendário). Com
/// lista, a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                               lista: usize, comp: u32, tipo: u32, lado: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32>(IDX_FIND_FIXTURES);
    if lista == 0 {
        let r = o(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        let n = find_fixtures(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        PLACAR.registrar(r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}) original={} rust={}",
            dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16));
        return r;
    }
    let (conta, cauda) = (*(lista as *const u16), read_u32(lista + 6));
    let r = o(this, dia0, ano0, dia1, ano1, clube, lista, comp, tipo, lado);
    let (n, iguais) = run_with_temp_list(lista, conta, cauda,
        |p| find_fixtures(this, dia0, ano0, dia1, ano1, clube, p, comp, tipo, lado));
    PLACAR.registrar(iguais && r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}, com lista) original={} rust={} itens iguais {}",
        dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16, iguais));
    r
}

/// Comparer for `collect_comp_fixtures_on_day` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_collect_comp_fixtures_on_day(this: usize, ano: u32, dia: u32, fatia: u32, comp: u32, rodada: u32,
                                                                tipo: u32, lista: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32>(IDX_COLLECT_COMP_FIXTURES_ON_DAY);
    let (conta, cauda) = (*(lista as *const u16), read_u32(lista + 6));
    let r = o(this, ano, dia, fatia, comp, rodada, tipo, lista);
    let (n, iguais) = run_with_temp_list(lista, conta, cauda,
        |p| collect_comp_fixtures_on_day(this, ano, dia, fatia, comp, rodada, tipo, p));
    PLACAR.registrar(iguais && r as u16 == n as u16, || format!("collect_comp_fixtures_on_day({}/{} fatia {}, comp {}, rodada {}, tipo {:x}) original={} rust={} itens iguais {}",
        dia as u16, ano as u16, fatia as u8, comp as i32, rodada as i32, tipo as u8, r as u16, n as u16, iguais));
    r
}

/// Comparer for `fix_man_comp_has_fixture_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pela devolução (só anda os cursores das listas do calendário).
pub(crate) unsafe extern "stdcall" fn comparar_fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(IDX_FIX_MAN_COMP_HAS_FIXTURE_ON_DATE)(data, comp);
    let n = fix_man_comp_has_fixture_on_date(data, comp);
    PLACAR.registrar(r == n, || format!("fix_man_comp_has_fixture_on_date(data {:x}, comp {}) original={} rust={}", data, comp as i32, r, n));
    r
}

/// Comparer for `find_next_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_fixture(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, fase: i32, clube_a: i32, clube_b: i32, a9: u32, a10: u32, a11: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize>(IDX_FIND_NEXT_FIXTURE);
    let antes = (*pano, *pdia, *pfatia);
    let reg = [(pano as usize, 2), (pdia as usize, 2), (pfatia as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg,
        || o(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11),
        || find_next_fixture(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11));
    PLACAR.registrar(igual && r == n, || format!("find_next_fixture(cursor {:?}, adiante {}, comp {}, fase {}, clubes {:x} {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        antes, adiante, comp, fase, clube_a, clube_b, r, n, mo, mr));
    r
}

/// Comparer for `get_time_slice` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_time_slice(this: usize, comp: usize, dia: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_TIME_SLICE)(this, comp, dia);
    let n = get_time_slice(this, comp, dia);
    PLACAR.registrar(r == n, || format!("get_time_slice(dia {}) original={} rust={}", dia as i16, r, n));
    r
}

/// Comparer for `add_away_trip_illness_risk` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_away_trip_illness_risk(this: usize, f: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_AWAY_TRIP_ILLNESS_RISK)(this, f)
}

/// Comparer for `fix_man_get_partner_key_nation_index` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_fix_man_get_partner_key_nation_index(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_FIX_MAN_GET_PARTNER_KEY_NATION_INDEX)(clube);
    let n = fix_man_get_partner_key_nation_index(clube);
    PLACAR.registrar(r == n, || format!("fix_man_get_partner_key_nation_index({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// Comparer for `get_fixture_list_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_get_fixture_list_for_date(data: usize, fatia: u32, grupo: u32) -> usize {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32) -> usize>(IDX_GET_FIXTURE_LIST_FOR_DATE)(data, fatia, grupo);
    let n = get_fixture_list_for_date(data, fatia, grupo);
    PLACAR.registrar(r == n, || format!("get_fixture_list_for_date({:x}, fatia {}, grupo {}) original={:x} rust={:x}", data, fatia, grupo, r, n));
    r
}

/// Comparer for `mark_calendar_year` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_mark_calendar_year(jogo: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_MARK_CALENDAR_YEAR);
    let tabela = *(G_CLUB_FIXTURE_YEAR_TABLE as *const usize);
    let reg: Vec<(usize, usize)> = if tabela == 0 {
        vec![]
    } else {
        [0xc, 0x10].iter().map(|&o| (tabela.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)), 2)).collect()
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo), || mark_calendar_year(jogo));
    PLACAR.registrar(igual, || format!("mark_calendar_year({:#x}) {:x?} x {:x?}", jogo, mo, mr));
}
