use super::*;



/// Range (start, end) of the 8-byte items of the discipline nation index ([this+0xc]) for the team's nation, found by
/// bsearch then widened.
///
/// [pt] Os itens (8 bytes: +0 id da pessoa, +4 id da nação) do índice das nações da disciplina ([this+0xc], conta +0x12
/// com sinal) da nação do `time` (+0x53), na ordem do índice: bsearch (0x946464 com 0x55e5f0, cdecl) pela chave (−1,
/// nação), que iguala qualquer pessoa da nação; volta até o primeiro igual e segue até o último. 0 itens sem achar.
pub(crate) unsafe fn nation_index_range(this: usize, team: usize) -> (usize, usize) {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = [u32::MAX, read_usize(read_usize(team + 0x53)) as u32];
    let base = read_usize(this + 0xc);
    let mut e = bsearch(key.as_ptr(), base, read_i16(this + 0x12) as i32, 8, 0x55e5f0);
    if e == 0 {
        return (0, 0);
    }
    // os itens são iguais à chave pela nação (+4): a chave tem −1 no id
    let same = |x: usize| read_usize(x + 4) as u32 == key[1];
    while e > base && same(e - 8) {
        e -= 8;
    }
    let mut end = e;
    while end <= base.wrapping_add((read_i16(this + 0x12) as i32 as usize).wrapping_mul(8)).wrapping_sub(8) && same(end) {
        end += 8;
    }
    (e, end)
}

/// in a national team match: the people of its nation in the nation index (binary search by nation) serve one match of
/// their ban under rule 0, one by one from a list of the ids (the count can release the ban and reorder the index)
/// Original: `Discipline::serve_international_bans` (0x557120, discipline.cpp, thiscall).
///
/// [pt] { 0x557120 } (thiscall, ret 4, discipline.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `Discipline::serve_international_bans(time)`, pelo jogo da seleção (0x7e8090, no órgão das suspensões). As pessoas da
/// nação do time no índice das nações (nation_index_range) vão para uma lista (0x68d0d0 com o item e 4: a cópia do id, porque
/// a contagem pode soltar a suspensão e reordenar o índice) e, para cada uma, um jogo cumprido na regra 0
/// (0x556e90 com 0 e 1). No fim a lista é destruída.
pub(crate) unsafe extern "thiscall" fn serve_international_bans(this: usize, team: usize) {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let count: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_DISCIPLINE_UPDATE_BAN_PROGRESS);
    let mut l = [0u8; 14];
    let list = l.as_mut_ptr() as usize;
    new(list);
    let (ini, fim) = nation_index_range(this, team);
    if ini != 0 {
        let mut e = ini;
        while e < fim {
            por(list, e, 4);
            e += 8;
        }
        // o cursor (+0xa) no começo (+2)
        core::ptr::write_unaligned((list + 0xa) as *mut usize, read_usize(list + 2));
        let mut p = next(list);
        while p != 0 {
            crate::comparacao::ramo("cqb_9 jogo cumprido");
            count(this, read_usize(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0
            p = next(list);
        }
    }
    soltar(list);
}


/// O plano do recurso: a entrada da pessoa no órgão da `regra` (0x7e82e0 sem criar), a suspensão (20 bytes em
/// [this+4]), os valores novos de +0 (jogos) ou +2 (dias) e se ela é solta.
pub(crate) struct AppealPlan {
    pub(crate) orgao: usize,
    pub(crate) entrada: usize,
    pub(crate) suspensao: usize,
    pub(crate) jogos: u8,
    pub(crate) dias: i16,
    pub(crate) solta: bool,
}

/// Plan of a discipline appeal for person id under a rule: finds the entry and ban, computes reduced matches/days and
/// whether the ban is lifted.
pub(crate) unsafe fn appeal_plan(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32) -> AppealPlan {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_RULINGBODY_FIND_OR_ADD_DISCIPLINE_ENTRY);
    let orgao = read_usize(read_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)));
    let entrada = achar(orgao, id, 0);
    let mut r = AppealPlan { orgao, entrada, suspensao: 0, jogos: 0, dias: 0, solta: false };
    if entrada == 0 || read_i16(entrada + 6) == -1 {
        return r;
    }
    let s = read_usize(this + 4).wrapping_add((read_i16(entrada + 6) as i32 as usize).wrapping_mul(20));
    r.suspensao = s;
    r.jogos = *(s as *const u8);
    r.dias = read_i16(s + 2);
    if r.jogos != 0xff {
        if novo as u8 != 0xff {
            r.jogos = r.jogos.wrapping_sub(novo as u8);
            if (r.jogos as i8) < 0 {
                r.jogos = 0;
                r.solta = true;
            }
        } else if reduz as u8 != 0 {
            r.jogos = r.jogos.wrapping_sub(menos_jogos as u8);
            r.solta = r.jogos as i8 <= *((s + 1) as *const i8);
        } else {
            r.solta = true;
        }
    } else if dias_fixo as u16 != 0xffff {
        r.dias = r.dias.wrapping_sub(menos_dias as i16);
        if r.dias <= 0 {
            r.dias = 0;
            r.solta = true;
        }
    } else if reduz as u8 != 0 {
        r.dias = r.dias.wrapping_sub(menos_dias as i16);
        r.solta = r.dias <= 0;
    } else {
        r.solta = true;
    }
    r
}

/// the result of a ban appeal: the ruling body drops the person's pending bans; a match ban loses matches (or stops at
/// the counter) and a timed ban loses days, and it is released when over or when the appeal lifts it
/// Original: `Discipline::apply_appeal_result` (0x557000, discipline.cpp, thiscall).
///
/// [pt] { 0x557000 } (thiscall, ret 0x20, discipline.cpp): `Discipline::apply_appeal_result(id, regra, reduz, menos_jogos,
/// menos_dias, novo, dias_fixo, _)`, pelo resultado do recurso (0x6abf99, notícias do técnico humano). Com a entrada da
/// pessoa no órgão (0x7e82e0 sem criar) e suspensão (+6): o órgão tira as suspensões pendentes dela (0x7e91d0); por
/// jogos (+0 ≠ 0xff): `novo` ≠ 0xff tira `novo` jogos (abaixo de 0: zera e solta), senão `reduz` tira `menos_jogos` e
/// solta se ficar até o contador (+1), senão solta; por dias: `dias_fixo` ≠ −1 tira `menos_dias` (até 0: zera e solta),
/// senão `reduz` tira e solta até 0, senão solta. Soltar: 0x556d00 com o índice.
pub(crate) unsafe extern "thiscall" fn apply_appeal_result(this: usize, id: u32, rule: u32, reduz: u32, minus_games: u32, minus_days: u32, new: u32, dias_fixo: u32, _x: u32) {
    let tirar_pendentes: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_RULINGBODY_REMOVE_PENDING_BANS);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
    let r = appeal_plan(this, id, rule, reduz, minus_games, minus_days, new, dias_fixo);
    if r.suspensao == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_9 recurso");
    tirar_pendentes(r.orgao, id);
    let s = r.suspensao;
    *(s as *mut u8) = r.jogos;
    core::ptr::write_unaligned((s + 2) as *mut i16, r.dias);
    if r.solta {
        // simplification: a original empurra o registrador com o índice na palavra baixa (o alto é sobra); a 0x556d00 só lê
        // a palavra
        soltar(this, id, read_i16(r.entrada + 6) as u16 as u32);
    }
}


/// Rule (ruling body byte, 0xff none) of ban slot `vaga` in the person's 29-byte discipline record ([this+0x14]).
///
/// [pt] A vaga `vaga` (byte com sinal) da suspensão da pessoa no registro de 29 bytes ([this+0x14]: 5 bytes por vaga): o
/// órgão da regra (0xff = sem vaga).
pub(crate) unsafe fn ban_slot_rule(this: usize, person: usize, slot: u32) -> u8 {
    let id = read_usize(person);
    *((read_usize(this + 0x14) + id.wrapping_mul(29) + (slot as i8 as isize as usize).wrapping_mul(5)) as *const u8)
}

/// the text of the ban in the person's slot, from the ruling body of its rule; with no rule in the slot, 0
/// Original: `Discipline::get_slot_text` (0x55d3c0, discipline.cpp, thiscall).
///
/// [pt] { 0x55d3c0 } (thiscall, ret 0x10, discipline.cpp): `Discipline::get_slot_text(pessoa, vaga, saída, tamanho)`, pela
/// tela da pessoa (0x87618a, com 0xc8). Pessoa de id a partir de [0xae2364]: 0. A regra da vaga (ban_slot_rule) 0xff: 0;
/// senão o texto do órgão dela ([[this] + 4 × regra], 0x7e88d0 com a pessoa, a saída e o tamanho). Pessoa nula: a
/// original (lê o id de 0).
pub(crate) unsafe extern "thiscall" fn get_slot_text(this: usize, person: usize, slot: u32, out: usize, size: u32) -> u32 {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(IDX_GET_SLOT_TEXT)(this, person, slot, out, size);
    }
    if read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let rule = ban_slot_rule(this, person, slot);
    if rule == 0xff {
        return 0;
    }
    let text: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_GET_BAN_TEXT);
    text(read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4))), person, out, size)
}






/// Ruling body of rule (signed byte): [[this] + 4 x rule].
///
/// [pt] O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
pub(crate) unsafe fn ruling_body_of_rule(this: usize, rule: u8) -> usize {
    read_u32_as_usize(read_u32_as_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4)))
}


/// Person id's 29-byte discipline record in [this+0x14] (4 slots of 5 bytes, history, stored ban count).
///
/// [pt] O registro de 29 bytes da pessoa `id` em [this+0x14] (4 vagas de 5 bytes: a regra e o índice da entrada; +0x14/+0x18
/// o histórico; +0x1c quantas suspensões guardadas).
pub(crate) unsafe fn discipline_record(this: usize, id: u32) -> usize {
    read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29))
}

/// Does the ruling body apply to the club (by nation +0x17, else continent +0x1b, else always)?
///
/// [pt] O órgão vale para o clube? Com nação (+0x17 ≠ −1): a nação do clube (+0x53) é ela; sem nação, com continente
/// (+0x1b ≠ −1): o continente da nação do clube (+0x71) é ele; sem os dois, vale sempre. Clube nulo ou sem nação: não.
pub(crate) unsafe fn ruling_body_covers_club(orgao: usize, clube: usize) -> bool {
    let nacao = read_u32_as_usize(orgao + 0x17) as u32;
    if nacao == 0xffff_ffff {
        let continent = read_u32_as_usize(orgao + 0x1b) as u32;
        if continent == 0xffff_ffff {
            return true;
        }
        if clube == 0 || read_u32_as_usize(clube + club::NATION) == 0 {
            return false;
        }
        let c = read_u32_as_usize(read_u32_as_usize(clube + club::NATION) + 0x71);
        return c != 0 && read_u32_as_usize(c) as u32 == continent;
    }
    clube != 0 && read_u32_as_usize(clube + club::NATION) != 0 && read_u32_as_usize(read_u32_as_usize(clube + club::NATION)) as u32 == nacao
}

/// Releases the entry's ban (if +6 > -1) via release_ban with the original's dirty high bits.
///
/// [pt] Solta a suspensão da entrada (se tem: +6 > −1) e libera a vaga `k` do registro da pessoa (regra 0xff, índice −1) e a
/// entrada (+0 = −1). O índice vai no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra).
pub(crate) unsafe fn release_entry_ban(this: usize, id: u32, e: usize) {
    let i = read_i16(e + 6);
    if i > -1 {
        let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
        soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
    }
}

/// Clears slot k of the person's discipline record (rule 0xff, index -1) and the entry (+0 = -1).
pub(crate) unsafe fn clear_discipline_slot(this: usize, id: u32, k: usize, e: usize) {
    let v = discipline_record(this, id).wrapping_add(k * 5);
    *(v as *mut u8) = 0xff;
    write_u32(v + 1, 0xffff_ffff);
    write_u32(e, 0xffff_ffff);
}

/// when the person already has a ban under one of the 4 rules, extends the old one (matches or days) and flags both;
/// otherwise copies the new ban into the vector's first free slot (or the end, growing by 100 up to 0x7d00), links each
/// rule to it (0x7e8510) and, with the fourth rule, adds {id, nation} to the nation index (qsort)
/// Original: `Discipline::add_ban` (0x556940, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x556940 } (thiscall, ret 8, discipline.cpp): `Discipline::add_ban(id, suspensao)`: se alguma das 4 regras da
/// suspensão nova (+0xc..+0xf) já tem suspensão da pessoa (a entrada do órgão com +6 > −1), estende a velha (jogos +0
/// somados, ou, com algum dos dois por dias (0xff), os dias +2) e marca as duas (+0x13 bit 0); senão (o bit 0 da nova
/// limpo a cada regra sem suspensão) copia a nova (0x556c70) na primeira vaga livre do vetor (+0 = 0xff e +2 = −1), ou
/// no fim (conta +0xa, capacidade +8; cresce 100, até 0x7d00, malloc/realloc, as vagas novas livres), liga cada regra
/// ao índice (0x7e8510) e, com a quarta regra, põe {id, nação} no índice das nações (+0xc, capacidade +0x10, conta
/// +0x12; cresce 20; qsort 0x55e5f0). Caixas "Error" (capacidade no máximo, sem memória) pela show_error_at.
pub(crate) unsafe extern "thiscall" fn add_ban(this: usize, id: u32, ban: usize) {
    for k in 0..4usize {
        let rule = *((ban + 0xc + k) as *const u8);
        if rule == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
        if e != 0 && read_i16(e + 6) > -1 {
            crate::comparacao::ramo("cpo_11 suspensao estendida");
            let s = read_u32_as_usize(this + 4).wrapping_add((read_i16(e + 6) as i32 as usize).wrapping_mul(20));
            let games = *(s as *const u8);
            let novos = *(ban as *const u8);
            if games != 0xff && novos != 0xff {
                *(s as *mut u8) = novos.wrapping_add(games);
            } else {
                write_u16(s + 2, (read_i16(s + 2) as u16).wrapping_add(read_i16(ban + 2) as u16));
            }
            *((ban + 0x13) as *mut u8) |= 1;
            *((s + 0x13) as *mut u8) |= 1;
            return;
        }
        *((ban + 0x13) as *mut u8) &= 0xfe;
    }
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_BANRECORD_OPERATOR_3D);
    let n = read_i16(this + 0xa);
    let mut slot: i16 = -1;
    for i in 0..n.max(0) {
        let s = read_u32_as_usize(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && read_i16(s + 2) == -1 {
            slot = i;
            copy(s, ban);
            break;
        }
    }
    if slot == -1 {
        let count = read_i16(this + 0xa);
        let cap = read_i16(this + 8);
        if count < cap {
            write_u16(this + 0xa, count.wrapping_add(1) as u16);
            copy(read_u32_as_usize(this + 4).wrapping_add((count as i32 as usize).wrapping_mul(20)), ban);
            slot = read_i16(this + 0xa).wrapping_sub(1);
        } else {
            if cap >= 0x7d00 {
                show_error_at(0x9b8f18, 0x401);
                return;
            }
            let new = cap.wrapping_add(100);
            write_u16(this + 8, new as u16);
            let size = (new as i32 as u32).wrapping_mul(20);
            let p = if read_u32_as_usize(this + 4) == 0 {
                let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
                malloc(size)
            } else {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realloc(read_u32_as_usize(this + 4), size)
            };
            write_u32(this + 4, p as u32);
            if p == 0 {
                show_error_at(0x9b8f18, 0x410);
                return;
            }
            slot = read_i16(this + 0xa);
            write_u16(this + 0xa, slot.wrapping_add(1) as u16);
            copy(p.wrapping_add((slot as i32 as usize).wrapping_mul(20)), ban);
            let mut i = read_i16(this + 0xa);
            while i < read_i16(this + 8) {
                let s = read_u32_as_usize(this + 4).wrapping_add((i as i32 as usize).wrapping_mul(20));
                *(s as *mut u8) = 0xff;
                write_u16(s + 2, 0xffff);
                i = i.wrapping_add(1);
            }
        }
    }
    crate::comparacao::ramo("cpo_11 suspensao nova");
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_RULINGBODY_ATTACH_BAN);
    for k in 0..4usize {
        let rule = *((ban + 0xc + k) as *const u8);
        if rule != 0xff {
            ligar(ruling_body_of_rule(this, rule), id, slot as u16 as u32); // liga a regra à suspensão
        }
    }
    if *((ban + 0xf) as *const u8) == 0xff {
        return;
    }
    let cap = read_i16(this + 0x10);
    if cap == read_i16(this + 0x12) {
        let new = cap.wrapping_add(0x14);
        write_u16(this + 0x10, new as u16);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_u32_as_usize(this + 0xc), (new as i32 as u32).wrapping_mul(8));
        write_u32(this + 0xc, p as u32);
        if p == 0 {
            show_error_at(0x9b8f18, 0x43e);
            return;
        }
    }
    let n = read_i16(this + 0x12);
    let item = read_u32_as_usize(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
    write_u32(item, id);
    let person = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    write_u32(item + 4, read_u32_as_usize(read_u32_as_usize(person + staff::NATION)) as u32); // a nação
    write_u16(this + 0x12, n.wrapping_add(1) as u16);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32_as_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0);
}

/// on a club move, frees each of the person's 4 record slots whose ruling body (nation +0x17 or continent +0x1b) does
/// not cover the new club: releases the ban, frees the slot and the entry; with held bans, shows the "Error" box
/// Original: `Discipline::clear_bans_invalid_for_club` (0x556270, discipline.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x556270 } (thiscall, ret 0xc, discipline.cpp): `Discipline::clear_bans_invalid_for_club(pessoa, _, clube)`: cada
/// vaga ocupada das 4 do registro da pessoa cujo órgão não vale para o clube novo (`ruling_body_covers_club`) é liberada: a
/// suspensão solta (0x556d00), a vaga livre e a entrada do órgão livre. Com suspensões guardadas (+0x1c ≠ 0), a caixa
/// "Error" antes (e segue). Da troca de clube (0x8c9720).
pub(crate) unsafe extern "thiscall" fn clear_bans_invalid_for_club(this: usize, person: usize, _nada: u32, clube: usize) {
    let id = read_u32_as_usize(person) as u32;
    let r = discipline_record(this, id);
    for k in 0..4usize {
        let rule = *((r + k * 5) as *const u8);
        if rule == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, rule);
        if ruling_body_covers_club(orgao, clube) {
            continue;
        }
        let e = ruling_body_entry(orgao, id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_11 transferido solta a vaga");
        release_entry_ban(this, id, e);
        if *((r + 0x1c) as *const u8) != 0 {
            show_error_at(0x9b8f18, 0x239);
        }
        clear_discipline_slot(this, id, k, e);
    }
}

/// O que a 0x556570 vai fazer, lido antes (a lista das guardadas perde e libera os itens): o contrato e o clube, os
/// itens guardados da pessoa (a vaga e os 5 bytes) e, depois deles, as vagas 0..2 não devolvidas que o clube não cobre.
pub(crate) struct ContractReturnPlan {
    pub(crate) contrato: usize,
    pub(crate) clube: usize,
    pub(crate) guardadas: Vec<(usize, [u8; 5])>,
    pub(crate) resto: u8,
    pub(crate) soltas: Vec<(usize, usize)>,
}

/// Read-only plan of 0x556570: the person's contract and club, stored bans restored from the list, and bans to release
/// for bodies not covering the club.
///
/// [pt] O plano da 0x556570 (o cursor da lista é zerado pela original na entrada; a busca sem criar só lê).
pub(crate) unsafe fn return_bans_plan(this: usize, person: usize) -> ContractReturnPlan {
    let id = read_u32_as_usize(person) as u32;
    let r = discipline_record(this, id);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    let clube = if contrato == 0 { 0 } else { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(contrato + 4).wrapping_mul(club::SIZE)) };
    let mut p = ContractReturnPlan { contrato, clube, guardadas: Vec::new(), resto: *((r + 0x1c) as *const u8), soltas: Vec::new() };
    let list = this + 0x1c;
    write_u32(list + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut slots = [0u8; 20];
    core::ptr::copy_nonoverlapping(r as *const u8, slots.as_mut_ptr(), 20);
    let mut marks = [false; 4];
    let mut item = next(list);
    while item != 0 && (p.resto as i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            let k = (*((item + 4) as *const u8) as i8 as isize as usize) & 3;
            let mut b = [0u8; 5];
            core::ptr::copy_nonoverlapping((item + 5) as *const u8, b.as_mut_ptr(), 5);
            slots[k * 5..k * 5 + 5].copy_from_slice(&b);
            marks[k] = true;
            p.guardadas.push((k, b));
            p.resto = p.resto.wrapping_sub(1);
        }
        item = next(list);
    }
    for k in 0..3usize {
        if marks[k] || slots[k * 5] == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, slots[k * 5]);
        if !ruling_body_covers_club(orgao, clube) {
            let e = ruling_body_entry(orgao, id);
            if e != 0 {
                p.soltas.push((k, e));
            }
        }
    }
    p
}

/// when a loan ends, puts the person's held bans back into their slots (freeing the occupied slot first; the item
/// leaves the list) and frees slots 0..2 not restored whose ruling body does not cover the contract club
/// Original: `Discipline::restore_held_bans` (0x556570, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x556570 } (thiscall, ret 4, discipline.cpp): `Discipline::restore_held_bans(pessoa)`: no fim do empréstimo
/// (0x4d07e0): o contrato (0x4cf5a0, guardado em [0xae1080]) dá o clube; cada suspensão guardada da pessoa na lista
/// [this+0x1c] (id, vaga +4, os 5 bytes +5), enquanto houver guardadas (+0x1c do registro, com sinal): a vaga ocupada é
/// liberada antes (a suspensão solta, 0x556d00), a guardada volta para a vaga, sai da lista (0x68d1f0, que a libera) e a
/// conta cai. Depois, as vagas 0..2 não devolvidas (a quarta nunca: `k != 3` da original) cujo órgão não vale para o
/// clube são liberadas.
pub(crate) unsafe extern "thiscall" fn restore_held_bans(this: usize, person: usize) {
    let id = read_u32_as_usize(person) as u32;
    let r = discipline_record(this, id);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, contrato as u32);
    let clube = if contrato == 0 { 0 } else { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(contrato + 4).wrapping_mul(club::SIZE)) };
    let list = this + 0x1c;
    write_u32(list + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut marks = [0u32; 4];
    let mut item = next(list);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            let k = *((item + 4) as *const u8) as i8 as isize as usize;
            let rule = *(r.wrapping_add(k.wrapping_mul(5)) as *const u8);
            if rule != 0xff {
                let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
                if e != 0 {
                    release_entry_ban(this, id, e);
                    clear_discipline_slot(this, id, k, e);
                }
            }
            crate::comparacao::ramo("cpo_11 guardada devolvida");
            marks[k & 3] = 1;
            let v = r.wrapping_add(k.wrapping_mul(5));
            write_u32(v, read_u32_as_usize(item + 5) as u32);
            *((v + 4) as *mut u8) = *((item + 9) as *const u8);
            remover(list, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = next(list);
    }
    for k in 0..4usize {
        let rule = *((r + k * 5) as *const u8);
        if marks[k] != 0 || k == 3 || rule == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, rule);
        if ruling_body_covers_club(orgao, clube) {
            continue;
        }
        let e = ruling_body_entry(orgao, id);
        if e != 0 {
            crate::comparacao::ramo("cpo_11 fim do emprestimo solta a vaga");
            release_entry_ban(this, id, e);
            clear_discipline_slot(this, id, k, e);
        }
    }
}

/// O caminho da 0x556940, lido antes: a suspensão velha estendida (e os valores novos dela), ou a vaga da nova e a
/// conta do vetor depois; `None` nas caixas (capacidade no máximo).
pub(crate) enum SuspensionPlan {
    Estender { s: usize, jogos: u8, dias: i16 },
    Nova { vaga: i16, conta: i16 },
    Caixa,
}

/// Plan for adding a ban: extend an existing one for the same body, reuse a free ban slot, append, or error box when
/// full.
pub(crate) unsafe fn add_ban_plan(this: usize, id: u32, ban: usize) -> SuspensionPlan {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, regra), id);
        if e != 0 && read_i16(e + 6) > -1 {
            let s = read_u32_as_usize(this + 4).wrapping_add((read_i16(e + 6) as i32 as usize).wrapping_mul(20));
            let (jogos, novos) = (*(s as *const u8), *(ban as *const u8));
            return if jogos != 0xff && novos != 0xff {
                SuspensionPlan::Estender { s, jogos: novos.wrapping_add(jogos), dias: read_i16(s + 2) }
            } else {
                SuspensionPlan::Estender { s, jogos, dias: read_i16(s + 2).wrapping_add(read_i16(ban + 2)) }
            };
        }
    }
    let n = read_i16(this + 0xa);
    for i in 0..n.max(0) {
        let s = read_u32_as_usize(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && read_i16(s + 2) == -1 {
            return SuspensionPlan::Nova { vaga: i, conta: n };
        }
    }
    if n < read_i16(this + 8) || read_i16(this + 8) < 0x7d00 {
        return SuspensionPlan::Nova { vaga: n, conta: n.wrapping_add(1) };
    }
    SuspensionPlan::Caixa
}






/// Person id's 29-byte discipline record in [this+0x14].
///
/// [pt] O registro de 29 bytes da pessoa `id` em [this+0x14] (+0x14/+0x18: o começo e o último deslocamento do histórico).
pub(crate) unsafe fn person_discipline_record(this: usize, id: usize) -> usize {
    read_usize(this + 0x14).wrapping_add(id.wrapping_mul(29))
}

/// Ruling body of rule (signed byte): [[this] + 4 x rule].
///
/// [pt] O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
pub(crate) unsafe fn ruling_body_for_rule(this: usize, rule: u8) -> usize {
    read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4)))
}

/// The person's 14-byte entry in a ruling body (0x7e82e0 without creating; +6 ban index).
///
/// [pt] A entrada (14 bytes; +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar (só lê).
pub(crate) unsafe fn ruling_body_entry(orgao: usize, id: u32) -> usize {
    let find: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_RULINGBODY_FIND_OR_ADD_DISCIPLINE_ENTRY);
    find(orgao, id, 0)
}

/// Person's item (id, nation) in the discipline nation index by bsearch on his nation, then on his second nation; 0 if
/// absent.
///
/// [pt] O item (id, nação) da pessoa no índice das nações (+0xc, 8 bytes por item, conta +0x12 com sinal): bsearch
/// (0x946464 com 0x55e5f0) pela nação (+0x1a) e, sem achar, pela segunda nação (+0x1e, se houver). 0 sem achar.
pub(crate) unsafe fn find_in_nation_index(this: usize, id: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let mut key = [id, read_usize(read_usize(person + staff::NATION)) as u32];
    let e = bsearch(key.as_ptr(), read_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0);
    if e != 0 {
        return e;
    }
    let second = read_usize(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)) + 0x1e);
    if second == 0 {
        return 0;
    }
    key[1] = read_usize(second) as u32;
    bsearch(key.as_ptr(), read_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0)
}

/// O que a 0x556e90 faz com o jogo: a entrada no órgão, a suspensão (20 bytes em [this+4]), o contador novo de jogos
/// cumpridos (+1) e se a suspensão termina.
pub(crate) struct SuspensionMatchUpdate {
    pub(crate) entrada: usize,
    pub(crate) suspensao: usize,
    pub(crate) contador: Option<u8>,
    pub(crate) cumprida: bool,
}

/// Read-only plan of a match for a ban: by matches (count +1 when started) or by days (ends when today reaches start +
/// days); reports completion.
///
/// [pt] O plano do jogo (só lê; a 0x7e82e0 sem criar e a 0x54c5b0 só gravam a data de rascunho dela): a suspensão da
/// pessoa no órgão da `regra`; por jogos (+0 ≠ 0xff): a partir do começo (dia +4, ano +6; hoje [0xae2c90]: dia +0,
/// ano +2), com `conta`, +1 no contador (+1), e termina ao chegar em +0; por dias (+0 = 0xff): termina quando hoje
/// chega ao começo + dias (+2) (0x54c5b0).
pub(crate) unsafe fn serve_ban_match_plan(this: usize, id: u32, rule: u32, count: u32) -> SuspensionMatchUpdate {
    let mut j = SuspensionMatchUpdate { entrada: 0, suspensao: 0, contador: None, cumprida: false };
    let e = ruling_body_entry(ruling_body_for_rule(this, rule as u8), id);
    if e == 0 || read_i16(e + 6) == -1 {
        return j;
    }
    j.entrada = e;
    let cinco = (read_i16(e + 6) as i32 as u32).wrapping_mul(5); // o lea da original (o alto vai no edx abaixo)
    let s = read_usize(this + 4).wrapping_add((cinco as usize).wrapping_mul(4));
    j.suspensao = s;
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = (today as u16 as i16, (today >> 16) as u16);
    let games = *(s as *const u8);
    if games != 0xff {
        let (dia0, ano0) = (read_i16(s + 4), read_i16(s + 6) as u16);
        if ano < ano0 || (ano == ano0 && dia < dia0) { // ainda não começou
            crate::comparacao::ramo("cpm_4 antes do inicio");
            return j;
        }
        if count == 0 {
            return j;
        }
        crate::comparacao::ramo("cpm_4 jogo cumprido");
        let n = (*((s + 1) as *const u8)).wrapping_add(1); // mais um jogo cumprido
        j.contador = Some(n);
        j.cumprida = n == games;
    } else {
        let mut end = [0u8; 8];
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        // a original empurra o edx com os dias no dx e, no alto, o do índice × 5 (zero abaixo de 13108 suspensões)
        let f = mais_dias(s + 4, end.as_mut_ptr(), (cinco & 0xffff_0000) | read_i16(s + 2) as u16 as u32);
        let (dia1, ano1) = (read_i16(f), read_i16(f + 2) as u16);
        if ano < ano1 || (ano == ano1 && dia < dia1) {
            return j;
        }
        j.cumprida = true;
    }
    j
}

/// the person's ban in the rule's ruling body: match bans, from the start date (+4/+6) and when counting, +1 to the
/// counter (+1); on reaching the total (+0), or day bans once today reaches start + days (+2, 0x54c5b0), it goes to the
/// history (0x58e340) and is released (0x556d00)
/// Original: `Discipline::update_ban_progress` (0x556e90, discipline.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x556e90 } (thiscall, ret 0xc, discipline.cpp): `Discipline::update_ban_progress(id, regra, conta)`: a suspensão
/// da pessoa no órgão da `regra` (plano em `serve_ban_match_plan`): por jogos, grava o contador novo (+1); cumprida, vai para o
/// histórico (0x58e340 sobre [this+0x18] com o começo e o último deslocamento do registro de 29 bytes) e é solta
/// (0x556d00). Sem SEH, sem sorteio, sem caixa.
pub(crate) unsafe extern "thiscall" fn update_ban_progress(this: usize, id: u32, rule: u32, count: u32) {
    let j = serve_ban_match_plan(this, id, rule, count);
    if let Some(n) = j.contador {
        *((j.suspensao + 1) as *mut u8) = n;
    }
    if !j.cumprida {
        return;
    }
    let r = person_discipline_record(this, id as usize);
    let history: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let g = history(read_usize(this + 0x18), r + 0x14, r + 0x18, j.suspensao);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
    // o índice vai no ax; o alto do eax é o da devolução da 0x58e340 (a 0x556d00 só lê a palavra)
    soltar(this, id, (g as u32 & 0xffff_0000) | read_i16(j.entrada + 6) as u16 as u32);
}

/// each ruling body of the ban's 4 rules (+0xc..+0xf) forgets it (0x7e8590); on the fourth, the person's item leaves
/// the nation index (last one moved in, qsort); the ban becomes free (+0 = 0xff, +2 = −1)
/// Original: `Discipline::release_ban` (0x556d00, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x556d00 } (thiscall, ret 8, discipline.cpp): `Discipline::release_ban(id, indice)`: a suspensão `indice` (20
/// bytes em [this+4]; a palavra com sinal): para cada uma das 4 regras (+0xc..+0xf, 0xff = sem), o órgão a esquece
/// (0x7e8590); na quarta, o item da pessoa sai do índice das nações (o último vem para o lugar, conta +0x12 − 1, qsort
/// 0x945333 com 0x55e5f0; sem achar, caixa "Error" e segue). Por fim a suspensão fica livre (+0 = 0xff, +2 = −1).
/// Devolve a suspensão.
pub(crate) unsafe extern "thiscall" fn release_ban(this: usize, id: u32, indice: u32) -> usize {
    let s = read_usize(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    let esquecer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_RULINGBODY_DETACH_BAN);
    for k in 0..4usize {
        let rule = *((s + 0xc + k) as *const u8);
        if rule == 0xff {
            continue;
        }
        crate::comparacao::ramo("cpm_4 soltar do orgao");
        esquecer(ruling_body_for_rule(this, rule), id); // o órgão esquece a suspensão
        if k != 3 {
            continue;
        }
        let e = find_in_nation_index(this, id);
        if e == 0 {
            show_error_at(0x9b8f18, 0x47d);
            continue;
        }
        let n = read_i16(this + 0x12).wrapping_sub(1);
        core::ptr::write_unaligned((this + 0x12) as *mut i16, n);
        let last = read_usize(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
        core::ptr::write_unaligned(e as *mut u32, read_usize(last) as u32);
        core::ptr::write_unaligned((e + 4) as *mut u32, read_usize(last + 4) as u32);
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0);
    }
    *(s as *mut u8) = 0xff;
    core::ptr::write_unaligned((s + 2) as *mut u16, 0xffff);
    s
}

/// the last offset of the person's ban history (+0x18 of the 29-byte record) = −1 (0x58f110)
/// Original: `Discipline::clear_ban_history` (0x55e1d0, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x55e1d0 } (thiscall, ret 4, discipline.cpp): `Discipline::clear_ban_history(pessoa)`: o histórico de suspensões
/// da pessoa (registro de 29 bytes do id em [this+0x14]) fica sem o último (0x58f110 sobre [this+0x18]: +0x18 = −1).
/// Devolve o que a 0x58f110 devolve. Pessoa nula ou não jogadora (+0x61): a original (caixa fatal).
pub(crate) unsafe extern "thiscall" fn clear_ban_history(this: usize, person: usize) -> usize {
    if person == 0 || read_usize(person + staff::PLAYER) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLEAR_BAN_HISTORY)(this, person); // caixa fatal: só a original
    }
    let r = person_discipline_record(this, read_usize(person));
    let without_last: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    without_last(read_usize(this + 0x18), r + 0x14, r + 0x18)
}




/// copies the ban record (0x14 bytes): fields, the date (+4) and bits 0..2 of +0x13
/// Original: `BanRecord::operator=` (0x556c70, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x556c70 } (thiscall, ret 4, discipline.cpp): `BanRecord::operator=`: copia o registro de suspensão (0x14 bytes) de
/// `origem`: +0 e +1 (bytes), +2 (palavra), a data em +4 (0x417c10, retorno numa local), +0xc..+0xf (byte a byte),
/// +0x10 (byte), +0x11 (palavra) e os bits 0..2 de +0x13 (os outros bits ficam). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn discipline_operator(this: usize, source: usize) -> usize {
    let b = |o: usize| *((this + o) as *mut u8) = *((source + o) as *const u8);
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((source + o) as *const u16));
    b(0);
    b(1);
    w(2);
    let copy: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copy(this + 4, &mut local, source + 4);
    for o in 0xc..0x10 {
        b(o);
    }
    b(0x10);
    w(0x11);
    let m = (this + 0x13) as *mut u8;
    *m = (*m & !7) | (*((source + 0x13) as *const u8) & 7); // as três marcas
    this
}





// a original roda primeiro e atualiza o físico do dia (0x62c680, com sorteio); a cópia, depois, acha o dia já feito
comparar_valor!(comparar_get_match_condition, get_match_condition, IDX_GET_MATCH_CONDITION, (jogador: usize, em_porcento: u32) -> u16);

/// starts the new season of the competition's discipline rules (virtual method 0)
/// Original: `discipline_new_season` (0x556210, discipline.cpp, thiscall).
///
/// [pt] { 0x556210 } (thiscall, ret 4, discipline.cpp): começa a temporada nova da competição `*id`: as
/// regras da disciplina dela (índice +0x50 da competição em [0xadadfc], 0xff sem) recebem o método 0
/// da tabela virtual.
pub(crate) unsafe extern "thiscall" fn discipline_new_season(this: usize, id: *const i32) {
    let competicao = *((*(G_COMP_OBJECT_TABLE as *const usize) as isize + 4 * *id as isize) as *const usize);
    if competicao == 0 {
        return;
    }
    let k = *((competicao + 0x50) as *const i8);
    if k == -1 {
        return;
    }
    let rules = *((*(this as *const usize) as isize + 4 * k as isize) as *const usize);
    if rules != 0 {
        let metodo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(*(*(rules as *const usize) as *const usize));
        metodo(rules);
    }
}

/// the day of each of the 43 discipline rules (virtual method 1)
/// Original: `discipline_daily_update` (0x556240, discipline.cpp, thiscall).
///
/// [pt] { 0x556240 } (thiscall, ret 4, discipline.cpp): o dia da disciplina: cada uma das 43 regras ([this],
/// as não nulas) recebe o método 1 da tabela virtual com a data `hoje`.
pub(crate) unsafe extern "thiscall" fn discipline_daily_update(this: usize, today: usize) {
    for k in 0..43usize {
        let rules = *((*(this as *const usize) + 4 * k) as *const usize);
        if rules != 0 {
            let metodo: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(*((*(rules as *const usize) + 4) as *const usize));
            metodo(rules, today);
        }
    }
}

/// writes the type and value of one of the person's slots in the discipline table
/// Original: `discipline_set_slot` (0x55cb20, discipline.cpp, thiscall).
///
/// [pt] { 0x55cb20 } (thiscall, ret 0x10, discipline.cpp): grava a vaga `vaga` (5 bytes: o byte `tipo` e o
/// valor) da pessoa `id` na tabela +0x14 (29 bytes por pessoa).
pub(crate) unsafe extern "thiscall" fn discipline_set_slot(this: usize, id: i32, slot: i8, kind: u8, value: u32) {
    let a = (*((this + 0x14) as *const usize) as isize + id as isize * 29 + slot as isize * 5) as usize;
    *(a as *mut u8) = kind;
    core::ptr::write_unaligned((a + 1) as *mut u32, value);
}

// simplification: as duas primeiras só despacham pela tabela virtual (efeitos fora): provadas pela carreira



/// suspended by the general body (0x7e85b0 with 1) or by the competition one (+0x50, with 0); in that case, in a human
/// club, he leaves his slot among the 20 at +0x1e0
/// Original: `Discipline::is_player_suspended` (0x55d270, discipline.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x55d270 } (thiscall, ret 0x10, discipline.cpp): `Discipline::is_player_suspended(jogador, competição, extra,
/// partida)`: jogador fora das pessoas ([0xae2364]) ou competição (id em `*comp`, vetor [0xadadfc]) inexistente: 0.
/// Suspenso pelo tribunal geral ([[this]], 0x7e85b0 com 1): 1. Senão, pelo tribunal da competição (+0x50, ≠ −1, com
/// 0): 1, e se o clube dele (+0x39) é de humano (0x600a50 com 0, 1, 0, 0), ele sai da vaga que ocupa entre as 20 de
/// +0x1e0. Senão 0.
pub(crate) unsafe extern "thiscall" fn is_player_suspended(this: usize, player: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    if player != 0 && read_usize(player) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let v = read_usize(G_COMP_OBJECT_TABLE);
    if v == 0 {
        return 0;
    }
    let c = read_usize(v.wrapping_add((*comp as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_RULINGBODY_IS_PLAYER_BANNED);
    let tribunais = read_usize(this);
    if julgar(read_usize(tribunais), player, 1, extra, partida) != 0 {
        return 1; // a suspensão geral
    }
    let t = *((c + 0x50) as *const i8);
    if t == -1 {
        return 0;
    }
    let b = read_usize(tribunais.wrapping_add((t as isize as usize).wrapping_mul(4)));
    if b == 0 || julgar(b, player, 0, extra, partida) == 0 { // sem a suspensão da competição
        return 0;
    }
    crate::comparacao::ramo("cpk_3 suspenso pela competicao");
    let clube = read_usize(player + 0x39);
    if clube != 0 {
        let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if human(clube, 0, 1, 0, 0) != 0 {
            if let Some(s) = (0..20).map(|k| clube + club::TEAM_SELECTED + k * 4).find(|&s| read_usize(s) == player) {
                *(s as *mut usize) = 0; // sai da vaga
            }
        }
    }
    1
}


/// removes from the held list (+0x1c) the items of the person while the +0x1c counter of her record is positive
/// Original: `Discipline::drop_held_bans` (0x5567d0, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
pub(crate) unsafe extern "thiscall" fn drop_held_bans(this: usize, person: *const u32) {
    let l = this + 0x1c;
    let n = stored_suspension_counter(this, *person as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 && *n > 0 {
        if read_usize(e) as u32 == *person {
            list_remove(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = contract_manager_list_next(l);
    }
}


/// the bans (slots 0 to 2) that do not apply at the loan club go to the held list (+0x1c), the slot becomes −1 and the
/// +0x1c counter goes up
/// Original: `Discipline::hold_bans_for_loan` (0x5563f0, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x5563f0 } (thiscall, ret 4, discipline.cpp): `Discipline::hold_bans_for_loan(jogador)`: com a ficha de contrato
/// do jogador (0x4cf6d0 sobre [0xae1050]), cada suspensão do registro dele (vagas 0 a 2) que não vale no clube do
/// empréstimo vai para a lista das guardadas (+0x1c, 0x68d0d0 com 10 bytes: o id, a vaga e os 5 bytes dela), a vaga
/// fica −1 e o contador +0x1c do registro sobe. Sem ficha: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn hold_bans_for_loan(this: usize, player: usize) {
    let sheet_of: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let sheet = sheet_of(read_usize(G_CONTRACT_MANAGER), player as *const i32);
    if sheet == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_BANS_FOR_LOAN)(this, player); // caixa: só a original
    }
    let (reg, vagas) = suspensions_to_store(this, sheet, player);
    let append: unsafe extern "thiscall" fn(usize, *const u8, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for k in vagas {
        let slot = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(read_usize(player) as u32).to_le_bytes());
        r[4] = k; // a vaga
        r[5..].copy_from_slice(core::slice::from_raw_parts(slot as *const u8, 5));
        append(this + 0x1c, r.as_ptr(), 10);
        core::ptr::write_unaligned(slot as *mut u32, u32::MAX);
        *((slot + 4) as *mut u8) = 0xff;
        *((reg + 0x1c) as *mut u8) = (*((reg + 0x1c) as *const u8)).wrapping_add(1);
    }
}


/// with a valid id, the question to the competition's ruling body (+0x50; 0x7e8d60 with the id and the competition's
/// +4); otherwise 0
/// Original: `Discipline::is_one_booking_from_ban` (0x55d360, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x55d360 } (thiscall, ret 8, discipline.cpp): `Discipline::is_one_booking_from_ban(pessoa, competicao)`: com id
/// válido, a competição ([[0xadadfc] + 4 × id]) e o órgão dela (+0x50, 0xff sem; [this] + 4 × órgão), a pergunta ao órgão
/// (0x7e8d60 com o id, 0, 0 e o +4 da competição); senão 0. Pela tela do clube (0x4849f0).
pub(crate) unsafe extern "thiscall" fn is_one_booking_from_ban(this: usize, person: usize, competicao: usize) -> u32 {
    if person != 0 && read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(competicao) as u32).wrapping_mul(4) as usize));
    if comp == 0 || person == 0 {
        return 0;
    }
    let orgao = *((comp + 0x50) as *const u8);
    if orgao == 0xff {
        return 0;
    }
    let o = read_usize(read_usize(this).wrapping_add((orgao as i8 as i32 as u32).wrapping_mul(4) as usize));
    if o == 0 {
        return 0;
    }
    let perguntar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_CHECK_ONE_BOOKING_FROM_BAN);
    perguntar(o, read_usize(person) as u32, 0, 0, read_usize(comp + 4) as u32)
}


/// the penalty at the table row and column, if the type matches
/// Original: `table_penalty` (0x55cae0, discipline.cpp, thiscall).
///
/// [pt] { 0x55cae0 } (thiscall, ret 0xc, discipline.cpp): a punição da linha `linha` (29 bytes) e da coluna
/// `coluna` (5 bytes) da tabela [this+0x14]: o dword +1 se o byte +0 for `tipo`, senão −1.
pub(crate) unsafe extern "thiscall" fn table_penalty(t: usize, row: i32, coluna: i8, kind: i8) -> i32 {
    let e = (*((t + 0x14) as *const usize))
        .wrapping_add((row as usize).wrapping_mul(29))
        .wrapping_add((coluna as isize * 5) as usize);
    if *(e as *const i8) == kind {
        core::ptr::read_unaligned((e + 1) as *const i32)
    } else {
        -1
    }
}

/// the competition ruling body judges the match (0x7e7ff0); without it, in friendlies, the league ruling bodies of both
/// clubs
/// Original: `Discipline::process_match` (0x55ca00, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x55ca00 } (thiscall, ret 8, discipline.cpp): `Discipline::process_match(partida, resultado)`: o tribunal da
/// competição da partida (+0, id > −1) julga (0x7e7ff0 com 1, 1, 1). Sem ele, num amistoso (0x5c6710 sobre [0xae2c74]
/// com +0x14) ou na competição [0x9cf684]: o tribunal da liga (+0x57) do mandante (+0x1c, com 1, 0, 0) e o do
/// visitante (+0x20, com 0, 1, 0). Devolve o que o primeiro tribunal devolve (os outros caminhos, 0; ninguém lê).
pub(crate) unsafe extern "thiscall" fn process_match(this: usize, partida: usize, result: u32) -> u32 {
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_PROCESS_MATCH);
    let id = read_usize(partida) as i32;
    if id > -1 {
        if let Some(b) = comp_tribunal(this, id) {
            return julgar(b, partida, result, 1, 1, 1);
        }
    }
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if friendly(read_usize(G_FRIENDLY), read_usize(partida + 0x14)) == 0 && read_usize(partida) != read_usize(KEY_CLUB_COMP_RESERVE) {
        return 0;
    }
    for (lado, a, b) in [(0x1cusize, 1u32, 0u32), (0x20, 0, 1)] {
        let league = read_usize(read_usize(partida + lado) + 0x57);
        if league == 0 {
            continue;
        }
        if let Some(t) = comp_tribunal(this, read_usize(league) as i32) { // o tribunal da liga do clube
            julgar(t, partida, result, a, b, 0);
        }
    }
    0
}


/// clears the ban history, releases the bans and frees the 4 slots, drops the held bans from the list and leaves the
/// 29-byte record free (0xff, count 0)
/// Original: `Discipline::remove_player` (0x556830, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x556830 } (thiscall, ret 4, discipline.cpp): `Discipline::remove_player(id)`: do regen (0x7abac0): apaga o
/// histórico de suspensões (0x55e1d0), libera as 4 vagas do registro de 29 bytes (com entrada no órgão: a suspensão
/// solta, 0x556d00, a vaga livre e a entrada +0 = −1), tira da lista [this+0x1c] as suspensões guardadas dele
/// (0x68d1f0, enquanto a conta +0x1c > 0) e o registro fica 0xff com a conta 0.
pub(crate) unsafe extern "thiscall" fn discipline_remove_player(this: usize, id: u32) {
    let r = read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let erase: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_DISCIPLINE_CLEAR_BAN_HISTORY);
    erase(this, read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    for k in 0..4usize {
        let rule = *((r + k * 5) as *const u8);
        if rule == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_13 regen solta a vaga");
        let i = core::ptr::read_unaligned((e + 6) as *const i16);
        if i > -1 {
            // o índice no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra)
            let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
            soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
        }
        let v = read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29) + k * 5);
        *(v as *mut u8) = 0xff;
        write_u32(v + 1, 0xffff_ffff);
        write_u32(e, 0xffff_ffff);
    }
    let list = this + 0x1c;
    write_u32(list + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut item = next(list);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            remover(list, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = next(list);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    *((r + 0x1c) as *mut u8) = 0;
}


/// the player's held ban in a slot, or none
/// Original: `Discipline::find_held_ban` (0x55cb60, discipline.cpp, thiscall).
///
/// [pt] { 0x55cb60 } (thiscall, ret 8, discipline.cpp): a punição guardada do jogador `id` na vaga `vaga`:
/// percorre a lista das guardadas (+0x1c, do começo, 0x68d1b0) enquanto o jogador tem alguma
/// (byte +0x1c do registro de 29 bytes em [+0x14]) e devolve a de mesmo jogador e vaga; 0 se nenhuma.
pub(crate) unsafe extern "thiscall" fn find_held_ban(disciplina: usize, id: i32, slot: i8) -> usize {
    let list = disciplina + 0x1c;
    let record = (*((disciplina + 0x14) as *const usize) as isize + id as isize * 29) as usize;
    core::ptr::write_unaligned((list + 0xa) as *mut u32, core::ptr::read_unaligned((list + 2) as *const u32));
    let proxima: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut p = proxima(list);
    while p != 0 {
        if *((record + 0x1c) as *const i8) <= 0 {
            return 0;
        }
        if *(p as *const i32) == id && *((p + 4) as *const i8) == slot {
            return p;
        }
        p = proxima(list);
    }
    0
}

