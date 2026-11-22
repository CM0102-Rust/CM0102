# Mutations (tel36_13). The pundit reader runs at the start of every career (the pundit file is read at a new game and
# at a load); the comparer catches these on the first lines of the file:
# - the radio code becomes 6;
# - the record keeps the number's low byte plus one.
MUTACOES = [
    ('tel36_13_radio_code', '(0xa79488, 3),', '(0xa79488, 6),'),
    ('tel36_13_pundit_id', 'write_u8(r + PUNDIT_ID, id as u8);', 'write_u8(r + PUNDIT_ID, (id as u8).wrapping_add(1));'),
]
