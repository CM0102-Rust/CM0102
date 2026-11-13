# Mutacoes (mwf_16), na virada italiana de meados de 2002: tres sobem e tres descem entre a A e a B em vez de quatro
# (a divisao de dois clubes muda); os clubes da C2 sem embaralhar (largura 1 em vez de 6: os que sobem mudam, o mesmo
# numero); os 36 da C1 de sul a norte (a ordenacao ao contrario: os grupos trocam; o comparador do reagrupamento tambem
# a pega). Acusadas pelo estado no fim da carreira, 2002-08-01.
MUTACOES = [
    ('mwf16_tres_sobem_tres_descem',
     'exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, 4, 4); // mwf16: four up and four down between A and B',
     'exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, 3, 3); // mutation: three up and three down',
     'mwf_16 italian chain'),
    ('mwf16_c2_sem_embaralhar',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 6); // mwf16: the C2 clubs by reputation, shuffled in sixes',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 1); // mutation: no shuffle',
     'mwf_16 serie c2 up to c1'),
    ('mwf16_c1_de_sul_a_norte',
     'exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize); // mwf16: north to south',
     'exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize); list[..n as usize].reverse(); // mutation: south to north',
     'mwf_16 serie c1 regrouping'),
]
