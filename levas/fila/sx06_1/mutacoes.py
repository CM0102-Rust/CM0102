# Mutacoes (sx06_1). A 0x449590 roda milhares de vezes (os recordes de cada partida, 0x444930 e companhia): o comparador
# dela (lado a lado pelo retorno) acusa as duas primeiras na primeira chamada do tipo; a de clube tambem muda os
# recordes e as noticias da carreira. A 0x447880 roda no fim da temporada de cada nacao (40 vezes ate 2002-06-01; por
# isso ATE = 2002-06-01 no rotinas.py): o que ela grava (registros, entradas, arquivo do historico) a carreira quase nao
# le de volta antes de junho; o comparador dela acusa as quatro ultimas (a temporada nao zerada e a contagem de
# temporadas pela memoria; o ano no arquivo pelo plano; o ano da nota pelo registro de todos os tempos, quando a nota da
# temporada supera a dele, o que acontece no primeiro fim de temporada de quase todo clube com nota).
MUTACOES = [
    ('sx06_tipo_de_clube',
     '!national() // sx06: club sides only',
     'national() // mutation: club types for national teams only',
     'sx06_1 club-only record type'),
    ('sx06_jogador_da_torcida',
     'period as u8 != 1 && !national() // sx06: not all-time',
     'period as u8 != 2 && !national() // mutation: only all-time',
     'sx06_1 club season record type'),
    ('sx06_temporada_nao_zerada',
     '    reset_records(season);\n    close_club_season(entry);',
     '    let _ = season; // mutation: the season record is not reset\n    close_club_season(entry);',
     'sx06_1 season archived'),
    ('sx06_temporadas_contadas',
     'entry.set_seasons(entry.seasons().wrapping_add(1)); // sx06: one more season',
     'entry.set_seasons(entry.seasons()); // mutation: the season is not counted',
     'sx06_1 seasons counted'),
    ('sx06_ano_no_arquivo',
     "record[..2].copy_from_slice(&year.to_le_bytes()); // sx06: the archived season's year",
     "record[..2].copy_from_slice(&year.wrapping_add(1).to_le_bytes()); // mutation: the next year",
     'sx06_1 season archived'),
    ('sx06_ano_da_nota',
     "write_u16(all_time + club_record::SEASON_RATING_YEAR, year); // sx06: the all-time rating's year",
     "write_u16(all_time + club_record::SEASON_RATING_YEAR, 0); // mutation: no year",
     'sx06_1 all-time rating'),
]
