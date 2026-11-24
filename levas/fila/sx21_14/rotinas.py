# Quem pode jogar na Espanha e os limites de elenco da Italia e da Franca (leitor sx21, rodada 5, 24/11/2026): rodam
# nas partidas dessas ligas (as 8 ligas).
# Convencoes conferidas: 0x8595e0 thiscall(jogador, partida) ret 8, al; fill_squad_restrictions thiscall(partida,
# saida) ret 8 (a Italia le [[+0x14]], a Franca le +0, sem conferir a partida).
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: o codigo (al) e G_CONTRACT_LOOKUP_RESULT; os 42 bytes lado a lado.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('spain_rules_get_ineligibility_code', 0x8595e0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
    ('italy_rules_fill_squad_restrictions', 0x6619e0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('france_rules_fill_squad_restrictions', 0x5c1a30, [0x8b, 0x54, 0x24, 0x08, 0x57]),
]
DOCS = ('quem pode jogar na Espanha e os limites de elenco das partidas da Itália e da França',
        'who may play a Spanish match and the squad limits of Italian and French matches',
        [('spain_rules_get_ineligibility_code', 'spain_rules_get_ineligibility_code', '0x8595e0', 'spain_rules.cpp', 'thiscall, ret 8',
          'regra: amistoso, partida de reservas ou sem competição: todos jogam (0); senão a resposta de base',
          'rule: friendlies, reserve matches or no competition: anyone plays (0); else the base answer'),
         ('italy_rules_fill_squad_restrictions', 'italy_rules_fill_squad_restrictions', '0x6619e0', 'italy_rules.cpp', 'thiscall, ret 8',
          'regra: a regra da idade (+6 = 21) e quantos fora dela (+8): 4 na Série A e na Série B, 5 nas outras',
          'rule: the age rule (+6 = 21) and how many outside it (+8): 4 in Serie A and Serie B, 5 elsewhere'),
         ('france_rules_fill_squad_restrictions', 'france_rules_fill_squad_restrictions', '0x5c1a30', 'france_rules.cpp', 'thiscall, ret 8',
          'regra: os de fora da UE (+5): 5 na primeira divisão francesa, 3 nas outras',
          'rule: the non-EU (+5): 5 in the French top flight, 3 elsewhere')])
CAMPOS = []
INTERESSA = []
