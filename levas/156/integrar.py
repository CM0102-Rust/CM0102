"""Integra a 156a leva ao motor.rs (sobre o commit da 155a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6d9e50, bytes: &[0x81, 0xec, 0x0c, 0x06, 0x00, 0x00] },
];
const N: usize = 343;""", """    Prologo { endereco: 0x6d9e50, bytes: &[0x81, 0xec, 0x0c, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x6d8d10, bytes: &[0xb8, 0xd8, 0x1b, 0x00, 0x00] },
    Prologo { endereco: 0x6b5260, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
];
const N: usize = 345;""")
troca("""        ler_eventos_cfg as *const () as usize,
    ];""", """        ler_eventos_cfg as *const () as usize,
        texto_do_evento as *const () as usize,
        confirmar_partida_em_rede as *const () as usize,
    ];""")
troca("""        comparar_ler_eventos_cfg as *const () as usize,
    ];""", """        comparar_ler_eventos_cfg as *const () as usize,
        comparar_texto_do_evento as *const () as usize,
        comparar_confirmar_partida_em_rede as *const () as usize,
    ];""")
troca('"motor_ler_eventos_cfg",\n', '"motor_ler_eventos_cfg", "motor_texto_do_evento", "motor_confirmar_partida_em_rede",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
