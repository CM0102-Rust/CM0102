# Mutacoes (cqr_7), nas copas italianas montadas no comeco (2001-000; os grupos jogam em agosto de 2001): as vagas do
# grupo 1 nao saem da lista (os 4 ou 5 clubes entram de novo no grupo seguinte); as linhas do grupo 1 em ordem inversa
# (os jogos do grupo mudam de mando e de data); o mata-mata montado duas vezes (um sorteio a mais, 2001-247).
MUTACOES = [
    ('cqr7_vaga_do_grupo_um_fica',
     'core::ptr::write_unaligned((cqr7_d(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: a vaga do grupo 1 sai da lista',
     '// mutacao: a vaga fica',
     'cqr_7 grupo 1 da copa da italia'),
    ('cqr7_linhas_em_ordem_inversa',
     'iniciar(this, i as u8 as u32, cqr7_d(vaga), 0, 0);',
     'iniciar(this, (cqr7_w(this + 0x3e) as i32 - 1 - i as i32) as u8 as u32, cqr7_d(vaga), 0, 0); // mutacao',
     'cqr_7 grupo 1 da copa da serie c'),
    ('cqr7_mata_mata_duas_vezes',
     'f(this); // cqr7: monta o mata-mata',
     'f(this); f(this); // mutacao',
     'cqr_7 mata-mata das copas italianas'),
]
