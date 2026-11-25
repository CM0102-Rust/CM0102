# Mutation through the load of the holiday save (prova_save.py --leva N with CARGA): every staff id kept in the rows
# (line-ups, playmakers, set piece takers) is cleared after the load, so the line-ups are chosen again and the
# impression changes.
MUTACOES = [
    ('rxg_2_escalacoes_vazias',
     '            if read_i32(row + slot) >= staff {',
     '            if read_i32(row + slot) >= 0 && staff != 0 { // mutation',
     'rxg_2 tactics loaded'),
]
