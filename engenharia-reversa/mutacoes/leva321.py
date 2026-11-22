# Mutações da 321a leva (itens tel41_5, tel41_6, tel42_1). Uso: python mutacoes.py mutacoes\leva321.py --prova-save 321
MUTACOES = [
    ('tel41_5_player_entry_short', '        comp_stats_write(read_usize(stats + comp_stats_obj::PLAYERS), comp_stats_obj::PLAYER_ENTRY_SIZE as u32,', '        comp_stats_write(read_usize(stats + comp_stats_obj::PLAYERS), comp_stats_obj::PLAYER_ENTRY_SIZE as u32 - 1,'),
    ('tel41_5_season_records', 'comp_stats_write(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x186d); // tel41_5: the season records', 'comp_stats_write(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x186d); // mutation: the all-time records'),
    ('tel41_6_club_attendance', 'file, 0x1777); // tel41_6: club entries read', 'file, 0x1777); write_u32(table + comp_stats_club::HOME_ATTENDANCE_TOTAL, read_u32(table + comp_stats_club::HOME_ATTENDANCE_TOTAL).wrapping_add(1)); // mutation'),
    ('tel41_6_season_records', 'comp_stats_read(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17c6); // tel41_6: the season records', 'comp_stats_read(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17c6); // mutation: into the all-time records'),
    ('tel42_1_sem_temporada', '            write_u32(r + field, u32::MAX); // tel42_1: no season yet', '            write_u32(r + field, 0); // tel42_1: no season yet (mutation)', 'tel42_1 new club history'),
    ('tel42_1_carga_perdida', '    exe::zclose(&mut z);\n    1\n}\n\n/// Comparer for `club_history_load`.', '    exe::zclose(&mut z);\n    0 // tel42_1 mutation\n}\n\n/// Comparer for `club_history_load`.', 'tel42_1 load club history'),
]
