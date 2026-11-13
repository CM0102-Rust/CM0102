# As viradas de temporada de três ligas, no molde da Série B brasileira (bra_nat_second_start_new_season, já em
# Rust: o mesmo código de máquina, só mudam o arquivo das caixas, as linhas e a rotina da tabela; conferido instrução
# por instrução pelo norm.py do leitor mwf). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01): cada uma roda
# 1 vez, na virada da temporada da sua nação (meados de 2002).
# Convenções: virtual +0x08 (start_new_season) chamada pela primeira divisão da nação -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a liga), com o ano +1 (compare_state_league_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fra_second_start_new_season', 0x5bf160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fra_third_start_new_season', 0x5c07c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c1a_start_new_season', 0x64b820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas da Segunda Divisão e do National franceses e da Série C1 A',
        'the season turnovers of the French Second Division, the French National and Serie C1 A',
        [('fra_second_start_new_season', 'FraSecond::start_new_season', '0x5bf160', 'fra_second.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Segunda Divisão francesa arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (FraSecond::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover the French Second Division archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (FraSecond::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('fra_third_start_new_season', 'FraThird::start_new_season', '0x5c07c0', 'fra_third.cpp', 'thiscall, ret 0', 'regra: a virada da temporada do National francês arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (FraThird::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover the French National archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (FraThird::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('ita_ser_c1a_start_new_season', 'ItaSerC1a::start_new_season', '0x64b820', 'ita_ser_c1a.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Série C1 A arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (ItaSerC1a::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover Serie C1 A archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (ItaSerC1a::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('liga +0x4c, +0xb1, +0xba, +0x30, +0x40 (a virada); valor/ponteiro; grava', 'league +0x4c, +0xb1, +0xba, +0x30, +0x40 (the turnover); value/pointer; writes')]
