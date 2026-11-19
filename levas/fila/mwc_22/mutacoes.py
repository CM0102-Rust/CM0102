# Mutacoes (mwc_22): na Copa da Belgica o sorteio entre os clubes da Quarta Divisao embaralha so os 66 melhores (outros
# entram); na Copa da Alemanha entram direto ate 2 amadores rebaixados da 2. Bundesliga em vez de 4 (outros amadores
# entram). Os jogos das copas de 2001/02 mudam.
MUTACOES = [
    ('mwc22_quarta_belga',
     'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x2c); // mwc22: the best 132 shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x16); // mutation: the best 66',
     'mwc_22 belgian cup entrants'),
    ('mwc22_amadores_alemaes',
     '&& k * 6 < 0x18 { // mwc22: up to 4 relegated amateur clubs',
     '&& k * 6 < 0xc { // mutation: up to 2',
     'mwc_22 german cup entrants'),
]
