# Os pontos dos técnicos (staff_records.cpp): a atualização do dia (Staff_Records::daily_update 0x86dda0, chamada todo
# dia pela 0x5cc850/0x5cccd0: zera os pontos do mês na virada e passa os do ano ao hall da fama em 1º de janeiro), o
# fim da temporada da competição (Staff_Records::end_season_for_nation 0x86d6e0, pela 0x5d5ed0: os pontos da liga e da
# copa ao hall da fama) e o técnico do mês (Staff_Records::find_manager_of_the_month 0x86f950: o de mais pontos no mês
# entre os que estão há duas semanas no cargo; 23 chamadas, os prêmios do mês e as telas). Sem SEH, sem sorteio, sem
# leitura de lixo (a data de duas semanas atrás é escrita pela 0x54cab0 antes de lida). Iguais no cm0102-1080.exe.
# Comparadores: as duas primeiras somam no hall da fama (não podem rodar duas vezes): só a original com o plano (os
# campos zerados, o mês e o ano; os quatro campos de cada registro); o técnico do mês pelo valor (o registro que a
# 0x86f5b0 cria na original já existe para a cópia).
SAVE = False
ROTINAS = [('pontos_dos_tecnicos_no_dia', 0x86dda0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('pontos_dos_tecnicos_no_fim_da_competicao', 0x86d6e0, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('tecnico_do_mes_na_liga', 0x86f950, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('os pontos dos técnicos: a virada do mês e do ano, o fim da temporada da competição e o técnico do mês',
        'manager points: the turn of the month and the year, the end of the competition season and the manager of the month',
        [('pontos_dos_tecnicos_no_dia', 'Staff_Records::daily_update', '0x86dda0', 'staff_records.cpp', 'thiscall, ret 4',
          'na virada do mês ou do ano zera os pontos do mês de todos os técnicos e guarda o mês e o ano; no primeiro dia do ano passa os pontos do ano ao hall da fama e os zera',
          'at the turn of the month or year clears every manager\'s monthly points and stores the month and year; on the first day of the year passes the yearly points to the hall of fame and clears them'),
         ('pontos_dos_tecnicos_no_fim_da_competicao', 'Staff_Records::end_season_for_nation', '0x86d6e0', 'staff_records.cpp', 'thiscall, ret 4',
          'no fim da temporada da competição passa ao hall da fama os pontos de liga (com os da copa da mesma competição) e de copa dos técnicos dela e os zera',
          'at the end of the competition\'s season passes its managers\' league points (with the cup points of the same competition) and cup points to the hall of fame and clears them'),
         ('tecnico_do_mes_na_liga', 'Staff_Records::find_manager_of_the_month', '0x86f950', 'staff_records.cpp', 'thiscall, ret 4',
          'o técnico da liga com mais pontos no mês (acima de 0) entre os que estão no cargo há duas semanas; liga sem tabela: a caixa "Error" e 0',
          'the league manager with the most points this month (above 0) among those two weeks in the job; a league without a table: the "Error" box and 0')])
CAMPOS = [('Staff_Records +8: os registros de pontos (0xd2 bytes, [0xae2368] + 16); ponteiro; lê (as três)',
           'Staff_Records +8: the points records (0xd2 bytes, [0xae2368] + 16); pointer; reads (all three)'),
          ('Staff_Records +0x14, +0x16: o mês e o ano guardados; valor; lê e grava (pontos_dos_tecnicos_no_dia)',
           'Staff_Records +0x14, +0x16: the stored month and year; value; reads and writes (pontos_dos_tecnicos_no_dia)'),
          ('registro +0x86: id da pessoa (−1 vazio); id; lê',
           'record +0x86: person id (−1 empty); id; reads'),
          ('registro +0xb6: pontos do ano; valor; lê e grava 0 (pontos_dos_tecnicos_no_dia)',
           'record +0xb6: yearly points; value; reads and writes 0 (pontos_dos_tecnicos_no_dia)'),
          ('registro +0xba/+0xbe e +0xc2/+0xc6: pontos de liga e copa e as competições; valor e id; lê e grava (pontos_dos_tecnicos_no_fim_da_competicao)',
           'record +0xba/+0xbe and +0xc2/+0xc6: league and cup points and their competitions; value and id; reads and writes (pontos_dos_tecnicos_no_fim_da_competicao)'),
          ('registro +0xca/+0xce: pontos do mês; valor; lê e grava 0 (pontos_dos_tecnicos_no_dia, tecnico_do_mes_na_liga)',
           'record +0xca/+0xce: monthly points; value; reads and writes 0 (pontos_dos_tecnicos_no_dia, tecnico_do_mes_na_liga)'),
          ('pessoa +0x39, +0x3e/+0x40: clube e data de entrada no cargo (dia com sinal, ano sem); ponteiro e valor; lê (tecnico_do_mes_na_liga)',
           'person +0x39, +0x3e/+0x40: club and job start date (signed day, unsigned year); pointer and value; reads (tecnico_do_mes_na_liga)'),
          ('liga +0x43, +0x3e, +0xb1: tipo (2 = tabela), conta e vetor da tabela (0x3b bytes); valor e ponteiro; lê (tecnico_do_mes_na_liga)',
           'league +0x43, +0x3e, +0xb1: type (2 = table), count and table array (0x3b bytes); value and pointer; reads (tecnico_do_mes_na_liga)'),
          ('competição +0x11c: guarda no hall da fama; valor; lê (pontos_dos_tecnicos_no_fim_da_competicao)',
           'competition +0x11c: kept in the hall of fame; value; reads (pontos_dos_tecnicos_no_fim_da_competicao)')]
INTERESSA = ['CMRTE']
