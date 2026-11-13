# Mutacoes (mwa_6), na Copa da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - a primeira rodada uma semana depois (20/1 -> 27/1);
# - o embaralhamento so entre os dezoito melhores da divisao de baixo (outros clubes entram);
# - a virada sobe o ano em 2.
MUTACOES = [
    ('mwa6_primeira_rodada_semana_depois',
     'cql10r((0xa, 0xb, 0, 6), (0x14, 0, 1, 5, 1, 0), 0x32, (8, 0, 3), 4, (0x20, 0x10, 0x20, 0), (1, 1, 4), (0, 0, 0)), // mwa6: the fifth round of the Irish Cup',
     'cql10r((0xa, 0xb, 0, 6), (0x1b, 0, 1, 5, 1, 0), 0x32, (8, 0, 3), 4, (0x20, 0x10, 0x20, 0), (1, 1, 4), (0, 0, 0)), // mutation: a week later',
     'mwa_6 irish cup rounds'),
    ('mwa6_sorteio_entre_os_dezoito',
     'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as i32 as u32, 0xc);',
     'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as i32 as u32, 6); // mutation: only the top eighteen shuffled',
     'mwa_6 irish cup entrants'),
    ('mwa6_virada_ano_mais_dois',
     'start_new_cup_season(Cup(this), 0x78ac90, 0xa69628, [0x17e, 0x185, 0x18c], 1, "mwa_6 irish cup turnover")',
     'start_new_cup_season(Cup(this), 0x78ac90, 0xa69628, [0x17e, 0x185, 0x18c], 2, "mwa_6 irish cup turnover")',
     'mwa_6 irish cup turnover'),
]
