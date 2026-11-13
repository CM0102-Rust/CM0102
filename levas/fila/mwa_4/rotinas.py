# The Irish President's Cup (ire_pres_cup.cpp, in cups\): the round table, the two clubs and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_1's helpers
# (irish_club_division_id, compare_irish_cup_entrants) and mwa_2's ire_club_is_in_leinster: it goes in the same batch
# or after them.
# - 0x639530 (IrePresCup::create_round_table, 2 calls): the single-stage round table template (the head checked
#   instruction by instruction; the one round read by the symbolic reader; the original writes the same fields in
#   another order, nothing overlaps).
# - 0x639670 (IrePresCup::collect_entrants, 2 calls).
# - 0x639820 (IrePresCup::start_new_season, 1 call): the turnover without predicted positions
#   (usa_mls_all_stars_cup_start_new_season with false; same.py against 0x90a760: only the entrant routine, the file
#   and the lines).
# The 2001 edition is played on 4/8/2001 (inside the career); the 2002 one, made by the turnover, on 4/8/2002 (after
# the end date): the turnover mutation is caught by the comparer (the year), not by the career.
# Conventions checked at the call sites: create_round_table the virtual +0x3c (thiscall, ret 0x10); collect_entrants
# (mov ecx, esi; call -> thiscall, ret 0); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x945501 calloc
# (push 6; push 2; add esp, 8 -> cdecl), 0x6399c0 (cdecl), the error box.
# Comparers: the round table side by side; the two clubs side by side (club, class and status); the turnover only the
# original, with the plan.
# Determinism: nothing reads stack, heap or an address as a value (the position table is zeroed with rep stosd).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_pres_cup_create_round_table', 0x639530, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_pres_cup_collect_entrants', 0x639670, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('ire_pres_cup_start_new_season', 0x639820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa do Presidente (Irlanda): a tabela da rodada, os dois clubes e a virada da temporada',
        "the Irish President's Cup: the round table, the two clubs and the season turnover",
        [('ire_pres_cup_create_round_table', 'IrePresCup::create_round_table', '0x639530', 'ire_pres_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa do Presidente é um jogo único entre dois clubes, sorteado em 5/7 e jogado em 4/8 (levado ao dia da semana 4) [inferência: código de mando 0x400]; regra de desempate 3 [inferência: pênaltis]; categoria 6; sem prêmio',
          "rule: the President's Cup is a single match between two clubs, drawn on 5/7 and played on 4/8 (moved to weekday 4) [inference: home side code 0x400]; tie rule 3 [inference: penalties]; category 6; no prize"),
         ('ire_pres_cup_collect_entrants', 'IrePresCup::collect_entrants', '0x639670', 'ire_pres_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa do Presidente é entre os dois clubes mais bem colocados da Premier Division da temporada passada sediados em Leinster (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone ou Kilkenny); com menos de dois, os mais bem colocados dos outros completam',
          "rule: the President's Cup is between the two best placed clubs of last season's Premier Division based in Leinster (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny); with fewer than two, the best placed of the others complete it"),
         ('ire_pres_cup_start_new_season', 'IrePresCup::start_new_season', '0x639820', 'ire_pres_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa do Presidente é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os dois clubes, as regras e a árvore dos confrontos são feitos de novo (sem posições previstas)',
          "rule: at the season turn the President's Cup is rebuilt: last edition's tables are freed, the statistics archived, the year goes up by one and the two clubs, the rules and the tie tree are made again (no predicted positions)")])
CAMPOS = []
INTERESSA = []
