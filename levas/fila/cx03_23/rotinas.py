# O construtor, a virada e as posicoes previstas da Copa da Suecia (swe_cup.cpp; leitor cx03, rodada 5, 24/11/2026).
# Competicoes da Suecia: LIGAS do grupo ENG SWE FIN NOR POR, ate 2002-08-01 (no mundo ate essa data: o construtor e a
# virada 1 vez cada, as posicoes previstas 2).
# - 0x88cbb0 SweCup::SweCup thiscall(ano, registro) ret 8 -> this: Cup::Cup 0x518640; +4, a vtable 0x970044, +0x40, +0xab
#   = 0, +0x44 = 0xff, +0x30 = -1, +0x2c = 8, +0x42 = 1, +0x49 = 5, +0x4a = 3, +0x50 = 0x1d, +0xb1 = 0, +0xb2 = 0;
#   Cup::load_from_disk 0x51fc00 (push 1) != 0: this. Novo: o vetor das fases (malloc 4 x +0x2c, 0 sem fases); a reserva
#   0x88da20; os inscritos 0x88d5a0 (so 0 mostra a caixa 0x55: o -1 de falta de memoria passa, DEFEITOS); +0x28 =
#   800.000 e a tabela pela virtual 0x3c em +0xa3; Cup::map_fixture_tree 0x518790 (0: a caixa 0x63); Comp_Stats (a caixa
#   0x6a); as 7 preliminares 0x88dda0 (push g num dword cujos bytes altos sao os do local reaproveitado); free/zera
#   +0xb2; a reserva de novo; rank_teams 0x88e420; free/zera +0xb2. SEH so para o new. Prologo push -1; push imm32.
# - 0x88d690 SweCup::start_new_season thiscall() ret 0 (a virtual +8): +0x4c = 0; free e zera +0xa7, +0xa3 e os
#   confrontos; Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1); +0x40 + 1, +0xab = 0, +0xb1 = 0; a reserva; os
#   inscritos (negativo: a caixa 0x1f1), as virtuais 0x8c (0x1f8) e 0x94 (0x1ff); as 7 preliminares; a reserva de novo
#   e a virtual 0x5c; free/zera +0xb2. As fases da temporada anterior nao sao destruidas (DEFEITOS: as novas ocupam os
#   slots e as velhas se perdem). Devolve o eax da ultima chamada (free): a copia devolve 0.
# - 0x88e420 SweCup::rank_teams thiscall() ret 0 (a virtual 0x5c): sem estatisticas a caixa 0x408; qsort da reserva
#   (0x945333, 4 bytes, comparador 0x4b07d0); a partir da posicao 1 da reserva (a 0 fica sem lugar: DEFEITOS), grupos de
#   32, 16, 8, 4, 2 e 1 com os lugares 1, 33, 49, 57, 61 e 63 (a tabela local de bytes lida de tras para a frente); os
#   restantes 63. Comp_Stats::set_club_final_position 0x4a2540 (push lugar num dword com lixo nos bytes altos; push
#   clube -> thiscall, ret 8).
# Usa SWE_CUP_CLUB_POOL e swe_cup_pool do cx03_21 (integrar depois dele).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o construtor e a virada so a original, com o plano (os campos; o ano + 1); as posicoes previstas lado a
# lado (a ordem da reserva reposta antes da copia; a ordem e o lugar de cada clube comparados).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_swe_cup', 0x88cbb0, [0x6a, 0xff, 0x68, 0x69, 0x30, 0x96, 0x00]),
           ('swe_cup_start_new_season', 0x88d690, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('swe_cup_rank_teams', 0x88e420, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('o construtor, a virada e as posições previstas da Copa da Suécia',
        "the Swedish Cup's constructor, season turnover and predicted positions",
        [('construct_swe_cup', 'SweCup::SweCup', '0x88cbb0', 'swe_cup.cpp', 'thiscall, ret 8',
          'a Copa da Suécia na criação da carreira: 5 reservas e 3 substituições, regra disciplinar 0x1d, oito fases (sete seções preliminares e a chave principal), prêmio de 800.000; uma copa nova monta a reserva, os seus 30 inscritos, as rodadas e a árvore dos confrontos, as estatísticas, as sete fases preliminares e as posições previstas',
          'the Swedish Cup at career creation: 5 substitutes named and 3 substitutions, discipline rule 0x1d, eight stages (seven preliminary sections and the main draw), an 800,000 prize fund; a new cup builds its pool, its own 30 entrants, its rounds and tie tree, its statistics, the seven preliminary stages and the predicted positions'),
         ('swe_cup_start_new_season', 'SweCup::start_new_season', '0x88d690', 'swe_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Suécia libera os times, as rodadas e os confrontos da temporada, arquiva as estatísticas e passa ao ano seguinte; depois uma reserva nova, os seus 30 inscritos, as rodadas, a árvore dos confrontos, as sete fases preliminares e as posições previstas; as fases da temporada anterior não são destruídas',
          "rule: at the turnover the Swedish Cup frees the season's teams, rounds and ties, archives its statistics and moves to the next year; then a new pool, its own 30 entrants, the rounds, the tie tree, the seven preliminary stages and the predicted positions; last season's stages are not destroyed"),
         ('swe_cup_rank_teams', 'SweCup::rank_teams', '0x88e420', 'swe_cup.cpp', 'thiscall',
          'regra: as posições previstas da Copa da Suécia: a reserva ordenada por reputação; do segundo clube em diante 32 dividem o lugar 1, os 16 seguintes o 33, depois 8 no 49, 4 no 57, 2 no 61 e 1 no 63; os demais também 63; o primeiro clube da reserva fica sem lugar',
          "rule: the Swedish Cup's predicted positions: the pool sorted by reputation; from the second club on, 32 clubs share place 1, the next 16 place 33, then 8 at 49, 4 at 57, 2 at 61 and 1 at 63; all the others 63 as well; the pool's first club gets no place")])
CAMPOS = []
INTERESSA = []
