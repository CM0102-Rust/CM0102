# A Copa da Liga da Irlanda do Norte (nir_lge_cup.cpp, em cups\): os grupos, os registros dos técnicos e as posições
# previstas. Depende do mwa_23 (irish_league_cup_group_stage, compare_irish_league_cup_group_stage). Leitor r4_mwab
# (20/11/2026), lista do mundo (Irlanda e Irlanda do Norte), continuação.
# - 0x78d3c0 (NirLgeCup::create_group_stage): o molde do mwa_23 com 5 clubes fixos e a lista local de vagas (2, 0, 0,
#   0); a varredura testa a contagem antes da primeira entrada (o mesmo para um grupo de cinco).
# - 0x78d9c0 (NirLgeCup::update_staff_records_for_status): welsh_cup_staff_records com 3 fases de grupo e 3 rodadas
#   (same.py contra 0x9271e0: 4 diferenças).
# - 0x78dcd0 (NirLgeCup::set_predicted_positions): italian_cup_predicted_positions com a fase 3 e faixas de 4 (same.py
#   contra 0x638420: só registros trocados, arquivo, linhas, fase e faixa).
# Convenções conferidas: como as do mwa_23.
# Comparadores: como os do mwa_23.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_lge_cup_create_group_stage', 0x78d3c0, [0x6a, 0xff, 0x68, 0x5e, 0xe4, 0x95, 0x00]),
           ('nir_lge_cup_update_staff_records_for_status', 0x78d9c0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('nir_lge_cup_set_predicted_positions', 0x78dcd0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga da Irlanda do Norte: os grupos, os registros dos técnicos e as posições previstas',
        "the Northern Irish League Cup: the groups, the managers' records and the predicted positions",
        [('nir_lge_cup_create_group_stage', 'NirLgeCup::create_group_stage', '0x78d3c0', 'nir_lge_cup.cpp', 'thiscall, ret 4',
          'regra: os grupos 2 a 4 da Copa da Liga da Irlanda do Norte (fases 0 a 2; a tabela da própria copa é o primeiro): 5 clubes cada, os próximos inscritos do bolo na ordem, numa fase de liga própria com duas vagas; depois a notícia do sorteio do grupo',
          "rule: the Northern Irish League Cup's groups 2 to 4 (stages 0 to 2; the cup's own table is the first): 5 clubs each, the next entrants of the pool in order, in a league stage of their own with two places through; then the group draw news"),
         ('nir_lge_cup_update_staff_records_for_status', 'NirLgeCup::update_staff_records_for_status', '0x78d9c0', 'nir_lge_cup.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos (fases abaixo de 3), quem passa ganha o registro de classificação (peso 40) e quem sai o de eliminação nos grupos (peso 40); no mata-mata (fase 3), quem passa ganha a classificação pela rodada e quem cai o de eliminado (peso 50 a 100 em 3 rodadas); o campeão e o vice, os registros da final',
          'rule: in the groups (stages below 3), a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 3) a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds); the winner and the runner-up the final records'),
         ('nir_lge_cup_set_predicted_positions', 'NirLgeCup::set_predicted_positions', '0x78dcd0', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas dos 20 inscritos, pela reputação: o mata-mata (fase 3) dá 1 ao campeão e 2 ao vice, depois uma posição a cada leva de eliminados; os outros em faixas de 4 (a expectativa da diretoria e da imprensa [inferência])',
          "rule: the 20 entrants' predicted positions, by reputation: the knockout (stage 3) gives 1 to the winner and 2 to the runner-up, then one place to each round's losers; the rest in bands of 4 (the board and press expectation [inference])")])
CAMPOS = []
INTERESSA = []
