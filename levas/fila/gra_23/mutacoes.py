# Mutations (gra_23): the store is built when the career is created, so another formation in a slot changes the shape
# the AI clubs (or the preset users) play with from the first match on, which the ENG career to 2002-06-01 catches; the
# comparer catches both at the creation too.
MUTACOES = [
    ('gra23_outra_formacao_da_ia',
     '    (0xa8b8fc, 0x72), // 442_v1.pct',
     '    (0xa8b8f0, 0x72), // mutation: 532_v1.pct in the slot of 442_v1.pct',
     'gra_23 tactics from the files'),
    ('gra23_outra_predefinida',
     '    (0xa8b784, 0xa782b0, 0x9d), // 442_default.pct, "4-4-2"',
     '    (0xa8b694, 0xa782b0, 0x9d), // mutation: 433_default.pct under the name "4-4-2"',
     'gra_23 tactics from the files'),
]
