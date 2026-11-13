# A virada da liga coreana (K League) e as regras da liga e da Copa da Liga coreana (kor_league.cpp,
# kor_league_cup.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01; docs/cobertura_mundo.txt: a virada 1
# vez; as regras 2 vezes, no construtor no começo do jogo e na virada).
# - 0x66bd60 (KorLeague::start_new_season): corpo próprio: as posições finais (virtual +0xa4) antes de tudo; não mexe
#   nas fases nem em +0x30 (a K League não tem fases); depois o molde (regras, tabela 0x66bba0, jogos, ordem, previstas,
#   expectativas, árbitros).
# - 0x66bcc0 (KorLeague::set_up_rules) e 0x66c870 (KorLeagueCup::set_up_rules): os campos das regras e a tabela de
#   rodadas da fase −1 (virtual +0x3c), no molde das regras da Série B (set_serie_b_rules / create_main_schedule).
# Convenções: virtuais +0x08 e +0x8c (mov ecx, esi; call [eax+n], sem push) -> thiscall, ret 0; dentro: a virtual
# +0x3c (4 pushes -> thiscall, ret 0x10), a +0xa4 (sem push -> thiscall, ret 0).
# Comparadores: a virada só a original (ano +1); as regras lado a lado sobre o objeto sem as saídas da tabela de
# rodadas (+0x3a, +0xa9, +0xba: league_regions_without_schedule).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_league_start_new_season', 0x66bd60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_set_up_rules', 0x66bcc0, [0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('kor_league_cup_set_up_rules', 0x66c870, [0x56, 0x8b, 0xf1, 0x33, 0xc0])]
DOCS = ('a virada da liga coreana e as regras da liga e da Copa da Liga coreana',
        'the Korean league turnover and the rules of the Korean league and League Cup',
        [('kor_league_start_new_season', 'KorLeague::start_new_season', '0x66bd60', 'kor_league.cpp', 'thiscall, ret 0',
          'regra: a virada da temporada da K League grava antes as posições finais, arquiva a temporada, limpa a tabela e as datas, passa ao ano seguinte e refaz as regras, a tabela com os dez clubes, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: the K League season turnover first records the final positions, archives the season, clears the table and the dates, moves to the next year and rebuilds the rules, the table with the ten clubs, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('kor_league_set_up_rules', 'KorLeague::set_up_rules', '0x66bcc0', 'kor_league.cpp', 'thiscall, ret 0',
          'regra: a K League é um returno triplo (3 turnos), 3 pontos a vitória e 1 o empate, desempate 1, 2 e 3, sem acesso nem rebaixamento, clubes, categoria de liga, 7 reservas e 3 trocas, números fixos; depois o calendário',
          'rule: the K League is a triple round robin (3 legs), 3 points a win and 1 a draw, tiebreakers 1, 2 and 3, no promotion or relegation, club sides, league category, 7 substitutes and 3 changes, fixed squad numbers; then the schedule'),
         ('kor_league_cup_set_up_rules', 'KorLeagueCup::set_up_rules', '0x66c870', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Liga coreana são turno e returno, 3 pontos a vitória e 1 o empate, desempate 1 e 2, os dois primeiros de cada grupo passam, clubes, categoria de copa (4), bits de jogo 0x83 e prêmio de 100 mil; depois o calendário dos grupos',
          'rule: the Korean League Cup groups are a double round robin, 3 points a win and 1 a draw, tiebreakers 1 and 2, the top two of each group go through, club sides, cup category (4), match rule bits 0x83 and a prize fund of 100,000; then the group schedule')])
CAMPOS = [('liga +0x3c, +0x42, +0x49, +0x4a, +0x52, +0x1c, +0x20, +0x28 (copa), +0xab, +0xdb, +0xbe..+0xc7 (regras); valor; grava',
           'league +0x3c, +0x42, +0x49, +0x4a, +0x52, +0x1c, +0x20, +0x28 (cup), +0xab, +0xdb, +0xbe..+0xc7 (rules); value; writes')]
