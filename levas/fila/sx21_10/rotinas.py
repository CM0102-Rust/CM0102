# Os limites de elenco das partidas da Grecia, da Croacia e da Polonia (slot 0x1c; leitor sx21, rodada 5, 24/11/2026):
# rodam nas partidas dessas ligas, entao so com elas carregadas.
# Convencoes conferidas: thiscall(partida, saida) ret 8; a Grecia le o id da competicao da partida (+0), com a partida
# nula dando 4; Croacia e Polonia nao leem a partida.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [
    ('greece_rules_fill_squad_restrictions', 0x5ed020, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('croatia_rules_fill_squad_restrictions', 0x518620, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('poland_rules_fill_squad_restrictions', 0x7cd960, [0x8b, 0x54, 0x24, 0x08, 0x57]),
]
DOCS = ('os limites de elenco das partidas da Grécia, da Croácia e da Polônia',
        'the squad limits of Greek, Croatian and Polish matches',
        [('greece_rules_fill_squad_restrictions', 'greece_rules_fill_squad_restrictions', '0x5ed020', 'greece_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo os de fora da UE (+5): 5 na primeira divisão grega, 4 nas outras',
          'rule: no limits (0xff) except the non-EU (+5): 5 in the Greek top flight, 4 elsewhere'),
         ('croatia_rules_fill_squad_restrictions', 'croatia_rules_fill_squad_restrictions', '0x518620', 'croatia_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo quatro estrangeiros (+0)',
          'rule: no limits (0xff) except four foreign players (+0)'),
         ('poland_rules_fill_squad_restrictions', 'poland_rules_fill_squad_restrictions', '0x7cd960', 'poland_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo cinco estrangeiros (+2)',
          'rule: no limits (0xff) except five foreign players (+2)')])
CAMPOS = []
INTERESSA = []
