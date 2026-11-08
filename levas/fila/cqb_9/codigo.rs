// ---------- %%LEVA%%a leva: as suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão (discipline.cpp) ----------

unsafe fn cqb9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os itens (8 bytes: +0 id da pessoa, +4 id da nação) do índice das nações da disciplina ([this+0xc], conta +0x12
/// com sinal) da nação do `time` (+0x53), na ordem do índice: bsearch (0x946464 com 0x55e5f0, cdecl) pela chave (−1,
/// nação), que iguala qualquer pessoa da nação; volta até o primeiro igual e segue até o último. 0 itens sem achar.
unsafe fn nation_index_range(this: usize, time: usize) -> (usize, usize) {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [u32::MAX, cqb9_d(cqb9_d(time + 0x53)) as u32];
    let base = cqb9_d(this + 0xc);
    let mut e = bsearch(chave.as_ptr(), base, cqb9_w(this + 0x12) as i32, 8, 0x55e5f0);
    if e == 0 {
        return (0, 0);
    }
    // os itens são iguais à chave pela nação (+4): a chave tem −1 no id
    let igual = |x: usize| cqb9_d(x + 4) as u32 == chave[1];
    while e > base && igual(e - 8) {
        e -= 8;
    }
    let mut fim = e;
    while fim <= base.wrapping_add((cqb9_w(this + 0x12) as i32 as usize).wrapping_mul(8)).wrapping_sub(8) && igual(fim) {
        fim += 8;
    }
    (e, fim)
}

/// { 0x557120 } (thiscall, ret 4, discipline.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `Discipline::serve_international_bans(time)`, pelo jogo da seleção (0x7e8090, no órgão das suspensões). As pessoas da
/// nação do time no índice das nações (nation_index_range) vão para uma lista (0x68d0d0 com o item e 4: a cópia do id, porque
/// a contagem pode soltar a suspensão e reordenar o índice) e, para cada uma, um jogo cumprido na regra 0
/// (0x556e90 com 0 e 1). No fim a lista é destruída.
unsafe extern "thiscall" fn serve_international_bans(this: usize, time: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contar: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x556e90usize);
    let mut l = [0u8; 14];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    let (ini, fim) = nation_index_range(this, time);
    if ini != 0 {
        let mut e = ini;
        while e < fim {
            por(lista, e, 4);
            e += 8;
        }
        // o cursor (+0xa) no começo (+2)
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqb9_d(lista + 2));
        let mut p = proximo(lista);
        while p != 0 {
            crate::comparacao::ramo("cqb_9 jogo cumprido");
            contar(this, cqb9_d(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0
            p = proximo(lista);
        }
    }
    soltar(lista);
}

/// Comparador: a contagem (0x556e90) grava o histórico no arquivo quando a suspensão termina e não pode rodar duas
/// vezes: o plano antes (as pessoas da nação e, de cada uma, o que a 0x556e90 fará, serve_ban_match_plan), a original roda;
/// confere, pessoa a pessoa, o contador novo ou a suspensão solta (livre: +0 = 0xff, +2 = −1).
unsafe extern "thiscall" fn comparar_serve_international_bans(this: usize, time: usize) {
    let (ini, fim) = nation_index_range(this, time);
    let mut plano = Vec::new();
    let mut e = ini;
    while ini != 0 && e < fim {
        let id = cqb9_d(e) as u32;
        let j = serve_ban_match_plan(this, id, 0, 1);
        let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
        plano.push((id, j, antes));
        e += 8;
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, time);
    let mut erro = None;
    for (id, j, antes) in &plano {
        if j.suspensao == 0 {
            continue;
        }
        let s = j.suspensao;
        let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), cqb9_w(s + 2));
        let ok = if j.cumprida { jogos == 0xff && dias == -1 } else { contador == j.contador.unwrap_or(*antes) };
        if !ok && erro.is_none() {
            erro = Some(format!("pessoa {} contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{}", id, contador, j.contador, antes, j.cumprida, jogos, dias));
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("serve_international_bans(time {:#x}, {} pessoas): {}", time, plano.len(), erro.clone().unwrap_or_default()));
}

/// O plano do recurso: a entrada da pessoa no órgão da `regra` (0x7e82e0 sem criar), a suspensão (20 bytes em
/// [this+4]), os valores novos de +0 (jogos) ou +2 (dias) e se ela é solta.
struct Cqb9Recurso {
    orgao: usize,
    entrada: usize,
    suspensao: usize,
    jogos: u8,
    dias: i16,
    solta: bool,
}

unsafe fn appeal_plan(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32) -> Cqb9Recurso {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    let orgao = cqb9_d(cqb9_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)));
    let entrada = achar(orgao, id, 0);
    let mut r = Cqb9Recurso { orgao, entrada, suspensao: 0, jogos: 0, dias: 0, solta: false };
    if entrada == 0 || cqb9_w(entrada + 6) == -1 {
        return r;
    }
    let s = cqb9_d(this + 4).wrapping_add((cqb9_w(entrada + 6) as i32 as usize).wrapping_mul(20));
    r.suspensao = s;
    r.jogos = *(s as *const u8);
    r.dias = cqb9_w(s + 2);
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

/// { 0x557000 } (thiscall, ret 0x20, discipline.cpp): `Discipline::apply_appeal_result(id, regra, reduz, menos_jogos,
/// menos_dias, novo, dias_fixo, _)`, pelo resultado do recurso (0x6abf99, notícias do técnico humano). Com a entrada da
/// pessoa no órgão (0x7e82e0 sem criar) e suspensão (+6): o órgão tira as suspensões pendentes dela (0x7e91d0); por
/// jogos (+0 ≠ 0xff): `novo` ≠ 0xff tira `novo` jogos (abaixo de 0: zera e solta), senão `reduz` tira `menos_jogos` e
/// solta se ficar até o contador (+1), senão solta; por dias: `dias_fixo` ≠ −1 tira `menos_dias` (até 0: zera e solta),
/// senão `reduz` tira e solta até 0, senão solta. Soltar: 0x556d00 com o índice.
unsafe extern "thiscall" fn apply_appeal_result(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, _x: u32) {
    let tirar_pendentes: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x7e91d0usize);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
    let r = appeal_plan(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
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
        soltar(this, id, cqb9_w(r.entrada + 6) as u16 as u32);
    }
}

/// Comparador: o órgão tira as suspensões pendentes (0x7e91d0, lista que solta nós) e a 0x556d00 pode gravar o
/// histórico: só a original, com o plano antes; confere a suspensão solta (+0 = 0xff, +2 = −1) ou os jogos e os dias
/// novos.
unsafe extern "thiscall" fn comparar_apply_appeal_result(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, x: u32) {
    let r = appeal_plan(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(%%I1%%)(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo, x);
    if r.suspensao == 0 {
        return;
    }
    let s = r.suspensao;
    let (jogos, dias) = (*(s as *const u8), cqb9_w(s + 2));
    let ok = if r.solta { jogos == 0xff && dias == -1 } else { jogos == r.jogos && dias == r.dias };
    PLACAR.registrar(ok, || format!("apply_appeal_result({}, regra {}) jogos {:#x} dias {} (esperava {:#x}/{}, solta={})", id, regra as i8, jogos, dias, r.jogos, r.dias, r.solta));
}

/// A vaga `vaga` (byte com sinal) da suspensão da pessoa no registro de 29 bytes ([this+0x14]: 5 bytes por vaga): o
/// órgão da regra (0xff = sem vaga).
unsafe fn ban_slot_rule(this: usize, pessoa: usize, vaga: u32) -> u8 {
    let id = cqb9_d(pessoa);
    *((cqb9_d(this + 0x14) + id.wrapping_mul(29) + (vaga as i8 as isize as usize).wrapping_mul(5)) as *const u8)
}

/// { 0x55d3c0 } (thiscall, ret 0x10, discipline.cpp): `Discipline::get_slot_text(pessoa, vaga, saída, tamanho)`, pela
/// tela da pessoa (0x87618a, com 0xc8). Pessoa de id a partir de [0xae2364]: 0. A regra da vaga (ban_slot_rule) 0xff: 0;
/// senão o texto do órgão dela ([[this] + 4 × regra], 0x7e88d0 com a pessoa, a saída e o tamanho). Pessoa nula: a
/// original (lê o id de 0).
unsafe extern "thiscall" fn get_slot_text(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(%%I2%%)(this, pessoa, vaga, saida, tamanho);
    }
    if cqb9_d(pessoa) as i32 >= cqb9_d(0xae2364) as i32 {
        return 0;
    }
    let regra = ban_slot_rule(this, pessoa, vaga);
    if regra == 0xff {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7e88d0usize);
    texto(cqb9_d(cqb9_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4))), pessoa, saida, tamanho)
}

/// Comparador: sem regra na vaga (ou pessoa fora da tabela), a devolução (0, sem gravar nada); com regra, o texto do órgão
/// (0x7e88d0) pode contar a suspensão por dias (0x556e90, que grava o histórico no arquivo): só a original.
unsafe extern "thiscall" fn comparar_get_slot_text(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(%%I2%%);
    if pessoa == 0 || (cqb9_d(pessoa) as i32) < cqb9_d(0xae2364) as i32 && ban_slot_rule(this, pessoa, vaga) != 0xff {
        // ponytail: o texto do órgão não roda duas vezes; a cópia só repassa a ele
        return o(this, pessoa, vaga, saida, tamanho);
    }
    let r = o(this, pessoa, vaga, saida, tamanho);
    let n = get_slot_text(this, pessoa, vaga, saida, tamanho);
    PLACAR.registrar(r == n, || format!("get_slot_text(pessoa {:#x}, vaga {}) original={} rust={}", pessoa, vaga as i8, r, n));
    r
}
