# Mutacoes (rx12_14), pela carreira com as 8 ligas ate 2003-08-15. So rodam com um empate numa fronteira de faixa no
# fim da temporada italiana (sorte): sem ele, escapam (a rotina nao roda).
# - qualquer empate vai ao desempate (tambem dentro da mesma faixa);
# - os empatados nao ficam a espera do play-off (situacao 0xff no lugar de 1: o resultado fecha a liga de cara).
MUTACOES = [
    ('rx12_14_qualquer_empate',
     '        && zones[(i as usize).min(63)] != zones[(i as usize + 1).min(63)] // rx12_14: only across a band boundary',
     '        && true // mutation',
     'rx12_14 italian tie-break play-off'),
    ('rx12_14_sem_espera',
     '            write_u8(r + league_table_row::STATUS, 1); // rx12_14: the pair waits for the play-off',
     '            write_u8(r + league_table_row::STATUS, 0xff); // mutation',
     'rx12_14 italian tie-break play-off'),
]
