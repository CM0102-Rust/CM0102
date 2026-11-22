# Mutations (tel41_6) by the save proof with the holiday save's load (CARGA; mutacoes.py --prova-save N). No 4th
# field: the base of the save proof runs with the routines off.
# - the first club entry of every competition loads with one more home attendance (the records, the attendance
#   reading and the re-saved .tmp change);
# - the season records load into the all-time ones (the season records keep the constructor's empty state).
MUTACOES = [
    ('tel41_6_club_attendance',
     'file, 0x1777); // tel41_6: club entries read',
     'file, 0x1777); write_u32(table + comp_stats_club::HOME_ATTENDANCE_TOTAL, read_u32(table + comp_stats_club::HOME_ATTENDANCE_TOTAL).wrapping_add(1)); // mutation'),
    ('tel41_6_season_records',
     'comp_stats_read(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17c6); // tel41_6: the season records',
     'comp_stats_read(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17c6); // mutation: into the all-time records'),
]
