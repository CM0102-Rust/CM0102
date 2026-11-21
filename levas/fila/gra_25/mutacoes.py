# Mutacoes (gra_25). A rotina roda em toda partida de copa da carreira ENG: a Copa da Liga comeca em agosto de 2001
# (1a rodada no fim de agosto, jogo unico), a FA Cup das divisoes de baixo com replays desde as rodadas
# preliminares; a rodada seguinte, o mando e as datas dela vem daqui.
# - quem passa: o lado vencedor trocado (o perdedor vai ao confronto seguinte e o vencedor sai) desde a 1a rodada da
#   Copa da Liga; o comparador (plano) tambem acusa, porque usa o mesmo ajudante;
# - o replay sem inverter o mando (o mandante do primeiro jogo joga em casa de novo): muda o resultado do replay;
# - o primeiro replay 7 dias mais tarde (o calendario procura a data livre a partir dali).
MUTACOES = [
    ('gra25_vencedor_trocado',
     '    if (r == cup_decision::HOME) == a_at_home {',
     '    if (r == cup_decision::HOME) != a_at_home {',
     'gra_25 tie decided'),
    ('gra25_replay_sem_inverter',
     '    cup_swap_fixture_sides(p); // the replay at the other ground',
     '    // gra25: the replay at the same ground',
     'gra_25 first replay'),
    ('gra25_replay_7_dias',
     '        let gap = read_u8(round_record(cup, round) + cup_round::LEG_GAP_DAYS) as i32;',
     '        let gap = read_u8(round_record(cup, round) + cup_round::LEG_GAP_DAYS) as i32 + 7;',
     'gra_25 first replay'),
]
