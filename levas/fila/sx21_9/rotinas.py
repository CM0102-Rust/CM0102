# Os construtores das regras de transferencia da Russia e da Espanha (leitor sx21, rodada 5, 24/11/2026): rodam uma
# vez, na montagem das 28 regras pelo gestor de transferencias (0x8d26b0), na criacao do jogo da carreira ENG.
# Convencoes conferidas: thiscall ret 0x14, os cinco argumentos repassados na mesma ordem ao 0x8e84a0; o init das
# janelas (0x7ec7e0, 0x859470, ja em Rust) thiscall ret 0.
# Depende da sx21_6 (construct_country_rules): integrar junto ou depois dela.
# Comparadores: compare_transfer_rules_ctor (england_rules.rs).
SAVE = False
ROTINAS = [
    ('construct_russia_rules', 0x7ec770, [0x6a, 0xff, 0x68, 0x88, 0x0a, 0x96, 0x00]),
    ('construct_spain_rules', 0x859400, [0x6a, 0xff, 0x68, 0x58, 0x29, 0x96, 0x00]),
]
DOCS = ('as regras de transferência da Rússia e da Espanha: a montagem',
        'the Russian and Spanish transfer rules: building them',
        [('construct_russia_rules', 'construct_russia_rules', '0x7ec770', 'russia_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96f340 e as duas janelas',
          'rule: the base, the vtable 0x96f340 and the two windows'),
         ('construct_spain_rules', 'construct_spain_rules', '0x859400', 'spain_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96fdec, as duas janelas e o limite de 6 de fora da UE',
          'rule: the base, the vtable 0x96fdec, the two windows and a non-EU limit of 6')])
CAMPOS = []
INTERESSA = []
