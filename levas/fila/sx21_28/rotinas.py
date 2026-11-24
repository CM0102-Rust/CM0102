# Portugal: quem um clube portugues pode contratar e de que oferta pode participar, com o limite de fora da UE pela
# divisao (6 na primeira, 5 na segunda, 4 nas outras) (leitor sx21, rodada 5, 24/11/2026): rodam nas transferencias
# dos clubes portugueses, so com Portugal carregado.
# Convencoes conferidas: 0x7d3ef0 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x7d4040 thiscall(oferta,
# clube, data, motivo) ret 0x10 (guarda o contrato do alvo em 0xae1080; conta os jogadores de troca sempre); 0x53d980
# cdecl(pessoa, nacao, 1); 0x53d740 cdecl(pessoa, 0).
# Defeitos do original (no cabecalho): na oferta do vendedor, o laco do elenco testa o alvo em vez de cada jogador, e o
# limite e o da divisao do comprador.
# Depende da sx21_1 (transfer_rules_can_sign_player, transfer_rules_can_make_offer, compare_rule_check): integrar junto
# ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [
    ('portugal_rules_can_sign_player', 0x7d3ef0, [0x83, 0xec, 0x08, 0x53, 0x55]),
    ('portugal_rules_can_make_offer', 0x7d4040, [0x83, 0xec, 0x14, 0x53, 0x55]),
]
DOCS = ('Portugal: quem um clube português pode contratar e de que oferta pode participar (o limite de fora da UE pela divisão)',
        'Portugal: who a Portuguese club may sign and which offers it may take part in (the non-EU limit by division)',
        [('portugal_rules_can_sign_player', 'portugal_rules_can_sign_player', '0x7d3ef0', 'portugal_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); o estrangeiro de fora da UE que passaria o limite da divisão (6, 5 ou 4) não (0x15); senão a regra de base',
          'rule: outside the windows no (9); a non-EU foreigner who would pass the division limit (6, 5 or 4) no (0x15); else the base rule'),
         ('portugal_rules_can_make_offer', 'portugal_rules_can_make_offer', '0x7d4040', 'portugal_rules.cpp', 'thiscall, ret 0x10',
          'regra: fora das janelas não (9); a transferência que passaria o limite de fora da UE do comprador não (0x15; no vendedor, o laço testa o alvo: defeito); senão a regra de base',
          "rule: outside the windows no (9); a move that would pass the buyer's non-EU limit no (0x15; for the seller the loop tests the target: defect); else the base rule")])
CAMPOS = []
INTERESSA = []
