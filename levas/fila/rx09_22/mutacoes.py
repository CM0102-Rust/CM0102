# Mutacoes (rx09_22) pela prova do save (mutacoes.py --prova-save N): as fases de liga das copas europeias da carreira ENG
# estao no save de ferias.
# - as datas das rodadas uma a menos: o .tmp da competicao no .qst fica 0x41 bytes mais curto (e a carga seguinte
#   desalinha);
# - a competicao gravada como -1 sempre: o .tmp muda (a carga perde a competicao da fase).
MUTACOES = [
    ('rx09_22_uma_rodada_a_menos',
     "write(l.round_dates(), 0x41, l.round_count() as i32 as u32); // rx09_22: every round's date",
     "write(l.round_dates(), 0x41, l.round_count() as i32 as u32 - 1); // mutation: one round short",
     'rx09_22 league stage saved'),
    ('rx09_22_sem_competicao',
     '    if comp != 0 {\n        write(comp, 4, 1);\n    } else {',
     '    if comp == 0x7fff_ffff { // mutation: never\n        write(comp, 4, 1);\n    } else {',
     'rx09_22 league stage saved'),
]
