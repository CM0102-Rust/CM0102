"""Integra a 166a leva ao motor.rs (sobre o commit da 165a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x5fdd30, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 370;""", """    Prologo { endereco: 0x5fdd30, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f6cd0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x600c90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4ca470, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
];
const N: usize = 373;""")
troca("""        gravar_dado_do_humano as *const () as usize,
    ];""", """        gravar_dado_do_humano as *const () as usize,
        registro_do_observador as *const () as usize,
        humano_manda_no_clube as *const () as usize,
        categoria_do_contrato as *const () as usize,
    ];""")
troca("""        comparar_gravar_dado_do_humano as *const () as usize,
    ];""", """        comparar_gravar_dado_do_humano as *const () as usize,
        comparar_registro_do_observador as *const () as usize,
        comparar_humano_manda_no_clube as *const () as usize,
        comparar_categoria_do_contrato as *const () as usize,
    ];""")
troca('"motor_gravar_dado_do_humano",', '"motor_gravar_dado_do_humano", "motor_registro_do_observador", "motor_humano_manda_no_clube", "motor_categoria_do_contrato",')
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
