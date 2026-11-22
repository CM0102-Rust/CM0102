# Mutations (tel25_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the training screen sorts its squad list on every build, and the sort
# comparers (compare_order!) catch both.
# - the value sort reversed (highest first);
# - the age sort reversed (youngest first).
MUTACOES = [
    ('tel25_1_value_reversed', 'let d = (Staff(read_usize(a)).value() as i32).wrapping_sub(Staff(read_usize(b)).value() as i32);',
     'let d = (Staff(read_usize(b)).value() as i32).wrapping_sub(Staff(read_usize(a)).value() as i32);'),
    ('tel25_1_age_reversed', 'let d = read_i8(read_usize(b) + staff::AGE) as i32 - read_i8(read_usize(a) + staff::AGE) as i32;',
     'let d = read_i8(read_usize(a) + staff::AGE) as i32 - read_i8(read_usize(b) + staff::AGE) as i32;'),
]
