# As eliminatórias sul-americanas da Copa: a tabela de rodadas, os isentos e as vagas (wc_south_american_league.cpp).
# Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x920590 (WcSouthAmericanLeague::create_schedule), 0x920d10 (WcSouthAmericanLeague::count_exempt_nations),
#   0x920eb0 (WcSouthAmericanLeague::build_team_slots).
# A tabela: a máquina da Copa América (GoldcupScheduleOp, build_phase_table), as listas tiradas da original por
# emulação (unicorn) para as fases −128..127 e os dois casos de isentos (+0xee com sinal >= 2: oito seleções, 14
# rodadas; senão dez, 18 rodadas). Os isentos: as sedes da Copa dois anos depois e o último campeão; o +0xef conta os
# da Oceania (a seção da Oceania o lê). As vagas: 10 − isentos, as seleções sul-americanas em ordem do banco, sem as
# sedes e sem o campeão (a 0x920eb0 não confere o [0xb5e134]; o contador de sedes não é lido).
# Convenções conferidas: thiscall (ret 0x10 a tabela, ret 0 as outras); 0x5fa730 (5 pushes; mov ecx -> thiscall, ret
# 0x14), 0x4afd80 e 0x53b3d0 e 0x944e46 (cdecl), 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10).
# Comparadores: a tabela lado a lado (compare_phase_table); os isentos lado a lado (só os dois contadores); as vagas só a
# original (a tabela e o histórico de posições são alocados), com o plano: o retorno e as linhas sul-americanas.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wc_south_american_league_create_schedule', 0x920590, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('wc_south_american_league_count_exempt_nations', 0x920d10, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('wc_south_american_league_build_team_slots', 0x920eb0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias sul-americanas da Copa: a tabela de rodadas, os isentos e as vagas',
        'the South American World Cup qualifying: the round table, the exempt nations and the team slots',
        [('wc_south_american_league_create_schedule', 'WcSouthAmericanLeague::create_schedule', '0x920590', 'wc_south_american_league.cpp', 'thiscall, ret 0x10', 'regra: as eliminatórias sul-americanas são um turno e returno só (fase −1): com dez seleções, 18 rodadas de 29 de março do ano da campanha a 15 de novembro do seguinte; com dois ou mais isentos, oito seleções e 14 rodadas', 'rule: the South American qualifying is one double round robin (stage −1): with ten nations, 18 rounds from 29 March of the campaign year to 15 November of the next; with two or more exempt, eight nations and 14 rounds'),
         ('wc_south_american_league_count_exempt_nations', 'WcSouthAmericanLeague::count_exempt_nations', '0x920d10', 'wc_south_american_league.cpp', 'thiscall, ret 0', 'regra: ficam isentas das eliminatórias as sedes sul-americanas da Copa seguinte e o último campeão do mundo sul-americano que não seja sede; a mesma conta é guardada para a Oceania', 'rule: the South American hosts of the next World Cup and the last South American world champion that is not a host are exempt from qualifying; the same count is kept for Oceania'),
         ('wc_south_american_league_build_team_slots', 'WcSouthAmericanLeague::build_team_slots', '0x920eb0', 'wc_south_american_league.cpp', 'thiscall, ret 0', 'regra: a tabela tem dez vagas menos as isentas, preenchidas com as seleções sul-americanas na ordem do banco, sem as sedes da Copa seguinte e sem o último campeão', 'rule: the table has ten places minus the exempt ones, filled with the South American national teams in database order, without the hosts of the next World Cup and the last winner')])
CAMPOS = [('eliminatórias sul-americanas +0xee isentas (byte, com sinal), +0xef isentas da Oceania (byte), +0x3e número de seleções, +0xb1 tabela; valor; lê e grava', 'South American qualifying +0xee exempt (byte, signed), +0xef Oceania exempt (byte), +0x3e number of nations, +0xb1 table; value; reads and writes')]
