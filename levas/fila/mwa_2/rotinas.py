# The Leinster Senior Cup (ire_leinster_cup.cpp, in cups\): the round table, the clubs and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_1's helpers
# (irish_club_division_id, irish_cup_club_count, compare_irish_cup_entrants): it goes in the same batch or after it.
# - 0x636760 (IreLeinsterCup::create_round_table, 2 calls): the single-stage round table template, checked
#   instruction by instruction (the four rounds read by the symbolic reader that reproduces BraCup's table).
# - 0x636a80 (IreLeinsterCup::select_clubs, 2 calls).
# - 0x636c20 (IreLeinsterCup::start_new_season, 1 call): the turnover template start_new_cup_season (only the entrant
#   routine, the file and the lines change against BraCup's).
# Conventions checked at the call sites: create_round_table the virtual +0x3c (thiscall, ret 0x10); select_clubs (mov
# ecx, esi; call -> thiscall, ret 0); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x944e46 (cdecl),
# 0x6399c0 ire_club_is_in_leinster (push club; add esp, 4 -> cdecl), 0x4afbd0 comp_util_sort_and_shuffle_top_clubs
# (3 pushes; add esp, 0xc -> cdecl), the error box.
# Comparers: the round table side by side; the clubs side by side with the generator state (club and status of the
# entries written); the turnover only the original, with the plan.
# Defect of the original: the Leinster League of Ireland clubs have no bound against the 16 entries (a 13th would
# write past the table); the career's count is smaller.
# Determinism: the pool is the uninitialised stack in the original; it is only read past the clubs found if the
# Leinster Senior League had fewer than four clubs (not in the database).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_leinster_cup_create_round_table', 0x636760, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_leinster_cup_select_clubs', 0x636a80, [0x81, 0xec, 0xa8, 0x02, 0x00, 0x00]),
           ('ire_leinster_cup_start_new_season', 0x636c20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa de Leinster: a tabela das rodadas, os clubes e a virada da temporada',
        'the Leinster Senior Cup: the round table, the clubs and the season turnover',
        [('ire_leinster_cup_create_round_table', 'IreLeinsterCup::create_round_table', '0x636760', 'ire_leinster_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa de Leinster é um mata-mata de 16 clubes em jogo único: a primeira rodada sorteada em 1/8 e jogada em 23/8, as quartas sorteadas em 25/8 e jogadas em 18/10, as semifinais sorteadas em 20/10 e jogadas em 17/1 do ano seguinte e a final sorteada em 19/1 e jogada em 14/3, cada data levada ao seu dia da semana; regra de desempate 3 [inferência: prorrogação e pênaltis]; categoria 4; o prêmio do vencedor é uma parte do fundo (−1)',
          "rule: the Leinster Senior Cup is a 16-club knockout of single matches: the first round drawn on 1/8 and played on 23/8, the quarter-finals drawn 25/8 and played 18/10, the semi-finals drawn 20/10 and played 17/1 of the next year and the final drawn 19/1 and played on 14/3, each moved to its weekday; tie rule 3 [inference: extra time and penalties]; category 4; the tie winner's prize is a share of the prize fund (−1)"),
         ('ire_leinster_cup_select_clubs', 'IreLeinsterCup::select_clubs', '0x636a80', 'ire_leinster_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa de Leinster tem 16 clubes: quatro da Leinster Senior League (os melhores, embaralhados entre os doze primeiros), nas vagas 0 a 3, e os clubes da Premier e da First Division sediados em Leinster (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone ou Kilkenny), da vaga 4 em diante',
          'rule: the Leinster Senior Cup takes 16 clubs: four of the Leinster Senior League (the best, shuffled among the top twelve), entries 0 to 3, and the Premier and First Division clubs based in Leinster (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny), from entry 4 on'),
         ('ire_leinster_cup_start_new_season', 'IreLeinsterCup::start_new_season', '0x636c20', 'ire_leinster_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa de Leinster é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os clubes, as regras, a árvore dos confrontos e as posições previstas são feitos de novo',
          "rule: at the season turn the Leinster Senior Cup is rebuilt: last edition's tables are freed, the statistics archived, the year goes up by one and the clubs, the rules, the tie tree and the predicted positions are made again")])
CAMPOS = []
INTERESSA = []
