# As regras de transferencia do Japao (construtor e janelas) e o construtor das de Portugal (leitor sx21, rodada 5,
# 24/11/2026): rodam uma vez, na montagem das 28 regras pelo gestor de transferencias (0x8d26b0), na criacao do jogo da
# carreira ENG. O sx04c apontou que os construtores do Japao e de Portugal estavam nesta lista.
# Convencoes conferidas: os construtores thiscall ret 0x14, os cinco argumentos repassados na mesma ordem ao 0x8e84a0;
# 0x666c40 thiscall ret 0 (conta +8 = 2 antes do malloc de 0x18; sem memoria, a caixa da linha 0x4d de japan_rules.cpp,
# o mesmo set_transfer_windows das outras nacoes); 0x7d3df0 (Rust) thiscall ret 0.
# Depende da sx21_6 (construct_country_rules): integrar junto ou depois dela.
# Comparadores: compare_transfer_rules_ctor (campos e janelas).
SAVE = False
ROTINAS = [
    ('construct_japan_rules', 0x666bd0, [0x6a, 0xff, 0x68, 0x08, 0xc3, 0x95, 0x00]),
    ('japan_rules_init_transfer_windows', 0x666c40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('construct_portugal_rules', 0x7d3d80, [0x6a, 0xff, 0x68, 0x58, 0xfc, 0x95, 0x00]),
]
DOCS = ('as regras de transferência do Japão (montagem e janelas) e a montagem das de Portugal',
        "Japan's transfer rules (building them and their windows) and building Portugal's",
        [('construct_japan_rules', 'construct_japan_rules', '0x666bd0', 'japan_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96ccfc e as duas janelas',
          'rule: the base, the vtable 0x96ccfc and the two windows'),
         ('japan_rules_init_transfer_windows', 'japan_rules_init_transfer_windows', '0x666c40', 'japan_rules.cpp', 'thiscall, ret 0',
          'regra (15): duas janelas, de 12 de dezembro (andando para o sábado) a 4 de maio e de 30 de maio (andando para o sábado) a 29 de outubro',
          'rule (15): two windows, from 12 December (moved to the Saturday) to 4 May and from 30 May (moved to the Saturday) to 29 October'),
         ('construct_portugal_rules', 'construct_portugal_rules', '0x7d3d80', 'portugal_rules.cpp', 'thiscall, ret 0x14',
          'regra: a base, a vtable 0x96e9c0 e as duas janelas',
          'rule: the base, the vtable 0x96e9c0 and the two windows')])
CAMPOS = []
INTERESSA = []
