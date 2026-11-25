# Mutation through the load of the holiday save (prova_save.py --leva N with CARGA): each schedule list goes into the
# next day's bucket, so the computer's offers and negotiations are handled a day late after the load and the impression
# changes.
MUTACOES = [
    ('rxg_1_agenda_um_dia',
     '            exe::llist_add_tail(this + SCHEDULE_BUCKETS + bucket * 0xe, &mut slot as *mut u32 as usize, 4);',
     '            exe::llist_add_tail(this + SCHEDULE_BUCKETS + (bucket + 3) % 0x96 * 0xe, &mut slot as *mut u32 as usize, 4); // mutation',
     'rxg_1 transfer manager loaded'),
]
