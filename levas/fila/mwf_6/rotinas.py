# As viradas de temporada de três divisões inglesas, no molde da Série B brasileira (bra_nat_second_start_new_season, já em
# Rust: o mesmo código de máquina, só mudam o arquivo das caixas, as linhas e a rotina da tabela; conferido instrução
# por instrução pelo norm.py do leitor mwf). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01): cada uma roda
# 1 vez, na virada da temporada da sua nação (meados de 2002).
# Convenções: virtual +0x08 (start_new_season) chamada pela primeira divisão da nação -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a liga), com o ano +1 (compare_state_league_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('eng_first_start_new_season', 0x572e30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_second_start_new_season', 0x576820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_third_start_new_season', 0x578170, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas da First, da Second e da Third Division inglesas',
        'the season turnovers of the English First, Second and Third Divisions',
        [('eng_first_start_new_season', 'EngFirst::start_new_season', '0x572e30', 'eng_first.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da First Division inglesa arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (EngFirst::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover the English First Division archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (EngFirst::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('eng_second_start_new_season', 'EngSecond::start_new_season', '0x576820', 'eng_second.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Second Division inglesa arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (EngSecond::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover the English Second Division archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (EngSecond::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('eng_third_start_new_season', 'EngThird::start_new_season', '0x578170', 'eng_third.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Third Division inglesa arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela (EngThird::create_table), os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros', 'rule: at the season turnover the English Third Division archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table (EngThird::create_table), the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('liga +0x4c, +0xb1, +0xba, +0x30, +0x40 (a virada); valor/ponteiro; grava', 'league +0x4c, +0xb1, +0xba, +0x30, +0x40 (the turnover); value/pointer; writes')]
