# Mutacoes (rx12_15), pela carreira com as 8 ligas ate 2003-08-15. So rodam com um empate numa fronteira da Serie B
# no fim da temporada (sorte): sem ele, escapam.
# - o desempate da Serie B sem faixas (nenhum empate vira play-off);
# - o vencedor do desempate nao sobe acima do adversario.
MUTACOES = [
    ('rx12_15_sem_faixas',
     '    create_italian_tiebreak_playoff(League(this), &serie_b_zones(), 0x9ea384, 0x28e, false); // rx12_15: the Serie B bands',
     '    create_italian_tiebreak_playoff(League(this), &[0u8; 64], 0x9ea384, 0x28e, false); // mutation',
     'rx12_14 italian tie-break play-off'),
    ('rx12_15_vencedor_embaixo',
     '                swap_table_rows(row(k), row(k - 1)); // rx12_15: the winner goes above',
     '                // mutation: no swap',
     'rx12_15 serie b play-off winner'),
]
