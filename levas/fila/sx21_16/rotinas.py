# Os limites de elenco e quem pode jogar na Noruega, e os limites de elenco de Portugal (leitor sx21, rodada 5,
# 24/11/2026): rodam nas partidas dessas ligas.
# Convencoes conferidas: fill_squad_restrictions thiscall(partida, saida) ret 8 (Portugal confere a partida nula e le
# [[+0x14]]); 0x794610 thiscall(jogador, partida) ret 8, al (quadro SEH sem destrutor); 0x40a7d0 thiscall(saida, ano)
# ret 8; 0x54c5b0 thiscall(saida, n) ret 8; 0x4cf6d0 thiscall(pessoa) ret 4.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o codigo (al) e G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [
    ('norway_rules_fill_squad_restrictions', 0x794740, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('portugal_rules_fill_squad_restrictions', 0x7d43c0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('norway_rules_get_ineligibility_code', 0x794610, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
]
DOCS = ('os limites de elenco e quem pode jogar na Noruega e os limites de elenco de Portugal',
        "Norway's squad limits and eligibility and Portugal's squad limits",
        [('norway_rules_fill_squad_restrictions', 'norway_rules_fill_squad_restrictions', '0x794740', 'norway_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo dois de fora da UE (+5)',
          'rule: no limits (0xff) except two non-EU players (+5)'),
         ('portugal_rules_fill_squad_restrictions', 'portugal_rules_fill_squad_restrictions', '0x7d43c0', 'portugal_rules.cpp', 'thiscall, ret 8',
          'regra: os de fora da UE (+3): 3 na segunda divisão B, 4 nas outras',
          'rule: the non-EU (+3): 3 in the Second Division B, 4 elsewhere'),
         ('norway_rules_get_ineligibility_code', 'norway_rules_get_ineligibility_code', '0x794610', 'norway_rules.cpp', 'thiscall, ret 8',
          'regra: amistoso, reservas ou sem competição: 0; quem chegou ao clube depois do fim da primeira janela (no ano da temporada): 5; quem chegou há menos de três dias (sem empréstimo): 2; senão a resposta de base',
          "rule: friendlies, reserves or no competition: 0; a player who joined after the first window closed (in the season's year): 5; one who joined less than three days ago (not on loan): 2; else the base answer")])
CAMPOS = []
INTERESSA = []
