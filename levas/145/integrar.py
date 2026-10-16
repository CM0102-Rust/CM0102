"""Integra a 145a leva ao motor.rs (sobre o commit anterior a leva)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b):
    global s
    assert s.count(a) == 1, a[:80]
    s = s.replace(a, b, 1)


troca("""    Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] },
];
const N: usize = 320;""", """    Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x6b98c0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
];
const N: usize = 321;""")
troca("""        conduzir as *const () as usize,
    ];""", """        conduzir as *const () as usize,
        laco_do_motor as *const () as usize,
    ];""")
troca("""        comparar_conduzir as *const () as usize,
    ];""", """        comparar_conduzir as *const () as usize,
        comparar_laco_do_motor as *const () as usize,
    ];""")
troca('"motor_conduzir",\n', '"motor_conduzir", "motor_laco",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
