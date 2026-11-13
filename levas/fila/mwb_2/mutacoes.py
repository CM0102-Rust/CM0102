# Mutacoes (mwb_2): as quartas da Copa da Escocia uma semana antes (11/3 -> 4/3, depois do sorteio de 20/2: outro dia,
# outro resultado); os 8 clubes de fora da liga sorteados entre os 12 primeiros em vez dos 24 (outros clubes entram);
# sem as 20 trocas da First (outra ordem, outro sorteio).
MUTACOES = [
    ('mwb2_quartas_uma_semana_antes',
     'cql10r((0x14, 1, 1, 6), (0xb, 2, 1, 5, 1, 0), 0x32, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0x4e20, 0)), // mwb2: the Scottish Cup quarter-finals',
     'cql10r((0x14, 1, 1, 6), (4, 2, 1, 5, 1, 0), 0x32, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0x4e20, 0)), // mutation: on 4/3',
     'mwb_2 scottish cup rounds'),
    ('mwb2_fora_da_liga_entre_doze',
     'exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, m as i16 as u16 as u32, 8); // mwb2: the eight non-league clubs',
     'exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, m as i16 as u16 as u32, 4); // mutation: among the first 12',
     'mwb_2 scottish cup entrants'),
    ('mwb2_sem_trocas_da_first',
     'for _ in 0..0x14 { // mwb2: the swaps among the last ten',
     'for _ in 0..0 { // mutation: no swaps',
     'mwb_2 scottish cup entrants'),
]
