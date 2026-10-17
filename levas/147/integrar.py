"""Integra a 147a leva ao motor.rs (sobre o commit anterior a leva)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] },
];
const N: usize = 322;""", """    Prologo { endereco: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d5950, bytes: &[0x51, 0x8d, 0x44, 0x24, 0x02] },
    Prologo { endereco: 0x6d5a70, bytes: &[0x53, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x702050, bytes: &[0x81, 0xec, 0x84, 0x06, 0x00, 0x00] },
];
const N: usize = 325;""")
troca("""        executar_passe as *const () as usize,
    ];""", """        executar_passe as *const () as usize,
        atras_no_agregado as *const () as usize,
        companheiro_livre as *const () as usize,
        chutar as *const () as usize,
    ];""")
troca("""        comparar_executar_passe as *const () as usize,
    ];""", """        comparar_executar_passe as *const () as usize,
        comparar_atras_no_agregado as *const () as usize,
        comparar_companheiro_livre as *const () as usize,
        comparar_chutar as *const () as usize,
    ];""")
troca('"motor_executar_passe",\n', '"motor_executar_passe", "motor_atras_no_agregado", "motor_companheiro_livre", "motor_chutar",\n')
# a porta de gravacao da 0x702050 sai: o comparador grava a unidade sempre que ha fita
i = s.index('        if let Some(t) = desvios::desviar("motor_porta_702050 (gravacao)"')
j = s.index('        }\n', i) + len('        }\n')
s = s[:i] + s[j:]
i = s.index('// Porta de 0x702050 (a cobranca')
j = s.index('\n}\n', i) + 3
s = s[:i] + s[j:].lstrip('\n')
troca("    let chute = GRAVAR_702050.swap(true, Relaxed);\n", "")
troca("    GRAVAR_702050.store(chute, Relaxed);\n", "")
troca("    GRAVAR_702050.store(true, Relaxed);\n", "")
troca("    GRAVAR_702050.store(false, Relaxed);\n", "")
troca("""unsafe extern "thiscall" fn comparar_penaltis(m: usize, modo: u32) {
    use std::sync::atomic::Ordering::Relaxed;
""", """unsafe extern "thiscall" fn comparar_penaltis(m: usize, modo: u32) {
""")
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
troca("comparar_valor!(comparar_perto_do_gol, perto_do_gol, 78, () -> u8);\n", "comparar_valor!(comparar_perto_do_gol, perto_do_gol, 78, () -> u8);\n"
      "comparar_valor!(comparar_atras_no_agregado, atras_no_agregado, 322, (lado: i8) -> u8);\n"
      "comparar_valor!(comparar_companheiro_livre, companheiro_livre, 323, (p: usize) -> usize);\n")
open(p, 'w', encoding='utf-8').write(s)
print('ok')
