# A Espanha: quem um clube espanhol pode contratar e de que oferta pode participar (leitor sx21, rodada 5, 24/11/2026):
# rodam nas transferencias dos clubes espanhois, so com a Espanha carregada.
# Convencoes conferidas: 0x859780 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x859850 thiscall(oferta,
# clube, data, motivo) ret 0x10 (guarda o contrato do alvo em 0xae1080; conta os jogadores de troca sempre, antes de
# decidir); 0x53d980 cdecl(pessoa, nacao, 1); 0x53d740 cdecl(pessoa, 0).
# Defeitos do original (no cabecalho das rotinas): a contratacao recusa o estrangeiro de fora da UE aos clubes das duas
# primeiras divisoes (e sem divisao), o contrario da oferta, que o recusa aos das outras divisoes; na oferta do
# vendedor, a divisao conferida e a do comprador.
# Depende da sx21_1 (transfer_rules_can_sign_player, transfer_rules_can_make_offer, compare_rule_check): integrar junto
# ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('spain_rules_can_sign_player', 0x859780, [0x51, 0x53, 0x55, 0x56, 0x57]),
    ('spain_rules_can_make_offer', 0x859850, [0x83, 0xec, 0x10, 0x53, 0x55]),
]
DOCS = ('a Espanha: quem um clube espanhol pode contratar e de que oferta pode participar',
        'Spain: who a Spanish club may sign and which offers it may take part in',
        [('spain_rules_can_sign_player', 'spain_rules_can_sign_player', '0x859780', 'spain_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); estrangeiro de fora da UE para clube das duas primeiras divisões ou sem divisão não (0x15; defeito: o contrário da oferta); senão a regra de base',
          'rule: outside the windows no (9); a non-EU foreigner for a club in the top two divisions or without one no (0x15; defect: the reverse of the offer check); else the base rule'),
         ('spain_rules_can_make_offer', 'spain_rules_can_make_offer', '0x859850', 'spain_rules.cpp', 'thiscall, ret 0x10',
          'regra: fora das janelas não (9); trazer estrangeiro de fora da UE para comprador fora das duas primeiras divisões não (0x15; o vendedor conta os jogadores de troca, menos o alvo); senão a regra de base',
          'rule: outside the windows no (9); bringing a non-EU foreigner to a buyer outside the top two divisions no (0x15; the seller counts the exchange players, less the target); else the base rule')])
CAMPOS = []
INTERESSA = []
