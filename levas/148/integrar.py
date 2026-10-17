"""Integra a 148a leva ao motor.rs (sobre o commit da 147a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x702050, bytes: &[0x81, 0xec, 0x84, 0x06, 0x00, 0x00] },
];
const N: usize = 325;""", """    Prologo { endereco: 0x702050, bytes: &[0x81, 0xec, 0x84, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x6b7e50, bytes: &[0x6a, 0xff, 0x68, 0x0c, 0xd3, 0x95, 0x00] },
    Prologo { endereco: 0x6b7f20, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6b97a0, bytes: &[0x6a, 0xff, 0x68, 0xda, 0xd3, 0x95, 0x00] },
];
const N: usize = 328;""")
troca("""        chutar as *const () as usize,
    ];""", """        chutar as *const () as usize,
        construir_motor as *const () as usize,
        preparar_partida as *const () as usize,
        destruir_motor as *const () as usize,
    ];""")
troca("""        comparar_chutar as *const () as usize,
    ];""", """        comparar_chutar as *const () as usize,
        comparar_construir_motor as *const () as usize,
        comparar_preparar_partida as *const () as usize,
        comparar_destruir_motor as *const () as usize,
    ];""")
troca('"motor_chutar",\n', '"motor_chutar", "motor_construir", "motor_preparar_partida", "motor_destruir",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
