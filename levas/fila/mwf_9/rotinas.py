# As viradas de temporada das copas jogadas como liga com fases de grupo: a Copa da Série C, a Copa da Itália e a Copa
# da Liga coreana (ita_c_cup.cpp, ita_cup.cpp, kor_league_cup.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até
# 2002-08-01): cada uma roda 1 vez (docs/cobertura_mundo.txt): as italianas na virada italiana de meados de 2002, a
# coreana na virada coreana.
# - 0x63d780 (ItaCCup::start_new_season), 0x63f3e0 (ItaCup::start_new_season) e 0x66c900
#   (KorLeagueCup::start_new_season): o mesmo corpo (as italianas conferidas instrução por instrução pelo norm.py do
#   leitor mwf; a coreana pela leitura: só usa ebx em vez de edi como o zero; mudam as rotinas, o número de grupos, 15,
#   7 e 1, o arquivo e as linhas): libera +0xee sem zerar (a rotina dos participantes aloca outra), os participantes
#   (sem conferir o retorno), as posições previstas, as regras, o grupo 1, os jogos, a ordem e as fases de grupo 0..n-1.
# Convenções: virtual +0x08 -> thiscall, ret 0; create_group_stage / create_group_b_stage: push g; mov ecx, esi ->
# thiscall, ret 4; as outras, mov ecx, esi; call -> thiscall, ret 0; League::sort_table push 0 -> thiscall, ret 4.
# Comparador: só a original (libera e refaz), com o ano +1 (compare_state_league_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ita_c_cup_start_new_season', 0x63d780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_cup_start_new_season', 0x63f3e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_cup_start_new_season', 0x66c900, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas da Copa da Série C, da Copa da Itália e da Copa da Liga coreana',
        'the season turnovers of the Serie C Italian Cup, the Coppa Italia and the Korean League Cup',
        [('ita_c_cup_start_new_season', 'ItaCCup::start_new_season', '0x63d780', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa da Série C arquiva a temporada, descarta as fases e a lista de participantes, passa ao ano seguinte, sorteia de novo os 90 clubes da Série C, as posições previstas, as regras, o grupo 1 de 5, os seus jogos e ordem e as 15 fases de grupo',
          'rule: the Serie C cup turnover archives the season, drops the stages and the entrant list, moves to the next year, draws the 90 Serie C clubs again, the predicted positions, the rules, the first group of 5, its fixtures and order, and the 15 group stages'),
         ('ita_cup_start_new_season', 'ItaCup::start_new_season', '0x63f3e0', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa da Itália é a da Copa da Série C, com os 48 participantes, o grupo 1 de 4 e 7 fases de grupo',
          "rule: the Coppa Italia turnover is the Serie C cup's, with the 48 entrants, the first group of 4 and 7 group stages"),
         ('kor_league_cup_start_new_season', 'KorLeagueCup::start_new_season', '0x66c900', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa da Liga coreana é a da Copa da Série C, com os dez clubes da K League escolhidos de novo (pela posição da última temporada), o grupo A (as cabeças pares, a própria copa) e a fase do grupo B (as ímpares)',
          "rule: the Korean League Cup turnover is the Serie C cup's, with the ten K League clubs selected again (by last season's position), group A (the even-numbered seeds, the cup itself) and the group B stage (the odd-numbered seeds)")])
CAMPOS = [('liga +0x4c, +0xb1, +0xba, +0x30, +0x40; +0xee liberado sem zerar; valor/ponteiro; grava',
           'league +0x4c, +0xb1, +0xba, +0x30, +0x40; +0xee freed without zeroing; value/pointer; writes')]
