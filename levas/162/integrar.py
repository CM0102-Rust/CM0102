"""Integra a 162a leva ao motor.rs (sobre o commit da 161a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x866f60, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
];
const N: usize = 358;""", """    Prologo { endereco: 0x866f60, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x896a90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8968f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8967d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 361;""")
troca("""        criterio_da_comissao as *const () as usize,
    ];""", """        criterio_da_comissao as *const () as usize,
        gravar_vaga_tatica as *const () as usize,
        pessoa_da_vaga_tatica as *const () as usize,
        gravar_formacao_tatica as *const () as usize,
    ];""")
troca("""        comparar_criterio_da_comissao as *const () as usize,
    ];""", """        comparar_criterio_da_comissao as *const () as usize,
        comparar_gravar_vaga_tatica as *const () as usize,
        comparar_pessoa_da_vaga_tatica as *const () as usize,
        comparar_gravar_formacao_tatica as *const () as usize,
    ];""")
troca('"motor_criterio_da_comissao",', '"motor_criterio_da_comissao", "motor_gravar_vaga_tatica", "motor_pessoa_da_vaga_tatica", "motor_gravar_formacao_tatica",')
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
