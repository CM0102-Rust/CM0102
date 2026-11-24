# Os construtores das regras de transferencia da Argentina, da Belgica e da Franca (leitor sx21, rodada 5, 24/11/2026):
# rodam uma vez, na montagem das 28 regras pelo gestor de transferencias (0x8d26b0), na criacao do jogo da carreira
# ENG (todas as nacoes sao montadas, carregadas ou nao).
# Convencoes conferidas: thiscall ret 0x14, os cinco argumentos repassados na mesma ordem ao 0x8e84a0; o init das
# janelas (0x40a620, 0x424c00, 0x5c15e0, ja em Rust) thiscall ret 0; quadro SEH so para o new, sem destrutor no
# caminho normal.
# Ajudante comum: construct_country_rules (usado pelos sx21_7 a sx21_9). Depende da sx21_1 so pelo desvio do 0x8e84a0
# (nao pelo codigo).
# Comparadores: compare_transfer_rules_ctor (england_rules.rs).
SAVE = False
ROTINAS = [
    ('construct_argentina_rules', 0x40a580, [0x6a, 0xff, 0x68, 0x08, 0x37, 0x95, 0x00]),
    ('construct_belgium_rules', 0x424b90, [0x6a, 0xff, 0x68, 0xc8, 0x42, 0x95, 0x00]),
    ('construct_france_rules', 0x5c1570, [0x6a, 0xff, 0x68, 0xb8, 0x9c, 0x95, 0x00]),
]
DOCS = ('as regras de transferência da Argentina, da Bélgica e da França: a montagem',
        'the Argentine, Belgian and French transfer rules: building them',
        [('construct_argentina_rules', 'construct_argentina_rules', '0x40a580', 'argentina_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x9674a0, as duas janelas e o limite de 20 estrangeiros',
          'rule: the base, the vtable 0x9674a0, the two windows and a limit of 20 foreign players'),
         ('construct_belgium_rules', 'construct_belgium_rules', '0x424b90', 'belgium_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x967c10, as três janelas e sem limite de estrangeiros nem de fora da UE (0xff)',
          'rule: the base, the vtable 0x967c10, the three windows and no foreign nor non-EU limit (0xff)'),
         ('construct_france_rules', 'construct_france_rules', '0x5c1570', 'france_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96ae48 e as três janelas',
          'rule: the base, the vtable 0x96ae48 and the three windows')])
CAMPOS = []
INTERESSA = []
