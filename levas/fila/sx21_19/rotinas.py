# Os limites de elenco da Belgica e da Alemanha e quem um clube alemao pode contratar (leitor sx21, rodada 5,
# 24/11/2026): rodam nas partidas e nas transferencias dessas ligas.
# Convencoes conferidas: fill_squad_restrictions thiscall(partida, saida) ret 8 (a Alemanha confere a partida nula);
# 0x5e00e0 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14 (o terceiro argumento, sem uso na base, e o clube
# vendedor); 0x8e8e50 thiscall(data, saida) ret 8; 0x8e8c40 thiscall(data, janela) ret 8.
# Depende da sx21_1 (transfer_rules_can_sign_player, compare_rule_check) e da sx21_5 (fill_squad_restrictions_with,
# compare_squad_restrictions): integrar junto ou depois delas.
# Comparadores: os 42 bytes lado a lado; a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ROTINAS = [
    ('belgium_rules_fill_squad_restrictions', 0x424f80, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('germany_rules_fill_squad_restrictions', 0x5e02d0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('germany_rules_can_sign_player', 0x5e00e0, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x18]),
]
DOCS = ('os limites de elenco da Bélgica e da Alemanha e quem um clube alemão pode contratar',
        'the squad limits of Belgium and Germany and who a German club may sign',
        [('belgium_rules_fill_squad_restrictions', 'belgium_rules_fill_squad_restrictions', '0x424f80', 'belgium_rules.cpp', 'thiscall, ret 8',
          'regra: a regra da idade (+6 = 21) com dois fora dela (+8)',
          'rule: the age rule (+6 = 21) with two outside it (+8)'),
         ('germany_rules_fill_squad_restrictions', 'germany_rules_fill_squad_restrictions', '0x5e02d0', 'germany_rules.cpp', 'thiscall, ret 8',
          'regra: os de fora da UE (+5): 3 na Copa da Alemanha, 5 nas outras',
          'rule: the non-EU (+5): 3 in the German Cup, 5 elsewhere'),
         ('germany_rules_can_sign_player', 'germany_rules_can_sign_player', '0x5e00e0', 'germany_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); na primeira janela a regra de base; na segunda, clube alemão fora das duas primeiras divisões não contrata (9), nem compra de clube alemão de divisão menos prestigiada ou sem divisão (9); senão a regra de base',
          'rule: outside the windows no (9); in the first window the base rule; in the second, a German club outside the top two divisions may not sign (9), nor buy from a German club of a less reputed division or of none (9); else the base rule')])
CAMPOS = []
INTERESSA = []
