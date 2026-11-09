use super::*;

/// Writes value v into news item slot k (NEWSITEM set_slot).
pub(crate) unsafe fn set_news_slot(n: usize, k: i8, v: u32) {
    exe::newsitem_set_slot(n, k, v);
}

/// Writes the person's id and the +0x33 keys of +4, +8, +0xc into news slots k..k+3.
///
/// [pt] O id e as três chaves (+0x33 de +4, +8, +0xc) da pessoa `p` nos dados `k`..`k + 3`.
pub(crate) unsafe fn set_news_person_slots(n: usize, k: i8, p: Staff) {
    set_news_slot(n, k, read_u32(p.addr()));
    set_news_slot(n, k + 1, read_u32(p.first_name() + 0x33));
    set_news_slot(n, k + 2, read_u32(p.second_name() + 0x33));
    set_news_slot(n, k + 3, read_u32(p.common_name() + 0x33));
}

/// The rehearsal of the news items: the copy stops at the point where it would build the news item (which draws at
/// random and goes to the managers' inboxes).
///
/// [pt] O ensaio das notícias: a cópia para no ponto em que montaria a notícia (que sorteia e vai às caixas dos técnicos).
pub(crate) static mut FINE_NEWS_DRY_RUN: bool = false;
pub(crate) static mut FINE_NEWS_WOULD_BUILD: bool = false;

/// Posts the news of a fined player's reaction: for a player under contract at a human club, news 0xbc6 with reason,
/// reaction, random(2), personality, period, contract type, future transfer and effects.
///
/// SEH frame for the local news item, no destructor. `post_fine_reaction_news(player, reason, reaction, period,
/// effects)`, called from the fine (0x5a5980, five calls). No player, no contract (0x4cf5a0, on `[0xae1080]`), no club
/// of his or no club contract (`exe::contract_manager_get_contract_for_club`): nothing; player of a club (+0x39) without a human (`news_club_has_human_manager`): nothing. News
/// 0xbc6 to the contract's club: the player (id and keys), reason, reaction, random(2), the personality (+0x5d + +0x5b
/// below 12, +0x59 below 6 and reputation +0xb above 5250), period 1, the type 7 contract (+0x4f), the future transfer
/// (`get_future_transfer`), the effects and the game by the type (+0x4f 1 with more than 23 years, or 2).
/// Original: `post_fine_reaction_news` (0x5a74b0, Fine.cpp, stdcall, ret 0x14).
///
/// [pt] { 0x5a74b0 } (stdcall, ret 0x14, Fine.cpp, SEH da notícia local, sem destrutor): `post_fine_reaction_news(jogador,
/// motivo, reação, prazo, efeitos)`, pela multa (0x5a5980, cinco chamadas). Sem jogador, sem contrato (0x4cf5a0, em
/// [0xae1080]), sem o clube dele ou sem o contrato do clube (0x4cf5f0): nada; jogador de clube (+0x39) sem humano
/// (0x7875e0): nada. A notícia 0xbc6 ao clube do contrato: o jogador (id e chaves), motivo, reação, sorteio(2), a
/// personalidade (+0x5d + +0x5b abaixo de 12, +0x59 abaixo de 6 e reputação +0xb acima de 5250), prazo 1, o contrato
/// de tipo 7 (+0x4f), a transferência futura (0x8cca10), os efeitos e o jogo pelo tipo (+0x4f 1 com mais de 23 anos,
/// ou 2).
pub(crate) unsafe extern "stdcall" fn post_fine_reaction_news(player: usize, reason: u32, reacao: u32, deadline: u32, efeitos: u32) {
    let player = Staff(player);
    if player.is_null() {
        return;
    }
    let k = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k.addr();
    if k.is_null() {
        return;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add(k.club().wrapping_mul(club::SIZE));
    if club == 0 {
        return;
    }
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = Contract(contract(read_usize(G_CONTRACT_MANAGER), player.addr(), club));
    if c.is_null() {
        return;
    }
    let current = player.club_job();
    if current != 0 && !club_has_human_manager(current) {
        return;
    }
    crate::comparison::branch("cpz_9 news of the reaction to the fine");
    if FINE_NEWS_DRY_RUN {
        FINE_NEWS_WOULD_BUILD = true;
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbc6, 0);
    set_news_person_slots(nt, 0, player);
    set_news_slot(nt, 4, reason as i8 as u32);
    set_news_slot(nt, 5, reacao as i8 as u32);
    let r = friendly_random(2);
    set_news_slot(nt, 6, r as u32);
    let p = (player.temperament() as i32) + (player.professionalism() as i32) < 0xc
        && (player.loyalty() as i32) < 6
        && core::ptr::read_unaligned((player.player() + 0xb) as *const i16) > 0x1482;
    set_news_slot(nt, 7, p as u32);
    set_news_slot(nt, 8, (deadline as u8 == 1) as u32);
    let kind = c.squad_status() & 0xf0;
    set_news_slot(nt, 9, (kind == 0x70) as u32);
    set_news_slot(nt, 0xa, (exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), player, (0) as u32) != 0) as u32);
    set_news_slot(nt, 0xb, efeitos as i8 as u32);
    let game = (((player.age() as i8) as i32) > 0x17 && kind == 0x10) || kind == 0x20;
    set_news_slot(nt, 0xc, game as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
}

/// Posts the coach's feedback on a fine: with player and coach both having a record and a nation, and the player having
/// a club, news 0x1394 with the outcome and random(5).
///
/// SEH frame for the local news item, no destructor. `post_coach_fine_feedback_news(player, coach, outcome)`, called
/// from the fines' daily pass (0x5a5820: the feedback marked as due, more than 30 days at the club and no serious
/// injury; the original leaves the Fine in ecx). Both with a record (+0x61) and a nation (+0x1a), the player with a
/// club (+0x39): news 0x1394 (the player and the coach, id and keys; the outcome; random(5)) to the player's club.
/// Original: `post_coach_fine_feedback_news` (0x5a7f20, Fine.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x5a7f20 } (stdcall, ret 0xc, Fine.cpp, SEH da notícia local, sem destrutor): `post_coach_fine_feedback_news
/// (jogador, treinador, desfecho)`, pelo diário das multas (0x5a5820: o retorno marcado vencido, mais de 30 dias de
/// casa e sem lesão grave; a original deixa o Fine em ecx). Os dois com ficha (+0x61) e nação (+0x1a), o jogador com
/// clube (+0x39): a notícia 0x1394 (o jogador e o treinador, id e chaves; o desfecho; sorteio(5)) ao clube do jogador.
pub(crate) unsafe extern "stdcall" fn post_coach_fine_feedback_news(player: usize, coach: usize, outcome: u32) {
    let coach = Staff(coach);
    let player = Staff(player);
    if player.is_null() || player.player() == 0 || player.nation() == 0 || player.club_job() == 0 {
        return;
    }
    if coach.is_null() || coach.player() == 0 || coach.nation() == 0 {
        return;
    }
    crate::comparison::branch("cpz_9 news of the coach return");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x1394, 0);
    set_news_person_slots(nt, 0, player);
    set_news_person_slots(nt, 4, coach);
    set_news_slot(nt, 8, outcome as i8 as u32);
    let r = friendly_random(5);
    set_news_slot(nt, 9, r as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, player.club_job());
}

// simplification: the news rolls and goes to the managers’ inboxes (allocates): original only, with the copy taking the branch; without
// news item, both do nothing.

