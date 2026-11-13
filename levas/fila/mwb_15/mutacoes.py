# Mutacoes (mwb_15): a virada da Premier pula um ano (o comparador; a temporada seguinte); o split depois de 32 jogos
# em vez de 33 (outra tabela, outros jogos da divisao); os seis de baixo na ordem da tabela em vez de do ultimo para
# cima (outra ordem, outros jogos do split).
MUTACOES = [
    ('mwb15_premier_virada_pula_um_ano',
     'league.set_year(league.year().wrapping_add(1)); // mwb15: the next Premier season',
     'league.set_year(league.year().wrapping_add(2)); // mutation: two years on',
     'mwb_15 scottish premier turnover'),
    ('mwb15_split_depois_de_32',
     'if read_i8(league.table() + k as usize * 0x3b + 0x11) < 0x21 { // mwb15: the split after 33 games',
     'if read_i8(league.table() + k as usize * 0x3b + 0x11) < 0x20 { // mutation: after 32 games',
     'mwb_15 scottish premier split day'),
    ('mwb15_seis_de_baixo_de_cima_para_baixo',
     'let row = (league.club_count() as i32 - k as i32 - 1) as usize; // mwb15: the bottom six from the last row up',
     'let row = (league.club_count() as i32 - 6 + k as i32) as usize; // mutation: the bottom six top down',
     'mwb_15 scottish premier split'),
]
