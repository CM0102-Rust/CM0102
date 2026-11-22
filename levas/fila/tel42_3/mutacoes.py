# The club history lists (screens): no mutation by the career (the screens batch runs none). For a session of Lucas's
# with JOGAR_COMPARANDO.bat, caught by the comparers (side by side):
# - the current season is shown even when the league is over: Club -> History after the season's last day (the league
#   history list gets one record more than the original's);
# - the current league record keeps the position + 1: the club screen header of any club (the record differs).
MUTACOES = [
    ('tel42_3_temporada_acabada',
     '        && complete == 0 // tel42_3: the season not over',
     '        && complete >= 0 // tel42_3: the season not over (mutation)',
     'tel42_3 current league season'),
    ('tel42_3_posicao',
     'write_u8(rec as usize + 6, read_u8(row + league_table_row::POSITION)); // tel42_3: the position',
     'write_u8(rec as usize + 6, read_u8(row + league_table_row::POSITION).wrapping_add(1)); // tel42_3: the position (mutation)',
     'tel42_3 current league row'),
]
