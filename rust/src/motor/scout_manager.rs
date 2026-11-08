use super::*;







/// Day/year is after the date at `data` (unsigned year +2, signed day +0).
///
/// [pt] `a` depois de `b` (datas: ano sem sinal em +2, dia com sinal em +0), como as comparações da original.
pub(crate) unsafe fn scout_manager_date_after(day: i16, year: u16, data: usize) -> bool {
    let (d, a) = (read_i16(data), read_i16(data + 2) as u16);
    year > a || (year == a && day > d)
}

/// O ensaio da 0x7f8750: a cópia só poda os pedidos (num vetor dela) e para antes dos relatórios.
pub(crate) static mut SCOUT_DAILY_DRY_RUN: bool = false;

/// Prunes a scouting request (club, player, date, report count): returns (remove, process, ...) by human club, own
/// player, request age, reports and injury/fixtures.
///
/// [pt] A poda de um pedido (0x11 bytes: +0 o clube, +4 o jogador, +8 a data, +0x10 quantos relatórios): devolve
/// (tirar, processar). Sai o pedido de clube que não é humano, de jogador do próprio clube (o +0x39 ou o do contrato),
/// com mais de um ano, com 3 relatórios, ou (depois de 30 dias) de jogador machucado (0x62c400 = 2); entre 30 dias e um
/// ano o pedido é processado (e sai), salvo jogador sem nação ou de clube que joga nos 14 dias seguintes, que fica.
pub(crate) unsafe fn judge_scout_request(request: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> i16 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    // 0x600a50: 5 pushes (clube, 1, 1, 0, 0); add esp, 0x14 -> cdecl; 0x4cf5a0: push jogador; mov ecx, [0xae1050] ->
    // thiscall, ret 4; 0x54c5b0: push n; push &saída; lea ecx, [pedido+8] -> thiscall, ret 8; 0x62c400: push jogador;
    // mov ecx, [0xb63d28] -> thiscall, ret 4 (al); 0x4cf5f0: push clube; push jogador; mov ecx, [0xae1050] -> thiscall,
    // ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24 (ax)
    let player = staff_records_staff_by_id(read_usize(request + 4));
    let clube = friendly_club_by_id(read_usize(request));
    if human(clube, 1, 1, 0, 0) == 0 {
        return (true, false, 0, 0, 0);
    }
    let manager = read_usize(clube + club::MANAGER);
    if read_usize(player + 0x39) == clube {
        return (true, false, 0, 0, 0);
    }
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), player);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
    let of_contract = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    if of_contract == clube {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let mut t = [0u32; 2];
    let year_after = mais(request + 8, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if scout_manager_date_after(dia, ano, year_after) {
        return (true, false, 0, 0, 0);
    }
    if (read_u8(request + 0x10) as i8) >= 3 { // cqf12: três relatórios
        return (true, false, 0, 0, 0);
    }
    let mut t2 = [0u32; 2];
    let month_after = mais(request + 8, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias
    if !scout_manager_date_after(dia, ano, month_after) {
        return (false, false, 0, 0, 0);
    }
    if lesao(read_usize(G_INJURY), player) == 2 {
        return (true, false, 0, 0, 0);
    }
    let seu = read_usize(player + 0x39);
    let mut nacao = if seu != 0 { read_usize(seu + 0x53) } else { 0 };
    if nacao == 0 {
        nacao = read_usize(player + 0x1a);
        if nacao == 0 {
            return (false, false, 0, 0, 0);
        }
    }
    let mut direto = false;
    if seu != 0 {
        let k = contract_with(read_usize(G_CONTRACT_MANAGER), player, seu);
        if k != 0 {
            let s = read_u8(k + 0x4f) & 0xf0;
            direto = s == 0x60 || s == 0x70 || s == 0;
        }
    }
    if !direto && seu != 0 {
        // o clube do jogador joga de hoje até 14 dias depois? (sem lista; a parte alta do 2º dword é sobra da pilha na
        // original e a 0x5acbe0 só lê a palavra baixa)
        let last = read_usize(catorze) as u32;
        let last_high = core::ptr::read_unaligned((catorze + 2) as *const u32);
        let a1 = (last_high & 0xffff_0000) | dia as u16 as u32;
        let a2 = ano as u32;
        if find(read_usize(G_FIXTURE_MANAGER), a1, a2, last, last_high, read_usize(seu) as u32, 0, -1, -2, 1) != 0 {
            return (false, false, 0, 0, 0);
        }
    }
    (true, true, player, manager, nacao)
}

/// on day slice 2: prunes the scouting requests (non-human club, player already at the club, older than a year, 3
/// reports, injured); a request older than 30 days becomes up to 3 reports from the manager's scouts covering the
/// player's nation and leaves; then the reports, the request order and one search tick on every human manager list
/// Original: `SCOUT_MANAGER::daily_update` (0x7f8750, scout_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x7f8750 } (thiscall, ret 0, scout_manager.cpp, quadro SEH da lista local e das datas): `SCOUT_MANAGER::
/// daily_update()`, pelo dia (só na fatia 2 do dia, [0xae2c88]; senão 0). Poda os pedidos de observação (+0x5c,
/// conta +0x64, 0x11 bytes; o tirado recebe o último), e o pedido processado vira até 3 relatórios dos olheiros do
/// técnico do clube (das 10 listas, 0x7f6cd0, a que cobre a nação do jogador, 0x7b7490): 0x7b62a0 com −1000, modo 1, e o
/// registro (0x67 bytes) entra na lista local se o byte +0x66 ficou ≥ 0. Depois os relatórios (0x7f9890), a ordem dos
/// pedidos (0x945333 com 0x7fc270) e uma busca (0x7b3e20) em cada uma das 10 listas dos técnicos humanos (as 16 últimas
/// pessoas com +0x5f = 1). Devolve 1.
pub(crate) unsafe extern "thiscall" fn scout_manager_daily_update(this: usize) -> u32 {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    // 0x68d020 / 0x68d040 / 0x54a110: lea ecx, local -> thiscall; 0x54c5b0: push 0xe; push &saída; mov ecx, 0xae2c90
    let mut d14 = [0u32; 2];
    let catorze = d14.as_mut_ptr() as usize;
    mais(G_CURRENT_DATE, catorze, 0xe);
    // o registro do relatório (0x67 bytes) com as datas que a entrada da original constrói (+0, +0x2f, +0x47)
    let mut record = [0u8; 0x68];
    let reg = record.as_mut_ptr() as usize;
    data_default(reg);
    data_default(reg + 0x2f);
    data_default(reg + 0x47);
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    new(l);
    if read_u8(G_CURRENT_DAY_SLICE) != 2 {
        soltar(l);
        return 0;
    }
    let record_of_scout: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_GET_SEARCH);
    let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_SCOUT_COVERS_NATION);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(FN_PLAYER_SEARCH_REPORT_ON_PLAYER);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let relatorios: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_SEND_SCOUT_REPORTS);
    let sort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let search: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_PLAYER_SEARCH_RUN_SCOUT_DAY);
    // 0x7f6cd0: push técnico; push k; mov ecx, this -> thiscall, ret 8; 0x7b7490: push nação; mov ecx, lista ->
    // thiscall, ret 4; 0x7b62a0: push 1; push 0; push 0; push 1; push −1000; push &registro; push jogador; mov ecx, lista
    // -> thiscall, ret 0x1c; 0x68d0d0: push 0x67; push &registro; lea ecx, local -> thiscall, ret 8; 0x7f9890: push
    // &local; mov ecx, this -> thiscall, ret 4; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x7b3e20: mov ecx,
    // lista -> thiscall, ret 0
    let mut i: i32 = 0;
    while i < read_i32(this + 0x64) {
        let request = read_usize(this + 0x5c) + i as usize * 0x11;
        let (tirar, processar, jogador, tecnico, nacao) = judge_scout_request(request, catorze);
        if processar && !SCOUT_DAILY_DRY_RUN {
            crate::comparacao::ramo("cqf_12 pedido processado");
            let mut feitos: i8 = 0;
            for k in 0..10u32 {
                if feitos >= 3 { // cqf12: até três relatórios
                    break;
                }
                let lo = record_of_scout(this, k, tecnico);
                if cobre(lo, nacao) != 0 {
                    relatorio(lo, jogador, reg, -1000, 1, 0, 0, 1);
                    feitos += 1;
                    if (read_u8(reg + 0x66) as i8) > -1 {
                        crate::comparacao::ramo("cqf_12 relatorio do pedido");
                        append(l, reg, 0x67);
                    }
                }
            }
        }
        if tirar {
            let n = read_i32(this + 0x64);
            let base = read_usize(this + 0x5c);
            core::ptr::copy(base.wrapping_add((n as usize).wrapping_mul(0x11)).wrapping_sub(0x11) as *const u8, request as *mut u8, 0x11);
            core::ptr::write_unaligned((this + 0x64) as *mut i32, n - 1);
            i -= 1;
        }
        i += 1;
    }
    if SCOUT_DAILY_DRY_RUN {
        soltar(l);
        return 1;
    }
    relatorios(this, l);
    sort(read_usize(this + 0x5c), read_i32(this + 0x64), 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = staff_records_staff_by_id((read_i32(G_STAFF_COUNT) - k - 1) as usize);
        if read_u8(p + 0x5f) == 1 {
            crate::comparacao::ramo("cqf_12 busca do humano");
            for j in 0..10u32 {
                search(record_of_scout(this, j, p)); // cqf12: a busca de cada lista
            }
        }
    }
    soltar(l);
    1
}


/// O registro das notícias de relatório que a 0x7f9a60 recebe (jogador, clube, quantos), ligado só no comparador da
/// 0x7f9890 para conferir os grupos da original.
pub(crate) static mut SCOUT_REPORT_GROUPS: Option<Vec<(u32, u32, u8)>> = None;

/// copies the list records into an array, sorts it and sends one news item per run of the same player and club
/// Original: `SCOUT_MANAGER::send_scout_reports` (0x7f9890, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f9890 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::send_scout_reports(lista)`, pelo dia dos olheiros
/// (0x7f8750) e pela busca (0x7f8c00). Com a lista não vazia (+0 conta): um vetor de conta × 0x67 (0x944e46; sem
/// memória, a caixa da linha 0x7dd) com cópias dos registros, ordenado (0x945333 com 0x7fc2a0); cada grupo seguido do
/// mesmo jogador (+0x2b) e do mesmo clube (+0x50) vira uma notícia de relatório (0x7f9a60 com o primeiro e quantos).
/// Libera o vetor (0x9452ca). A lista fica com o cursor no começo.
pub(crate) unsafe extern "thiscall" fn send_scout_reports(this: usize, list: usize) {
    let count = read_i16(list);
    if count == 0 {
        return;
    }
    let n = count as i32;
    let alocar: unsafe extern "C" fn(i32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let sort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_SEND_SCOUT_REPORT_NEWS);
    // 0x944e46 (malloc): push; add esp, 4 -> cdecl; 0x9452ca (free): push; add esp, 4 -> cdecl; 0x68d1b0: mov ecx, lista
    // -> thiscall; 0x945333: 4 pushes; add esp, 0x10 -> cdecl; 0x7f9a60: push quantos; push registro; mov ecx, this ->
    // thiscall, ret 8
    let v = alocar(n.wrapping_mul(0x67));
    if v == 0 && n != 0 {
        show_error_at(0xa7b094, 0x7dd);
        return;
    }
    core::ptr::write_unaligned((list + 0xa) as *mut usize, read_usize(list + 2));
    let mut dst = v;
    let mut item = next(list);
    while item != 0 {
        core::ptr::copy_nonoverlapping(item as *const u8, dst as *mut u8, 0x67);
        dst += 0x67;
        item = next(list);
    }
    sort(v, n, 0x67, 0x7fc2a0);
    let mut i: i32 = 0;
    while i < n {
        let r = v + i as usize * 0x67;
        let club_id = read_usize(friendly_club_by_id(read_usize(r + 0x50)));
        let player_id = read_usize(staff_records_staff_by_id(read_usize(r + 0x2b)));
        let start = i;
        let mut how_many: u8 = 1;
        let mut j = i + 1;
        while j < n {
            let s = v + j as usize * 0x67;
            if read_usize(s + 0x50) != club_id || read_usize(s + 0x2b) != player_id {
                break;
            }
            how_many = how_many.wrapping_add(1);
            j += 1;
            i += 1;
        }
        let first = v + start as usize * 0x67;
        if SCOUT_DAILY_DRY_RUN {
            if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
                g.push((read_usize(first + 0x2b) as u32, read_usize(first + 0x50) as u32, how_many));
            }
        } else {
            noticia(this, first, how_many as u32);
        }
        i += 1;
    }
    free(v);
}


/// the 0x138e news item with up to 3 records, the scout, the match and the player's strengths for the club, to the club
/// manager
/// Original: `SCOUT_MANAGER::send_scout_report_news` (0x7f9a60, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9a60 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da notícia local): `SCOUT_MANAGER::
/// send_scout_report_news(registro, quantos)`, pelos relatórios (0x7f9890). A notícia 0x138e (0x77bbe0): quantos (até
/// 3), +0x66, o olheiro (0x7e5160 com +0x1f), +0x59, +0x5a, o clube +0x50, os dados do jogo (0x7e4de0), +0x5e, +0x62,
/// +0x58, +0x54, de cada um dos registros (+0x37 + 0x67 × k) +0xc, +0, +4, +8 e +0x26 + 8 × +0x25, 0x420e9 e as forças
/// do jogador (+0x2b) para o clube (0x546ab0); vai ao técnico do clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn send_scout_report_news(this: usize, r: usize, how_many: u32) {
    let _ = this;
    let n = (how_many as u8 as i8).min(3);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let scout: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_STAFF_NAME_REF_TO_NEWS);
    let dados: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_GET_PLAYER_STRENGTH_FLAGS);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    // 0x77bbe0: push 0; push 0x138e; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7e5160
    // e 0x7e4de0: push &índice; push &notícia; push registro; add esp, 0xc -> cdecl; 0x546ab0: push clube; push
    // jogador; add esp, 8 -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    let mut noticia = [0u32; 0x38];
    let nt = noticia.as_mut_ptr() as usize;
    construct(nt, 0x138e, 0);
    write(nt, 0, n as i32 as u32);
    let mut idx: u8 = 1;
    let mut pr = |idx: &mut u8, v: u32| {
        write(nt, *idx as u32, v);
        *idx = idx.wrapping_add(1);
    };
    pr(&mut idx, read_u8(r + 0x66) as i8 as i32 as u32);
    scout(r + 0x1f, nt, &mut idx);
    pr(&mut idx, read_u8(r + 0x59) as i8 as i32 as u32);
    pr(&mut idx, read_i16(r + 0x5a) as i32 as u32);
    pr(&mut idx, read_usize(r + 0x50) as u32);
    dados(r, nt, &mut idx);
    pr(&mut idx, read_usize(r + 0x5e) as u32);
    pr(&mut idx, read_usize(r + 0x62) as u32);
    pr(&mut idx, read_u8(r + 0x58) as i8 as i32 as u32);
    pr(&mut idx, read_usize(r + 0x54) as u32);
    for k in 0..n.max(0) as usize {
        let p = r + 0x37 + k * 0x67;
        pr(&mut idx, read_usize(p + 0xc) as u32);
        pr(&mut idx, read_usize(p) as u32);
        pr(&mut idx, read_usize(p + 4) as u32);
        pr(&mut idx, read_usize(p + 8) as u32);
        pr(&mut idx, ((read_u8(p + 0x26) as i8 as i32) + (read_u8(p + 0x25) as i8 as i32) * 8) as u32);
    }
    pr(&mut idx, 0x420e9);
    let c = read_i32(r + 0x50);
    let clube = if c > -1 { friendly_club_by_id(c as usize) } else { 0 };
    let k = idx;
    idx = idx.wrapping_add(1);
    let _ = idx;
    write(nt, k as u32, forcas(staff_records_staff_by_id(read_usize(r + 0x2b)), clube));
    deliver(read_usize(G_NEWS), nt, friendly_club_by_id(read_usize(r + 0x50)));
}






/// Date r equals today's (year and day).
///
/// [pt] A data `r` é a de hoje (o ano e o dia)?
pub(crate) unsafe fn is_today(r: usize, today: usize) -> bool {
    read_u16(r + 2) == read_u16(today + 2) && read_i16(r) == read_i16(today)
}

/// Marks clubs with fixtures on date d, the day before and the day after, leaving d unchanged.
///
/// [pt] Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
pub(crate) unsafe fn mark_clubs_three_days(this: usize, d: usize) {
    let mark: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_MARK_CLUBS_WITH_FIXTURES);
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    crate::comparacao::ramo("cqc_12 marca os clubes");
    mark(this, d);
    copy(d, pu, minus(d, pt, 1));
    mark(this, d);
    copy(d, pu, mais(d, pt, 2));
    mark(this, d);
    copy(d, pu, minus(d, pt, 1));
}

/// For each fixture in the list: national team call-ups and per-club call-up records (including B teams per flags),
/// outside finals tournaments.
///
/// [pt] Os jogos da lista `l` (do dia daqui a duas semanas): para cada um, as convocações (0x76bab0) e, fora das fases
/// finais (a competição +0x14 numa das chaves 0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0,
/// 0x9cf78c, 0x9cf964), com mandante que não é a seleção B (0x5409b0) e a marca +0x40: os convocados por clube
/// (0x7771c0) das duas seleções; com o bit 0x200 de +0x4d, também os das seleções B (0x53b4f0 da nação +0x53 de cada
/// uma) e, sem ele e com o 0x100, os das duas com `b` = 1.
pub(crate) unsafe fn select_squads_for_fixtures(this: usize, l: usize) {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let convocar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_SELECT_SQUADS_FOR_FIXTURE);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let register: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_RECORD_CLUB_CALL_UPS);
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    write_usize(l + 0xa, read_usize(l + 2));
    let mut p = next(l);
    while p != 0 {
        crate::comparacao::ramo("cqc_12 convoca");
        convocar(this, p); // cqc12: as convocações para o jogo
        let comp = read_usize(p + 0x14);
        let final_ = comp != 0
            && [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter().any(|&k| read_usize(comp) == read_usize(k));
        if !final_ && second(read_usize(p + 0x1c)) == 0 && *((p + 0x40) as *const u8) != 0 {
            register(this, read_usize(p + 0x1c), 0, p);
            register(this, read_usize(p + 0x20), 0, p);
            let f = read_u16(p + 0x4d);
            if f & 0x200 != 0 {
                let b1 = b_national_team(read_usize(read_usize(p + 0x1c) + 0x53));
                register(this, b1, 0, p);
                let b2 = b_national_team(read_usize(read_usize(p + 0x20) + 0x53));
                register(this, b2, 0, p);
            } else if f & 0x100 != 0 {
                register(this, read_usize(p + 0x1c), 1, p);
                register(this, read_usize(p + 0x20), 1, p);
            }
        }
        p = next(l);
    }
}


/// Number of items in the list at [this] whose day (+0x10) equals `dia`, without moving the cursor.
///
/// [pt] Quantos itens da lista em +0 são do dia `dia` (+0x10), percorrendo os nós sem mexer no cursor.
pub(crate) unsafe fn count_items_on_day(this: usize, day: i16) -> u32 {
    let l = read_usize(this);
    if l == 0 {
        return 0;
    }
    let mut n = 0;
    let mut no = read_usize(l + 2);
    while no != 0 {
        if read_i16(read_usize(no) + 0x10) == day {
            n += 1;
        }
        no = read_usize(no + 6);
    }
    n
}


/// Index (0..15) of the current human manager; Err without manager, Ok(None) when out of range.
///
/// [pt] O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
pub(crate) unsafe fn current_human_index() -> Result<Option<usize>, ()> {
    let t = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// Search record k (0x2b0 bytes) of session s.
///
/// [pt] O registro de busca `k` (0x2b0 bytes) da sessão `s`.
pub(crate) unsafe fn search_record(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// the current search of the current human manager session has no club (+0x191 = −2)
/// Original: `SCOUT_MANAGER::current_search_has_no_club` (0x7f8420, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
pub(crate) unsafe extern "thiscall" fn current_search_has_no_club(this: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CURRENT_SEARCH_HAS_NO_CLUB)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = read_usize(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    (read_usize(search_record(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}


/// one of the 10 searches of the current human manager session is for the club
/// Original: `SCOUT_MANAGER::is_club_in_session_searches` (0x7f8500, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
pub(crate) unsafe extern "thiscall" fn is_club_in_session_searches(this: usize, clube: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_CLUB_IN_SESSION_SEARCHES)(this, clube), // a caixa
        Ok(Some(i)) => i,
    };
    let s = read_usize(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    let id = read_usize(clube);
    // simplification: a original testa o contador contra 0xff (nunca) para usar a busca atual (+0); fica o contador
    (0..10).any(|k| read_usize(search_record(s, k) + 0x191) == id) as u32
}



/// the scouts at the match: for each human manager, the 10 scouts in a drawn order; one for the playing club starts the
/// assignment, and each watches the match; at the end the reports go out
/// Original: `SCOUT_MANAGER::scout_match` (0x7f8c00, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8c00 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `SCOUT_MANAGER::scout_match(partida, resultado)`, pelo fechamento da partida do dia (0x6b5f00, Rust). Para cada um
/// dos 16 últimos registros de pessoa ([0xae23bc], 0x6e bytes, do último para trás) com +0x5f = 1 (o técnico humano):
/// num vetor zerado de 10 marcas (0x945501), os 10 observadores numa ordem sorteada (0x90cfc0 de 10 até achar um não
/// marcado); o registro de cada um (0x7f6cd0) com +0x19d, numa partida com as marcas +0x40 e +0x41 e do clube do
/// técnico (+0x39 = +0x1c ou +0x20 da partida), começa a observação (0x7ae9c0 com −1, −1, −1, 0, 1); e observa a
/// partida (0x7b5be0, os relatórios na lista local). O vetor sai (0x9452ca). Por fim os relatórios vão (0x7f9890) e a
/// lista é destruída. Devolve 1.
pub(crate) unsafe extern "thiscall" fn scout_manager_scout_match(this: usize, p: usize, result: usize) -> u32 {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let record: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_GET_SEARCH);
    let comecar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_PLAYER_SEARCH_START_SCOUT_ASSIGNMENT);
    let observar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_PLAYER_SEARCH_SCOUT_MATCH);
    let enviar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_SEND_SCOUT_REPORTS);
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); sorteio: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &lista; push resultado; push partida; mov ecx, registro -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    new(list);
    for k in 0..16i32 {
        let idx = (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(k).wrapping_sub(1);
        let person = read_usize(G_STAFF_TABLE).wrapping_add((idx as usize).wrapping_mul(staff::SIZE));
        if *((person + staff::CLASSIFICATION) as *const u8) != 1 {
            continue;
        }
        crate::comparacao::ramo("cqc_13 olheiros do humano");
        let marks = zerado(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = sortear(10) as i8; // cqc13: o observador sorteado
                if read_usize(marks.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: um ainda não marcado
                }
            };
            core::ptr::write_unaligned(marks.wrapping_add((q as isize * 4) as usize) as *mut u32, 1);
            // simplification: a original empurra o dword com o byte do observador e sobra de pilha em cima; a 0x7f6cd0 lê o
            // byte
            let r = record(this, q as u8 as u32, person);
            if read_usize(r + 0x19d) != 0 && *((p + 0x40) as *const u8) != 0 && *((p + 0x41) as *const u8) != 0 {
                let clube = read_usize(person + staff::CLUB_JOB);
                if read_usize(p + 0x1c) == clube || read_usize(p + 0x20) == clube {
                    comecar(r, u32::MAX, u32::MAX, u32::MAX, 0, 1);
                }
            }
            observar(r, p, result, list);
        }
        free(marks);
    }
    enviar(this, list);
    soltar(list);
    1
}


/// Index (0..15) of the given manager (null: current human); None when missing or out of range.
///
/// [pt] O índice (0..15) do técnico `tecnico` (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um
/// byte da original ((id − [0xae2364]) + 16 em 8 bits); None nos casos da caixa (sem técnico ou fora).
pub(crate) unsafe fn scout_manager_human_manager_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (*(t as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// the human manager's searches edit session
/// Original: `SCOUT_MANAGER::get_edit_session` (0x7f6be0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6be0 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_edit_session(tecnico)`, pela montagem dos
/// conhecidos da busca (0x7afeb7). A sessão de edição do técnico humano (+0xc + 4 × índice). Sem técnico ou índice
/// fora: a caixa (linhas 0xde e 0xe8; a original) e 0.
pub(crate) unsafe extern "thiscall" fn get_edit_session(this: usize, manager: usize) -> usize {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_EDIT_SESSION)(this, manager); // a caixa
    };
    read_usize(this + 0xc + i * 4)
}


/// stores the human manager's searches edit session and returns 1
/// Original: `SCOUT_MANAGER::set_edit_session` (0x7f6af0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6af0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_edit_session(sessao, tecnico)`, pelas telas
/// das buscas (0x8085bc, 0x812d88...). Grava a sessão em +0xc + 4 × índice do técnico e devolve 1. Sem técnico ou
/// índice fora: a caixa (linhas 0xb8 e 0xc2; a original) e 0.
pub(crate) unsafe extern "thiscall" fn set_edit_session(this: usize, sessao: usize, manager: usize) -> u32 {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SET_EDIT_SESSION)(this, sessao, manager); // a caixa
    };
    core::ptr::write_unaligned((this + 0xc + i * 4) as *mut usize, sessao);
    1
}




/// As datas estáticas da 0x54c770 (valor e guarda) e da 0x54c5b0, 0x54c8e0 e 0x54cdb0.
pub(crate) const SCOUT_MANAGER_DATE_STATICS: [(usize, usize); 8] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_ADD_WEEKS_RESULT_DATE, 8), (0xae2430, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Body of Friendly::update_international_friendlies (0x5cbb00); in dry run returns the (first entry, count) that would
/// go to 0x5cbdf0.
///
/// [pt] O corpo da 0x5cbb00. No `ensaio` (o comparador), nada se grava e nada se chama além das consultas: devolve o par
/// (primeira entrada, quantas) que iria à 0x5cbdf0, ou None se a original não a chamaria.
pub(crate) unsafe fn update_friendlies_body(this: usize, ensaio: bool) -> Option<(usize, u32)> {
    // 0x54a110: thiscall(data) (lea ecx); 0x54c770: thiscall(0xae2c90, saída, 0xc) (mov ecx, 0xae2c90; 2 push);
    // 0x68d080: thiscall(lista) (mov ecx, lista; sem push); 0x945333: cdecl(vetor, conta, tamanho, comparação) (add esp,
    // 0x10); 0x5cbdd0: thiscall(entrada, ano) (mov ecx, entrada; 1 push); 0x417c10: thiscall(destino, saída, origem)
    // (lea ecx, destino; 2 push); 0x946464: cdecl(chave, vetor, conta, 0x14, 0x5cc610) (add esp, 0x14); 0x456770 e
    // 0x456740: thiscall(data, baixo, alto) (lea ecx, data; 2 push); 0x5cbdf0: thiscall(this, primeira, quantas) (mov
    // ecx, esi; 2 push)
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let weeks: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let data_of_entry: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_INTERNATIONAL_DATE_RESOLVE);
    let copy: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let before: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let after: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let mark: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_ARRANGE_INTERNATIONAL_FRIENDLIES);
    // a chave da busca: a data em +3, como as entradas de 20 bytes (a original passa a data local − 3)
    let mut key = [0u8; 0x14];
    let d = key.as_mut_ptr() as usize + 3;
    data_default(d);
    let mut lim = [0u8; 8];
    weeks(G_CURRENT_DATE, &mut lim, 0xc); // cpy4: hoje + 12 semanas
    let today = read_u16(G_CURRENT_DATE);
    if today == 0 && !ensaio {
        crate::comparacao::ramo("cpy_4 virada do ano");
        let mut i = 0i32;
        while i < read_usize(G_NATION_COUNT) as i32 {
            let l = read_usize(read_usize(this + 0x34f6).wrapping_add(i as usize * 4));
            if l != 0 {
                clear(l);
            }
            i += 1;
        }
        sort(read_usize(this + 0x34fa), read_usize(this + 0x34fe) as u32, 4, 0x5cc5f0); // cpy4: as nações pela reputação
        // o primeiro ano de Copa (ano − 2 múltiplo de 4) a partir deste; a metade alta do dword que a original soma é
        // sobra de pilha: aqui, zero
        let mut year = read_u16(G_CURRENT_DATE_YEAR) as u32;
        while ((year & 0xffff) as i32 - 2) % 4 != 0 {
            year = year.wrapping_add(1);
        }
        let mut k = 0i32;
        while k < read_usize(this + 0x34f0) as i32 {
            let e = read_usize(this + 0x34ec).wrapping_add(k as usize * 0x14);
            let mut a = ((*((e + 0xb) as *const i8)) as i16 as u16 as u32).wrapping_add(year);
            if (a & 0xffff) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32 > 3 {
                a = a.wrapping_add(0xfffc);
            }
            data_of_entry(e, a);
            k += 1;
        }
        sort(read_usize(this + 0x34ec), read_usize(this + 0x34f0) as u32, 0x14, 0x5cc610);
    }
    // a data de 0xdd6ec8 depois de hoje + 12 semanas: nada
    let x_year = read_u16(G_GAME_START_DATE_YEAR);
    let l_year = core::ptr::read_unaligned(lim.as_ptr().add(2) as *const u16);
    if x_year > l_year || (x_year == l_year && *(G_GAME_START_DATE as *const i16) > core::ptr::read_unaligned(lim.as_ptr() as *const i16)) {
        return None;
    }
    if read_u16(this + 0x34f4) as u32 == today as i16 as i32 as u32 {
        return None;
    }
    let mut lixo = [0u8; 8];
    copy(d, &mut lixo, lim.as_ptr() as usize);
    let base = read_usize(this + 0x34ec);
    let n = read_usize(this + 0x34f0);
    let p = bsearch(key.as_ptr() as usize, base, n as u32, 0x14, 0x5cc610);
    if p == 0 {
        return None;
    }
    let same = |e: usize| before(d, read_usize(e + 3) as u32, read_usize(e + 7) as u32) == 0 && after(d, read_usize(e + 3) as u32, read_usize(e + 7) as u32) == 0;
    // volta até a primeira entrada da data
    let mut e = p;
    while e > base {
        if !same(e - 0x14) {
            break;
        }
        e -= 0x14;
    }
    let first = e;
    let last = base.wrapping_add(n.wrapping_mul(0x14)).wrapping_sub(0x14);
    let mut how_many = 0u32;
    while e <= last && same(e) {
        how_many += 1;
        e += 0x14;
    }
    if ensaio {
        return Some((first, how_many));
    }
    crate::comparacao::ramo("cpy_4 amistosos marcados");
    mark(this, first, how_many); // cpy4: marcar os amistosos da data
    *((this + 0x34f4) as *mut u16) = read_u16(G_CURRENT_DATE);
    Some((first, how_many))
}



/// Index (id - G_STAFF_COUNT + 16) of the manager (null: current human) among the 16 humans; None on error.
///
/// [pt] O técnico (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) e o índice dele entre os 16 humanos (id −
/// [0xae2364] + 16), ou None nos casos de erro da original (sem técnico ou fora de 0..15).
pub(crate) unsafe fn manager_human_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10);
    if !(0..=0xf).contains(&i) {
        return None;
    }
    Some(i as usize)
}

/// the list selected by the human manager (null: the current one)
/// Original: `SCOUT_MANAGER::get_selected_list` (0x7f6e30, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6e30 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_selected_list(tecnico)`, pelas telas de
/// busca (0x804248). A lista escolhida pelo técnico humano (byte em +0x4c + índice). Sem técnico ou fora de 0..15: a
/// original (caixa "Error", 0xff).
pub(crate) unsafe extern "thiscall" fn get_selected_list(this: usize, manager: usize) -> u8 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_SELECTED_LIST)(this, manager); // caixa: só a original
    };
    *((this + 0x4c + i) as *const u8)
}


/// stores the list selected by the human manager (null: the current one)
/// Original: `SCOUT_MANAGER::set_selected_list` (0x7f6f20, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6f20 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_selected_list(lista, tecnico)`, pelas telas
/// de busca (0x804460). Grava a lista escolhida (byte) em +0x4c + índice do técnico humano e devolve 1. Sem técnico ou
/// fora de 0..15: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn set_selected_list(this: usize, list: u32, manager: usize) -> u32 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_SET_SELECTED_LIST)(this, list, manager); // caixa: só a original
    };
    *((this + 0x4c + i) as *mut u8) = list as u8;
    1
}




/// tells whether the fixture has a watched club or an available player with a scout request (at the club or national
/// team, not loaned out, injured, suspended or barred, and not yet played)
/// Original: `SCOUT_MANAGER::fixture_has_scouted_player` (0x7f9720, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f9720 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::fixture_has_scouted_player(partida)` (pela
/// match_without_human 0x6b66a0): competição (+0) negativa, 0. Um clube da lista vigiada (LLIST estática 0xb74438,
/// cursor reposto; +0 do item) igual a +0xc ou +0x10 da partida, 1. Depois os pedidos de observação ([this+0x5c],
/// 0x11 bytes, a pessoa em +4, conta [this+0x64], relidos a cada passo): a pessoa cujo clube (+0x39) ou seleção (+0x24)
/// é um dos dois da partida (+0x1c primeiro, depois +0x20), que não está cedida (0x8c7560, fora de seleção 0x53b900),
/// nem lesionada (0x62c400 = 2), nem suspensa (0x55d270), nem impedida (0x7757d0), e que ainda não jogou (0x8cf5e0
/// = 0), dá 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn fixture_has_scouted_player(this: usize, partida: usize) -> u32 {
    let (c1, c2) = (read_u32_as_usize(partida + 0x1c), read_u32_as_usize(partida + 0x20));
    if read_i32(partida) < 0 {
        crate::comparacao::ramo("cps_8 competicao negativa");
        return 0; // cps8: competição negativa
    }
    core::ptr::write_unaligned(G_SCOUT_CLUB_TABLE_LIST_CURSOR as *mut u32, read_u32_as_usize(G_SCOUT_CLUB_TABLE_LIST_HEAD) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = next(G_SCOUT_CLUB_TABLE_LIST);
    while e != 0 {
        let v = read_u32_as_usize(e);
        if read_u32_as_usize(partida + 0xc) == v || read_u32_as_usize(partida + 0x10) == v {
            return 1;
        }
        e = next(G_SCOUT_CLUB_TABLE_LIST);
    }
    if read_u32_as_usize(this + 0x5c) == 0 || read_i32(this + 0x64) <= 0 {
        crate::comparacao::ramo("cps_8 sem pedidos");
        return 0; // cps8: sem pedidos de observação
    }
    let national_team: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    let mut k = 0i32;
    while k < read_i32(this + 0x64) {
        let id = read_i32(read_u32_as_usize(this + 0x5c).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        let (clube, sel) = (read_u32_as_usize(p + 0x39), read_u32_as_usize(p + 0x24));
        let x = if c1 == clube || c1 == sel {
            c1
        } else if c2 == clube || c2 == sel {
            c2
        } else {
            continue;
        };
        if national_team(x) == 0 && cedido(read_u32_as_usize(G_TRANSFER_MANAGER), p, x, 0) != 0 {
            continue;
        }
        if lesao(read_u32_as_usize(G_INJURY), p) == 2 {
            continue;
        }
        if suspenso(read_u32_as_usize(G_DISCIPLINE), p, read_u32_as_usize(partida + 0x14), 1, 0) != 0 {
            continue;
        }
        if impedido(read_u32_as_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, x, 0, 0) != 0 {
            continue;
        }
        if ja_jogou(read_u32_as_usize(G_TRANSFER_MANAGER), p, partida) == 0 {
            return 1;
        }
    }
    0
}


/// The current human's manager record ([0xb74c26 + 0x300 x current screen slot]).
///
/// [pt] O gerente do humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
pub(crate) unsafe fn scout_manager_current_human_manager() -> usize {
    let k = core::ptr::read_unaligned(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    read_u32_as_usize((0xb74c26isize + 0x300 * k) as usize)
}

/// tells whether the player is on the human scout shortlist (through the open edit session or the first type 1 search)
/// Original: `SCOUT_MANAGER::is_player_shortlisted` (0x7f7490, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f7490 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::is_player_shortlisted(pessoa, gerente)` (pelos
/// clubes interessados 0x842180, pelo estatuto 0x849d60, pela 0x8cde70 e por telas): pessoa nula, a caixa "Error"
/// (linha 0x231) e 0; sem gerente, o do humano da vez (sem ele, a caixa da linha 0x23b e 0). A vaga do humano (byte
/// +0 do gerente − byte de [0xae2364] + 16, com sinal, 0..15): com a edição aberta ([this + vaga × 4 + 0xc]), a
/// resposta dela (0x804540); senão a primeira das 10 buscas (0x7f6cd0) de tipo 1 (+0xa) responde (0x7af600). Sem ela, 0.
pub(crate) unsafe extern "thiscall" fn is_player_shortlisted(this: usize, person: usize, manager: usize) -> u32 {
    if person == 0 {
        show_error_at(0xa7b094, 0x231);
        return 0;
    }
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        show_error_at(0xa7b094, 0x23b);
        return 0;
    }
    let slot = (*(m as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        return 0;
    }
    let s = read_u32_as_usize(this + slot as usize * 4 + 0xc);
    if s != 0 {
        let da_edicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SEARCHEDITSESSION_IS_PLAYER_ON_SHORT_LIST);
        return da_edicao(s, person);
    }
    // simplification: a original empurra um dword local com o índice só no byte baixo; a 0x7f6cd0 lê só esse byte
    let search: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_GET_SEARCH);
    let has: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_CONTAINS_PLAYER);
    for i in 0..10u32 {
        let l = search(this, i, m);
        if l != 0 && *((l + 0xa) as *const u8) == 1 {
            let r = has(l, person);
            crate::comparacao::ramo("cps_8 lista do olheiro");
            return r; // cps8: a resposta da lista do olheiro
        }
    }
    0
}



// simplification: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.

/// the human manager's scouting record 0..9 (0x2b0 bytes)
/// Original: `scout_record` (0x7f6cd0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6cd0 } (thiscall, ret 8, scout_manager.cpp): o registro de observação `qual` 0..9 do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]): o índice (id −
/// [0xae2364] + 16) × 10 + `qual`, até [this+8] − 1, com 0x2b0 bytes cada a partir de [this].
/// `qual` fora, sem técnico ou índice fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn scout_record(t: usize, qual: i8, manager: *const i32) -> usize {
    let mut p = manager;
    if (0..=9).contains(&qual) && p.is_null() {
        let vez = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if !(0..=9).contains(&qual) || p.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, qual, manager);
    }
    let i = ((*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10)).wrapping_mul(10).wrapping_add(qual as i32);
    if i < 0 || i > (*((t + 8) as *const i32)).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, qual, manager);
    }
    crate::comparacao::ramo("registro do observador");
    (*(t as *const usize)).wrapping_add(i as usize * 0x2b0)
}




/// empties the list and adds the person of each scout request of the club; returns whether there is any
/// Original: `SCOUT_MANAGER::collect_club_scout_targets` (0x7f9690, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9690 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::collect_club_scout_targets(clube, lista)`: com
/// `lista`, esvazia-a (0x68d080) e acrescenta (0x68d0d0, 0x6e bytes) a pessoa de cada pedido de observação do clube
/// `*clube`. Devolve se há algum. Clube nulo: 0 (a lista fica vazia).
pub(crate) unsafe extern "thiscall" fn collect_club_scout_targets(this: usize, clube: *const u32, list: usize) -> u32 {
    if list != 0 {
        let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
        esvaziar(list);
    }
    if clube.is_null() {
        return 0;
    }
    let alvos = scouting_targets_of_club(this, *clube);
    if list != 0 {
        let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
        for &p in &alvos {
            append(list, p, 0x6e);
        }
    }
    (!alvos.is_empty()) as u32
}


/// finds the request (club, person; 17 bytes), pulls the next ones up and sorts again (0x7fc270); null club = the
/// current human manager club
/// Original: `SCOUT_MANAGER::remove_scout_request` (0x7f8ee0, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f8ee0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::remove_scout_request(pessoa, clube)`: o clube
/// nulo é o do técnico humano atual ([0xb74c26 + 0x300 × [0xb77c7a]], +0x39; sem ele, nada). Com a pessoa, busca
/// (bsearch 0x946464 com 0x7fc270, que lê só +0 clube e +4 pessoa; a data padrão da chave em +8, o byte +0x10 sem valor
/// e a cópia usa zero) o pedido no vetor ([this+0x5c], [this+0x64] registros de 17 bytes); achou: puxa os seguintes um
/// lugar (+0, +4, a data +8 pela 0x417c10 e o byte +0x10), a contagem desce e, sobrando algum, reordena (qsort).
pub(crate) unsafe extern "thiscall" fn remove_scout_request(this: usize, person: usize, clube: usize) {
    let mut key = [0u8; 0x11];
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    default(key.as_mut_ptr() as usize + 8);
    let mut c = clube;
    if c == 0 {
        let i = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32;
        let manager = read_u32((0xb74c26 + i * 0x300) as usize) as usize; // o técnico humano atual
        if manager == 0 {
            return;
        }
        c = read_u32(manager + 0x39) as usize;
        if c == 0 {
            return;
        }
    }
    if person == 0 {
        return;
    }
    let id_club = if c != 0 { read_u32(c) } else { u32::MAX };
    key[..4].copy_from_slice(&id_club.to_le_bytes());
    key[4..8].copy_from_slice(&read_u32(person).to_le_bytes());
    let base = read_u32(this + 0x5c) as usize;
    let s = crt_bsearch(key.as_ptr() as usize, base, read_u32(this + 0x64), 0x11, 0x7fc270);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(base) as u32 as u64 * 0xf0f0f0f1) >> 36) as i32; // o lugar do pedido
    let copy_data: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    while k < read_u32(this + 0x64) as i32 - 1 {
        let r = (read_u32(this + 0x5c) as usize).wrapping_add(k as usize * 17);
        core::ptr::write_unaligned(r as *mut u32, read_u32(r + 0x11));
        core::ptr::write_unaligned((r + 4) as *mut u32, read_u32(r + 0x15));
        let mut local = [0u8; 8];
        copy_data(r + 8, &mut local, r + 0x19);
        *((r + 0x10) as *mut u8) = *((r + 0x21) as *const u8);
        k += 1;
    }
    let n = read_u32(this + 0x64).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x64) as *mut u32, n); // um pedido a menos
    if n != 0 {
        media_crt_qsort(read_u32(this + 0x5c) as usize, n as i32, 0x11, 0x7fc270);
    }
}


/// poor (balance < 1,000,000) and small (reputation < 5000) club outside the nation's region: 0; else 1
/// Original: `SCOUT_MANAGER::can_club_scout_nation` (0x7f9190, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9190 } (thiscall, ret 8, scout_manager.cpp; `this` não é lido): `SCOUT_MANAGER::can_club_scout_nation(nacao,
/// clube)`: o clube (ou o do humano da vez) não cobre a nação só se tem as finanças (0x5a0590 em [0xae2a38]) com saldo
/// de 64 bits abaixo de 1.000.000, reputação (+0x80) abaixo de 5000 e a região (+0x76) da nação dele (+0x53) diferente
/// da região da `nacao`: 0. Senão 1. Sem clube nenhum: a original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn can_club_scout_nation(this: usize, nacao: usize, clube: usize) -> u32 {
    let c = club_or_current_human(clube);
    if c == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_CLUB_SCOUT_NATION)(this, nacao, clube); // caixa: só a original
    }
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = get_club_finance(read_usize(G_FINANCE_TABLE), c);
    if f != 0 && core::ptr::read_unaligned(f as *const i64) < 1_000_000 && read_i16(c + 0x80) < 5000 {
        if *((read_usize(c + 0x53) + 0x76) as *const u8) != *((nacao + nation::ACTUAL_REGION) as *const u8) { // fora da região
            crate::comparacao::ramo("cpk_6 olheiro fora da regiao");
            return 0;
        }
    }
    1
}


/// removes from the manager's scout lists the scouts no longer at the club and gives each club scout without a list the
/// first free list; with the edit session open, it refreshes the scouts
/// Original: `SCOUT_MANAGER::sync_scout_lists` (0x7f7270, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f7270 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::sync_scout_lists(técnico)`, quando a comissão do
/// clube humano muda (a saída do cargo 0x690890 e a da vaga dos olheiros, as transferências e contratações de pessoal).
/// Sem técnico, o humano da vez; sem ele, a caixa da linha 0x1ba e 0. Das 10 buscas do técnico, as listas de olheiro
/// (+0xa = 4) cujo olheiro (0x7ae950) não é mais olheiro (+0x3d = 9) do clube do técnico (+0x39) ficam sem olheiro
/// (0x7ae8a0 com 0; sem clube, todas; com clube, as sem olheiro ficam como estão). Com clube, cada um dos 7 olheiros do
/// clube (0x53cd60) que não tem lista vai para a primeira lista de olheiro sem olheiro. Depois a vaga do técnico (byte
/// +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x212 e 0): com a edição aberta ([this + vaga × 4 +
/// 0xc]), ela refaz os olheiros das listas (0x8043c0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn sync_scout_lists(this: usize, manager: usize) -> u32 {
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        show_error_at(0xa7b094, 0x1ba);
        return 0;
    }
    let clube = read_usize(m + 0x39);
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l == 0 || read_u8(l + 0xa) != 4 {
            continue;
        }
        let o = search_scout(l);
        if clube != 0 {
            if o == 0 || (read_usize(o + 0x39) == clube && read_u8(o + 0x3d) == 9) {
                continue;
            }
        }
        crate::comparacao::ramo("cqa_6 lista sem olheiro");
        set_search_scout(l, 0);
    }
    if clube != 0 {
        let olheiros: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_CLUB_SCOUTS);
        let mut v = [0usize; 7];
        olheiros(clube, v.as_mut_ptr());
        for &s in v.iter() {
            if s == 0 {
                continue;
            }
            let mut free: i8 = -1;
            let mut has = false;
            for i in 0..10u8 {
                let l = get_search(this, i, m);
                if l == 0 || read_u8(l + 0xa) != 4 {
                    continue;
                }
                if search_scout(l) == s {
                    has = true;
                    break;
                }
                if search_scout(l) == 0 && free == -1 {
                    free = i as i8;
                }
            }
            if !has && free > -1 {
                crate::comparacao::ramo("cqa_6 olheiro na lista livre");
                set_search_scout(get_search(this, free as u8, m), s);
            }
        }
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        show_error_at(0xa7b094, 0x212);
        return 0;
    }
    let s = read_usize(this + slot as usize * 4 + 0xc);
    if s != 0 {
        let refazer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SEARCHEDITSESSION_REFRESH_SCOUT_ASSIGNMENTS);
        refazer(s);
    }
    1
}


/// the player's slot in the record of the human's search that scouts his club, or 0
/// Original: `SCOUT_MANAGER::find_club_search_entry` (0x7f8610, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f8610 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::find_club_search_entry(pessoa)`, das telas da
/// pessoa e da busca (0x808540, 0x813210, 0x870650, 0x87a510): sem o humano da vez ou sem o clube da pessoa (+0x39), 0.
/// A vaga do humano (byte +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x4b5 e 0). A primeira das 10
/// buscas (0x7f6cd0) com o clube observado (+0x190) igual ao da pessoa e o registro dele (+0x170) dá a vaga da pessoa
/// nele (0x7bd700 sem reservar); nenhuma, 0.
pub(crate) unsafe extern "thiscall" fn find_club_search_entry(this: usize, p: usize) -> usize {
    let m = scout_manager_current_human_manager();
    if m == 0 || read_usize(p + 0x39) == 0 {
        return 0;
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        show_error_at(0xa7b094, 0x4b5);
        return 0;
    }
    let search: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_GET_SEARCH);
    for i in 0..10u32 {
        let l = search(this, i, m);
        // simplification: a original lê +0x190 sem conferir a busca (cairia com 0); aqui a busca nula não conta
        if l == 0 || read_usize(l + 0x190) != read_usize(read_usize(p + 0x39)) || read_usize(search(this, i, m) + 0x170) == 0 {
            continue;
        }
        let slot_of_player: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT);
        return slot_of_player(read_usize(search(this, i, m) + 0x170), p, 0);
    }
    0
}


/// the translated scout list name: Short List, Player Search, Staff Search or Scout #n
/// Original: `scout_manager_write_list_name` (0x7fc150, scout_manager.cpp, cdecl).
///
/// [pt] { 0x7fc150 } (cdecl, scout_manager.cpp): `scout_manager_write_list_name(indice, texto, tamanho)`, pelo construtor do
/// SCOUT_MANAGER (0x7f6760) e pelas telas de busca. O nome traduzido (0x66f4e0) da lista `indice`: 0 "Short List", 1
/// "Player Search", 2 "Staff Search", 3..9 "Scout #<%d - number>" com `indice` − 2. Devolve 1. Índice fora de 0..9,
/// texto nulo ou tamanho abaixo de 50: a original (caixa "Error", 0).
pub(crate) unsafe extern "C" fn scout_manager_write_list_name(indice: u32, text: usize, size: i32) -> u32 {
    // 0x66f4e0: cdecl(texto, formato, ...) (add esp, 8 ou 0xc)
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || text == 0 || size < 0x32 {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(IDX_SCOUT_MANAGER_WRITE_LIST_NAME)(indice, text, size); // caixa: só a original
    }
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match k {
        0 => translate(text, 0xa7dd94),
        1 => translate(text, 0xa7dd84),
        2 => translate(text, 0xa7dd74),
        _ => translate(text, 0xa7dd5c, k as i32 - 2),
    };
    1
}


/// whether the club (or the current human's) asked to scout the player, with fewer than 3 reports and, if asked, with
/// the date already due
/// Original: `SCOUT_MANAGER::has_scout_request` (0x7f8d70, scout_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7f8d70 } (thiscall, ret 0xc, scout_manager.cpp): `SCOUT_MANAGER::has_scout_request(jogador, clube, so_vencidos)`:
/// se o clube (nulo: o do humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]] +0x39) pediu a observação do jogador: busca
/// binária (0x946464 com 0x7fc270) pelo par (id do clube, id do jogador) nos pedidos (+0x5c, conta +0x64, 17 bytes)
/// com menos de 3 relatórios (+0x10, com sinal); com `so_vencidos`, só se a data do pedido (+8: dia, +0xa: ano) é
/// anterior a hoje ([0xae2c90], ano sem sinal, dia com sinal). Devolve 1 ou 0. Jogador nulo: a original (caixa da
/// linha 0x5da). A data local da original (0x54a110) só ocupa a chave depois do par.
pub(crate) unsafe extern "thiscall" fn has_scout_request(this: usize, player: usize, clube: usize, so_vencidos: u32) -> u32 {
    if player == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_HAS_SCOUT_REQUEST)(this, player, clube, so_vencidos);
    }
    let mut clube = clube;
    if clube == 0 {
        let human = read_ptr(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize));
        if human == 0 {
            return 0;
        }
        clube = read_ptr(human + 0x39);
        if clube == 0 {
            return 0;
        }
    }
    let list = read_ptr(this + 0x5c);
    let found = if list == 0 {
        0
    } else {
        let data: unsafe extern "thiscall" fn(*mut u8) -> *mut u8 = core::mem::transmute(FN_DATE_DATE_54A110);
        let search: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        let mut key = [0u8; 16];
        data(key.as_mut_ptr().add(8));
        key[4..8].copy_from_slice(&read_u32(player).to_le_bytes());
        key[0..4].copy_from_slice(&read_u32(clube).to_le_bytes());
        search(key.as_ptr(), list, read_u32(this + 0x64), 0x11, 0x7fc270)
    };
    if found == 0 {
        crate::comparacao::ramo("cpq_12 sem pedido");
        return 0; // sem pedido
    }
    if *((found + 0x10) as *const i8) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let year = core::ptr::read_unaligned((found + 0xa) as *const u16);
        let today = *(G_CURRENT_DATE_YEAR as *const u16);
        if year > today {
            return 0;
        }
        if year == today && core::ptr::read_unaligned((found + 8) as *const i16) >= *(G_CURRENT_DATE as *const i16) {
            return 0;
        }
    }
    1
}

