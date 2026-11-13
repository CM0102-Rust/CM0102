# Mutacoes (mwa_3), na Copa de Munster, carreira ENG IRL NIR ate 2002-08-01:
# - a preliminar jogada uma semana depois (23/8 -> 30/8);
# - o embaralhamento so entre os doze melhores (outros clubes regionais entram);
# - a virada sobe o ano em 2.
MUTACOES = [
    ('mwa3_preliminar_semana_depois',
     'cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa3: the preliminary round of the Munster cup',
     'cql10r((1, 7, 0, 1), (0x1e, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mutation: a week later',
     'mwa_3 munster cup rounds'),
    ('mwa3_sorteio_entre_os_doze',
     'exe::comp_util_sort_and_shuffle_top_clubs(munster.as_mut_ptr() as usize, n as i32 as u32, 0x10);',
     'exe::comp_util_sort_and_shuffle_top_clubs(munster.as_mut_ptr() as usize, n as i32 as u32, 4); // mutation: only the top twelve shuffled',
     'mwa_3 munster cup entrants'),
    ('mwa3_virada_ano_mais_dois',
     'start_new_cup_season(Cup(this), 0x638e30, 0x9e9f84, [0x179, 0x180, 0x187], 1, "mwa_3 munster cup turnover")',
     'start_new_cup_season(Cup(this), 0x638e30, 0x9e9f84, [0x179, 0x180, 0x187], 2, "mwa_3 munster cup turnover")',
     'mwa_3 munster cup turnover'),
]
