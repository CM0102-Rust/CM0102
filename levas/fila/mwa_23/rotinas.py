# A Copa da Liga da Irlanda (ire_lge_cup.cpp, em cups\): os grupos, os registros dos técnicos e as posições previstas;
# o molde dos grupos das copas da liga irlandesas (irish_league_cup_group_stage, usado pelo mwa_24). Leitor r4_mwab
# (20/11/2026), lista do mundo (Irlanda e Irlanda do Norte), continuação.
# - 0x6379c0 (IreLgeCup::create_group_stage): o quadro do ItaCCup::create_group_stage (lido instrução por instrução): o
#   tamanho do grupo é local (3, ou 4 a partir da fase 5), a varredura do bolo para na contagem e uma lista local de
#   vagas (1, 0, 0, 0) entra no lugar do &+0xbe.
# - 0x638110 (IreLgeCup::update_staff_records_for_status): welsh_cup_staff_records (mwb_19) com 7 fases de grupo e 3
#   rodadas no mata-mata (same.py contra 0x9271e0: 4 diferenças, as três constantes e a vaga do mata-mata).
# - 0x638420 (IreLgeCup::set_predicted_positions): italian_cup_predicted_positions (cqx_7) com a fase 7 e faixas de 8
#   (same.py contra 0x63e580: 6 diferenças, arquivo, linhas, fase e faixa).
# Convenções conferidas: o grupo thiscall, ret 4 (a virtual +0x3c push 0; push &rótulo; push &rodadas; push g -> ret
# 0x10; 0x944e46 e 0x944cf1 cdecl; 0x68b100 thiscall, 22 argumentos, ret 0x58; 0x684230 mov ecx, a fase -> ret 0); os
# registros thiscall, ret 0x18; as posições thiscall, ret 0.
# Comparadores: o grupo só a original, com o plano (os inscritos na tabela da fase, o bolo zerado, a fase atual); os
# registros só a original (o histórico), retorno 0; as posições lado a lado (compare_italian_cup_predicted).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_lge_cup_create_group_stage', 0x6379c0, [0x6a, 0xff, 0x68, 0xbe, 0xb4, 0x95, 0x00]),
           ('ire_lge_cup_update_staff_records_for_status', 0x638110, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('ire_lge_cup_set_predicted_positions', 0x638420, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga da Irlanda: os grupos, os registros dos técnicos e as posições previstas',
        "the Irish League Cup: the groups, the managers' records and the predicted positions",
        [('ire_lge_cup_create_group_stage', 'IreLgeCup::create_group_stage', '0x6379c0', 'ire_lge_cup.cpp', 'thiscall, ret 4',
          'regra: os grupos 2 a 8 da Copa da Liga da Irlanda (fases 0 a 6; a tabela da própria copa é o primeiro): 3 clubes nas fases 0 a 4 e 4 nas fases 5 e 6, os próximos inscritos do bolo na ordem, numa fase de liga própria com uma vaga; depois a notícia do sorteio do grupo',
          "rule: the Irish League Cup's groups 2 to 8 (stages 0 to 6; the cup's own table is the first): 3 clubs in stages 0 to 4 and 4 in stages 5 and 6, the next entrants of the pool in order, in a league stage of their own with one place through; then the group draw news"),
         ('ire_lge_cup_update_staff_records_for_status', 'IreLgeCup::update_staff_records_for_status', '0x638110', 'ire_lge_cup.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos (fases abaixo de 7), quem passa ganha o registro de classificação (peso 40) e quem sai o de eliminação nos grupos (peso 40); no mata-mata (fase 7), quem passa ganha a classificação pela rodada e quem cai o de eliminado (peso 50 a 100 em 3 rodadas); o campeão e o vice, os registros da final',
          'rule: in the groups (stages below 7), a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7) a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds); the winner and the runner-up the final records'),
         ('ire_lge_cup_set_predicted_positions', 'IreLgeCup::set_predicted_positions', '0x638420', 'ire_lge_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas dos 26 inscritos, pela reputação: o mata-mata (fase 7) dá 1 ao campeão e 2 ao vice, depois uma posição a cada leva de eliminados; os outros em faixas de 8 (a expectativa da diretoria e da imprensa [inferência])',
          "rule: the 26 entrants' predicted positions, by reputation: the knockout (stage 7) gives 1 to the winner and 2 to the runner-up, then one place to each round's losers; the rest in bands of 8 (the board and press expectation [inference])")])
CAMPOS = []
INTERESSA = []
