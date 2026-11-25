# Mutations through the save proof (prova_save.py --leva N; mutacoes.py --prova-save N):
# - rxf_1_sem_notas_escolhidas: the new game builds no selected nations' year ratings (+0xc stays 0; every user checks
#   for null): the career's .qst carries "awards.dat" without that table's count (AWARD_MANAGER::save skips it).
# - rxf_1_carga_perdida: load answers 0 after reading, so the constructor builds new awards and ratings over the loaded
#   ones: the load of the holiday save loses its awards (the month awards of August and the ratings that choose them;
#   caught if the load impression up to 2001-08-28 covers them, otherwise by the Awards screens of the loaded game).
# The destructor has no mutation: what it frees is never read again (a leak or a double free would only show as a crash
# or in the memory, not in the .qst nor in the impression).
MUTACOES = [
    ('rxf_1_sem_notas_escolhidas',
     '    write_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS, award_manager_new_year_ratings(b::SELECTED_RATINGS_DAY, 0));',
     '    write_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS, 0); // rxf_1 mutation',
     'rxf_1 new award manager'),
    ('rxf_1_carga_perdida',
     '    1 // rxf_1: the awards came from the save',
     '    0 // rxf_1: the awards came from the save (mutation)',
     'rxf_1 load awards.dat'),
]
