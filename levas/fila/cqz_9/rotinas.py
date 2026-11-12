# As honras da Copa da Ásia, as previsões da Copa das Nações da Oceania e as fases das eliminatórias da Oceania
# (asia_nations.cpp, oceania_nations.cpp, wc_oceania_league.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x40f590 (AsiaNations::record_season_honours), 0x79a320 (OceaniaNations::set_predicted_positions),
#   0x91f0d0 (WcOceaniaLeague::advance_stage).
# As honras: salto para a virtual +0x30 da terceira fase (o mata-mata), como a da Copa Africana (que usa a quarta). As
# previsões: o molde da Copa América (0x5e2340), com a contagem +0xf2 em word com sinal e as colocações 1, 2, 3, 3, 5,
# 5 (sem conferir a contagem). As fases: na fase 1 espera a seção sul-americana completa (virtual +0x10 com 1, 1 do
# objeto da competição dela); passar da última é a caixa da linha 0x329; para 1 o confronto dos vencedores dos grupos
# (0x91f640, já em Rust), para 2 a repescagem (0x91f800).
# Convenções conferidas: thiscall, ret 0 (as três); +0x30 e +0x10 virtuais; 0x944e46, 0x945333 (qsort), 0x9452ca
# (cdecl); 0x4a2540 (2 pushes; mov ecx -> thiscall, ret 8).
# Comparadores: só a original nas três (o histórico, as estatísticas e as fases não rodam duas vezes), com o plano nas
# duas últimas.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_nations_record_season_honours', 0x40f590, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x08]),
           ('oceania_nations_set_predicted_positions', 0x79a320, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_advance_stage', 0x91f0d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as honras da Copa da Ásia, as previsões da Copa das Nações da Oceania e as fases das eliminatórias da Oceania',
        'the Asian Cup honours, the Oceania Nations Cup predictions and the Oceania qualifying stages',
        [('asia_nations_record_season_honours', 'AsiaNations::record_season_honours', '0x40f590', 'asia_nations.cpp', 'thiscall, ret 0', 'regra: as honras da temporada da Copa da Ásia (campeão, vice, terceiro) são as do mata-mata, a terceira fase', "rule: the Asian Cup's season honours (winner, runner-up, third) are those of the knockout, the third stage"),
         ('oceania_nations_set_predicted_positions', 'OceaniaNations::set_predicted_positions', '0x79a320', 'oceania_nations.cpp', 'thiscall, ret 0', 'regra: as seis seleções sorteadas da Copa das Nações da Oceania, pela reputação, têm as colocações previstas 1, 2, 3, 3, 5 e 5', 'rule: the six drawn Oceania Nations Cup nations, by reputation, get the predicted places 1, 2, 3, 3, 5 and 5'),
         ('wc_oceania_league_advance_stage', 'WcOceaniaLeague::advance_stage', '0x91f0d0', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: as eliminatórias da Oceania passam dos grupos ao confronto dos vencedores (fase 1) e dele à repescagem intercontinental (fase 2) só quando as eliminatórias sul-americanas acabaram (o adversário vem delas)', 'rule: the Oceania qualifying moves from the groups to the tie of the group winners (stage 1) and from it to the inter-continental play-off (stage 2) only once the South American qualifying is over (the opponent comes from it)')])
CAMPOS = [('competição +0xc fases, +0x30 fase atual, +0x2c número de fases, +8 estatísticas, +0xee/+0xf2 lista sorteada e contagem (word); valor/ponteiro; lê e grava', 'competition +0xc stages, +0x30 current stage, +0x2c stage count, +8 statistics, +0xee/+0xf2 drawn list and count (word); value/pointer; reads and writes')]
