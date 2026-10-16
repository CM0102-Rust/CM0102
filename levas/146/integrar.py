"""Integra a 146a leva ao motor.rs (sobre o commit anterior a leva)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b):
    global s
    assert s.count(a) == 1, a[:80]
    s = s.replace(a, b, 1)


troca("""    Prologo { endereco: 0x6b98c0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
];
const N: usize = 321;""", """    Prologo { endereco: 0x6b98c0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] },
];
const N: usize = 322;""")
troca("""        laco_do_motor as *const () as usize,
    ];""", """        laco_do_motor as *const () as usize,
        executar_passe as *const () as usize,
    ];""")
troca("""        comparar_laco_do_motor as *const () as usize,
    ];""", """        comparar_laco_do_motor as *const () as usize,
        comparar_executar_passe as *const () as usize,
    ];""")
troca('"motor_laco",\n', '"motor_laco", "motor_executar_passe",\n')
# a porta de gravacao da 0x6fad40 sai: o comparador grava a unidade
i = s.index('        if let Some(t) = desvios::desviar("motor_porta_6fad40 (gravacao)"')
j = s.index('        }\n', i) + len('        }\n')
s = s[:i] + s[j:]
i = s.index('// Porta de 0x6fad40 (executa o passe')
j = s.index('\n}\n', i) + 3
s = s[:i] + s[j:].lstrip('\n')
# o evento 0x82 da falta no passe: a 0x68d0d0 confere o conteudo
troca("    if gravando_fechamento() || gravando_lesao() || gravando_movimento() {",
      "    if gravando_fechamento() || gravando_lesao() || gravando_movimento() || gravando_passe() {")
# o tracado do passe (so a 0x6fad40 o chama) vira unidade da fita dela
troca("""unsafe extern "thiscall" fn comparar_tracar(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;""", """unsafe extern "thiscall" fn comparar_tracar(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    if gravando_no_topo() {
        // unidade da fita da execucao do passe (a unica que o chama); por dentro, a propria comparacao
        let xy = x0 as u8 as u32 | (y0 as u8 as u32) << 8 | (x1 as u8 as u32) << 16 | (y1 as u8 as u32) << 24;
        return chamar_gravando(0x6ccb60, this, [xy, codigo as u8 as u32, a6, a7, 0], || comparar_tracar(this, x0, y0, x1, y1, codigo, a6, a7) as u32) as u8;
    }
    use std::sync::atomic::Ordering::Relaxed;""")
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
