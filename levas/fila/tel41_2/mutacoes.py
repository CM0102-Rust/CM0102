# Mutations (tel41_2), by the ENG career to 2002-06-01 (the media, the awards and the AI read the sorted tables and
# the player statistics) and by the comparers (side by side: value, counts and blocks):
# - a player's statistic is read as the next statistic;
# - the team table keeps the clubs with 0 at the bottom;
# - the player table takes the players outside the asked position groups and leaves out those inside.
MUTACOES = [
    ('tel41_2_player_stat',
     'exe::comp_stats_get_player_entry_value(stats, entry, stat) // tel41_2: the player statistic',
     'exe::comp_stats_get_player_entry_value(stats, entry, stat.wrapping_add(1)) // mutation: the next statistic'),
    ('tel41_2_trailing_zeros',
     '            // tel41_2: the trailing clubs with 0 leave the table\n            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 {',
     '            // mutation: the clubs with 0 stay\n            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 || true {'),
    ('tel41_2_group_filter',
     'comp_stats_get_player_position_group(sheet) & groups as u16 != 0',
     'comp_stats_get_player_position_group(sheet) & groups as u16 == 0'),
]
