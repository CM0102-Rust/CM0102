# Mutações da 213a leva (itens sim_lote_a1, sim_lote_a2, sim_lote_a3). Uso: python mutacoes.py mutacoes\leva213.py --carreira 2002-06-01
MUTACOES = [
    ('tr_recusa', '        return 0; // refused, no second question', '        return 1; // refused, no second question'),
    ('tr_sem_trocas', '        return 0; // no swaps marked', '        return 1; // no swaps marked'),
    ('nt_total', '        return *total; // message box total', '        return *total + 1; // message box total'),
    ('nt_marca', 'value); // mark in the box', 'value ^ 1); // mark in the box'),
    ('nt_humano', '    1 // humano\n', '    0 // humano\n'),
    ('cr_ano_sorteio', 'a.wrapping_sub(base as u8); // year of the draw', 'a.wrapping_sub(base as u8).wrapping_add(1); // year of the draw'),
    ('cr_dia_jogo', 'd); // match day', 'd.wrapping_add(1)); // match day'),
    ('fr_menos_um', '                *id = 0xffff_ffff;', '                *id = 0xffff_fffe;'),
]
