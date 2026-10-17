"""Integra a 150a leva ao motor.rs (sobre o commit da 149a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b42a0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9] },
];
const N: usize = 331;""", """    Prologo { endereco: 0x6b42a0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x6dbc50, bytes: &[0x6a, 0xff, 0x68, 0x52, 0xd4, 0x95, 0x00] },
];
const N: usize = 332;""")
troca("""        escolher_partida_do_dia as *const () as usize,
    ];""", """        escolher_partida_do_dia as *const () as usize,
        escalar_automatico as *const () as usize,
    ];""")
troca("""        comparar_escolher_partida_do_dia as *const () as usize,
    ];""", """        comparar_escolher_partida_do_dia as *const () as usize,
        comparar_escalar_automatico as *const () as usize,
    ];""")
troca('"motor_escolher_partida_do_dia",\n', '"motor_escolher_partida_do_dia", "motor_escalar_automatico",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
