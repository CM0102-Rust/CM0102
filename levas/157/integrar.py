"""Integra a 157a leva ao motor.rs (sobre o commit da 156a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b5260, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
];
const N: usize = 345;""", """    Prologo { endereco: 0x6b5260, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6d7b00, bytes: &[0xb8, 0x18, 0x26, 0x00, 0x00] },
];
const N: usize = 346;""")
troca("""        confirmar_partida_em_rede as *const () as usize,
    ];""", """        confirmar_partida_em_rede as *const () as usize,
        narrar_no_painel as *const () as usize,
    ];""")
troca("""        comparar_confirmar_partida_em_rede as *const () as usize,
    ];""", """        comparar_confirmar_partida_em_rede as *const () as usize,
        comparar_narrar_no_painel as *const () as usize,
    ];""")
troca('"motor_confirmar_partida_em_rede",\n', '"motor_confirmar_partida_em_rede", "motor_narrar_no_painel",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
