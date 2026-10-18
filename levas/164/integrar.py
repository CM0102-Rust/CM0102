"""Integra a 164a leva ao motor.rs (sobre o commit da 163a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x452fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];
const N: usize = 364;""", """    Prologo { endereco: 0x452fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x68a030, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x0d, 0xe8, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x68a1c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5af180, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
];
const N: usize = 367;""")
troca("""        reiniciar_recordes as *const () as usize,
    ];""", """        reiniciar_recordes as *const () as usize,
        girar_rodada as *const () as usize,
        registrar_jogo_da_rodada as *const () as usize,
        marcar_ano_no_calendario as *const () as usize,
    ];""")
troca("""        comparar_reiniciar_recordes as *const () as usize,
    ];""", """        comparar_reiniciar_recordes as *const () as usize,
        comparar_girar_rodada as *const () as usize,
        comparar_registrar_jogo_da_rodada as *const () as usize,
        comparar_marcar_ano_no_calendario as *const () as usize,
    ];""")
troca('"motor_reiniciar_recordes",', '"motor_reiniciar_recordes", "motor_girar_rodada", "motor_registrar_jogo_da_rodada", "motor_marcar_ano_no_calendario",')
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
