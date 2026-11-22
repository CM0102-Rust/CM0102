# As escapadas da 321a leva (itens de save da rodada 2 das telas: tel41_5, tel41_6, tel42_1; prova do save).

ESCAPADAS = {
    'tel41_6_club_attendance': (
        'LACUNA EXPLICADA. Na carga das estatisticas da competicao, o total de publico em casa do primeiro clube sobe 1 '
        'depois de lido. A prova da carga segue a carreira ate 2001-239 depois de carregar, e o total de publico so e lido '
        'pelas medias de publico das telas e pelo recorde de publico do fim da temporada; nada que a simulacao leia antes '
        'disso depende dele. A gravacao da mesma tabela foi acusada pela irma tel41_5_player_entry_short (o item da Copa '
        'America muda de tamanho).',
        'EXPLAINED GAP. In the competition statistics load, the first club\'s home attendance total goes up by 1 after it '
        'is read. The load proof follows the career to 2001-239 after loading, and the attendance total is only read by '
        'the screens\' attendance averages and the season-end attendance record; nothing the simulation reads before then '
        'depends on it. Saving the same table was caught by the sibling tel41_5_player_entry_short (the Copa America item '
        'changes size).'),
    'tel41_6_season_records': (
        'LACUNA EXPLICADA. Na carga, os recordes da temporada vao para a area dos recordes de sempre. Os recordes so sao '
        'lidos no fim de cada competicao (update_season_end_records, depois de 2001-239) e pelas telas de estatisticas; a '
        'carreira carregada ate 2001-239 nao os le. A gravacao deles foi acusada pela irma tel41_5_season_records (o item '
        'da primeira divisao inglesa muda em +0xe41).',
        'EXPLAINED GAP. On load, the season records go into the all-time records area. Records are only read at each '
        'competition\'s end (update_season_end_records, after 2001-239) and by the statistics screens; the loaded career '
        'to 2001-239 does not read them. Saving them was caught by the sibling tel41_5_season_records (the English first '
        'division item changes at +0xe41).'),
}  # nome: (explicacao pt, explicacao en)
