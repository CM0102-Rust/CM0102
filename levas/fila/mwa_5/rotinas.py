# The Northern Irish Charity Shield (nir_charity.cpp, in cups\): the round table, the two clubs and the season
# turnover. Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01.
# - 0x78a0a0 (NirCharity::create_round_table, 2 calls): the single-stage round table template (the head checked
#   instruction by instruction; the one round read by the symbolic reader).
# - 0x78a1e0 (NirCharity::select_teams, 2 calls): SpaSuper::select_teams's template (same.py against 0x858aa0: only the
#   two key globals and the file change), over its helpers (alloc_two_team_list, set_team_slot, finish_two_team_list).
# - 0x78a410 (NirCharity::start_new_season, 1 call): the turnover without predicted positions
#   (usa_mls_all_stars_cup_start_new_season with false).
# The 2001 edition is played on 5/8/2001 (inside the career); the 2002 one, made by the turnover, after the end date:
# the turnover mutation is caught by the comparer (the year), not by the career.
# Conventions checked at the call sites: create_round_table the virtual +0x3c (thiscall, ret 0x10); select_teams (mov
# ecx, esi; call -> thiscall, ret 0); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x944e46 (cdecl),
# 0x4afd80 comp_util_get_latest_placed_club (push place; push comp; add esp, 8 -> cdecl), the error box.
# Comparers: the round table side by side; the two clubs side by side (compare_team_list, the 12 bytes); the turnover
# only the original, with the plan.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_charity_create_round_table', 0x78a0a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_charity_select_teams', 0x78a1e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_charity_start_new_season', 0x78a410, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supercopa da Irlanda do Norte (Charity Shield): a tabela da rodada, os dois clubes e a virada da temporada',
        'the Northern Irish Charity Shield: the round table, the two clubs and the season turnover',
        [('nir_charity_create_round_table', 'NirCharity::create_round_table', '0x78a0a0', 'nir_charity.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Irlanda do Norte é um jogo único entre dois clubes, sorteado em 5/7 e jogado em 5/8 (levado ao dia da semana 5) no campo do primeiro [inferência: código de mando 0], com os bits de regra 0x200 (sem notícia do sorteio [inferência]); categoria 6; sem prêmio',
          'rule: the Northern Irish Charity Shield is a single match between two clubs, drawn on 5/7 and played on 5/8 (moved to weekday 5) at the first club\'s ground [inference: home side code 0], with tie rule bits 0x200 (no draw news [inference]); category 6; no prize'),
         ('nir_charity_select_teams', 'NirCharity::select_teams', '0x78a1e0', 'nir_charity.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Irlanda do Norte é entre o campeão da Copa da Irlanda (do Norte) e o campeão da Premier Division; se o mesmo clube ganhou as duas, o vice da Copa entra no lugar; sem um deles, a caixa de erro',
          'rule: the Northern Irish Charity Shield is between the Irish Cup winner and the Premier Division champion; if the same club won both, the Irish Cup runner-up takes its place; if one is missing, the error box'),
         ('nir_charity_start_new_season', 'NirCharity::start_new_season', '0x78a410', 'nir_charity.cpp', 'thiscall, ret 0',
          'regra: na virada a Supercopa da Irlanda do Norte é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os dois clubes, as regras e a árvore dos confrontos são feitos de novo (sem posições previstas)',
          "rule: at the season turn the Northern Irish Charity Shield is rebuilt: last edition's tables are freed, the statistics archived, the year goes up by one and the two clubs, the rules and the tie tree are made again (no predicted positions)")])
CAMPOS = []
INTERESSA = []
