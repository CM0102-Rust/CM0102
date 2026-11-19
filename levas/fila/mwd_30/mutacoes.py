# Mutacoes (mwd_30): na Copa da Grecia o sorteio entre os clubes da Terceira Divisao embaralha so os 24 melhores (outros
# entram); na Copa da Polonia os clubes da divisao mais baixa entram com a classe 2 em vez da 3 (o sorteio e as rodadas
# de entrada mudam). Os jogos das copas de 2001/02 mudam; o comparador lado a lado tambem pega.
MUTACOES = [
    ('mwd30_terceira_grega',
     'exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x10); // mwd30: the best 48 third division clubs shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x8); // mutation: the best 24',
     'mwd_30 greek cup entrants'),
    ('mwd30_classe_da_divisao_polonesa_mais_baixa',
     'set_cup_entry(this, k as usize, lower[j as usize], 3); // mwd30: the lowest division clubs class 3',
     'set_cup_entry(this, k as usize, lower[j as usize], 2); // mutation: class 2',
     'mwd_30 polish cup entrants'),
]
