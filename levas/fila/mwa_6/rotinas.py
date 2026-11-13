# The (Northern) Irish Cup (nir_cup.cpp, in cups\): the round table, the entrants and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_1's helpers
# (irish_club_division_id, irish_cup_club_count, compare_irish_cup_entrants): it goes in the same batch or after it.
# - 0x78a870 (NirCup::create_round_table, 2 calls): the single-stage round table template (the head checked instruction
#   by instruction; the five rounds read by the symbolic reader) plus the three semi-final match-day slots
#   (0x521f10 cup_set_round_match_day, 7 pushes; add esp -> cdecl).
# - 0x78ac90 (NirCup::collect_entrants, 2 calls).
# - 0x78ae30 (NirCup::start_new_season, 1 call): the turnover template start_new_cup_season.
# Conventions checked at the call sites: create_round_table the virtual +0x3c (thiscall, ret 0x10); collect_entrants
# (mov ecx, esi; call -> thiscall, ret 0); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x944e46 (cdecl),
# 0x4afbd0 (cdecl), the error box.
# Comparers: the round table side by side (the written bytes and the three slots); the entrants side by side with the
# generator state (club, class and status); the turnover only the original, with the plan.
# Determinism: the pool is the uninitialised stack in the original; it is only read past the clubs found if the lower
# division had fewer than twelve clubs (not in the database).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_cup_create_round_table', 0x78a870, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_cup_collect_entrants', 0x78ac90, [0x81, 0xec, 0x7c, 0x02, 0x00, 0x00]),
           ('nir_cup_start_new_season', 0x78ae30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Irlanda do Norte: a tabela das rodadas, os inscritos e a virada da temporada',
        'the Northern Irish Cup: the round table, the entrants and the season turnover',
        [('nir_cup_create_round_table', 'NirCup::create_round_table', '0x78a870', 'nir_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Irlanda do Norte tem cinco rodadas em jogo único com jogo-desempate para 32 clubes: a quinta rodada sorteada em 10/12 e jogada em 20/1 do ano seguinte (32 clubes, 16 confrontos; desempate 4 dias depois), a sexta sorteada em 21/1 e jogada em 17/2, as quartas sorteadas em 18/2 e jogadas em 10/3, as semifinais sorteadas em 11/3 e jogadas em 7/4 em dois dias (quinta e sexta [inferência: dias 4 e 5]) em campo neutro [inferência: mando 1] e a final sorteada em 8/4 e jogada em 5/5 [inferência: mando 4], cada data levada ao seu dia da semana; bits de sorteio 8 nas três primeiras, regras 0x100 nas semifinais e 2 na final; categoria 4; os perdedores ganham 250, 1.500, 3.000 e 5.500 da sexta rodada em diante, o campeão 10.000',
          'rule: the Northern Irish Cup has five single-leg rounds with replays for 32 clubs: the fifth round drawn on 10/12 and played on 20/1 of the next year (32 clubs, 16 ties; replay 4 days later), the sixth drawn 21/1 and played 17/2, the quarter-finals drawn 18/2 and played 10/3, the semi-finals drawn 11/3 and played 7/4 over two days (Thursday and Friday [inference: weekdays 4 and 5]) at a neutral ground [inference: home side code 1] and the final drawn 8/4 and played on 5/5 [inference: home side code 4], each moved to its weekday; draw bits 8 in the first three rounds, tie rules 0x100 in the semi-finals and 2 in the final; category 4; the losers get 250, 1,500, 3,000 and 5,500 from the sixth round on, the winner 10,000'),
         ('nir_cup_collect_entrants', 'NirCup::collect_entrants', '0x78ac90', 'nir_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Irlanda do Norte tem 32 clubes: doze da divisão de baixo (os melhores, embaralhados entre os 30 primeiros), nas vagas 0 a 11 com classe 2, e os 20 clubes da Premier e da First Division, nas vagas 12 a 31 com classe 1',
          'rule: the Northern Irish Cup takes 32 clubs: twelve of the lower division (the best, shuffled among the top 30), entries 0 to 11 with class 2, and the 20 clubs of the Premier and First Divisions, entries 12 to 31 with class 1'),
         ('nir_cup_start_new_season', 'NirCup::start_new_season', '0x78ae30', 'nir_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Irlanda do Norte é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os inscritos, as regras, a árvore dos confrontos e as posições previstas são feitos de novo',
          "rule: at the season turn the Northern Irish Cup is rebuilt: last edition's tables are freed, the statistics archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made again")])
CAMPOS = []
INTERESSA = []
