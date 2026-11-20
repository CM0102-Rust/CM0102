# Mutações da 310a leva (itens mwc_18, mwc_19, mwc_20, mwc_21, mwc_22, mwc_23, mwc_24, mwc_25). Uso: python mutacoes.py mutacoes\leva310.py --carreira 2002-06-01
MUTACOES = [
    ('mwc20_sobem_outros', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc20: the best twelve shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled', 'mwc_20 danish lowest division up'),
    ('mwc21_sobem_outros', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc21: the best twelve shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled', 'mwc_21 german regional division up'),
    ('mwc22_quarta_belga', 'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x2c); // mwc22: the best 132 shuffled', 'exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x16); // mutation: the best 66', 'mwc_22 belgian cup entrants'),
    ('mwc22_amadores_alemaes', '&& k * 6 < 0x18 { // mwc22: up to 4 relegated amateur clubs', '&& k * 6 < 0xc { // mutation: up to 2', 'mwc_22 german cup entrants'),
    ('mwc23_vice_no_lugar_do_campeao', 'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwc23: the league champion', 'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 1)); // mutation: the runner-up', 'mwc_23 super cup clubs'),
    ('mwc23_ligapokal_sem_cabecas', 'if k >= 4 { // mwc23: the holder and the best placed after him seeded', 'if k >= 6 { // mutation: nobody seeded', 'mwc_23 german league cup entrants'),
    ('mwc24_periodos_belgas', 'add_result_by_period(this, fixture, arg, &[10, 22], 0x4204d0) // mwc24: the Belgian periods', 'add_result_by_period(this, fixture, arg, &[12, 22], 0x4204d0) // mutation: the second period from round 12', 'mwc_24 belgian second division result'),
    ('mwc24_periodos_holandeses', 'add_result_by_period(this, fixture, arg, &[8, 16, 26], 0x5f6ad0) // mwc24: the Dutch periods', 'add_result_by_period(this, fixture, arg, &[10, 16, 26], 0x5f6ad0) // mutation: the second period from round 10', 'mwc_24 eerste divisie result'),
]
