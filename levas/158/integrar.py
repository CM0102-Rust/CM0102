"""Integra a 158a leva ao motor.rs e ao win.rs (sobre o commit da 157a)."""
import os
aqui = os.path.dirname(os.path.abspath(__file__))
src = os.path.join(aqui, '..', '..', 'rust', 'src')

w = os.path.join(src, 'win.rs')
t = open(w, encoding='utf-8').read()
a = "    pub fn GetModuleHandleW(nome: *const u16) -> *mut c_void;\n"
assert t.count(a) == 1
t = t.replace(a, a + "    pub fn SetFilePointer(h: *mut c_void, baixo: i32, alto: *mut i32, metodo: u32) -> u32;\n    pub fn GetLastError() -> u32;\n")
open(w, 'w', encoding='utf-8').write(t)

p = os.path.join(src, 'motor.rs')
s = open(p, encoding='utf-8').read()


def troca(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:80], s.count(a))
    s = s.replace(a, b)


troca("""    Prologo { endereco: 0x6d7b00, bytes: &[0xb8, 0x18, 0x26, 0x00, 0x00] },
];
const N: usize = 346;""", """    Prologo { endereco: 0x6d7b00, bytes: &[0xb8, 0x18, 0x26, 0x00, 0x00] },
    Prologo { endereco: 0x9326e0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x933080, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x933170, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
];
const N: usize = 349;""")
troca("""        narrar_no_painel as *const () as usize,
    ];""", """        narrar_no_painel as *const () as usize,
        abrir_no_conteiner as *const () as usize,
        buscar_no_conteiner as *const () as usize,
        posicao_no_conteiner as *const () as usize,
    ];""")
troca("""        comparar_narrar_no_painel as *const () as usize,
    ];""", """        comparar_narrar_no_painel as *const () as usize,
        comparar_abrir_no_conteiner as *const () as usize,
        comparar_buscar_no_conteiner as *const () as usize,
        comparar_posicao_no_conteiner as *const () as usize,
    ];""")
troca('"motor_narrar_no_painel",\n', '"motor_narrar_no_painel", "motor_abrir_no_conteiner", "motor_buscar_no_conteiner", "motor_posicao_no_conteiner",\n')
novo = open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read()
anc = '// ---------- centesima trigesima nona leva: o fechamento da partida'
troca(anc, novo.lstrip('\n') + '\n' + anc)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
