# Mutations (tel36_8). The automatic career opens no screen; these prove the comparer in a session of Lucas's (the
# screen: a competition's screen > the officials tab, opened twice so the list exists).
# - "Lenient" and "Strict" swap;
# - the reds column shows the yellows.
MUTACOES = [
    ('tel36_8_harshness', '2 => 0xa6a1b0,\n                4 => 0xa6a1bc,', '2 => 0xa6a1bc,\n                4 => 0xa6a1b0,'),
    ('tel36_8_reds', '(3, read_u8(record + OFFICIAL_STATS_RED) as i32)', '(3, read_i16(record + OFFICIAL_STATS_YELLOW) as i32)'),
]
