# O Japao: quem um clube japones pode contratar e de que oferta pode participar, com cinco estrangeiros no maximo
# (leitor sx21, rodada 5, 24/11/2026): rodam nas transferencias dos clubes japoneses, so com o Japao carregado.
# Convencoes conferidas: 0x666d40 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x666e20 thiscall(oferta,
# clube, data, motivo) ret 0x10 (guarda o contrato do alvo em 0xae1080; conta os jogadores de troca sempre); o
# estrangeiro e o da nacao das regras (+0xa), nao a do clube; 0x53d980 cdecl(pessoa, nacao, 1).
# Defeito do original (no cabecalho): na oferta do vendedor, tira-se um quando o alvo NAO e estrangeiro.
# Depende da sx21_1 (transfer_rules_can_sign_player, transfer_rules_can_make_offer, compare_rule_check): integrar junto
# ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('japan_rules_can_sign_player', 0x666d40, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
    ('japan_rules_can_make_offer', 0x666e20, [0x83, 0xec, 0x0c, 0x53, 0x55]),
]
DOCS = ('o Japão: quem um clube japonês pode contratar e de que oferta pode participar (cinco estrangeiros no máximo)',
        'Japan: who a Japanese club may sign and which offers it may take part in (five foreign players at most)',
        [('japan_rules_can_sign_player', 'japan_rules_can_sign_player', '0x666d40', 'japan_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); o estrangeiro que levaria o elenco a mais de cinco estrangeiros não (0x11); senão a regra de base',
          'rule: outside the windows no (9); a foreigner who would take the squad past five foreign players no (0x11); else the base rule'),
         ('japan_rules_can_make_offer', 'japan_rules_can_make_offer', '0x666e20', 'japan_rules.cpp', 'thiscall, ret 0x10',
          'regra: fora das janelas não (9); a transferência que deixaria o comprador (o alvo menos os estrangeiros de troca) ou o clube do alvo (os de troca, menos um se o alvo não for estrangeiro: defeito) com mais de cinco estrangeiros não (0x11); senão a regra de base',
          "rule: outside the windows no (9); a move that would leave the buyer (the target less the foreign exchange players) or the target's club (the exchange players, less one when the target is not foreign: defect) with more than five foreigners no (0x11); else the base rule")])
CAMPOS = []
INTERESSA = []
