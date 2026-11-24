# Os limites de elenco e quem pode jogar na Coreia do Sul, e os limites de elenco da Argentina (leitor sx21, rodada 5,
# 24/11/2026): rodam nas partidas dessas ligas.
# Convencoes conferidas: fill_squad_restrictions thiscall(partida, saida) ret 8 (nao leem a partida); 0x66e770
# thiscall(jogador, partida) ret 8: repassa os dois ao 0x8e8a60 com o mesmo ecx (corpo proprio, nao um salto).
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o codigo (al) e G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ROTINAS = [
    ('korea_rules_fill_squad_restrictions', 0x66e750, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('korea_rules_get_ineligibility_code', 0x66e770, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
    ('argentina_rules_fill_squad_restrictions', 0x40aa10, [0x8b, 0x54, 0x24, 0x08, 0x57]),
]
DOCS = ('os limites de elenco e quem pode jogar na Coreia do Sul e os limites de elenco da Argentina',
        "South Korea's squad limits and eligibility and Argentina's squad limits",
        [('korea_rules_fill_squad_restrictions', 'korea_rules_fill_squad_restrictions', '0x66e750', 'korea_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo três estrangeiros (+0)',
          'rule: no limits (0xff) except three foreign players (+0)'),
         ('korea_rules_get_ineligibility_code', 'korea_rules_get_ineligibility_code', '0x66e770', 'korea_rules.cpp', 'thiscall, ret 8',
          'regra: a resposta de base, sem mudança',
          'rule: the base answer, unchanged'),
         ('argentina_rules_fill_squad_restrictions', 'argentina_rules_fill_squad_restrictions', '0x40aa10', 'argentina_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo quatro estrangeiros (+2)',
          'rule: no limits (0xff) except four foreign players (+2)')])
CAMPOS = []
INTERESSA = []
