# Mutacoes (cqs_13), na Copa Africana de 2002 (sorteio e jogos dentro da janela): so as 16 primeiras africanas na ordem
# das nacoes entram (em vez de 50); as nao sedes sem a ordem pela injury_word_0c (os potes mudam); os times de cada fase de
# tras para a frente (o mando e a ordem dos jogos de cada rodada mudam). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqs13_so_16_africanas',
     'n < 0x32 { // cqs13: até 50 africanas',
     'n < 0x10 { // mutacao: so 16',
     'cqs_13 african cup draw'),
    ('cqs13_sem_ordem_de_reputacao',
     'ordenar(cqs13_d(this + 0xee) + (h as isize * 4) as usize, (n as i32 - h) as u32, 4, 0x4b07d0); // cqs13: as não sedes pela reputação',
     'let _ = (ordenar, h); // mutacao: sem ordenar',
     'cqs_13 african cup draw'),
    ('cqs13_times_de_tras_para_frente',
     'cqs13_pd(times + n * 4, cqs13_d(cqs13_d(this + 0xee) + ((g as usize + 1) * 4 + n) * 4)); // cqs13: o time na vaga do grupo',
     'cqs13_pd(times + (3 - n) * 4, cqs13_d(cqs13_d(this + 0xee) + ((g as usize + 1) * 4 + n) * 4)); // mutacao: de tras para a frente',
     'cqs_13 group team'),
]
