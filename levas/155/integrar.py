"""Integra a 155a leva ao motor.rs (sobre o commit da 154a e a correcao 601d37c)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b55c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
];
const N: usize = 342;""", """    Prologo { endereco: 0x6b55c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x6d9e50, bytes: &[0x81, 0xec, 0x0c, 0x06, 0x00, 0x00] },
];
const N: usize = 343;""")
troca("""        passo_em_rede as *const () as usize,
    ];""", """        passo_em_rede as *const () as usize,
        ler_eventos_cfg as *const () as usize,
    ];""")
troca("""        comparar_passo_em_rede as *const () as usize,
    ];""", """        comparar_passo_em_rede as *const () as usize,
        comparar_ler_eventos_cfg as *const () as usize,
    ];""")
troca('"motor_passo_em_rede",\n', '"motor_passo_em_rede", "motor_ler_eventos_cfg",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
