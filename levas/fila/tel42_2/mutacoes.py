# Mutations (tel42_2), in the season a finished competition writes into the club history (runs when each competition
# closes its season: the cups' last round, the leagues' last day; ENG to 2002-06-01 [inference: the first ones in
# August 2001, the Charity Shield, the Super Cup and the Intertoto, the leagues and the FA Cup in May 2002]):
# - a league record keeps the position + 1 (every club one place down in its history);
# - a cup run records the team's seed (+4) instead of its outcome (+5).
# Nothing in the career reads the club history (only the club history screens and the save): the Rust careers do not
# see these mutations; the routine's comparer does (the plan checked in the history files), in the comparison run
# (--confirmar with the game comparing to the date, or the chain's comparison run).
MUTACOES = [
    ('tel42_2_posicao',
     'rec[6] = read_u8(row + league_table_row::POSITION); // tel42_2: the final position',
     'rec[6] = read_u8(row + league_table_row::POSITION).wrapping_add(1); // tel42_2: the final position (mutation)',
     'tel42_2 league table into the history'),
    ('tel42_2_semente',
     'add_club(read_usize(entry), read_u8(entry + 5), read_u16(stage + cup::STAGE_LABEL),',
     'add_club(read_usize(entry), read_u8(entry + 4), read_u16(stage + cup::STAGE_LABEL),',
     'tel42_2 cup runs into the history'),
]
