"""Integra a 153a leva ao motor.rs (sobre o commit da 152a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6b4360, bytes: &[0x6a, 0xff, 0x68, 0xdc, 0xd1, 0x95, 0x00] },
];
const N: usize = 336;""", """    Prologo { endereco: 0x6b4360, bytes: &[0x6a, 0xff, 0x68, 0xdc, 0xd1, 0x95, 0x00] },
    Prologo { endereco: 0x6b3c10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x714d30, bytes: &[0x8a, 0x41, 0x27, 0xc3, 0x90] },
    Prologo { endereco: 0x6d8c70, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
];
const N: usize = 339;""")
troca("""        jogar_o_dia as *const () as usize,
    ];""", """        jogar_o_dia as *const () as usize,
        montar_o_dia as *const () as usize,
        lado_do_jogador as *const () as usize,
        sons_terminados as *const () as usize,
    ];""")
troca("""        comparar_jogar_o_dia as *const () as usize,
    ];""", """        comparar_jogar_o_dia as *const () as usize,
        comparar_montar_o_dia as *const () as usize,
        comparar_lado_do_jogador as *const () as usize,
        comparar_sons_terminados as *const () as usize,
    ];""")
troca('"motor_jogar_o_dia",\n', '"motor_jogar_o_dia", "motor_montar_o_dia", "motor_lado_do_jogador", "motor_sons_terminados",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
