use super::*;

// ---------- 264a leva: as suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão (discipline.cpp) ----------



/// Os itens (8 bytes: +0 id da pessoa, +4 id da nação) do índice das nações da disciplina ([this+0xc], conta +0x12
/// com sinal) da nação do `time` (+0x53), na ordem do índice: bsearch (0x946464 com 0x55e5f0, cdecl) pela chave (−1,
/// nação), que iguala qualquer pessoa da nação; volta até o primeiro igual e segue até o último. 0 itens sem achar.
pub(crate) unsafe fn cqb9_itens(this: usize, time: usize) -> (usize, usize) {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let chave = [u32::MAX, read_usize(read_usize(time + 0x53)) as u32];
    let base = read_usize(this + 0xc);
    let mut e = bsearch(chave.as_ptr(), base, read_i16(this + 0x12) as i32, 8, 0x55e5f0);
    if e == 0 {
        return (0, 0);
    }
    // os itens são iguais à chave pela nação (+4): a chave tem −1 no id
    let igual = |x: usize| read_usize(x + 4) as u32 == chave[1];
    while e > base && igual(e - 8) {
        e -= 8;
    }
    let mut fim = e;
    while fim <= base.wrapping_add((read_i16(this + 0x12) as i32 as usize).wrapping_mul(8)).wrapping_sub(8) && igual(fim) {
        fim += 8;
    }
    (e, fim)
}

/// in a national team match: the people of its nation in the nation index (binary search by nation) serve one match of
/// their ban under rule 0, one by one from a list of the ids (the count can release the ban and reorder the index)
/// Original: `Discipline::serve_international_bans` (0x557120, discipline.cpp, thiscall).
///
/// [pt] { 0x557120 } (thiscall, ret 4, discipline.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `Discipline::serve_international_bans(time)`, pelo jogo da seleção (0x7e8090, no órgão das suspensões). As pessoas da
/// nação do time no índice das nações (cqb9_itens) vão para uma lista (0x68d0d0 com o item e 4: a cópia do id, porque
/// a contagem pode soltar a suspensão e reordenar o índice) e, para cada uma, um jogo cumprido na regra 0
/// (0x556e90 com 0 e 1). No fim a lista é destruída.
pub(crate) unsafe extern "thiscall" fn cumprir_suspensoes_internacionais(this: usize, time: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let contar: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_DISCIPLINE_UPDATE_BAN_PROGRESS);
    let mut l = [0u8; 14];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    let (ini, fim) = cqb9_itens(this, time);
    if ini != 0 {
        let mut e = ini;
        while e < fim {
            por(lista, e, 4);
            e += 8;
        }
        // o cursor (+0xa) no começo (+2)
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
        let mut p = proximo(lista);
        while p != 0 {
            crate::comparacao::ramo("cqb_9 jogo cumprido");
            contar(this, read_usize(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0
            p = proximo(lista);
        }
    }
    soltar(lista);
}

/// Comparador: a contagem (0x556e90) grava o histórico no arquivo quando a suspensão termina e não pode rodar duas
/// vezes: o plano antes (as pessoas da nação e, de cada uma, o que a 0x556e90 fará, cpm4_jogo), a original roda;
/// confere, pessoa a pessoa, o contador novo ou a suspensão solta (livre: +0 = 0xff, +2 = −1).
pub(crate) unsafe extern "thiscall" fn comparar_cumprir_suspensoes_internacionais(this: usize, time: usize) {
    let (ini, fim) = cqb9_itens(this, time);
    let mut plano = Vec::new();
    let mut e = ini;
    while ini != 0 && e < fim {
        let id = read_usize(e) as u32;
        let j = cpm4_jogo(this, id, 0, 1);
        let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
        plano.push((id, j, antes));
        e += 8;
    }
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1482)(this, time);
    let mut erro = None;
    for (id, j, antes) in &plano {
        if j.suspensao == 0 {
            continue;
        }
        let s = j.suspensao;
        let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), read_i16(s + 2));
        let ok = if j.cumprida { jogos == 0xff && dias == -1 } else { contador == j.contador.unwrap_or(*antes) };
        if !ok && erro.is_none() {
            erro = Some(format!("pessoa {} contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{}", id, contador, j.contador, antes, j.cumprida, jogos, dias));
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("cumprir_suspensoes_internacionais(time {:#x}, {} pessoas): {}", time, plano.len(), erro.clone().unwrap_or_default()));
}

/// O plano do recurso: a entrada da pessoa no órgão da `regra` (0x7e82e0 sem criar), a suspensão (20 bytes em
/// [this+4]), os valores novos de +0 (jogos) ou +2 (dias) e se ela é solta.
pub(crate) struct Cqb9Recurso {
    pub(crate) orgao: usize,
    pub(crate) entrada: usize,
    pub(crate) suspensao: usize,
    pub(crate) jogos: u8,
    pub(crate) dias: i16,
    pub(crate) solta: bool,
}

pub(crate) unsafe fn cqb9_recurso(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32) -> Cqb9Recurso {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_RULINGBODY_FIND_OR_ADD_DISCIPLINE_ENTRY);
    let orgao = read_usize(read_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)));
    let entrada = achar(orgao, id, 0);
    let mut r = Cqb9Recurso { orgao, entrada, suspensao: 0, jogos: 0, dias: 0, solta: false };
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
pub(crate) unsafe extern "thiscall" fn recurso_da_suspensao(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, _x: u32) {
    let tirar_pendentes: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_RULINGBODY_REMOVE_PENDING_BANS);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
    let r = cqb9_recurso(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
    if r.suspensao == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_9 recurso");
    tirar_pendentes(r.orgao, id);
    let s = r.suspensao;
    *(s as *mut u8) = r.jogos;
    core::ptr::write_unaligned((s + 2) as *mut i16, r.dias);
    if r.solta {
        // ponytail: a original empurra o registrador com o índice na palavra baixa (o alto é sobra); a 0x556d00 só lê
        // a palavra
        soltar(this, id, read_i16(r.entrada + 6) as u16 as u32);
    }
}

/// Comparador: o órgão tira as suspensões pendentes (0x7e91d0, lista que solta nós) e a 0x556d00 pode gravar o
/// histórico: só a original, com o plano antes; confere a suspensão solta (+0 = 0xff, +2 = −1) ou os jogos e os dias
/// novos.
pub(crate) unsafe extern "thiscall" fn comparar_recurso_da_suspensao(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, x: u32) {
    let r = cqb9_recurso(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(1483)(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo, x);
    if r.suspensao == 0 {
        return;
    }
    let s = r.suspensao;
    let (jogos, dias) = (*(s as *const u8), read_i16(s + 2));
    let ok = if r.solta { jogos == 0xff && dias == -1 } else { jogos == r.jogos && dias == r.dias };
    PLACAR.registrar(ok, || format!("recurso_da_suspensao({}, regra {}) jogos {:#x} dias {} (esperava {:#x}/{}, solta={})", id, regra as i8, jogos, dias, r.jogos, r.dias, r.solta));
}

/// A vaga `vaga` (byte com sinal) da suspensão da pessoa no registro de 29 bytes ([this+0x14]: 5 bytes por vaga): o
/// órgão da regra (0xff = sem vaga).
pub(crate) unsafe fn cqb9_vaga(this: usize, pessoa: usize, vaga: u32) -> u8 {
    let id = read_usize(pessoa);
    *((read_usize(this + 0x14) + id.wrapping_mul(29) + (vaga as i8 as isize as usize).wrapping_mul(5)) as *const u8)
}

/// the text of the ban in the person's slot, from the ruling body of its rule; with no rule in the slot, 0
/// Original: `Discipline::get_slot_text` (0x55d3c0, discipline.cpp, thiscall).
///
/// [pt] { 0x55d3c0 } (thiscall, ret 0x10, discipline.cpp): `Discipline::get_slot_text(pessoa, vaga, saída, tamanho)`, pela
/// tela da pessoa (0x87618a, com 0xc8). Pessoa de id a partir de [0xae2364]: 0. A regra da vaga (cqb9_vaga) 0xff: 0;
/// senão o texto do órgão dela ([[this] + 4 × regra], 0x7e88d0 com a pessoa, a saída e o tamanho). Pessoa nula: a
/// original (lê o id de 0).
pub(crate) unsafe extern "thiscall" fn texto_da_vaga_da_suspensao(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(1484)(this, pessoa, vaga, saida, tamanho);
    }
    if read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let regra = cqb9_vaga(this, pessoa, vaga);
    if regra == 0xff {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_GET_BAN_TEXT);
    texto(read_usize(read_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4))), pessoa, saida, tamanho)
}

/// Comparador: sem regra na vaga (ou pessoa fora da tabela), a devolução (0, sem gravar nada); com regra, o texto do órgão
/// (0x7e88d0) pode contar a suspensão por dias (0x556e90, que grava o histórico no arquivo): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_texto_da_vaga_da_suspensao(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(1484);
    if pessoa == 0 || (read_usize(pessoa) as i32) < read_usize(G_STAFF_COUNT) as i32 && cqb9_vaga(this, pessoa, vaga) != 0xff {
        // ponytail: o texto do órgão não roda duas vezes; a cópia só repassa a ele
        return o(this, pessoa, vaga, saida, tamanho);
    }
    let r = o(this, pessoa, vaga, saida, tamanho);
    let n = texto_da_vaga_da_suspensao(this, pessoa, vaga, saida, tamanho);
    PLACAR.registrar(r == n, || format!("texto_da_vaga_da_suspensao(pessoa {:#x}, vaga {}) original={} rust={}", pessoa, vaga as i8, r, n));
    r
}

// ---------- 251a leva: as suspensões na troca de clube: acrescentar a suspensão, soltar as vagas que não valem no clube novo e devolver as suspensões guardadas no fim do empréstimo (discipline.cpp) ----------





/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
pub(crate) unsafe fn cpo11_orgao(this: usize, regra: u8) -> usize {
    read_u32_as_usize(read_u32_as_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}


/// O registro de 29 bytes da pessoa `id` em [this+0x14] (4 vagas de 5 bytes: a regra e o índice da entrada; +0x14/+0x18
/// o histórico; +0x1c quantas suspensões guardadas).
pub(crate) unsafe fn cpo11_registro(this: usize, id: u32) -> usize {
    read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29))
}

/// O órgão vale para o clube? Com nação (+0x17 ≠ −1): a nação do clube (+0x53) é ela; sem nação, com continente
/// (+0x1b ≠ −1): o continente da nação do clube (+0x71) é ele; sem os dois, vale sempre. Clube nulo ou sem nação: não.
pub(crate) unsafe fn cpo11_cobre(orgao: usize, clube: usize) -> bool {
    let nacao = read_u32_as_usize(orgao + 0x17) as u32;
    if nacao == 0xffff_ffff {
        let continente = read_u32_as_usize(orgao + 0x1b) as u32;
        if continente == 0xffff_ffff {
            return true;
        }
        if clube == 0 || read_u32_as_usize(clube + club::NATION) == 0 {
            return false;
        }
        let c = read_u32_as_usize(read_u32_as_usize(clube + club::NATION) + 0x71);
        return c != 0 && read_u32_as_usize(c) as u32 == continente;
    }
    clube != 0 && read_u32_as_usize(clube + club::NATION) != 0 && read_u32_as_usize(read_u32_as_usize(clube + club::NATION)) as u32 == nacao
}

/// Solta a suspensão da entrada (se tem: +6 > −1) e libera a vaga `k` do registro da pessoa (regra 0xff, índice −1) e a
/// entrada (+0 = −1). O índice vai no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra).
pub(crate) unsafe fn cpo11_liberar(this: usize, id: u32, e: usize) {
    let i = read_i16(e + 6);
    if i > -1 {
        let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
        soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
    }
}

pub(crate) unsafe fn cpo11_limpar_vaga(this: usize, id: u32, k: usize, e: usize) {
    let v = cpo11_registro(this, id).wrapping_add(k * 5);
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
/// +0x12; cresce 20; qsort 0x55e5f0). Caixas "Error" (capacidade no máximo, sem memória) pela aviso_em.
pub(crate) unsafe extern "thiscall" fn acrescentar_suspensao(this: usize, id: u32, ban: usize) {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpm4_entrada(cpo11_orgao(this, regra), id);
        if e != 0 && read_i16(e + 6) > -1 {
            crate::comparacao::ramo("cpo_11 suspensao estendida");
            let s = read_u32_as_usize(this + 4).wrapping_add((read_i16(e + 6) as i32 as usize).wrapping_mul(20));
            let jogos = *(s as *const u8);
            let novos = *(ban as *const u8);
            if jogos != 0xff && novos != 0xff {
                *(s as *mut u8) = novos.wrapping_add(jogos);
            } else {
                write_u16(s + 2, (read_i16(s + 2) as u16).wrapping_add(read_i16(ban + 2) as u16));
            }
            *((ban + 0x13) as *mut u8) |= 1;
            *((s + 0x13) as *mut u8) |= 1;
            return;
        }
        *((ban + 0x13) as *mut u8) &= 0xfe;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_BANRECORD_OPERATOR_3D);
    let n = read_i16(this + 0xa);
    let mut vaga: i16 = -1;
    for i in 0..n.max(0) {
        let s = read_u32_as_usize(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && read_i16(s + 2) == -1 {
            vaga = i;
            copiar(s, ban);
            break;
        }
    }
    if vaga == -1 {
        let conta = read_i16(this + 0xa);
        let cap = read_i16(this + 8);
        if conta < cap {
            write_u16(this + 0xa, conta.wrapping_add(1) as u16);
            copiar(read_u32_as_usize(this + 4).wrapping_add((conta as i32 as usize).wrapping_mul(20)), ban);
            vaga = read_i16(this + 0xa).wrapping_sub(1);
        } else {
            if cap >= 0x7d00 {
                aviso_em(0x9b8f18, 0x401);
                return;
            }
            let nova = cap.wrapping_add(100);
            write_u16(this + 8, nova as u16);
            let tamanho = (nova as i32 as u32).wrapping_mul(20);
            let p = if read_u32_as_usize(this + 4) == 0 {
                let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
                malloc(tamanho)
            } else {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realloc(read_u32_as_usize(this + 4), tamanho)
            };
            write_u32(this + 4, p as u32);
            if p == 0 {
                aviso_em(0x9b8f18, 0x410);
                return;
            }
            vaga = read_i16(this + 0xa);
            write_u16(this + 0xa, vaga.wrapping_add(1) as u16);
            copiar(p.wrapping_add((vaga as i32 as usize).wrapping_mul(20)), ban);
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
        let regra = *((ban + 0xc + k) as *const u8);
        if regra != 0xff {
            ligar(cpo11_orgao(this, regra), id, vaga as u16 as u32); // liga a regra à suspensão
        }
    }
    if *((ban + 0xf) as *const u8) == 0xff {
        return;
    }
    let cap = read_i16(this + 0x10);
    if cap == read_i16(this + 0x12) {
        let nova = cap.wrapping_add(0x14);
        write_u16(this + 0x10, nova as u16);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_u32_as_usize(this + 0xc), (nova as i32 as u32).wrapping_mul(8));
        write_u32(this + 0xc, p as u32);
        if p == 0 {
            aviso_em(0x9b8f18, 0x43e);
            return;
        }
    }
    let n = read_i16(this + 0x12);
    let item = read_u32_as_usize(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
    write_u32(item, id);
    let pessoa = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
    write_u32(item + 4, read_u32_as_usize(read_u32_as_usize(pessoa + staff::NATION)) as u32); // a nação
    write_u16(this + 0x12, n.wrapping_add(1) as u16);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32_as_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0);
}

/// on a club move, frees each of the person's 4 record slots whose ruling body (nation +0x17 or continent +0x1b) does
/// not cover the new club: releases the ban, frees the slot and the entry; with held bans, shows the "Error" box
/// Original: `Discipline::clear_bans_invalid_for_club` (0x556270, discipline.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x556270 } (thiscall, ret 0xc, discipline.cpp): `Discipline::clear_bans_invalid_for_club(pessoa, _, clube)`: cada
/// vaga ocupada das 4 do registro da pessoa cujo órgão não vale para o clube novo (`cpo11_cobre`) é liberada: a
/// suspensão solta (0x556d00), a vaga livre e a entrada do órgão livre. Com suspensões guardadas (+0x1c ≠ 0), a caixa
/// "Error" antes (e segue). Da troca de clube (0x8c9720).
pub(crate) unsafe extern "thiscall" fn tirar_suspensoes_de_fora_do_clube(this: usize, pessoa: usize, _nada: u32, clube: usize) {
    let id = read_u32_as_usize(pessoa) as u32;
    let r = cpo11_registro(this, id);
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if cpo11_cobre(orgao, clube) {
            continue;
        }
        let e = cpm4_entrada(orgao, id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_11 transferido solta a vaga");
        cpo11_liberar(this, id, e);
        if *((r + 0x1c) as *const u8) != 0 {
            aviso_em(0x9b8f18, 0x239);
        }
        cpo11_limpar_vaga(this, id, k, e);
    }
}

/// O que a 0x556570 vai fazer, lido antes (a lista das guardadas perde e libera os itens): o contrato e o clube, os
/// itens guardados da pessoa (a vaga e os 5 bytes) e, depois deles, as vagas 0..2 não devolvidas que o clube não cobre.
pub(crate) struct Cpo11Devolver {
    pub(crate) contrato: usize,
    pub(crate) clube: usize,
    pub(crate) guardadas: Vec<(usize, [u8; 5])>,
    pub(crate) resto: u8,
    pub(crate) soltas: Vec<(usize, usize)>,
}

/// O plano da 0x556570 (o cursor da lista é zerado pela original na entrada; a busca sem criar só lê).
pub(crate) unsafe fn cpo11_plano_devolver(this: usize, pessoa: usize) -> Cpo11Devolver {
    let id = read_u32_as_usize(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contrato_de(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    let clube = if contrato == 0 { 0 } else { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(contrato + 4).wrapping_mul(0x245)) };
    let mut p = Cpo11Devolver { contrato, clube, guardadas: Vec::new(), resto: *((r + 0x1c) as *const u8), soltas: Vec::new() };
    let lista = this + 0x1c;
    write_u32(lista + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut vagas = [0u8; 20];
    core::ptr::copy_nonoverlapping(r as *const u8, vagas.as_mut_ptr(), 20);
    let mut marcas = [false; 4];
    let mut item = proximo(lista);
    while item != 0 && (p.resto as i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            let k = (*((item + 4) as *const u8) as i8 as isize as usize) & 3;
            let mut b = [0u8; 5];
            core::ptr::copy_nonoverlapping((item + 5) as *const u8, b.as_mut_ptr(), 5);
            vagas[k * 5..k * 5 + 5].copy_from_slice(&b);
            marcas[k] = true;
            p.guardadas.push((k, b));
            p.resto = p.resto.wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..3usize {
        if marcas[k] || vagas[k * 5] == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, vagas[k * 5]);
        if !cpo11_cobre(orgao, clube) {
            let e = cpm4_entrada(orgao, id);
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
pub(crate) unsafe extern "thiscall" fn restaurar_suspensoes_guardadas(this: usize, pessoa: usize) {
    let id = read_u32_as_usize(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contrato_de(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32(0xae1080, contrato as u32);
    let clube = if contrato == 0 { 0 } else { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(contrato + 4).wrapping_mul(0x245)) };
    let lista = this + 0x1c;
    write_u32(lista + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut marcas = [0u32; 4];
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            let k = *((item + 4) as *const u8) as i8 as isize as usize;
            let regra = *(r.wrapping_add(k.wrapping_mul(5)) as *const u8);
            if regra != 0xff {
                let e = cpm4_entrada(cpo11_orgao(this, regra), id);
                if e != 0 {
                    cpo11_liberar(this, id, e);
                    cpo11_limpar_vaga(this, id, k, e);
                }
            }
            crate::comparacao::ramo("cpo_11 guardada devolvida");
            marcas[k & 3] = 1;
            let v = r.wrapping_add(k.wrapping_mul(5));
            write_u32(v, read_u32_as_usize(item + 5) as u32);
            *((v + 4) as *mut u8) = *((item + 9) as *const u8);
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if marcas[k] != 0 || k == 3 || regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if cpo11_cobre(orgao, clube) {
            continue;
        }
        let e = cpm4_entrada(orgao, id);
        if e != 0 {
            crate::comparacao::ramo("cpo_11 fim do emprestimo solta a vaga");
            cpo11_liberar(this, id, e);
            cpo11_limpar_vaga(this, id, k, e);
        }
    }
}

/// O caminho da 0x556940, lido antes: a suspensão velha estendida (e os valores novos dela), ou a vaga da nova e a
/// conta do vetor depois; `None` nas caixas (capacidade no máximo).
pub(crate) enum Cpo11Plano {
    Estender { s: usize, jogos: u8, dias: i16 },
    Nova { vaga: i16, conta: i16 },
    Caixa,
}

pub(crate) unsafe fn cpo11_plano_suspensao(this: usize, id: u32, ban: usize) -> Cpo11Plano {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpm4_entrada(cpo11_orgao(this, regra), id);
        if e != 0 && read_i16(e + 6) > -1 {
            let s = read_u32_as_usize(this + 4).wrapping_add((read_i16(e + 6) as i32 as usize).wrapping_mul(20));
            let (jogos, novos) = (*(s as *const u8), *(ban as *const u8));
            return if jogos != 0xff && novos != 0xff {
                Cpo11Plano::Estender { s, jogos: novos.wrapping_add(jogos), dias: read_i16(s + 2) }
            } else {
                Cpo11Plano::Estender { s, jogos, dias: read_i16(s + 2).wrapping_add(read_i16(ban + 2)) }
            };
        }
    }
    let n = read_i16(this + 0xa);
    for i in 0..n.max(0) {
        let s = read_u32_as_usize(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && read_i16(s + 2) == -1 {
            return Cpo11Plano::Nova { vaga: i, conta: n };
        }
    }
    if n < read_i16(this + 8) || read_i16(this + 8) < 0x7d00 {
        return Cpo11Plano::Nova { vaga: n, conta: n.wrapping_add(1) };
    }
    Cpo11Plano::Caixa
}

/// Comparador: a 0x556940 realoca os vetores e liga os órgãos (não pode rodar duas vezes): o plano antes, a original
/// roda; confere a velha estendida (jogos, dias, bit 0 das duas), ou a nova na vaga (jogos, dias, as 4 regras), a
/// conta, as entradas das regras apontando para a vaga e, com a quarta regra, o índice das nações com um a mais.
pub(crate) unsafe extern "thiscall" fn comparar_acrescentar_suspensao(this: usize, id: u32, ban: usize) {
    let plano = cpo11_plano_suspensao(this, id, ban);
    let indice = read_i16(this + 0x12);
    let mut regras = [0u8; 4];
    core::ptr::copy_nonoverlapping((ban + 0xc) as *const u8, regras.as_mut_ptr(), 4);
    orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(997)(this, id, ban);
    let (ok, detalhe) = match plano {
        Cpo11Plano::Caixa => return,
        Cpo11Plano::Estender { s, jogos, dias } => {
            let ok = *(s as *const u8) == jogos && read_i16(s + 2) == dias && *((s + 0x13) as *const u8) & 1 == 1
                && *((ban + 0x13) as *const u8) & 1 == 1;
            (ok, format!("estender: jogos {} (esperava {}), dias {} (esperava {})", *(s as *const u8), jogos, read_i16(s + 2), dias))
        }
        Cpo11Plano::Nova { vaga, conta } => {
            let s = read_u32_as_usize(this + 4).wrapping_add((vaga as i32 as usize).wrapping_mul(20));
            let mut ok = read_i16(this + 0xa) == conta && *(s as *const u8) == *(ban as *const u8) && read_i16(s + 2) == read_i16(ban + 2);
            for k in 0..4usize {
                ok &= *((s + 0xc + k) as *const u8) == regras[k];
                if regras[k] != 0xff {
                    let e = cpm4_entrada(cpo11_orgao(this, regras[k]), id);
                    ok &= e != 0 && read_i16(e + 6) == vaga;
                }
            }
            if regras[3] != 0xff {
                ok &= read_i16(this + 0x12) == indice.wrapping_add(1);
            }
            (ok, format!("nova: vaga {} conta {} (esperava {}), índice {} (antes {})", vaga, read_i16(this + 0xa), conta, read_i16(this + 0x12), indice))
        }
    };
    PLACAR.registrar(ok, || format!("acrescentar_suspensao({}) {}", id, detalhe));
}

/// Comparador: a 0x556270 solta suspensões (índice das nações, órgãos): o plano antes (as vagas que o clube não cobre
/// e as entradas), a original roda; confere as vagas liberadas (0xff, −1, a entrada −1) e as outras intactas. Uma
/// entrada repetida em duas vagas não entra na conferência da segunda.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_suspensoes_de_fora_do_clube(this: usize, pessoa: usize, nada: u32, clube: usize) {
    let id = read_u32_as_usize(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    let mut plano: [Option<usize>; 4] = [None; 4];
    let mut duvida = [false; 4];
    for k in 0..4usize {
        let regra = antes[k * 5];
        if regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if !cpo11_cobre(orgao, clube) {
            let e = cpm4_entrada(orgao, id);
            if e != 0 {
                duvida[k] = plano[..k].iter().any(|x| *x == Some(e));
                plano[k] = Some(e);
            }
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(998)(this, pessoa, nada, clube);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut ok = depois[20..] == antes[20..];
    for k in 0..4usize {
        if duvida[k] {
            continue;
        }
        let v = &depois[k * 5..k * 5 + 5];
        ok &= match plano[k] {
            Some(e) => v == [0xff; 5] && read_u32_as_usize(e) as u32 == 0xffff_ffff,
            None => *v == antes[k * 5..k * 5 + 5],
        };
    }
    PLACAR.registrar(ok, || format!("tirar_suspensoes_de_fora_do_clube({}) plano {:x?} antes {:02x?} depois {:02x?}", id, plano, antes, depois));
}

/// Comparador: a 0x556570 tira e libera itens da lista (lição 1: o esperado é montado ANTES; nenhum item é lido depois)
/// e solta suspensões: o plano antes, a original roda; confere [0xae1080], as vagas devolvidas (os 5 bytes guardados), as
/// soltas (0xff, −1, a entrada −1), a conta de guardadas e as outras vagas intactas.
pub(crate) unsafe extern "thiscall" fn comparar_restaurar_suspensoes_guardadas(this: usize, pessoa: usize) {
    let id = read_u32_as_usize(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let p = cpo11_plano_devolver(this, pessoa);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(999)(this, pessoa);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut esperado = antes;
    for (k, b) in &p.guardadas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(b);
    }
    let mut ok = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT) == p.contrato;
    for (k, e) in &p.soltas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(&[0xff; 5]);
        ok &= read_u32_as_usize(*e) as u32 == 0xffff_ffff;
    }
    esperado[0x1c] = p.resto;
    ok &= depois == esperado;
    PLACAR.registrar(ok, || format!("restaurar_suspensoes_guardadas({}) clube {:x} esperado {:02x?} depois {:02x?}", id, p.clube, esperado, depois));
}

// ---------- 249a leva: a disciplina: contar o jogo cumprido da suspensão, soltar a suspensão e apagar o histórico de suspensões da pessoa (discipline.cpp) ----------



/// O registro de 29 bytes da pessoa `id` em [this+0x14] (+0x14/+0x18: o começo e o último deslocamento do histórico).
pub(crate) unsafe fn cpm4_registro(this: usize, id: usize) -> usize {
    read_usize(this + 0x14).wrapping_add(id.wrapping_mul(29))
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
pub(crate) unsafe fn cpm4_orgao(this: usize, regra: u8) -> usize {
    read_usize(read_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada (14 bytes; +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar (só lê).
pub(crate) unsafe fn cpm4_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_RULINGBODY_FIND_OR_ADD_DISCIPLINE_ENTRY);
    achar(orgao, id, 0)
}

/// O item (id, nação) da pessoa no índice das nações (+0xc, 8 bytes por item, conta +0x12 com sinal): bsearch
/// (0x946464 com 0x55e5f0) pela nação (+0x1a) e, sem achar, pela segunda nação (+0x1e, se houver). 0 sem achar.
pub(crate) unsafe fn cpm4_no_indice(this: usize, id: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
    let mut chave = [id, read_usize(read_usize(pessoa + staff::NATION)) as u32];
    let e = bsearch(chave.as_ptr(), read_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0);
    if e != 0 {
        return e;
    }
    let segunda = read_usize(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e)) + 0x1e);
    if segunda == 0 {
        return 0;
    }
    chave[1] = read_usize(segunda) as u32;
    bsearch(chave.as_ptr(), read_usize(this + 0xc), read_i16(this + 0x12) as i32, 8, 0x55e5f0)
}

/// O que a 0x556e90 faz com o jogo: a entrada no órgão, a suspensão (20 bytes em [this+4]), o contador novo de jogos
/// cumpridos (+1) e se a suspensão termina.
pub(crate) struct Cpm4Jogo {
    pub(crate) entrada: usize,
    pub(crate) suspensao: usize,
    pub(crate) contador: Option<u8>,
    pub(crate) cumprida: bool,
}

/// O plano do jogo (só lê; a 0x7e82e0 sem criar e a 0x54c5b0 só gravam a data de rascunho dela): a suspensão da
/// pessoa no órgão da `regra`; por jogos (+0 ≠ 0xff): a partir do começo (dia +4, ano +6; hoje [0xae2c90]: dia +0,
/// ano +2), com `conta`, +1 no contador (+1), e termina ao chegar em +0; por dias (+0 = 0xff): termina quando hoje
/// chega ao começo + dias (+2) (0x54c5b0).
pub(crate) unsafe fn cpm4_jogo(this: usize, id: u32, regra: u32, conta: u32) -> Cpm4Jogo {
    let mut j = Cpm4Jogo { entrada: 0, suspensao: 0, contador: None, cumprida: false };
    let e = cpm4_entrada(cpm4_orgao(this, regra as u8), id);
    if e == 0 || read_i16(e + 6) == -1 {
        return j;
    }
    j.entrada = e;
    let cinco = (read_i16(e + 6) as i32 as u32).wrapping_mul(5); // o lea da original (o alto vai no edx abaixo)
    let s = read_usize(this + 4).wrapping_add((cinco as usize).wrapping_mul(4));
    j.suspensao = s;
    let hoje = read_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
    let jogos = *(s as *const u8);
    if jogos != 0xff {
        let (dia0, ano0) = (read_i16(s + 4), read_i16(s + 6) as u16);
        if ano < ano0 || (ano == ano0 && dia < dia0) { // ainda não começou
            crate::comparacao::ramo("cpm_4 antes do inicio");
            return j;
        }
        if conta == 0 {
            return j;
        }
        crate::comparacao::ramo("cpm_4 jogo cumprido");
        let n = (*((s + 1) as *const u8)).wrapping_add(1); // mais um jogo cumprido
        j.contador = Some(n);
        j.cumprida = n == jogos;
    } else {
        let mut fim = [0u8; 8];
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        // a original empurra o edx com os dias no dx e, no alto, o do índice × 5 (zero abaixo de 13108 suspensões)
        let f = mais_dias(s + 4, fim.as_mut_ptr(), (cinco & 0xffff_0000) | read_i16(s + 2) as u16 as u32);
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
/// da pessoa no órgão da `regra` (plano em `cpm4_jogo`): por jogos, grava o contador novo (+1); cumprida, vai para o
/// histórico (0x58e340 sobre [this+0x18] com o começo e o último deslocamento do registro de 29 bytes) e é solta
/// (0x556d00). Sem SEH, sem sorteio, sem caixa.
pub(crate) unsafe extern "thiscall" fn contar_jogo_da_suspensao(this: usize, id: u32, regra: u32, conta: u32) {
    let j = cpm4_jogo(this, id, regra, conta);
    if let Some(n) = j.contador {
        *((j.suspensao + 1) as *mut u8) = n;
    }
    if !j.cumprida {
        return;
    }
    let r = cpm4_registro(this, id as usize);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let g = historico(read_usize(this + 0x18), r + 0x14, r + 0x18, j.suspensao);
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
pub(crate) unsafe extern "thiscall" fn soltar_suspensao(this: usize, id: u32, indice: u32) -> usize {
    let s = read_usize(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    let esquecer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_RULINGBODY_DETACH_BAN);
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        crate::comparacao::ramo("cpm_4 soltar do orgao");
        esquecer(cpm4_orgao(this, regra), id); // o órgão esquece a suspensão
        if k != 3 {
            continue;
        }
        let e = cpm4_no_indice(this, id);
        if e == 0 {
            aviso_em(0x9b8f18, 0x47d);
            continue;
        }
        let n = read_i16(this + 0x12).wrapping_sub(1);
        core::ptr::write_unaligned((this + 0x12) as *mut i16, n);
        let ultimo = read_usize(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
        core::ptr::write_unaligned(e as *mut u32, read_usize(ultimo) as u32);
        core::ptr::write_unaligned((e + 4) as *mut u32, read_usize(ultimo + 4) as u32);
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
pub(crate) unsafe extern "thiscall" fn apagar_historico_de_suspensoes(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || read_usize(pessoa + staff::PLAYER) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(936)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpm4_registro(this, read_usize(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    sem_ultimo(read_usize(this + 0x18), r + 0x14, r + 0x18)
}

/// Comparador: a 0x58e340 grava o histórico no arquivo (não pode rodar duas vezes): o plano antes, a original roda;
/// confere o contador, ou a suspensão solta (livre e a entrada do órgão sem índice), ou nada mudado.
pub(crate) unsafe extern "thiscall" fn comparar_contar_jogo_da_suspensao(this: usize, id: u32, regra: u32, conta: u32) {
    let j = cpm4_jogo(this, id, regra, conta);
    let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(934)(this, id, regra, conta);
    if j.suspensao == 0 {
        return;
    }
    let s = j.suspensao;
    let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), read_i16(s + 2));
    let ok = if j.cumprida {
        jogos == 0xff && dias == -1 && read_i16(j.entrada + 6) == -1
    } else {
        contador == j.contador.unwrap_or(antes)
    };
    PLACAR.registrar(ok, || {
        format!("contar_jogo_da_suspensao({}, {}, {}) contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{} entrada {}",
            id, regra as u8, conta, contador, j.contador, antes, j.cumprida, jogos, dias, read_i16(j.entrada + 6))
    });
}

/// Comparador: lado a lado pela suspensão, o índice das nações (conta e itens) e as entradas da pessoa nos órgãos das
/// regras (achadas antes, sem criar). Quarta regra sem o item no índice: só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_soltar_suspensao(this: usize, id: u32, indice: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(935);
    let s = read_usize(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    if *((s + 0xf) as *const u8) != 0xff && cpm4_no_indice(this, id) == 0 {
        return o(this, id, indice);
    }
    let mut reg = vec![(s, 20), (this + 0x10, 4)];
    let n = read_i16(this + 0x12);
    if n > 0 {
        reg.push((read_usize(this + 0xc), n as usize * 8));
    }
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra != 0xff {
            let e = cpm4_entrada(cpm4_orgao(this, regra), id);
            if e != 0 {
                reg.push((e, 14));
            }
        }
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id, indice), || soltar_suspensao(this, id, indice));
    PLACAR.registrar(igual && r == m, || format!("soltar_suspensao({}, {}) {:x?} x {:x?}", id, indice as u16 as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn comparar_apagar_historico_de_suspensoes(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(936);
    if pessoa == 0 || read_usize(pessoa + staff::PLAYER) == 0 {
        return o(this, pessoa);
    }
    let r = cpm4_registro(this, read_usize(pessoa));
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 0x14, 8)], || o(this, pessoa), || apagar_historico_de_suspensoes(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("apagar_historico_de_suspensoes({}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
    a
}

// ---------- 243a leva: a cópia do registro de suspensão (discipline.cpp), o salário padrão do contrato (staff_contracts.cpp) e a condição de jogo (injury.cpp) ----------

/// copies the ban record (0x14 bytes): fields, the date (+4) and bits 0..2 of +0x13
/// Original: `BanRecord::operator=` (0x556c70, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x556c70 } (thiscall, ret 4, discipline.cpp): `BanRecord::operator=`: copia o registro de suspensão (0x14 bytes) de
/// `origem`: +0 e +1 (bytes), +2 (palavra), a data em +4 (0x417c10, retorno numa local), +0xc..+0xf (byte a byte),
/// +0x10 (byte), +0x11 (palavra) e os bits 0..2 de +0x13 (os outros bits ficam). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_registro_de_suspensao(this: usize, origem: usize) -> usize {
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    b(0);
    b(1);
    w(2);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copiar(this + 4, &mut local, origem + 4);
    for o in 0xc..0x10 {
        b(o);
    }
    b(0x10);
    w(0x11);
    let m = (this + 0x13) as *mut u8;
    *m = (*m & !7) | (*((origem + 0x13) as *const u8) & 7); // as três marcas
    this
}



pub(crate) unsafe extern "thiscall" fn comparar_copiar_registro_de_suspensao(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(823);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, origem), || copiar_registro_de_suspensao(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_suspensao {:x?} x {:x?}", mo, mr));
    r
}


// a original roda primeiro e atualiza o físico do dia (0x62c680, com sorteio); a cópia, depois, acha o dia já feito
comparar_valor!(comparar_condicao_de_jogo_do_jogador, condicao_de_jogo_do_jogador, 825, (jogador: usize, em_porcento: u32) -> u16);

// ---------- 184a leva: a disciplina (discipline.cpp): a temporada nova, o dia e a vaga da tabela ----------

/// starts the new season of the competition's discipline rules (virtual method 0)
/// Original: `discipline_new_season` (0x556210, discipline.cpp, thiscall).
///
/// [pt] { 0x556210 } (thiscall, ret 4, discipline.cpp): começa a temporada nova da competição `*id`: as
/// regras da disciplina dela (índice +0x50 da competição em [0xadadfc], 0xff sem) recebem o método 0
/// da tabela virtual.
pub(crate) unsafe extern "thiscall" fn disciplina_temporada_nova(this: usize, id: *const i32) {
    let competicao = *((*(G_COMP_OBJECT_TABLE as *const usize) as isize + 4 * *id as isize) as *const usize);
    if competicao == 0 {
        return;
    }
    let k = *((competicao + 0x50) as *const i8);
    if k == -1 {
        return;
    }
    let regras = *((*(this as *const usize) as isize + 4 * k as isize) as *const usize);
    if regras != 0 {
        let metodo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(*(*(regras as *const usize) as *const usize));
        metodo(regras);
    }
}

/// the day of each of the 43 discipline rules (virtual method 1)
/// Original: `discipline_daily_update` (0x556240, discipline.cpp, thiscall).
///
/// [pt] { 0x556240 } (thiscall, ret 4, discipline.cpp): o dia da disciplina: cada uma das 43 regras ([this],
/// as não nulas) recebe o método 1 da tabela virtual com a data `hoje`.
pub(crate) unsafe extern "thiscall" fn disciplina_do_dia(this: usize, hoje: usize) {
    for k in 0..43usize {
        let regras = *((*(this as *const usize) + 4 * k) as *const usize);
        if regras != 0 {
            let metodo: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(*((*(regras as *const usize) + 4) as *const usize));
            metodo(regras, hoje);
        }
    }
}

/// writes the type and value of one of the person's slots in the discipline table
/// Original: `discipline_set_slot` (0x55cb20, discipline.cpp, thiscall).
///
/// [pt] { 0x55cb20 } (thiscall, ret 0x10, discipline.cpp): grava a vaga `vaga` (5 bytes: o byte `tipo` e o
/// valor) da pessoa `id` na tabela +0x14 (29 bytes por pessoa).
pub(crate) unsafe extern "thiscall" fn disciplina_gravar_vaga(this: usize, id: i32, vaga: i8, tipo: u8, valor: u32) {
    let a = (*((this + 0x14) as *const usize) as isize + id as isize * 29 + vaga as isize * 5) as usize;
    *(a as *mut u8) = tipo;
    core::ptr::write_unaligned((a + 1) as *mut u32, valor);
}

// ponytail: as duas primeiras só despacham pela tabela virtual (efeitos fora): provadas pela carreira
pub(crate) unsafe extern "thiscall" fn comparar_disciplina_temporada_nova(this: usize, id: *const i32) {
    orig::<unsafe extern "thiscall" fn(usize, *const i32)>(424)(this, id)
}

pub(crate) unsafe extern "thiscall" fn comparar_disciplina_do_dia(this: usize, hoje: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(425)(this, hoje)
}

pub(crate) unsafe extern "thiscall" fn comparar_disciplina_gravar_vaga(this: usize, id: i32, vaga: i8, tipo: u8, valor: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i8, u8, u32)>(426);
    let a = (*((this + 0x14) as *const usize) as isize + id as isize * 29 + vaga as isize * 5) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 5)], || o(this, id, vaga, tipo, valor), || disciplina_gravar_vaga(this, id, vaga, tipo, valor));
    PLACAR.registrar(igual, || format!("disciplina_gravar_vaga({}, {}) {:x?} x {:x?}", id, vaga, mo, mr));
}

/// suspended by the general body (0x7e85b0 with 1) or by the competition one (+0x50, with 0); in that case, in a human
/// club, he leaves his slot among the 20 at +0x1e0
/// Original: `Discipline::is_player_suspended` (0x55d270, discipline.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x55d270 } (thiscall, ret 0x10, discipline.cpp): `Discipline::is_player_suspended(jogador, competição, extra,
/// partida)`: jogador fora das pessoas ([0xae2364]) ou competição (id em `*comp`, vetor [0xadadfc]) inexistente: 0.
/// Suspenso pelo tribunal geral ([[this]], 0x7e85b0 com 1): 1. Senão, pelo tribunal da competição (+0x50, ≠ −1, com
/// 0): 1, e se o clube dele (+0x39) é de humano (0x600a50 com 0, 1, 0, 0), ele sai da vaga que ocupa entre as 20 de
/// +0x1e0. Senão 0.
pub(crate) unsafe extern "thiscall" fn esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    if jogador != 0 && read_usize(jogador) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
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
    if julgar(read_usize(tribunais), jogador, 1, extra, partida) != 0 {
        return 1; // a suspensão geral
    }
    let t = *((c + 0x50) as *const i8);
    if t == -1 {
        return 0;
    }
    let b = read_usize(tribunais.wrapping_add((t as isize as usize).wrapping_mul(4)));
    if b == 0 || julgar(b, jogador, 0, extra, partida) == 0 { // sem a suspensão da competição
        return 0;
    }
    crate::comparacao::ramo("cpk_3 suspenso pela competicao");
    let clube = read_usize(jogador + 0x39);
    if clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) != 0 {
            if let Some(s) = (0..20).map(|k| clube + club::TEAM_SELECTED + k * 4).find(|&s| read_usize(s) == jogador) {
                *(s as *mut usize) = 0; // sai da vaga
            }
        }
    }
    1
}

/// Comparador: lado a lado pelas 20 vagas (+0x1e0) do clube do jogador (a 0x7e85b0 e a 0x600a50 só leem).
pub(crate) unsafe extern "thiscall" fn comparar_esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32, usize) -> u32>(891);
    let clube = if jogador != 0 { read_usize(jogador + 0x39) } else { 0 };
    let regioes: &[(usize, usize)] = if clube != 0 { &[(clube + club::TEAM_SELECTED, 80)] } else { &[] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(this, jogador, comp, extra, partida), || esta_suspenso(this, jogador, comp, extra, partida));
    PLACAR.registrar(igual && r == n, || format!("esta_suspenso({:x}, {}) original={} rust={} {:x?} x {:x?}", jogador, *comp, r, n, mo, mr));
    r
}

/// removes from the held list (+0x1c) the items of the person while the +0x1c counter of her record is positive
/// Original: `Discipline::drop_held_bans` (0x5567d0, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
pub(crate) unsafe extern "thiscall" fn soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 && *n > 0 {
        if read_usize(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = cpj2_proximo(l);
    }
}

/// Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
pub(crate) unsafe extern "thiscall" fn comparar_soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *n;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta > 0 && read_usize(e) as u32 == *pessoa {
            resta -= 1;
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(867)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == resta, || format!("soltar_suspensoes_guardadas({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *n, resta));
}

/// the bans (slots 0 to 2) that do not apply at the loan club go to the held list (+0x1c), the slot becomes −1 and the
/// +0x1c counter goes up
/// Original: `Discipline::hold_bans_for_loan` (0x5563f0, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x5563f0 } (thiscall, ret 4, discipline.cpp): `Discipline::hold_bans_for_loan(jogador)`: com a ficha de contrato
/// do jogador (0x4cf6d0 sobre [0xae1050]), cada suspensão do registro dele (vagas 0 a 2) que não vale no clube do
/// empréstimo vai para a lista das guardadas (+0x1c, 0x68d0d0 com 10 bytes: o id, a vaga e os 5 bytes dela), a vaga
/// fica −1 e o contador +0x1c do registro sobe. Sem ficha: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let ficha = ficha_de(read_usize(G_CONTRACT_MANAGER), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador); // caixa: só a original
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for k in vagas {
        let vaga = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(read_usize(jogador) as u32).to_le_bytes());
        r[4] = k; // a vaga
        r[5..].copy_from_slice(core::slice::from_raw_parts(vaga as *const u8, 5));
        acrescentar(this + 0x1c, r.as_ptr(), 10);
        core::ptr::write_unaligned(vaga as *mut u32, u32::MAX);
        *((vaga + 4) as *mut u8) = 0xff;
        *((reg + 0x1c) as *mut u8) = (*((reg + 0x1c) as *const u8)).wrapping_add(1);
    }
}

/// Comparador: o plano da cópia (as vagas que não valem no clube do empréstimo) antes; a original guarda; confere o
/// registro do jogador: as vagas do plano em −1, as outras iguais e o contador somado.
pub(crate) unsafe extern "thiscall" fn comparar_guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let ficha = ficha_de(read_usize(G_CONTRACT_MANAGER), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let mut esperado = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    for &k in &vagas {
        esperado[5 * k as usize..5 * k as usize + 5].fill(0xff);
    }
    esperado[0x1c] = esperado[0x1c].wrapping_add(vagas.len() as u8);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    let depois = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    PLACAR.registrar(depois == esperado, || format!("guardar_suspensoes_do_emprestado({:x}) vagas {:?} {:x?} x {:x?}", jogador, vagas, depois, esperado));
}

/// with a valid id, the question to the competition's ruling body (+0x50; 0x7e8d60 with the id and the competition's
/// +4); otherwise 0
/// Original: `Discipline::is_one_booking_from_ban` (0x55d360, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x55d360 } (thiscall, ret 8, discipline.cpp): `Discipline::is_one_booking_from_ban(pessoa, competicao)`: com id
/// válido, a competição ([[0xadadfc] + 4 × id]) e o órgão dela (+0x50, 0xff sem; [this] + 4 × órgão), a pergunta ao órgão
/// (0x7e8d60 com o id, 0, 0 e o +4 da competição); senão 0. Pela tela do clube (0x4849f0).
pub(crate) unsafe extern "thiscall" fn um_cartao_da_suspensao(this: usize, pessoa: usize, competicao: usize) -> u32 {
    if pessoa != 0 && read_i32(pessoa) >= read_i32(0xae2364) {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(competicao) as u32).wrapping_mul(4) as usize));
    if comp == 0 || pessoa == 0 {
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
    perguntar(o, read_usize(pessoa) as u32, 0, 0, read_usize(comp + 4) as u32)
}

pub(crate) unsafe extern "thiscall" fn comparar_um_cartao_da_suspensao(this: usize, pessoa: usize, competicao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1295)(this, pessoa, competicao);
    let n = um_cartao_da_suspensao(this, pessoa, competicao);
    PLACAR.registrar(r == n, || format!("um_cartao_da_suspensao(pessoa {:x}, competicao {:x}) original={:x} rust={:x}", pessoa, competicao, r, n));
    r
}

/// the penalty at the table row and column, if the type matches
/// Original: `table_penalty` (0x55cae0, discipline.cpp, thiscall).
///
/// [pt] { 0x55cae0 } (thiscall, ret 0xc, discipline.cpp): a punição da linha `linha` (29 bytes) e da coluna
/// `coluna` (5 bytes) da tabela [this+0x14]: o dword +1 se o byte +0 for `tipo`, senão −1.
pub(crate) unsafe extern "thiscall" fn punicao_da_tabela(t: usize, linha: i32, coluna: i8, tipo: i8) -> i32 {
    let e = (*((t + 0x14) as *const usize))
        .wrapping_add((linha as usize).wrapping_mul(29))
        .wrapping_add((coluna as isize * 5) as usize);
    if *(e as *const i8) == tipo {
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
pub(crate) unsafe extern "thiscall" fn disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_PROCESS_MATCH);
    let id = read_usize(partida) as i32;
    if id > -1 {
        if let Some(b) = cpj6_tribunal(this, id) {
            return julgar(b, partida, resultado, 1, 1, 1);
        }
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if amistoso(read_usize(G_FRIENDLY), read_usize(partida + 0x14)) == 0 && read_usize(partida) != read_usize(KEY_CLUB_COMP_RESERVE) {
        return 0;
    }
    for (lado, a, b) in [(0x1cusize, 1u32, 0u32), (0x20, 0, 1)] {
        let liga = read_usize(read_usize(partida + lado) + 0x57);
        if liga == 0 {
            continue;
        }
        if let Some(t) = cpj6_tribunal(this, read_usize(liga) as i32) { // o tribunal da liga do clube
            julgar(t, partida, resultado, a, b, 0);
        }
    }
    0
}

/// Comparador: o tribunal (0x7e7ff0) dá as suspensões: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    crate::comparacao::ramo("disciplina_da_partida");
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(880)(this, partida, resultado)
}

/// clears the ban history, releases the bans and frees the 4 slots, drops the held bans from the list and leaves the
/// 29-byte record free (0xff, count 0)
/// Original: `Discipline::remove_player` (0x556830, discipline.cpp, thiscall, ret 4).
///
/// [pt] { 0x556830 } (thiscall, ret 4, discipline.cpp): `Discipline::remove_player(id)`: do regen (0x7abac0): apaga o
/// histórico de suspensões (0x55e1d0), libera as 4 vagas do registro de 29 bytes (com entrada no órgão: a suspensão
/// solta, 0x556d00, a vaga livre e a entrada +0 = −1), tira da lista [this+0x1c] as suspensões guardadas dele
/// (0x68d1f0, enquanto a conta +0x1c > 0) e o registro fica 0xff com a conta 0.
pub(crate) unsafe extern "thiscall" fn tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let apagar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_DISCIPLINE_CLEAR_BAN_HISTORY);
    apagar(this, read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e)));
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpm4_entrada(cpo11_orgao(this, regra), id);
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
    let lista = this + 0x1c;
    write_u32(lista + 0xa, read_u32_as_usize(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if read_u32_as_usize(item) as u32 == id {
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    *((r + 0x1c) as *mut u8) = 0;
}

/// Comparador: a 0x556830 solta suspensões e tira (e libera) itens da lista: o plano antes (as entradas das vagas), a
/// original roda; confere o registro (0xff, conta 0) e as entradas livres (+0 = −1).
pub(crate) unsafe extern "thiscall" fn comparar_tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = read_u32_as_usize(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let mut entradas = Vec::new();
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra != 0xff {
            let e = cpm4_entrada(cpo11_orgao(this, regra), id);
            if e != 0 {
                entradas.push(e);
            }
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, u32)>(1005)(this, id);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let ok = depois[..0x1c].iter().all(|&b| b == 0xff) && depois[0x1c] == 0
        && entradas.iter().all(|&e| read_u32_as_usize(e) as u32 == 0xffff_ffff);
    PLACAR.registrar(ok, || format!("tirar_jogador_da_disciplina({}) registro {:02x?} entradas {:x?}", id, depois, entradas));
}

/// the player's held ban in a slot, or none
/// Original: `Discipline::find_held_ban` (0x55cb60, discipline.cpp, thiscall).
///
/// [pt] { 0x55cb60 } (thiscall, ret 8, discipline.cpp): a punição guardada do jogador `id` na vaga `vaga`:
/// percorre a lista das guardadas (+0x1c, do começo, 0x68d1b0) enquanto o jogador tem alguma
/// (byte +0x1c do registro de 29 bytes em [+0x14]) e devolve a de mesmo jogador e vaga; 0 se nenhuma.
pub(crate) unsafe extern "thiscall" fn punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let lista = disciplina + 0x1c;
    let registro = (*((disciplina + 0x14) as *const usize) as isize + id as isize * 29) as usize;
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let proxima: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut p = proxima(lista);
    while p != 0 {
        if *((registro + 0x1c) as *const i8) <= 0 {
            return 0;
        }
        if *(p as *const i32) == id && *((p + 4) as *const i8) == vaga {
            return p;
        }
        p = proxima(lista);
    }
    0
}

pub(crate) unsafe extern "thiscall" fn comparar_punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i8) -> usize>(447);
    // a lista guarda o cursor: as duas partem do mesmo e têm de deixá-lo igual
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(disciplina + 0x1c, 0x12)], || o(disciplina, id, vaga), || punicao_guardada(disciplina, id, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("punicao_guardada({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", id, vaga, ro, rr, mo, mr));
    ro
}
