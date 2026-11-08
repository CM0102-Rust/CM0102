use super::*;

// The comparers of the routines in ../fine.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../fine.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `post_fine_reaction_news` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador em ensaio: a cópia para antes da notícia; a original roda a partir do mesmo estado ([0xae1080] e
/// sorteio). Sem notícia, tudo igual; com notícia (que sorteia e vai às caixas), só [0xae1080].
pub(crate) unsafe extern "stdcall" fn comparar_post_fine_reaction_news(player: usize, reason: u32, reacao: u32, deadline: u32, efeitos: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32, u32, u32)>(IDX_POST_FINE_REACTION_NEWS);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    FINE_NEWS_DRY_RUN = true;
    FINE_NEWS_WOULD_BUILD = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_fine_reaction_news(player, reason, reacao, deadline, efeitos)));
    FINE_NEWS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(player, reason, reacao, deadline, efeitos);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let montaria = FINE_NEWS_WOULD_BUILD;
    let same = if montaria { mo.0[0].1 == mr.0[0].1 } else { mo == mr };
    SCOREBOARD.record(same, || format!("post_fine_reaction_news(jogador {:#x}) montaria={} original={:x?} rust={:x?}", player, montaria, mo, mr));
}

/// Comparer for `post_coach_fine_feedback_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_post_coach_fine_feedback_news(player: usize, coach: usize, outcome: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_POST_COACH_FINE_FEEDBACK_NEWS)(player, coach, outcome)
}
