"""Integra a 152a leva ao motor.rs (sobre o commit da 151a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b5f00, bytes: &[0x81, 0xec, 0x58, 0x02, 0x00, 0x00] },
];
const N: usize = 335;""", """    Prologo { endereco: 0x6b5f00, bytes: &[0x81, 0xec, 0x58, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b4360, bytes: &[0x6a, 0xff, 0x68, 0xdc, 0xd1, 0x95, 0x00] },
];
const N: usize = 336;""")
troca("""        fechar_partida_do_dia as *const () as usize,
    ];""", """        fechar_partida_do_dia as *const () as usize,
        jogar_o_dia as *const () as usize,
    ];""")
troca("""        comparar_fechar_partida_do_dia as *const () as usize,
    ];""", """        comparar_fechar_partida_do_dia as *const () as usize,
        comparar_jogar_o_dia as *const () as usize,
    ];""")
troca('"motor_fechar_partida_do_dia",\n', '"motor_fechar_partida_do_dia", "motor_jogar_o_dia",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
