# Mutacoes (mwd_27): os 26 clubes da categoria B entram na Copa da Turquia com a classe 2 (o sorteio muda); na Copa da
# Russia o sorteio entre os clubes das zonas embaralha so os 21 melhores (outros clubes entram).
MUTACOES = [
    ('mwd27_classe_da_categoria_b',
     'set_cup_entry(this, k, lower[0x19 - k], 1); // mwd27: category B from the 26th backwards',
     'set_cup_entry(this, k, lower[0x19 - k], 2); // mutation: class 2',
     'mwd_27 turkish cup entrants'),
    ('mwd27_zonas_russas',
     'exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 0xe); // mwd27: the best 42 zone clubs shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 7); // mutation: the best 21',
     'mwd_27 russian cup entrants'),
]
