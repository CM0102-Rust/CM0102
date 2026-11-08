# Mutacoes (cqr_7), nas copas italianas montadas no comeco (2001-000; os grupos jogam em agosto de 2001): as vagas do
# grupo 1 nao saem da lista (os 4 ou 5 clubes entram de novo no grupo seguinte); os dois primeiros clubes do grupo 1 trocam de
# linha (outros jogos no grupo; a troca das linhas em ordem inversa derrubava o jogo); o mata-mata montado duas vezes (um sorteio a mais, 2001-247).
MUTACOES = [
    ('cqr7_vaga_do_grupo_um_fica',
     'core::ptr::write_unaligned((cqr7_d(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: a vaga do grupo 1 sai da lista',
     '// mutacao: a vaga fica',
     'cqr_7 grupo 1 da copa da italia'),
    ('cqr7_dois_primeiros_trocados', '    let mut i: i8 = 0;\n    while (i as i16) < cqr7_w(this + 0x3e) {', '    let mut i: i8 = 0;\n    { let l = cqr7_d(this + 0xee); let a = cqr7_d(l); core::ptr::write_unaligned(l as *mut u32, cqr7_d(l + 4) as u32); core::ptr::write_unaligned((l + 4) as *mut u32, a as u32); } // mutacao\n    while (i as i16) < cqr7_w(this + 0x3e) {', 'cqr_7 grupo 1 da copa da serie c'),
    ('cqr7_mata_mata_duas_vezes',
     'f(this); // cqr7: monta o mata-mata',
     'f(this); f(this); // mutacao',
     'cqr_7 italian cups knockout'),
]
