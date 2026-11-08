use super::*;

// ---------- 262a leva: a forma ruim do jogador e as notícias das multas: o incidente de forma com a multa da IA, a reação à multa e o retorno do treinador (awol.cpp, Fine.cpp) ----------





/// Writes value v into news item slot k (NEWSITEM set_slot).
pub(crate) unsafe fn set_news_slot(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    f(n, k, v);
}

/// Writes the person's id and the +0x33 keys of +4, +8, +0xc into news slots k..k+3.
///
/// [pt] O id e as três chaves (+0x33 de +4, +8, +0xc) da pessoa `p` nos dados `k`..`k + 3`.
pub(crate) unsafe fn set_news_person_slots(n: usize, k: i8, p: usize) {
    set_news_slot(n, k, read_u32(p));
    set_news_slot(n, k + 1, read_u32(read_usize(p + 4) + 0x33));
    set_news_slot(n, k + 2, read_u32(read_usize(p + 8) + 0x33));
    set_news_slot(n, k + 3, read_u32(read_usize(p + 0xc) + 0x33));
}


/// O ensaio das notícias: a cópia para no ponto em que montaria a notícia (que sorteia e vai às caixas dos técnicos).
pub(crate) static mut CPZ9_ENSAIO: bool = false;
pub(crate) static mut CPZ9_MONTARIA: bool = false;



/// player under contract at a human club: news 0xbc6 with reason, reaction, draw(2), personality, period, contract
/// type, future transfer and effects
/// Original: `post_fine_reaction_news` (0x5a74b0, Fine.cpp, stdcall, ret 0x14).
///
/// [pt] { 0x5a74b0 } (stdcall, ret 0x14, Fine.cpp, SEH da notícia local, sem destrutor): `post_fine_reaction_news(jogador,
/// motivo, reação, prazo, efeitos)`, pela multa (0x5a5980, cinco chamadas). Sem jogador, sem contrato (0x4cf5a0, em
/// [0xae1080]), sem o clube dele ou sem o contrato do clube (0x4cf5f0): nada; jogador de clube (+0x39) sem humano
/// (0x7875e0): nada. A notícia 0xbc6 ao clube do contrato: o jogador (id e chaves), motivo, reação, sorteio(2), a
/// personalidade (+0x5d + +0x5b abaixo de 12, +0x59 abaixo de 6 e reputação +0xb acima de 5250), prazo 1, o contrato
/// de tipo 7 (+0x4f), a transferência futura (0x8cca10), os efeitos e o jogo pelo tipo (+0x4f 1 com mais de 23 anos,
/// ou 2).
pub(crate) unsafe extern "stdcall" fn post_fine_reaction_news(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    if jogador == 0 {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let k = contract_of(read_usize(G_CONTRACT_MANAGER), jogador);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k;
    if k == 0 {
        return;
    }
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(0x245));
    if clube == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), jogador, clube);
    if c == 0 {
        return;
    }
    let atual = read_usize(jogador + 0x39);
    if atual != 0 && !club_has_human_manager(atual) {
        return;
    }
    crate::comparacao::ramo("cpz_9 noticia da reacao a multa");
    if CPZ9_ENSAIO {
        CPZ9_MONTARIA = true;
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc6, 0);
    set_news_person_slots(nt, 0, jogador);
    set_news_slot(nt, 4, motivo as i8 as u32);
    set_news_slot(nt, 5, reacao as i8 as u32);
    let r = friendly_random(2);
    set_news_slot(nt, 6, r as u32);
    let p = read_i8_as_i32(jogador + 0x5d) + read_i8_as_i32(jogador + 0x5b) < 0xc
        && read_i8_as_i32(jogador + 0x59) < 6
        && core::ptr::read_unaligned((read_usize(jogador + 0x61) + 0xb) as *const i16) > 0x1482;
    set_news_slot(nt, 7, p as u32);
    set_news_slot(nt, 8, (prazo as u8 == 1) as u32);
    let tipo = *((c + 0x4f) as *const u8) & 0xf0;
    set_news_slot(nt, 9, (tipo == 0x70) as u32);
    set_news_slot(nt, 0xa, (futura(read_usize(G_TRANSFER_MANAGER), jogador, 0) != 0) as u32);
    set_news_slot(nt, 0xb, efeitos as i8 as u32);
    let jogo = (read_i8_as_i32(jogador + 0x18) > 0x17 && tipo == 0x10) || tipo == 0x20;
    set_news_slot(nt, 0xc, jogo as u32);
    entregar(read_usize(G_NEWS), nt, clube);
}

/// O comparador em ensaio: a cópia para antes da notícia; a original roda a partir do mesmo estado ([0xae1080] e
/// sorteio). Sem notícia, tudo igual; com notícia (que sorteia e vai às caixas), só [0xae1080].
pub(crate) unsafe extern "stdcall" fn comparar_post_fine_reaction_news(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32, u32, u32)>(IDX_POST_FINE_REACTION_NEWS);
    let mut regioes = vec![(0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPZ9_ENSAIO = true;
    CPZ9_MONTARIA = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| post_fine_reaction_news(jogador, motivo, reacao, prazo, efeitos)));
    CPZ9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, motivo, reacao, prazo, efeitos);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let montaria = CPZ9_MONTARIA;
    let igual = if montaria { mo.0[0].1 == mr.0[0].1 } else { mo == mr };
    PLACAR.registrar(igual, || format!("post_fine_reaction_news(jogador {:#x}) montaria={} original={:x?} rust={:x?}", jogador, montaria, mo, mr));
}

/// player and coach with a record and nation, player with a club: news 0x1394 with the outcome and draw(5)
/// Original: `post_coach_fine_feedback_news` (0x5a7f20, Fine.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x5a7f20 } (stdcall, ret 0xc, Fine.cpp, SEH da notícia local, sem destrutor): `post_coach_fine_feedback_news
/// (jogador, treinador, desfecho)`, pelo diário das multas (0x5a5820: o retorno marcado vencido, mais de 30 dias de
/// casa e sem lesão grave; a original deixa o Fine em ecx). Os dois com ficha (+0x61) e nação (+0x1a), o jogador com
/// clube (+0x39): a notícia 0x1394 (o jogador e o treinador, id e chaves; o desfecho; sorteio(5)) ao clube do jogador.
pub(crate) unsafe extern "stdcall" fn post_coach_fine_feedback_news(jogador: usize, treinador: usize, desfecho: u32) {
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || read_usize(jogador + 0x1a) == 0 || read_usize(jogador + 0x39) == 0 {
        return;
    }
    if treinador == 0 || read_usize(treinador + 0x61) == 0 || read_usize(treinador + 0x1a) == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_9 noticia do retorno do treinador");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x1394, 0);
    set_news_person_slots(nt, 0, jogador);
    set_news_person_slots(nt, 4, treinador);
    set_news_slot(nt, 8, desfecho as i8 as u32);
    let r = friendly_random(5);
    set_news_slot(nt, 9, r as u32);
    entregar(read_usize(G_NEWS), nt, read_usize(jogador + 0x39));
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original, com a cópia levando o ramo; sem
// notícia as duas não fazem nada.
pub(crate) unsafe extern "stdcall" fn comparar_post_coach_fine_feedback_news(jogador: usize, treinador: usize, desfecho: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_POST_COACH_FINE_FEEDBACK_NEWS)(jogador, treinador, desfecho)
}

