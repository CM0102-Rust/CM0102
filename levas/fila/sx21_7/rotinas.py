# Os construtores das regras de transferencia da Alemanha, da Grecia e da Italia (leitor sx21, rodada 5, 24/11/2026):
# rodam uma vez, na montagem das 28 regras pelo gestor de transferencias (0x8d26b0), na criacao do jogo da carreira ENG.
# Convencoes conferidas: thiscall ret 0x14, os cinco argumentos repassados na mesma ordem ao 0x8e84a0; o init das
# janelas (0x5dffe0, 0x5ecf20, 0x6613f0, ja em Rust) thiscall ret 0.
# Depende da sx21_6 (construct_country_rules): integrar junto ou depois dela.
# Comparadores: compare_transfer_rules_ctor (england_rules.rs).
SAVE = False
ROTINAS = [
    ('construct_germany_rules', 0x5dff70, [0x6a, 0xff, 0x68, 0xd8, 0xa7, 0x95, 0x00]),
    ('construct_greece_rules', 0x5eceb0, [0x6a, 0xff, 0x68, 0x78, 0xab, 0x95, 0x00]),
    ('construct_italy_rules', 0x661380, [0x6a, 0xff, 0x68, 0x58, 0xc0, 0x95, 0x00]),
]
DOCS = ('as regras de transferência da Alemanha, da Grécia e da Itália: a montagem',
        'the German, Greek and Italian transfer rules: building them',
        [('construct_germany_rules', 'construct_germany_rules', '0x5dff70', 'germany_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96b3cc e as duas janelas',
          'rule: the base, the vtable 0x96b3cc and the two windows'),
         ('construct_greece_rules', 'construct_greece_rules', '0x5eceb0', 'greece_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96b784, as duas janelas e o limite de 6 de fora da UE',
          'rule: the base, the vtable 0x96b784, the two windows and a non-EU limit of 6'),
         ('construct_italy_rules', 'construct_italy_rules', '0x661380', 'italy_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96c96c, as três janelas e o limite de 40 de fora da UE',
          'rule: the base, the vtable 0x96c96c, the three windows and a non-EU limit of 40')])
CAMPOS = []
INTERESSA = []
