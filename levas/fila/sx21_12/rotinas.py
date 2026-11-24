# Os limites de elenco das partidas da Australia, dos EUA e do Japao (slot 0x1c; leitor sx21, rodada 5, 24/11/2026):
# rodam nas partidas dessas ligas (as 8 ligas).
# Convencoes conferidas: thiscall(partida, saida) ret 8; os EUA leem o id da competicao da partida (+0), com a partida
# nula sem limite; Australia e Japao nao leem a partida.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('australia_rules_fill_squad_restrictions', 0x412db0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('usa_rules_fill_squad_restrictions', 0x90ba60, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('japan_rules_fill_squad_restrictions', 0x667090, [0x8b, 0x54, 0x24, 0x08, 0x57]),
]
DOCS = ('os limites de elenco das partidas da Austrália, dos EUA e do Japão',
        'the squad limits of Australian, American and Japanese matches',
        [('australia_rules_fill_squad_restrictions', 'australia_rules_fill_squad_restrictions', '0x412db0', 'australia_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo seis estrangeiros (+2)',
          'rule: no limits (0xff) except six foreign players (+2)'),
         ('usa_rules_fill_squad_restrictions', 'usa_rules_fill_squad_restrictions', '0x90ba60', 'usa_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo quatro estrangeiros (+2), menos no jogo das estrelas da MLS',
          'rule: no limits (0xff) except four foreign players (+2), but not in the MLS All-Star game'),
         ('japan_rules_fill_squad_restrictions', 'japan_rules_fill_squad_restrictions', '0x667090', 'japan_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo três estrangeiros (+2)',
          'rule: no limits (0xff) except three foreign players (+2)')])
CAMPOS = []
INTERESSA = []
