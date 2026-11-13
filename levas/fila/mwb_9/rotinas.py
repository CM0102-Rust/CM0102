# A Second Division escocesa (sco_second.cpp): o calendário e a virada. Depende do mwb_5 (as divisões), do mwb_6 (ScottishSchedule, os casos do fim do ano, british_division_new_season) e do mwb_7 (SCOTTISH_SECOND_DIVISION): integrar junto ou depois deles.
# Leitor mwb (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x7f3540 (ScoSecond::create_schedule): 36 rodadas de 7/8 a 6/5 (as da Second e da Third são as mesmas); a do fim do ano
#   pelo dia da semana, como na First; só a fase principal. Caixas: 0xa5 e 0x109; virada 0x1c3, 0x1ca, 0x1d0, 0x1d7.
# - 0x7f4030 (ScoSecond::start_new_season): a virada; roda uma vez, no meio de 2002.
# Convenções conferidas: como as do mwb_6 (a Second e a Third não liberam o vetor de fases nem têm o caso de 1999).
# Comparadores: o calendário lado a lado; a virada só a original (o ano subiu um).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_second_create_schedule', 0x7f3540, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('sco_second_start_new_season', 0x7f4030, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Second Division escocesa: o calendário e a virada',
        'the Scottish Second Division: the schedule and the turnover',
        [('sco_second_create_schedule', 'ScoSecond::create_schedule', '0x7f3540', 'sco_second.cpp', 'thiscall, ret 0x10',
          'regra: 36 rodadas (10 clubes, 4 turnos), aos sábados de 7/8 a 6/5 do ano seguinte: 18 antes do fim do ano (nenhuma em 9/10 e 11/12), a do fim do ano em 26, 27 ou 28/12 conforme o dia da semana e 17 depois (3/1 no próprio dia, nenhuma em 8/1); só a fase principal',
          'rule: 36 rounds (10 clubs, 4 rounds), Saturdays from 7/8 to 6/5 of the next year: 18 before the year end (none on 9/10 and 11/12), the year-end round on 26, 27 or 28/12 by the weekday and 17 after (3/1 on its own day, none on 8/1); only the main stage'),
         ('sco_second_start_new_season', 'ScoSecond::start_new_season', '0x7f4030', 'sco_second.cpp', 'thiscall, ret 0',
          'regra: na virada a Second é refeita: arquiva a temporada, libera tabela, rodadas e fases, passa ao ano seguinte, refaz regras, tabela, jogos e ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: at the turnover the Second Division is rebuilt: the season is archived, the table, rounds and stages freed, the year moves on, the rules, the table, the fixtures and the order are rebuilt; then the predicted positions, the board expectations and the renewal of the referees')])
