# Os limites de elenco e a inscricao por outro clube da Irlanda, e a inscricao por outro clube da Irlanda do Norte
# (leitor sx21, rodada 5, 24/11/2026): rodam nas partidas dessas ligas.
# Convencoes conferidas: 0x63c330 thiscall(partida, saida) ret 8 (so os 0xff); 0x63c350 e 0x7937f0 thiscall(jogador,
# partida) ret 8, al; 0x4a4a30 thiscall(ficha) ret 4.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code, cup_tied_entry,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o codigo (al) e G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [
    ('ireland_rules_fill_squad_restrictions', 0x63c330, [0x57, 0x8b, 0x7c, 0x24, 0x0c]),
    ('ireland_rules_get_ineligibility_code', 0x63c350, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
    ('northern_ireland_rules_get_ineligibility_code', 0x7937f0, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
]
DOCS = ('os limites de elenco e a inscrição por outro clube da Irlanda e da Irlanda do Norte',
        'the squad limits and cup-tied players of Ireland and Northern Ireland',
        [('ireland_rules_fill_squad_restrictions', 'ireland_rules_fill_squad_restrictions', '0x63c330', 'ireland_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (todos os 42 bytes em 0xff)',
          'rule: no limits (all 42 bytes at 0xff)'),
         ('ireland_rules_get_ineligibility_code', 'ireland_rules_get_ineligibility_code', '0x63c350', 'ireland_rules.cpp', 'thiscall, ret 8',
          'regra: na Copa da Liga, na Senior Challenge Cup e na Supercopa da Irlanda, quem já jogou a copa por outro clube não joga (1); senão a resposta de base',
          'rule: in the Irish League Cup, Senior Challenge Cup and Super Cup, a player who already played the cup for another club may not play (1); else the base answer'),
         ('northern_ireland_rules_get_ineligibility_code', 'northern_ireland_rules_get_ineligibility_code', '0x7937f0', 'northern_ireland_rules.cpp', 'thiscall, ret 8',
          'regra: na Copa, na Copa da Liga e na Gold Cup da Irlanda do Norte, quem já jogou a copa por outro clube não joga (1); senão a resposta de base',
          'rule: in the Northern Irish Cup, League Cup and Gold Cup, a player who already played the cup for another club may not play (1); else the base answer')])
CAMPOS = []
INTERESSA = []
