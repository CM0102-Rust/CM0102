# Mutations through the save proof (prova_save.py --leva N; mutacoes.py --prova-save N):
# - tel42_1_sem_temporada: a new game's club records start at 0 instead of -1 (no season): the career's .qst carries
#   "club history.dat" with other bytes (the save proof writes the .qst of a new career).
# - tel42_1_carga_perdida: load answers 0, so the constructor throws the loaded records away and builds new ones (-1):
#   the load of the holiday save keeps no club history (caught if the load impression covers the records; otherwise by
#   the club history screen of the loaded game, Club -> History, all seasons gone).
MUTACOES = [
    ('tel42_1_sem_temporada',
     '            write_u32(r + field, u32::MAX); // tel42_1: no season yet',
     '            write_u32(r + field, 0); // tel42_1: no season yet (mutation)',
     'tel42_1 new club history'),
    ('tel42_1_carga_perdida',
     '    exe::zclose(&mut z);\n    1\n}\n\n/// Comparer for `club_history_load`.',
     '    exe::zclose(&mut z);\n    0 // tel42_1 mutation\n}\n\n/// Comparer for `club_history_load`.',
     'tel42_1 load club history'),
]
