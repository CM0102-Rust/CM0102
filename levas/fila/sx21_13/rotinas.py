# Os limites de elenco e a inscricao por outro clube do Brasil, e os limites de elenco da Espanha (leitor sx21, rodada
# 5, 24/11/2026): rodam nas partidas dessas ligas (as 8 ligas).
# Convencoes conferidas: fill_squad_restrictions thiscall(partida, saida) ret 8 (o Brasil confere a partida nula; a
# Espanha le +0 e +0x2c sem conferir); 0x43f210 thiscall(jogador, partida) ret 8, al; 0x5c6710 thiscall(comp) ret 4;
# 0x4a4a30 thiscall(ficha) ret 4.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o codigo (al) e G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('brazil_rules_fill_squad_restrictions', 0x43f2a0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('brazil_rules_get_ineligibility_code', 0x43f210, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
    ('spain_rules_fill_squad_restrictions', 0x859570, [0x8b, 0x54, 0x24, 0x08, 0x57]),
]
DOCS = ('os limites de elenco e a inscrição por outro clube do Brasil e os limites de elenco da Espanha',
        "Brazil's squad limits and cup-tied players and Spain's squad limits",
        [('brazil_rules_fill_squad_restrictions', 'brazil_rules_fill_squad_restrictions', '0x43f2a0', 'brazil_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo dois estrangeiros (+2), menos nas partidas de reservas',
          'rule: no limits (0xff) except two foreign players (+2), but not in reserve matches'),
         ('brazil_rules_get_ineligibility_code', 'brazil_rules_get_ineligibility_code', '0x43f210', 'brazil_rules.cpp', 'thiscall, ret 8',
          'regra: amistoso e partida de reservas, todos jogam (0); quem já jogou a competição por outro clube não joga (4); senão a resposta de base',
          'rule: in friendlies and reserve matches anyone plays (0); a player who already played the competition for another club may not (4); else the base answer'),
         ('spain_rules_fill_squad_restrictions', 'spain_rules_fill_squad_restrictions', '0x859570', 'spain_rules.cpp', 'thiscall, ret 8',
          'regra: os de fora da UE (+5): na primeira divisão e na Copa do Rei 4 na temporada 2000 e 3 depois; nas outras 4 em 2000, 3 em 2001 e 2002 e 2 depois',
          "rule: the non-EU (+5): in the first division and the King's Cup 4 in the 2000 season and 3 after; elsewhere 4 in 2000, 3 in 2001 and 2002 and 2 after")])
CAMPOS = []
INTERESSA = []
