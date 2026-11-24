# A minicopa dos amistosos: o construtor, a tabela de rodadas e os quatro clubes (mini_cup.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao, carreira ENG (sem LIGAS): os torneios de pre-temporada de julho de 2001 criam as minicopas
# (Friendly::process_tournament_replies 0x5c5820, original; 3 construcoes, 3 tabelas e 3 listas de clubes na cobertura
# ENG ate 2002-06-01).
# - 0x765bf0 MiniCup::MiniCup thiscall(clubes, registro, inicio, ano, lados, regras) ret 0x18: Cup::Cup 0x518640 (ja
#   em Rust pelo cx01_1, se integrado: chamada pelo endereco), Date::Date 0x54a110 e Date::operator= 0x417c10
#   (thiscall), set_up_clubs (call direto), operator new 0x944cf1 e Comp_Stats::Comp_Stats 0x49ee70 (push this; mov
#   ecx -> thiscall, ret 4), a virtual +0x3c (4 pushes -> thiscall, ret 0x10), Cup::map_fixture_tree 0x518790
#   (thiscall). Prologo push -1; push imm32 (7 bytes).
# - 0x765f70 MiniCup::create_round_table thiscall(fase, &rodadas, &extra, &liga) ret 0x10 (virtual +0x3c): malloc,
#   Date::get_weekday 0x54a3f0 (thiscall), cup_set_round_match_day 0x521f10 (7 pushes; add esp -> cdecl), Date::
#   add_days 0x54c5b0 (2 pushes -> thiscall, ret 8) e Date::operator=. Prologo mov eax, fs:[0] (6 bytes).
# - 0x766290 MiniCup::set_up_clubs thiscall(clubes) ret 4. Prologo sub esp, 0x200 (6 bytes).
# O registro da minicopa: +0xb2 a data de inicio (8 bytes), +0xba a palavra das regras de desempate (copiada na
# TIE_RULES de cada rodada; a semi com 0x100 a mais).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original nas tres (alocam), com o plano: a vtable, o registro, o ano, 4 times e 3 rodadas; os
# codigos 0x82, 0x96 e 0x8c e a semi no dia do inicio; a tabela de times com os clubes na ordem.
SAVE = False
ROTINAS = [('construct_mini_cup', 0x765bf0, [0x6a, 0xff, 0x68, 0x5a, 0xda, 0x95, 0x00]),
           ('mini_cup_create_round_table', 0x765f70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('mini_cup_set_up_clubs', 0x766290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a minicopa dos amistosos', 'the friendly mini cup',
        [('construct_mini_cup', 'MiniCup::MiniCup', '0x765bf0', 'mini_cup.cpp', 'thiscall, ret 0x18',
          'regra: a minicopa de um torneio de amistosos tem quatro clubes, 9 reservas e 9 substituições, o estádio do clube-sede e a data de início do torneio; monta os clubes, as estatísticas, a tabela de rodadas e a árvore de confrontos',
          'rule: a friendly tournament\'s mini cup has four clubs, 9 substitutes named and 9 allowed, the host club\'s ground and the tournament\'s start date; it sets up the clubs, the statistics, the round table and the fixture tree'),
         ('mini_cup_create_round_table', 'MiniCup::create_round_table', '0x765f70', 'mini_cup.cpp', 'thiscall, ret 0x10',
          'regra: a minicopa tem três rodadas: as semifinais no dia do início (dois jogos no dia da semana da data), a final e a disputa do terceiro lugar no dia seguinte, em jogo único',
          'rule: the mini cup has three rounds: the semi-finals on the start day (two matches on that weekday), the final and the third-place match the next day, single matches'),
         ('mini_cup_set_up_clubs', 'MiniCup::set_up_clubs', '0x766290', 'mini_cup.cpp', 'thiscall, ret 4',
          'regra: a tabela de times da minicopa tem os quatro clubes do torneio, na ordem, sem cabeça de chave nem situação',
          'rule: the mini cup\'s team table has the tournament\'s four clubs, in order, with no seed nor status')])
CAMPOS = [('minicopa +0xb2 data de início (8 bytes), +0xba palavra das regras de desempate; lê e grava', 'mini cup +0xb2 start date (8 bytes), +0xba tie rules word; reads and writes')]
INTERESSA = []
