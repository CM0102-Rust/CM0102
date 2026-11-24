# A minliga dos amistosos: o construtor, o calendario e a tabela (mini_league.cpp; leitor cx05, rodada 5, 24/11/2026).
# Simulacao, carreira ENG (sem LIGAS): os torneios de pre-temporada de julho de 2001 criam as minligas
# (Friendly::process_tournament_replies 0x5c5820, original; 5 construcoes, 5 calendarios e 5 tabelas na cobertura ENG
# ate 2002-06-01).
# - 0x766500 MiniLeague::MiniLeague thiscall(n, clubes, registro, inicio, vitoria, empate, desempates, acesso, ano,
#   lados, regras) ret 0x2c: League::League 0x682200, Date::Date 0x54a110 e Date::operator= 0x417c10 (thiscall), a
#   virtual +0x3c, alloc_table (call direto), League::create_fixtures 0x6835c0, League::sort_table 0x6827d0 (push 0),
#   operator new e Comp_Stats::Comp_Stats 0x49ee70, League::set_predicted_positions 0x68a850 (thiscall). Prologo push
#   -1; push imm32 (7 bytes).
# - 0x766970 MiniLeague::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10 (virtual +0x3c): malloc,
#   Date::get_weekday 0x54a3f0, league_round_set_match_slot 0x68a1c0 (7 pushes; add esp -> cdecl), Date::add_days
#   0x54c5b0 e Date::operator=. Prologo mov eax, fs:[0] (6 bytes).
# - 0x766bf0 MiniLeague::alloc_table thiscall(clubes) ret 4: malloc e League::init_table_row 0x687430 (4 pushes;
#   mov ecx -> thiscall, ret 0x10). Prologo sub esp, 0x204 (6 bytes).
# O registro da minliga: +0xee a data de inicio (8 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_12 (venue_of_club).
# Comparadores: so a original nas tres (alocam), com o plano: a vtable, o registro, o ano, os clubes, uma volta e 3
# rodadas; as rodadas no dia do inicio e nos seguintes; as linhas com os clubes na ordem.
SAVE = False
ROTINAS = [('construct_mini_league', 0x766500, [0x6a, 0xff, 0x68, 0xfa, 0xda, 0x95, 0x00]),
           ('mini_league_create_schedule', 0x766970, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('mini_league_alloc_table', 0x766bf0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a minliga dos amistosos', 'the friendly mini league',
        [('construct_mini_league', 'MiniLeague::MiniLeague', '0x766500', 'mini_league.cpp', 'thiscall, ret 0x2c',
          'regra: a minliga de um torneio de amistosos joga em turno único, com os pontos e os desempates do torneio, 9 reservas e 9 substituições, o estádio do clube-sede e a data de início do torneio; monta a tabela, os jogos, as estatísticas e as posições previstas',
          'rule: a friendly tournament\'s mini league is a single round robin, with the tournament\'s points and tie-breaks, 9 substitutes named and 9 allowed, the host club\'s ground and the tournament\'s start date; it builds the table, the fixtures, the statistics and the predicted positions'),
         ('mini_league_create_schedule', 'MiniLeague::create_schedule', '0x766970', 'mini_league.cpp', 'thiscall, ret 0x10',
          'regra: a minliga tem três rodadas em dias seguidos a partir do início, com dois jogos por rodada no dia da semana da data',
          'rule: the mini league has three rounds on consecutive days from the start, two matches a round on that weekday'),
         ('mini_league_alloc_table', 'MiniLeague::alloc_table', '0x766bf0', 'mini_league.cpp', 'thiscall, ret 4',
          'regra: a tabela da minliga tem uma linha por clube, na ordem da lista',
          'rule: the mini league table has a row per club, in list order')])
CAMPOS = [('minliga +0xee data de início (8 bytes); lê e grava', 'mini league +0xee start date (8 bytes); reads and writes')]
INTERESSA = []
