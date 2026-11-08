// ---------- %%LEVA%%a leva: a forma ruim do jogador e as notícias das multas: o incidente de forma com a multa da IA, a reação à multa e o retorno do treinador (awol.cpp, Fine.cpp) ----------

unsafe fn cpz9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz9_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz9_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpz9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn set_news_slot(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

/// O id e as três chaves (+0x33 de +4, +8, +0xc) da pessoa `p` nos dados `k`..`k + 3`.
unsafe fn set_news_person_slots(n: usize, k: i8, p: usize) {
    set_news_slot(n, k, cpz9_u(p));
    set_news_slot(n, k + 1, cpz9_u(cpz9_d(p + 4) + 0x33));
    set_news_slot(n, k + 2, cpz9_u(cpz9_d(p + 8) + 0x33));
    set_news_slot(n, k + 3, cpz9_u(cpz9_d(p + 0xc) + 0x33));
}

/// Um humano dirige o clube? (news_club_has_human_manager 0x7875e0: stdcall, 1 arg; a original deixa o News em ecx.)
unsafe fn cpz9_clube_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// O ensaio das notícias: a cópia para no ponto em que montaria a notícia (que sorteia e vai às caixas dos técnicos).
static mut FINE_NEWS_DRY_RUN: bool = false;
static mut FINE_NEWS_WOULD_BUILD: bool = false;

/// { 0x41add0 } (thiscall, ret 8, awol.cpp, SEH da data e da notícia local, sem destrutor): `AWOL::
/// report_poor_fitness(jogador, clube)`, pelo diário das pessoas (0x5ce2d0: a queda de condição grande do dia). O
/// registro (jogador, clube, hoje e um byte que na original é sobra de pilha) entra na lista dos incidentes ([this+8],
/// 0x68d0d0, 0x11 bytes). Clube de humano (0x7875e0): um treinador das 5 vagas (+0x19f, sorteio(5), até achar um ou
/// sorteio(20) dar zero) e a notícia 0x138f ao clube (o jogador, sorteio(2), as chaves dele e do treinador ou −1).
/// Clube da IA (0x600a50) com técnico (+0xcf, +0x69): sorteio(+0x14 dele) acima de sorteio(10) leva à multa (0x5a5980,
/// prazo 1 se sorteio(nota 0x544410 com o +0x19) passar de sorteio(5), senão 2; motivo 1).
unsafe extern "thiscall" fn report_poor_fitness(this: usize, jogador: usize, clube: usize) {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut reg = [0u8; 0x11];
    iniciar_data(reg.as_mut_ptr() as usize + 8);
    if jogador == 0 || clube == 0 {
        return;
    }
    reg[..4].copy_from_slice(&cpz9_u(jogador).to_le_bytes());
    reg[4..8].copy_from_slice(&cpz9_u(clube).to_le_bytes());
    let atribuir: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut temporaria = [0u8; 8];
    atribuir(reg.as_mut_ptr() as usize + 8, temporaria.as_mut_ptr() as usize, 0xae2c90);
    // ponytail: o byte +0x10 do registro é sobra de pilha na original; a cópia grava 0 (ninguém o lê)
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    acrescentar(cpz9_d(this + 8), reg.as_ptr() as usize, 0x11);
    if cpz9_clube_humano(clube) {
        let mut treinador;
        loop {
            let r = cpz9_sortear(5);
            treinador = cpz9_d(clube.wrapping_add((r as usize).wrapping_mul(4)) + 0x19f);
            if cpz9_sortear(0x14) == 0 || treinador != 0 {
                break;
            }
        }
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construir(nt, 0x138f, 0);
        set_news_slot(nt, 0, cpz9_u(jogador));
        set_news_slot(nt, 1, cpz9_u(clube));
        let r = cpz9_sortear(2);
        set_news_slot(nt, 2, r as u32);
        set_news_slot(nt, 3, cpz9_u(cpz9_d(jogador + 4) + 0x33));
        set_news_slot(nt, 4, cpz9_u(cpz9_d(jogador + 8) + 0x33));
        set_news_slot(nt, 5, cpz9_u(cpz9_d(jogador + 0xc) + 0x33));
        if treinador != 0 {
            set_news_person_slots(nt, 6, treinador);
        } else {
            for k in 6..10 {
                set_news_slot(nt, k, u32::MAX);
            }
        }
        entregar(cpz9_d(0xb7431c), nt, clube);
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    let tecnico = cpz9_d(clube + 0xcf);
    if tecnico == 0 {
        return;
    }
    let np = cpz9_d(tecnico + 0x69);
    if np == 0 {
        return;
    }
    crate::comparison::branch("cpz_9 fine evaluated");
    let a = cpz9_sortear(cpz9_sb(np + 0x14));
    let b = cpz9_sortear(10); // cpz9: o sorteio da multa
    if a <= b {
        return;
    }
    let np = cpz9_d(cpz9_d(clube + 0xcf) + 0x69);
    crate::comparison::branch("cpz_9 fine by form");
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    // ponytail: a original empurra ecx com o byte +0x19 só no cl (os altos são sobra); a 0x544410 só lê o byte
    let c =cpz9_sortear(nota(np, *((np + 0x19) as *const u8) as u32) as i32);
    let e = cpz9_sortear(5); // cpz9: o sorteio do prazo
    let prazo = if c > e { 1 } else { 2 }; // cpz9: o prazo da multa
    let multar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x5a5980usize);
    multar(cpz9_d(0xae2a3c), jogador, prazo, 1);
}

/// O comparador do incidente: a original roda sozinha (põe o registro na lista, dá a notícia e multa) e confere o
/// plano: jogador e clube não nulos põem logo depois da cauda de antes (ou na cabeça, lista vazia) o registro com o
/// jogador, o clube e a data de hoje (16 bytes; o byte +0x10 é sobra de pilha).
unsafe extern "thiscall" fn compare_report_poor_fitness(this: usize, jogador: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let l = cpz9_d(this + 8);
    if jogador == 0 || clube == 0 || l == 0 {
        return o(this, jogador, clube);
    }
    let cauda = cpz9_d(l + 6);
    let mut plano = [0u8; 0x10];
    plano[..4].copy_from_slice(&cpz9_u(jogador).to_le_bytes());
    plano[4..8].copy_from_slice(&cpz9_u(clube).to_le_bytes());
    plano[8..].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    o(this, jogador, clube);
    let no = if cauda != 0 { cpz9_d(cauda + 6) } else { cpz9_d(l + 2) };
    let dados = if no != 0 { cpz9_d(no) } else { 0 };
    let igual = dados != 0 && core::slice::from_raw_parts(dados as *const u8, 0x10) == plano;
    SCOREBOARD.record(igual, || format!("report_poor_fitness(jogador {:#x}, clube {:#x}) plano={:x?} lista: no {:#x} dados {:#x}", jogador, clube, plano, no, dados));
}

/// { 0x5a74b0 } (stdcall, ret 0x14, Fine.cpp, SEH da notícia local, sem destrutor): `post_fine_reaction_news(jogador,
/// motivo, reação, prazo, efeitos)`, pela multa (0x5a5980, cinco chamadas). Sem jogador, sem contrato (0x4cf5a0, em
/// [0xae1080]), sem o clube dele ou sem o contrato do clube (0x4cf5f0): nada; jogador de clube (+0x39) sem humano
/// (0x7875e0): nada. A notícia 0xbc6 ao clube do contrato: o jogador (id e chaves), motivo, reação, sorteio(2), a
/// personalidade (+0x5d + +0x5b abaixo de 12, +0x59 abaixo de 6 e reputação +0xb acima de 5250), prazo 1, o contrato
/// de tipo 7 (+0x4f), a transferência futura (0x8cca10), os efeitos e o jogo pelo tipo (+0x4f 1 com mais de 23 anos,
/// ou 2).
unsafe extern "stdcall" fn post_fine_reaction_news(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    if jogador == 0 {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contract_of(cpz9_d(0xae1050), jogador);
    *(0xae1080 as *mut usize) = k;
    if k == 0 {
        return;
    }
    let clube = cpz9_d(0xae23b4).wrapping_add(cpz9_d(k + 4).wrapping_mul(0x245));
    if clube == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contrato(cpz9_d(0xae1050), jogador, clube);
    if c == 0 {
        return;
    }
    let atual = cpz9_d(jogador + 0x39);
    if atual != 0 && !cpz9_clube_humano(atual) {
        return;
    }
    crate::comparison::branch("cpz_9 news of the reaction to the fine");
    if FINE_NEWS_DRY_RUN {
        FINE_NEWS_WOULD_BUILD = true;
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc6, 0);
    set_news_person_slots(nt, 0, jogador);
    set_news_slot(nt, 4, motivo as i8 as u32);
    set_news_slot(nt, 5, reacao as i8 as u32);
    let r = cpz9_sortear(2);
    set_news_slot(nt, 6, r as u32);
    let p = cpz9_sb(jogador + 0x5d) + cpz9_sb(jogador + 0x5b) < 0xc
        && cpz9_sb(jogador + 0x59) < 6
        && core::ptr::read_unaligned((cpz9_d(jogador + 0x61) + 0xb) as *const i16) > 0x1482;
    set_news_slot(nt, 7, p as u32);
    set_news_slot(nt, 8, (prazo as u8 == 1) as u32);
    let tipo = *((c + 0x4f) as *const u8) & 0xf0;
    set_news_slot(nt, 9, (tipo == 0x70) as u32);
    set_news_slot(nt, 0xa, (futura(cpz9_d(0xde1f54), jogador, 0) != 0) as u32);
    set_news_slot(nt, 0xb, efeitos as i8 as u32);
    let jogo = (cpz9_sb(jogador + 0x18) > 0x17 && tipo == 0x10) || tipo == 0x20;
    set_news_slot(nt, 0xc, jogo as u32);
    entregar(cpz9_d(0xb7431c), nt, clube);
}

/// O comparador em ensaio: a cópia para antes da notícia; a original roda a partir do mesmo estado ([0xae1080] e
/// sorteio). Sem notícia, tudo igual; com notícia (que sorteia e vai às caixas), só [0xae1080].
unsafe extern "stdcall" fn compare_post_fine_reaction_news(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32, u32, u32)>(%%I1%%);
    let mut regioes = vec![(0xae1080, 4)];
    regioes.extend(crate::rng::STATE_REGIONS);
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    FINE_NEWS_DRY_RUN = true;
    FINE_NEWS_WOULD_BUILD = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_fine_reaction_news(jogador, motivo, reacao, prazo, efeitos)));
    FINE_NEWS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    o(jogador, motivo, reacao, prazo, efeitos);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    let montaria = FINE_NEWS_WOULD_BUILD;
    let igual = if montaria { mo.0[0].1 == mr.0[0].1 } else { mo == mr };
    SCOREBOARD.record(igual, || format!("post_fine_reaction_news(jogador {:#x}) montaria={} original={:x?} rust={:x?}", jogador, montaria, mo, mr));
}

/// { 0x5a7f20 } (stdcall, ret 0xc, Fine.cpp, SEH da notícia local, sem destrutor): `post_coach_fine_feedback_news
/// (jogador, treinador, desfecho)`, pelo diário das multas (0x5a5820: o retorno marcado vencido, mais de 30 dias de
/// casa e sem lesão grave; a original deixa o Fine em ecx). Os dois com ficha (+0x61) e nação (+0x1a), o jogador com
/// clube (+0x39): a notícia 0x1394 (o jogador e o treinador, id e chaves; o desfecho; sorteio(5)) ao clube do jogador.
unsafe extern "stdcall" fn post_coach_fine_feedback_news(jogador: usize, treinador: usize, desfecho: u32) {
    if jogador == 0 || cpz9_d(jogador + 0x61) == 0 || cpz9_d(jogador + 0x1a) == 0 || cpz9_d(jogador + 0x39) == 0 {
        return;
    }
    if treinador == 0 || cpz9_d(treinador + 0x61) == 0 || cpz9_d(treinador + 0x1a) == 0 {
        return;
    }
    crate::comparison::branch("cpz_9 news of the coach return");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x1394, 0);
    set_news_person_slots(nt, 0, jogador);
    set_news_person_slots(nt, 4, treinador);
    set_news_slot(nt, 8, desfecho as i8 as u32);
    let r = cpz9_sortear(5);
    set_news_slot(nt, 9, r as u32);
    entregar(cpz9_d(0xb7431c), nt, cpz9_d(jogador + 0x39));
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original, com a cópia levando o ramo; sem
// notícia as duas não fazem nada.
unsafe extern "stdcall" fn compare_post_coach_fine_feedback_news(jogador: usize, treinador: usize, desfecho: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(%%I2%%)(jogador, treinador, desfecho)
}
