# Mutacoes (sbD_9), pela carreira com a Argentina ate 2002-08-01 e pelos comparadores (o init conferido contra as linhas
# semeadas; o update lado a lado nos promedios):
# - uma linha semeada com 1 ponto a mais (o segundo clube, a temporada mais nova);
# - as zonas substituem os pontos da temporada em vez de soma-los.
MUTACOES = [
    ('sbD_9_linha_semeada', '    (0x9d01ec, 63, 34, 50, 26),', '    (0x9d01ec, 63, 34, 51, 26), // mutation',
     'sbD_9 argentine second division averages seeded'),
    ('sbD_9_zonas_substituem', 'write_i16(a + AVERAGE_ROW_SEASONS + 6, read_i16(a + AVERAGE_ROW_SEASONS + 6).wrapping_add(read_i16(row.addr() + league_table_row::POINTS)));',
     'write_i16(a + AVERAGE_ROW_SEASONS + 6, read_i16(row.addr() + league_table_row::POINTS)); // mutation',
     'sbD_9 argentine second division averages updated'),
]
