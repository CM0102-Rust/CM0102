# A Belgica: quem um clube belga pode contratar em cada uma das tres janelas e de que oferta pode participar (leitor
# sx21, rodada 5, 24/11/2026): rodam nas transferencias dos clubes belgas, so com a Belgica carregada.
# Convencoes conferidas: 0x424d30 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x424eb0 thiscall(oferta,
# clube, data, motivo) ret 0x10 (guarda o contrato do alvo em 0xae1080; chama o slot 0x10 da propria vtable, que e o
# 0x424d30).
# Depende da sx21_1 (transfer_rules_can_sign_player, transfer_rules_can_make_offer, compare_rule_check): integrar junto
# ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ROTINAS = [
    ('belgium_rules_can_sign_player', 0x424d30, [0x51, 0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x20]),
    ('belgium_rules_can_make_offer', 0x424eb0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
]
DOCS = ('a Bélgica: quem um clube belga pode contratar em cada janela e de que oferta pode participar',
        'Belgium: who a Belgian club may sign in each window and which offers it may take part in',
        [('belgium_rules_can_sign_player', 'belgium_rules_can_sign_player', '0x424d30', 'belgium_rules.cpp', 'thiscall, ret 0x14',
          'regra: na primeira janela a regra de base; na segunda, de clube belga só de divisão mais prestigiada (ou da primeira divisão), senão 9; na terceira só os clubes da primeira divisão contratam, e não de clube belga (9); fora delas 9',
          'rule: in the first window the base rule; in the second, from a Belgian club only from a more reputed division (or the first division), else 9; in the third only first division clubs sign, and not from a Belgian club (9); outside them 9'),
         ('belgium_rules_can_make_offer', 'belgium_rules_can_make_offer', '0x424eb0', 'belgium_rules.cpp', 'thiscall, ret 0x10',
          'regra: o comprador passa pela própria regra de contratação (com o clube do alvo como vendedor, nenhum se o contrato for do tipo 5); o vendedor com jogadores de troca também, como se contratasse o alvo do comprador; senão a regra de base',
          "rule: the buyer goes through the rule set's own signing check (with the target's club as the seller, none for a type 5 contract); a seller with exchange players too, as if signing the target from the buyer; else the base rule")])
CAMPOS = []
INTERESSA = []
