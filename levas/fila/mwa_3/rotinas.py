# The Munster Senior Cup (ire_munster_cup.cpp, in cups\): the round table, the entrants and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_1's helpers
# (irish_club_division_id, irish_cup_club_count, compare_irish_cup_entrants): it goes in the same batch or after it.
# - 0x638a60 (IreMunsterCup::create_round_table, 2 calls): the single-stage round table template, checked
#   instruction by instruction (the five rounds read by the symbolic reader that reproduces BraCup's table).
# - 0x638e30 (IreMunsterCup::collect_entrants, 2 calls).
# - 0x6390d0 (IreMunsterCup::start_new_season, 1 call): the turnover template start_new_cup_season.
# Conventions checked at the call sites: create_round_table the virtual +0x3c (thiscall, ret 0x10); collect_entrants
# (mov ecx, esi; call -> thiscall, ret 0); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x944e46 (cdecl),
# 0x545880 club_get_english_name (push club; add esp, 4 -> cdecl), the inline strcmp against the four names, 0x4afbd0
# (cdecl), the error box.
# Comparers: the round table side by side; the entrants side by side with the generator state; the turnover only the
# original, with the plan.
# Determinism: the pool is the uninitialised stack in the original; it is only read past the clubs found if the
# Munster Senior League had fewer than sixteen clubs (not in the database).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_munster_cup_create_round_table', 0x638a60, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_munster_cup_collect_entrants', 0x638e30, [0x81, 0xec, 0xb0, 0x02, 0x00, 0x00]),
           ('ire_munster_cup_start_new_season', 0x6390d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa de Munster: a tabela das rodadas, os inscritos e a virada da temporada',
        'the Munster Senior Cup: the round table, the entrants and the season turnover',
        [('ire_munster_cup_create_round_table', 'IreMunsterCup::create_round_table', '0x638a60', 'ire_munster_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa de Munster tem cinco rodadas em jogo único: a preliminar sorteada em 1/8 e jogada em 23/8 (16 clubes, 8 confrontos), a seguinte sorteada em 25/8 e jogada em 18/10 (os 8 vencedores), as quartas sorteadas em 20/10 e jogadas em 17/1 do ano seguinte (os 4 vencedores e os 4 que entram agora), as semifinais sorteadas em 19/1 e jogadas em 14/3 e a final sorteada em 15/3 e jogada em 9/5, cada data levada ao seu dia da semana; regra de desempate 3 [inferência]; categoria 4, e 6 na final; o prêmio do vencedor é uma parte do fundo (−1)',
          "rule: the Munster Senior Cup has five knockout rounds of single matches: the preliminary round drawn on 1/8 and played on 23/8 (16 clubs, 8 ties), the next drawn 25/8 and played 18/10 (the 8 winners), the quarter-finals drawn 20/10 and played 17/1 of the next year (the 4 winners and the 4 entering now), the semi-finals drawn 19/1 and played 14/3 and the final drawn 15/3 and played on 9/5, each moved to its weekday; tie rule 3 [inference]; category 4, and 6 in the final; the tie winner's prize is a share of the prize fund (−1)"),
         ('ire_munster_cup_collect_entrants', 'IreMunsterCup::collect_entrants', '0x638e30', 'ire_munster_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa de Munster tem 20 clubes: dezesseis da Munster Senior League (os melhores, embaralhados entre os 48 primeiros), nas vagas 0 a 15, e os quatro clubes de Munster da League of Ireland (Cork City, Cobh Ramblers, Waterford United e Limerick), nas vagas 16 a 19',
          'rule: the Munster Senior Cup takes 20 clubs: sixteen of the Munster Senior League (the best, shuffled among the top 48), entries 0 to 15, and the four Munster clubs of the League of Ireland (Cork City, Cobh Ramblers, Waterford United and Limerick), entries 16 to 19'),
         ('ire_munster_cup_start_new_season', 'IreMunsterCup::start_new_season', '0x6390d0', 'ire_munster_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa de Munster é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os inscritos, as regras, a árvore dos confrontos e as posições previstas são feitos de novo',
          "rule: at the season turn the Munster Senior Cup is rebuilt: last edition's tables are freed, the statistics archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made again")])
CAMPOS = []
INTERESSA = []
