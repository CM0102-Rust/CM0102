"""Integra a 159a leva ao motor.rs (sobre o commit da DLL do save)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x933170, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
];
const N: usize = 349;""", """    Prologo { endereco: 0x933170, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x5b7d20, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b7e60, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b8960, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
];
const N: usize = 352;""")
troca("""        posicao_no_conteiner as *const () as usize,
    ];""", """        posicao_no_conteiner as *const () as usize,
        mascara_de_posicoes as *const () as usize,
        casa_no_campo as *const () as usize,
        vagas_sem_conflito as *const () as usize,
    ];""")
troca("""        comparar_posicao_no_conteiner as *const () as usize,
    ];""", """        comparar_posicao_no_conteiner as *const () as usize,
        comparar_mascara_de_posicoes as *const () as usize,
        comparar_casa_no_campo as *const () as usize,
        comparar_vagas_sem_conflito as *const () as usize,
    ];""")
troca('"motor_posicao_no_conteiner",', '"motor_posicao_no_conteiner", "motor_mascara_de_posicoes", "motor_casa_no_campo", "motor_vagas_sem_conflito",')
# depois do comparar_valor! (o macro tem de vir antes no texto)
anc = 'comparar_valor!(comparar_companheiro_livre, companheiro_livre, 323, (p: usize) -> usize);\n'
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
troca(anc, anc + '\n' + novo.strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
