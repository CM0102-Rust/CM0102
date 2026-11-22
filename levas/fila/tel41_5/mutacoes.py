# Mutations (tel41_5) by the save proof (mutacoes.py --prova-save N): each one changes the competition .tmp files that
# go into the .qst (fwrite returns the record count, so none falls into the error box). No 4th field (counter): the
# base of the save proof runs with the routines off.
# - the player entries written with 0x2c bytes (every competition .tmp with players shorter and shifted);
# - the season records written from the all-time ones.
MUTACOES = [
    ('tel41_5_player_entry_short',
     '        comp_stats_write(read_usize(stats + comp_stats_obj::PLAYERS), comp_stats_obj::PLAYER_ENTRY_SIZE as u32,',
     '        comp_stats_write(read_usize(stats + comp_stats_obj::PLAYERS), comp_stats_obj::PLAYER_ENTRY_SIZE as u32 - 1,'),
    ('tel41_5_season_records',
     'comp_stats_write(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x186d); // tel41_5: the season records',
     'comp_stats_write(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x186d); // mutation: the all-time records'),
]
