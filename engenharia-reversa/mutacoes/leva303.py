# Mutações da 303a leva (itens mwd_27, mwd_28). Uso: python mutacoes.py mutacoes\leva303.py --carreira 2002-06-01
MUTACOES = [
    ('mwd27_classe_da_categoria_b', 'set_cup_entry(this, k, lower[0x19 - k], 1); // mwd27: category B from the 26th backwards', 'set_cup_entry(this, k, lower[0x19 - k], 2); // mutation: class 2', 'mwd_27 turkish cup entrants'),
    ('mwd27_zonas_russas', 'exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 0xe); // mwd27: the best 42 zone clubs shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 7); // mutation: the best 21', 'mwd_27 russian cup entrants'),
    ('mwd28_polonesa_rebaixa_5', 'let down = if League(this).year() == 0x7d1 { 6 } else { 4 }; // mwd28: six down in 2001', 'let down = if League(this).year() == 0x7d1 { 5 } else { 4 }; // mutation: five down', 'mwd_28 polish second division rules'),
]
