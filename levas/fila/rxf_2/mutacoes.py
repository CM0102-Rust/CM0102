# Mutations through the save proof (prova_save.py --leva N; mutacoes.py --prova-save N):
# - rxf_2_inglaterra_pela_argentina: the English awards are gated by Argentina's key (not played in the test): the new
#   career has no EnglandAwards object, so its .qst "awards.dat" is shorter (the career would also lose the English
#   player and manager of the month).
# - rxf_2_sem_notas: the save writes no year ratings: "awards.dat" loses both tables' counts and entries.
MUTACOES = [
    ('rxf_2_inglaterra_pela_argentina',
     '    (AwardSlotGate::Nation(KEY_NATION_ENGLAND), 0x578780, None), // EnglandAwards',
     '    (AwardSlotGate::Nation(KEY_NATION_ARGENTINA), 0x578780, None), // EnglandAwards (rxf_2 mutation)',
     'rxf_2 create the award objects'),
    ('rxf_2_sem_notas',
     '            save_ratings(ratings, z); // rxf_2: the year ratings',
     '            let _ = (save_ratings, ratings); // rxf_2: the year ratings (mutation)',
     'rxf_2 save awards.dat'),
]
