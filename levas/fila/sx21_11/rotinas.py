# Os limites de elenco das partidas da Russia e da Turquia e o emprestimo grego (leitor sx21, rodada 5, 24/11/2026):
# rodam nas partidas e nos emprestimos dessas ligas, entao so com elas carregadas.
# Convencoes conferidas: fill_squad_restrictions thiscall(partida, saida) ret 8; a Russia com a partida nula nao poe
# limite; a Turquia nao confere a partida (le +0 e +0x14); 0x5ed060 "mov eax, 1; ret 0x10", o slot 0x20 da Grecia, da
# Croacia e da Polonia (corpo proprio, 5 bytes de prologo).
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o valor.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [
    ('russia_rules_fill_squad_restrictions', 0x7ec950, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('turkey_rules_fill_squad_restrictions', 0x9021d0, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('greece_rules_can_loan_player', 0x5ed060, [0xb8, 0x01, 0x00, 0x00, 0x00]),
]
DOCS = ('os limites de elenco das partidas da Rússia e da Turquia e o empréstimo grego',
        'the squad limits of Russian and Turkish matches and the Greek loan check',
        [('russia_rules_fill_squad_restrictions', 'russia_rules_fill_squad_restrictions', '0x7ec950', 'russia_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo três estrangeiros (+2) na primeira divisão russa',
          'rule: no limits (0xff) except three foreign players (+2) in the Russian top flight'),
         ('turkey_rules_fill_squad_restrictions', 'turkey_rules_fill_squad_restrictions', '0x9021d0', 'turkey_rules.cpp', 'thiscall, ret 8',
          'regra: seis estrangeiros (+2) na primeira divisão e na Copa da Turquia, nenhum na divisão de baixo; nas partidas de reservas a regra da idade (+0x16 = 21, +0x17 = 0, +0x26 = a competição de reservas)',
          'rule: six foreign players (+2) in the top flight and the Turkish Cup, none in the division below; in reserve matches the age rule (+0x16 = 21, +0x17 = 0, +0x26 = the reserve competition)'),
         ('greece_rules_can_loan_player', 'greece_rules_can_loan_player', '0x5ed060', 'greece_rules.cpp', 'thiscall, ret 0x10',
          'regra: todo empréstimo pode (1); slot da Grécia, da Croácia e da Polônia',
          'rule: every loan is allowed (1); the slot of Greece, Croatia and Poland')])
CAMPOS = []
INTERESSA = []
