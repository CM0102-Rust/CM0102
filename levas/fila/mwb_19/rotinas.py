# A Copa da Liga galesa (wel_lge_cup.cpp): as honrarias, os registros dos técnicos e os grupos; os moldes dos registros
# e dos grupos das duas copas galesas (welsh_cup_staff_records, welsh_cup_group_stage, usados pelo mwb_21). Depende do
# mwb_18 (WELSH_CUP_POOL). Leitor mwb (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x926890 (WelLgeCup::record_season_honours): repassa à fase do mata-mata (a 6a vaga, virtual +0x30).
# - 0x9271e0 (WelLgeCup::update_staff_records_for_status): os registros por resultado e fase (pesos 40 nos grupos, 50 a
#   100 no mata-mata).
# - 0x926b00 (WelLgeCup::create_group_stage): os grupos 0 a 4, de 3 clubes, como League_Stage (0x68b100).
# Convenções conferidas: as honrarias thiscall, ret 0 (pulo para a virtual +0x30 da fase, com o mesmo ecx dela); os
# registros thiscall, ret 0x18 (0x86bdd0, 0x86c1d0, 0x86c000, 0x86a800, 0x86b0b0 thiscall sobre [0xdd7f00]; 0x4bf850
# cdecl); o grupo thiscall, ret 4 (a virtual +0x3c push 0; push &extra; push &rodadas; push g -> ret 0x10; 0x944e46 e
# 0x944cf1 cdecl; 0x68b100 thiscall com 21 argumentos, ret 0x58; 0x684230 com ecx = a fase).
# Comparadores: as honrarias e os registros só a original (o histórico); o grupo só a original, com o plano (a fase na
# vaga, o +0x30 e o bolo zerado).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_lge_cup_record_season_honours', 0x926890, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x14]),
           ('wel_lge_cup_update_staff_records_for_status', 0x9271e0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('wel_lge_cup_create_group_stage', 0x926b00, [0x6a, 0xff, 0x68, 0x7e, 0x5d, 0x96, 0x00])]
DOCS = ('a Copa da Liga galesa: as honrarias, os registros dos técnicos e os grupos',
        "the Welsh League Cup: the honours, the managers' records and the groups",
        [('wel_lge_cup_record_season_honours', 'WelLgeCup::record_season_honours', '0x926890', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: as honrarias da temporada da Copa da Liga galesa são as do seu mata-mata',
          "rule: the Welsh League Cup's season honours are those of its knockout"),
         ('wel_lge_cup_update_staff_records_for_status', 'WelLgeCup::update_staff_records_for_status', '0x9271e0', 'wel_lge_cup.cpp', 'thiscall, ret 0x18',
          'regra: nos seis grupos, quem passa ganha o registro de classificação (peso 40) e quem sai o de eliminação nos grupos (peso 40); no mata-mata (fase 5), quem passa ganha a classificação pela rodada e quem cai o de eliminado (peso 50 a 100 em 5 rodadas); o campeão e o vice, os registros da final',
          'rule: in the six groups, a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 5) a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 5 rounds); the winner and the runner-up the final records'),
         ('wel_lge_cup_create_group_stage', 'WelLgeCup::create_group_stage', '0x926b00', 'wel_lge_cup.cpp', 'thiscall, ret 4',
          'regra: o grupo g (0 a 4) tem 3 clubes, os da coluna g + 1 do bolo de inscritos, numa fase de liga própria; depois a notícia do sorteio do grupo',
          'rule: group g (0 to 4) has 3 clubs, those of column g + 1 of the entrant pool, in a league stage of its own; then the group draw news')])
