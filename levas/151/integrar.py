"""Integra a 151a leva ao motor.rs (sobre o commit da 150a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6dbc50, bytes: &[0x6a, 0xff, 0x68, 0x52, 0xd4, 0x95, 0x00] },
];
const N: usize = 332;""", """    Prologo { endereco: 0x6dbc50, bytes: &[0x6a, 0xff, 0x68, 0x52, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x6b5a70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6b5ce0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6b5f00, bytes: &[0x81, 0xec, 0x58, 0x02, 0x00, 0x00] },
];
const N: usize = 335;""")
troca("""        escalar_automatico as *const () as usize,
    ];""", """        escalar_automatico as *const () as usize,
        iniciar_proxima_partida as *const () as usize,
        andar_partida_do_dia as *const () as usize,
        fechar_partida_do_dia as *const () as usize,
    ];""")
troca("""        comparar_escalar_automatico as *const () as usize,
    ];""", """        comparar_escalar_automatico as *const () as usize,
        comparar_iniciar_proxima_partida as *const () as usize,
        comparar_andar_partida_do_dia as *const () as usize,
        comparar_fechar_partida_do_dia as *const () as usize,
    ];""")
troca('"motor_escalar_automatico",\n', '"motor_escalar_automatico", "motor_iniciar_proxima_partida", "motor_andar_partida_do_dia", "motor_fechar_partida_do_dia",\n')
# o aviso de erro (sem parar o jogo) passa a receber o arquivo do codigo-fonte
troca("""/// A caixa de erro "v%s %s.%s %d" de match_man.cpp na linha `linha` (o jogo segue).
unsafe fn aviso_de_match_man(linha: u32) {""", """/// A caixa de erro "v%s %s.%s %d" do fonte `arquivo` (0xa15da4 match_man.cpp, 0xa15708
/// match_day.cpp) na linha `linha` (o jogo segue).
unsafe fn aviso_em(arquivo: usize, linha: u32) {""")
troca("""    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa15da4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}
""", """    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}
""")
troca("aviso_de_match_man(0x4c9);", "aviso_em(0xa15da4, 0x4c9);")
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
