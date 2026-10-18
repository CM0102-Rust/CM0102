"""Integra a 163a leva ao motor.rs (sobre o commit da 162a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x8967d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 361;""", """    Prologo { endereco: 0x8967d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b3c70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b50d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x452fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 364;""")
troca("""        gravar_formacao_tatica as *const () as usize,
    ];""", """        gravar_formacao_tatica as *const () as usize,
        casa_guardada as *const () as usize,
        ligar_marca_da_formacao as *const () as usize,
        reiniciar_recordes as *const () as usize,
    ];""")
troca("""        comparar_gravar_formacao_tatica as *const () as usize,
    ];""", """        comparar_gravar_formacao_tatica as *const () as usize,
        comparar_casa_guardada as *const () as usize,
        comparar_ligar_marca_da_formacao as *const () as usize,
        comparar_reiniciar_recordes as *const () as usize,
    ];""")
troca('"motor_gravar_formacao_tatica",', '"motor_gravar_formacao_tatica", "motor_casa_guardada", "motor_ligar_marca_da_formacao", "motor_reiniciar_recordes",')
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
