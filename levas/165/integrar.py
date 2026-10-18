"""Integra a 165a leva ao motor.rs (sobre o commit da 164a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x5af180, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
];
const N: usize = 367;""", """    Prologo { endereco: 0x5af180, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x897ee0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76d430, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fdd30, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 370;""")
troca("""        marcar_ano_no_calendario as *const () as usize,
    ];""", """        marcar_ano_no_calendario as *const () as usize,
        vaga_extra_tatica as *const () as usize,
        idades_da_selecao as *const () as usize,
        gravar_dado_do_humano as *const () as usize,
    ];""")
troca("""        comparar_marcar_ano_no_calendario as *const () as usize,
    ];""", """        comparar_marcar_ano_no_calendario as *const () as usize,
        comparar_vaga_extra_tatica as *const () as usize,
        comparar_idades_da_selecao as *const () as usize,
        comparar_gravar_dado_do_humano as *const () as usize,
    ];""")
troca('"motor_marcar_ano_no_calendario",', '"motor_marcar_ano_no_calendario", "motor_vaga_extra_tatica", "motor_idades_da_selecao", "motor_gravar_dado_do_humano",')
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
