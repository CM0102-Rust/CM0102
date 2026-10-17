"""Integra a 154a leva ao motor.rs (sobre o commit da 153a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6d8c70, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
];
const N: usize = 339;""", """    Prologo { endereco: 0x6d8c70, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x6d98b0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d9c40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b55c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
];
const N: usize = 342;""")
troca("""        sons_terminados as *const () as usize,
    ];""", """        sons_terminados as *const () as usize,
        gravar_eventos as *const () as usize,
        ler_eventos as *const () as usize,
        passo_em_rede as *const () as usize,
    ];""")
troca("""        comparar_sons_terminados as *const () as usize,
    ];""", """        comparar_sons_terminados as *const () as usize,
        comparar_gravar_eventos as *const () as usize,
        comparar_ler_eventos as *const () as usize,
        comparar_passo_em_rede as *const () as usize,
    ];""")
troca('"motor_sons_terminados",\n', '"motor_sons_terminados", "motor_gravar_eventos", "motor_ler_eventos", "motor_passo_em_rede",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
