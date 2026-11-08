# Mutacoes (cqa_10), todas no 0x6aaee0 (10 chamadas ate 2002-06-01 na cobertura de 17/10, pela noticia da diretoria
# depois do jogo 0x6a5fa0; o contador "cqa_10 the board draws" diz quantas passam das condicoes).
# - Os dois sorteios do comeco (o da ficha do tecnico e o sorteio(10)) saem sempre que o clube tem tecnico com ficha e
#   nao e selecao: sem um deles a fila do sorteio anda uma casa a menos e todos os sorteios seguintes mudam.
# - A confianca da diretoria (+6) cai 10 x o +0x57 do tecnico pelos jogadores rejeitados com nota abaixo de 7: sem a
#   queda, a confianca fica outra (pesa na demissao do tecnico). Pode escapar: o ramo e raro (pass_ball no sorteio, ter
#   rejeitado do clube abaixo da injury_word_0c e com nota baixa); o contador "cqa_10 board blames" diz se rodou.
MUTACOES = [
    ('cqa10_diretoria_sem_sorteio', '    let r2 = sorteio(10); // cqa10: o sorteio da diretoria',
     '    let r2 = 0; // mutacao: sem o sorteio da diretoria',
     'cqa_10 the board draws'),
    ('cqa10_tecnico_sem_sorteio', '    let r1 = sorteio(cqa10_b(cqa10_d(t + 0x69) + 0x16) as i32); // cqa10: o sorteio pela ficha do técnico',
     '    let r1 = 10; // mutacao: sem o sorteio pela ficha do técnico',
     'cqa_10 the board draws'),
    ('cqa10_rejeitados_sem_culpa', '        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_sub(x.wrapping_mul(10))); // cqa10: a confiança pelos rejeitados',
     '        let _ = (conf, x); // mutacao: a diretoria não culpa os rejeitados',
     'cqa_10 board blames'),
]
