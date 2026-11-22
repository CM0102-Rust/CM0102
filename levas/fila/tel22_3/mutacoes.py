# Mutations (tel22_3). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the Fixtures tab's round link with id 0x13 instead of 0x12 (competition screen > Fixtures, a cup round known);
# - the Seedings tab's second half starting below the first instead of at the top (competition screen > Seedings).
MUTACOES = [
    ('tel22_3_fixtures_round_id', 'JOB_TEXT as u32, 0, 0x12, value as u32, area]',
     'JOB_TEXT as u32, 0, 0x13, value as u32, area]'),
    ('tel22_3_seedings_second_half', '(2u32, (counter as u8).wrapping_sub(half as u8) as i8 as i32)',
     '(2u32, counter as u8 as i8 as i32)'),
]
