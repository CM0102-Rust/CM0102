# Mutações da 213a leva (itens sim_lote_a1, sim_lote_a2, sim_lote_a3). Uso: python mutacoes.py mutacoes\leva213.py --carreira 2002-06-01
MUTACOES = [
    ('tr_recusa', '        return 0; // recusado, sem segunda pergunta', '        return 1; // recusado, sem segunda pergunta'),
    ('tr_sem_trocas', '        return 0; // sem trocas marcadas', '        return 1; // sem trocas marcadas'),
    ('nt_total', '        return *total; // total da caixa', '        return *total + 1; // total da caixa'),
    ('nt_marca', 'value); // marca na caixa', 'value ^ 1); // marca na caixa'),
    ('nt_humano', '    1 // humano\n', '    0 // humano\n'),
    ('cr_ano_sorteio', 'a.wrapping_sub(base as u8); // ano do sorteio', 'a.wrapping_sub(base as u8).wrapping_add(1); // ano do sorteio'),
    ('cr_dia_jogo', 'd); // dia do jogo', 'd.wrapping_add(1)); // dia do jogo'),
    ('fr_menos_um', '                *id = 0xffff_ffff;', '                *id = 0xffff_fffe;'),
]
