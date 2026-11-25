# Mutations (rxf_6), with the 8 leagues up to 2003-08-15. No box opens.
# - rxf_6_europa_sem_uefa: Europe's second entrant is picked like the first (no UEFA Cup argument): another club plays
#   the July/August 2003 groups (the career's matches and results change).
MUTACOES = [
    ('rxf_6_europa_sem_uefa',
     '        let comp = if slot == 5 { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { -1 }; // rxf_6: Europe\'s second through the UEFA Cup',
     '        let comp = if slot == 5 { -1 } else { -1 }; // rxf_6 mutation',
     'rxf_6 world club championship entrants'),
]
