# Mutations (tel14_2). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat), both on the club's screen > Tactics (any view):
# - the first penalty takers coloured as the others (Tactics > Penalty Takers, the Condition column);
# - the zone grid values with x and y swapped (the With Ball tab).
MUTACOES = [
    ('tel14_2_penalty_colour', 'let colour = if n < 5 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };',
     'let colour = if n < 4 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };'),
    ('tel14_2_zone_value', '0x1f, (zy + 100 * zx) as u32, u32::MAX]);', '0x1f, (zx + 100 * zy) as u32, u32::MAX]);'),
]
