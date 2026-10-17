"""Integra a 149a leva ao motor.rs (sobre o commit da 148a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b97a0, bytes: &[0x6a, 0xff, 0x68, 0xda, 0xd3, 0x95, 0x00] },
];
const N: usize = 328;""", """    Prologo { endereco: 0x6b97a0, bytes: &[0x6a, 0xff, 0x68, 0xda, 0xd3, 0x95, 0x00] },
    Prologo { endereco: 0x6db500, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b4190, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] },
    Prologo { endereco: 0x6b42a0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9] },
];
const N: usize = 331;""")
troca("""        destruir_motor as *const () as usize,
    ];""", """        destruir_motor as *const () as usize,
        escalar_lado as *const () as usize,
        encerrar_dia as *const () as usize,
        escolher_partida_do_dia as *const () as usize,
    ];""")
troca("""        comparar_destruir_motor as *const () as usize,
    ];""", """        comparar_destruir_motor as *const () as usize,
        comparar_escalar_lado as *const () as usize,
        comparar_encerrar_dia as *const () as usize,
        comparar_escolher_partida_do_dia as *const () as usize,
    ];""")
troca('"motor_destruir",\n', '"motor_destruir", "motor_escalar_lado", "motor_encerrar_dia", "motor_escolher_partida_do_dia",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
