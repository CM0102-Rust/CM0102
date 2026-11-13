# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Usa create_league_schedule e compare_cfa_schedule (ja no motor); nao depende do mwc_1.
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('bel_first_create_schedule', 0x41e010, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_second_create_schedule', 0x5de5d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_prm_create_schedule', 0x5f78d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da primeira belga, da segunda Bundesliga e da Eredivisie',
        'the schedules of the Belgian First Division, the 2. Bundesliga and the Eredivisie',
        [('bel_first_create_schedule', 'BelFirst::create_schedule', '0x41e010', 'bel_first.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a primeira divisão belga: 34 rodadas, de 12/8 a 20/5; 24 com jogos em faixas de TV', 'rule: the Belgian First Division schedule: 34 rounds, from 12 Aug to 20 May; 24 with matches in TV slots'),
         ('ger_second_create_schedule', 'GerSecond::create_schedule', '0x5de5d0', 'ger_second.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a segunda Bundesliga: 34 rodadas, de 30/7 a 6/5; 28 com jogos em faixas de TV', 'rule: the 2. Bundesliga schedule: 34 rounds, from 30 Jul to 6 May; 28 with matches in TV slots'),
         ('hol_prm_create_schedule', 'HolPrm::create_schedule', '0x5f78d0', 'hol_prm.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a primeira divisão holandesa (Eredivisie): 34 rodadas, de 14/8 a 14/5; 32 com jogos em faixas de TV', 'rule: the Dutch Eredivisie schedule: 34 rounds, from 14 Aug to 14 May; 32 with matches in TV slots')])
