"""Integra a 144a leva ao motor.rs (sobre o commit anterior a leva)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b):
    global s
    assert s.count(a) == 1, a[:80]
    s = s.replace(a, b, 1)


troca("""    Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] },
];
const N: usize = 319;""", """    Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] },
];
const N: usize = 320;""")
troca("""        mover_jogador as *const () as usize,
    ];""", """        mover_jogador as *const () as usize,
        conduzir as *const () as usize,
    ];""")
troca("""        comparar_mover as *const () as usize,
    ];""", """        comparar_mover as *const () as usize,
        comparar_conduzir as *const () as usize,
    ];""")
troca('"motor_mover",\n', '"motor_mover", "motor_conduzir",\n')
# a porta de gravacao da 0x7109f0 sai: o comparador grava a unidade
i = s.index('        if let Some(t) = desvios::desviar("motor_porta_7109f0 (gravacao)"')
j = s.index('        }\n', i) + len('        }\n')
s = s[:i] + s[j:]
i = s.index('// Porta de 0x7109f0 (o jogador com a bola, 13 KB)')
j = s.index('\n}\n', i) + 3
s = s[:i] + s[j:].lstrip('\n')
# o original determinizado: a 0x7109f0 le lixo da pilha (as notas do passe e da pressao no modo 4)
troca("""    (Prologo { endereco: 0x70c040, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] }, 0x2000),
];""", """    (Prologo { endereco: 0x70c040, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] }, 0x2000),
    (Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] }, 0x2000),
];""")
troca("const ZERADAS: [(Prologo, u32); 4] = [", "const ZERADAS: [(Prologo, u32); 5] = [")
# a copia da pressao para a lista na pilha da conducao: sem o endereco da lista
troca("    chamar_gravando(0x68d0d0, l, [p as u32, n, 0, 0, 0], || {", "    chamar_gravando(0x68d0d0, lista_gravada(l, n), [p as u32, n, 0, 0, 0], || {")
troca("            chamar(0x68d0d0, l, [p as u32, 0x1be, 0, 0, 0], [u32::MAX; 5]);", "            chamar(0x68d0d0, lista_gravada(l, 0x1be), [p as u32, 0x1be, 0, 0, 0], [u32::MAX; 5]);")
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
