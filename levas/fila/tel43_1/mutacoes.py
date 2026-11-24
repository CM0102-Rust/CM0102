# Mutations (tel43_1). record_match_result runs on every match of the ENG career; the comparer catches what it
# writes side by side on the matches that only touch memory (most of them), and the career sees the entries through
# the board (games played), get_club_stat (form, runs), the awards and media (player tables) and the money (fines):
# - the club entry's game count does not go up (the board compares it with 8 and 25; the averages divide by it);
# - a win or draw does not end the losing run (get_club_stat 7, the commentary);
# - a player's games count twice (the awards and the average rating read them);
# - the level-0 FA fine from 16 card points instead of 65 (fines early in the season: the clubs' money; the
#   comparer leaves fines to the original, so only the career sees this one);
# - a starter gets one minute more (the rates of get_player_entry_value need the minutes).
MUTACOES = [
    ('tel43_1_club_games',
     '        s.entry.set_matches_played(s.entry.matches_played().wrapping_add(1)); // tel43_1: one more game',
     '        // mutation: the game is not counted'),
    ('tel43_1_losing_run',
     '        write_i32(entry + comp_stats_club::LOSING_RUN, 0); // tel43_1: a win or a draw ends the losing run',
     '        // mutation: the losing run goes on'),
    ('tel43_1_player_games',
     '    add(f::GAMES, 1); // tel43_1: one more game for the player',
     '    add(f::GAMES, 2); // mutation: two games'),
    ('tel43_1_fine_threshold',
     '        0 => Some(0x41),',
     '        0 => Some(0x10), // mutation: fined from 16 points'),
    ('tel43_1_starter_minutes',
     "        write_i32(minutes, read_i32(minutes).wrapping_add(until)); // tel43_1: a starter's minutes",
     '        write_i32(minutes, read_i32(minutes).wrapping_add(until + 1)); // mutation: one minute more'),
]
