# A Copa Premier galesa (wel_prm_cup.cpp): os registros dos técnicos e o segundo grupo. Depende do mwb_19
# (welsh_cup_staff_records, welsh_cup_group_stage) e do mwb_18 (WELSH_CUP_POOL): integrar junto ou depois deles. Leitor
# mwb (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x928af0 (WelPrmCup::update_staff_records_for_status): pesos 40 nos grupos, 50 a 100 no mata-mata (3 rodadas).
# - 0x928450 (WelPrmCup::create_group_stage): o segundo grupo (fase 0), de 4 (inscritos 1, 3, 5 e 7).
# Convenções conferidas: como as do mwb_19.
# Comparadores: como os do mwb_19.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_prm_cup_update_staff_records_for_status', 0x928af0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('wel_prm_cup_create_group_stage', 0x928450, [0x6a, 0xff, 0x68, 0x1e, 0x5e, 0x96, 0x00])]
DOCS = ('a Copa Premier galesa: os registros dos técnicos e o segundo grupo',
        "the Welsh Premier Cup: the managers' records and the second group",
        [('wel_prm_cup_update_staff_records_for_status', 'WelPrmCup::update_staff_records_for_status', '0x928af0', 'wel_prm_cup.cpp', 'thiscall, ret 0x18',
          'regra: nos dois grupos, quem passa ganha o registro de classificação (peso 40) e quem sai o de eliminação nos grupos (peso 40); no mata-mata (fase 1), quem passa ganha a classificação pela rodada e quem cai o de eliminado (peso 50 a 100 em 3 rodadas); o campeão e o vice, os registros da final',
          'rule: in the two groups, a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 1) a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds); the winner and the runner-up the final records'),
         ('wel_prm_cup_create_group_stage', 'WelPrmCup::create_group_stage', '0x928450', 'wel_prm_cup.cpp', 'thiscall, ret 4',
          'regra: o segundo grupo (fase 0) tem 4 clubes, os inscritos 1, 3, 5 e 7 do bolo, numa fase de liga própria; depois a notícia do sorteio do grupo',
          'rule: the second group (stage 0) has 4 clubs, entrants 1, 3, 5 and 7 of the pool, in a league stage of its own; then the group draw news')])
