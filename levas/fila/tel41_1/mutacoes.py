# Mutations (tel41_1), by the ENG career to 2002-06-01 (the entries are looked up at every match, by the sorts, the
# board and the AI) and by the comparers (side by side, the entry returned and the value):
# - the club entry is searched with the next id (a club finds no entry, or another club's);
# - the player entry is searched with the next id;
# - a club's statistic is read as the next statistic.
MUTACOES = [
    ('tel41_1_club_key',
     'read_u32(club)); // tel41_1: the key is the club id',
     'read_u32(club).wrapping_add(1)); // mutation: the next id'),
    ('tel41_1_player_key',
     'let key = read_u32(player); // tel41_1: the key is the player id',
     'let key = read_u32(player).wrapping_add(1); // mutation: the next id'),
    ('tel41_1_club_stat',
     'exe::comp_stats_get_team_entry_value(stats, entry, stat) // tel41_1: the club statistic',
     'exe::comp_stats_get_team_entry_value(stats, entry, stat.wrapping_add(1)) // mutation: the next statistic'),
]
