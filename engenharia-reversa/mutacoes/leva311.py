# Mutações da 311a leva (itens mwd_29, mwd_30, mwd_31). Uso: python mutacoes.py mutacoes\leva311.py --carreira 2002-06-01
MUTACOES = [
    ('mwd29_sobem_outros', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd29: the best twelve shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled', 'mwd_29 polish lowest division up'),
    ('mwd30_terceira_grega', 'exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x10); // mwd30: the best 48 third division clubs shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x8); // mutation: the best 24', 'mwd_30 greek cup entrants'),
    ('mwd30_classe_da_divisao_polonesa_mais_baixa', 'set_cup_entry(this, k as usize, lower[j as usize], 3); // mwd30: the lowest division clubs class 3', 'set_cup_entry(this, k as usize, lower[j as usize], 2); // mutation: class 2', 'mwd_30 polish cup entrants'),
]
