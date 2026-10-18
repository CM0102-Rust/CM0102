"""Integra a 161a leva ao motor.rs (sobre o commit da 160a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x5b8dc0, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
];
const N: usize = 355;""", """    Prologo { endereco: 0x5b8dc0, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
    Prologo { endereco: 0x844300, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x83bd00, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x866f60, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
];
const N: usize = 358;""")
troca("""        zagueiros_sem_conflito as *const () as usize,
    ];""", """        zagueiros_sem_conflito as *const () as usize,
        serve_para_a_vaga as *const () as usize,
        codigo_da_posicao as *const () as usize,
        criterio_da_comissao as *const () as usize,
    ];""")
troca("""        comparar_zagueiros_sem_conflito as *const () as usize,
    ];""", """        comparar_zagueiros_sem_conflito as *const () as usize,
        comparar_serve_para_a_vaga as *const () as usize,
        comparar_codigo_da_posicao as *const () as usize,
        comparar_criterio_da_comissao as *const () as usize,
    ];""")
troca('"motor_zagueiros_sem_conflito",', '"motor_zagueiros_sem_conflito", "motor_serve_para_a_vaga", "motor_codigo_da_posicao", "motor_criterio_da_comissao",')
anc = 'comparar_valor!(comparar_zagueiros_sem_conflito, zagueiros_sem_conflito, 354, () -> u8);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
