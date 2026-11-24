# Mutacoes (cx13_9), com a Escocia ate 2002-08-01:
# - o split com a metade de baixo a partir do 8º (o 7º do split pode passar os seis primeiros: outra tabela final e
#   outras vagas; acusada tambem pelo compare_order!).
# Os recordes e o texto da First ficam sem mutacao propria: fora de 1999 o texto so passa ao comum (o ramo do grupo da
# repescagem nao roda numa carreira de 2001) e os recordes usam o corpo do cx13_6, que tem as mutacoes dele; a prova
# deles e a comparacao (o retorno e o status da linha contra o plano).
MUTACOES = [
    ('cx13_9_split_a_partir_do_oitavo',
     'if read_u8(read_usize(row as usize + league_table_row::POSITION_HISTORY) + 0x20) as i8 >= 6 { 2 } else { 1 }; // cx13_9: the split\'s bottom six',
     'if read_u8(read_usize(row as usize + league_table_row::POSITION_HISTORY) + 0x20) as i8 >= 7 { 2 } else { 1 }; // mutation: from 8th',
     'cx13_9 scottish split order'),
]
