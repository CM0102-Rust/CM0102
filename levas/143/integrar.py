"""Integra a 143a leva ao motor.rs (sobre o commit anterior a leva)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b):
    global s
    assert s.count(a) == 1, a[:80]
    s = s.replace(a, b, 1)


troca("""    Prologo { endereco: 0x70a910, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
];
const N: usize = 318;""", """    Prologo { endereco: 0x70a910, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] },
];
const N: usize = 319;""")
troca("""        chute_a_gol as *const () as usize,
    ];""", """        chute_a_gol as *const () as usize,
        mover_jogador as *const () as usize,
    ];""")
troca("""        comparar_chute_a_gol as *const () as usize,
    ];""", """        comparar_chute_a_gol as *const () as usize,
        comparar_mover as *const () as usize,
    ];""")
troca('"motor_chute_a_gol",\n', '"motor_chute_a_gol", "motor_mover",\n')
# a porta de gravacao da 0x6f09f0 sai: o comparador grava a unidade
troca("""        if let Some(t) = desvios::desviar("motor_porta_6f09f0 (gravacao)", &PROLOGO_6F09F0, porta_6f09f0 as *const () as usize) {
            PORTA_6F09F0 = t;
        }
""", "")
i = s.index('unsafe extern "thiscall" fn porta_6f09f0(')
j = s.index('\n}\n', i) + 3
s = s[:i] + s[j:]
troca("static mut PORTA_6F09F0: usize = 0;\n", "")
troca("const PROLOGO_6F09F0: Prologo = Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] };\n", "")
troca("    if gravando_fechamento() || gravando_lesao() {", "    if gravando_fechamento() || gravando_lesao() || gravando_movimento() {")
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
i = s.index('unsafe extern "thiscall" fn comparar_chute_a_gol')
j = s.index('\n}\n', i) + 3
s = s[:j] + novo + s[j:]
open(p, 'w', encoding='utf-8').write(s)
print('ok')
