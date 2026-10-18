"""Integra a 160a leva ao motor.rs (sobre o commit da 159a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x5b8960, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
];
const N: usize = 352;""", """    Prologo { endereco: 0x5b8960, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
    Prologo { endereco: 0x7c2150, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c1950, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b8dc0, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
];
const N: usize = 355;""")
troca("""        vagas_sem_conflito as *const () as usize,
    ];""", """        vagas_sem_conflito as *const () as usize,
        estatistica_do_jogador as *const () as usize,
        media_das_notas as *const () as usize,
        zagueiros_sem_conflito as *const () as usize,
    ];""")
troca("""        comparar_vagas_sem_conflito as *const () as usize,
    ];""", """        comparar_vagas_sem_conflito as *const () as usize,
        comparar_estatistica_do_jogador as *const () as usize,
        comparar_media_das_notas as *const () as usize,
        comparar_zagueiros_sem_conflito as *const () as usize,
    ];""")
troca('"motor_vagas_sem_conflito",', '"motor_vagas_sem_conflito", "motor_estatistica_do_jogador", "motor_media_das_notas", "motor_zagueiros_sem_conflito",')
anc = 'comparar_valor!(comparar_vagas_sem_conflito, vagas_sem_conflito, 351, () -> u8);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
