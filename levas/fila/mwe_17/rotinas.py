# The Portuguese Super Cup: its teams, its round table and its turnover (reader mwe, whole-world round, 12/11/2026):
# - 0x7d3260 PorSuper::select_teams (560 bytes): at the creation of the game (the constructor) and at the turnover;
#   the French Super Cup's mould (cqm_11), but the double goes to the cup runner-up (not the league runner-up);
# - 0x7d30f0 PorSuper::create_round_table (368 bytes): from set_up_rules (creation and turnover); one round, the
#   year 2000 with its own venue, tie rules and legs (the career starts in 2001: only the other branch runs); the
#   original also writes the word +0x11 = 3, not written by eng_cc_cup_write_cup_round (written apart);
# - 0x7d3490 PorSuper::start_new_season (416 bytes): the knockout cups' turnover (start_new_cup_season) without the
#   predicted positions at the end (the tie tree's eax is returned); in the summer of 2002.
# Equal in cm0102-1080.exe (whole).
# Conventions checked: select_teams direct (mov ecx; call) -> thiscall, ret 0; create_round_table through the virtual
# +0x3c (push &out; push &extra; push &rounds; push -1) -> thiscall, ret 0x10; start_new_season through vtable +8 ->
# thiscall, ret 0. Inside: malloc, free, get_latest_placed_club 0x4afd80 (push place; push record; add esp, 8 ->
# cdecl), 0x521e60 / 0x521eb0 (cdecl), archive_season_and_reset (push 1), the virtuals +0x8c and +0x94.
# Comparers: select_teams side by side (eng_cc_cup_compare_entrant_list), the round table side by side
# (compare_cup_round_table), the turnover only the original (the year + 1).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('por_super_select_teams', 0x7d3260, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_super_create_round_table', 0x7d30f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_super_start_new_season', 0x7d3490, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supertaça portuguesa: os times, a tabela da rodada e a virada',
        'the Portuguese Super Cup: its teams, its round table and its turnover',
        [('por_super_select_teams', 'PorSuper::select_teams', '0x7d3260', 'por_super.cpp', 'thiscall, ret 0',
          'a Supertaça é entre o campeão da primeira divisão e o vencedor da Taça de Portugal; se o mesmo clube ganhou as duas, entra o finalista da Taça; sem um dos clubes, a caixa de erro',
          'the Super Cup is between the Premier Division champion and the Portuguese Cup winner; if the same club won both, the Cup runner-up takes the place; if a club is missing, the error box'),
         ('por_super_create_round_table', 'PorSuper::create_round_table', '0x7d30f0', 'por_super.cpp', 'thiscall, ret 0x10',
          'a Supertaça é uma rodada: sorteio em 5 de julho e um jogo em 12 de agosto em campo neutro [inferência: o local 4] (em 2000, ida e volta nos campos dos clubes); categoria 6; sem prêmios',
          'the Super Cup is one round: draw on 5 July and one match on 12 August at a neutral ground [inference: venue 4] (in 2000, two legs at the clubs\' grounds); category 6; no prizes'),
         ('por_super_start_new_season', 'PorSuper::start_new_season', '0x7d3490', 'por_super.cpp', 'thiscall, ret 0',
          'a virada da Supertaça: soltas as tabelas da edição anterior, a temporada arquivada e o ano + 1; depois os dois times, as rodadas e a árvore dos confrontos (sem posições previstas)',
          'the Super Cup turnover: last edition\'s tables freed, the season archived and the year + 1; then the two teams, the rounds and the tie tree (no predicted positions)')])
CAMPOS = [('rodada de copa +0x11 palavra sem nome = 3 (a Supertaça portuguesa); valor; grava', 'cup round +0x11 unnamed word = 3 (the Portuguese Super Cup); value; writes')]
INTERESSA = []
