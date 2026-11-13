# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Usa create_league_schedule e compare_cfa_schedule (ja no motor); nao depende do mwc_1. A mutacao mexe na tabela do ano impar (2001/02).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('den_first_create_schedule', 0x5519b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('den_second_create_schedule', 0x554170, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('den_prm_create_schedule', 0x552900, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários das três divisões dinamarquesas (ano ímpar e par)',
        'the schedules of the three Danish divisions (odd and even years)',
        [('den_first_create_schedule', 'DenFirst::create_schedule', '0x5519b0', 'den_first.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a primeira divisão dinamarquesa: 30 rodadas, de 3/8 a 24/5; 0 com jogos em faixas de TV; a primavera muda com a paridade do ano da temporada (a partir da rodada 15: ímpar, como 2001/02, e par)', 'rule: the Danish First Division schedule: 30 rounds, from 3 Aug to 24 May; 0 with matches in TV slots; the spring changes with the parity of the season year (from round 15: odd, as 2001/02, and even)'),
         ('den_second_create_schedule', 'DenSecond::create_schedule', '0x554170', 'den_second.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a segunda divisão dinamarquesa: 30 rodadas, de 2/8 a 31/5; 0 com jogos em faixas de TV; a primavera muda com a paridade do ano da temporada (a partir da rodada 15: ímpar, como 2001/02, e par)', 'rule: the Danish Second Division schedule: 30 rounds, from 2 Aug to 31 May; 0 with matches in TV slots; the spring changes with the parity of the season year (from round 15: odd, as 2001/02, and even)'),
         ('den_prm_create_schedule', 'DenPrm::create_schedule', '0x552900', 'den_prm.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a Superliga dinamarquesa: 33 rodadas, de 27/7 a 24/5; 30 com jogos em faixas de TV; a primavera muda com a paridade do ano da temporada (a partir da rodada 18: ímpar, como 2001/02, e par)', 'rule: the Danish Superliga schedule: 33 rounds, from 27 Jul to 24 May; 30 with matches in TV slots; the spring changes with the parity of the season year (from round 18: odd, as 2001/02, and even)')])
