# Mutations (tel30_11). accumulate runs after every match (the chain's comparison catches it); the values and labels
# in a session of Lucas's (JOGAR_COMPARANDO.bat): a competition screen > Player Stats / Team Stats.
MUTACOES = [
    ('tel30_11_bench_rows', '(k >= 0xb && read_i8(row + 0x32) == -1)', '(k >= 0xc && read_i8(row + 0x32) == -1)'),
    ('tel30_11_tackles_minutes', 'minutes <= 0x95 {', 'minutes <= 0x96 {'),
]
