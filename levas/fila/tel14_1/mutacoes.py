# Mutations (tel14_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the club's screen > Tactics.
# - the first column view of a screen outside a match taken as the match rating;
# - the record index parameter written as zero.
MUTACOES = [
    ('tel14_1_first_view', 'let first_view: u32 = if read_u32(ts + t::MATCH) != 0 { 6 } else { 2 };',
     'let first_view: u32 = if read_u32(ts + t::MATCH) != 0 { 6 } else { 6 };'),
    ('tel14_1_record_param', 'set_screen_param(7, record as usize);', 'set_screen_param(7, 0);'),
]
