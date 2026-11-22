# Mutations (tel30_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat) with the background pictures on (Options > "change backgrounds"): any club
# or nation screen asks the finders; a screen without its own picture takes the random one.
# - the finders take the first matching subsection instead of the last;
# - the random picture does not skip the stadiums.
MUTACOES = [
    ('tel30_1_first_match', '                hit = j;\n            }', '                hit = j;\n                break;\n            }'),
    ('tel30_1_random_stadiums', 'if subs != 0 && k != BACKGROUND_STADIUMS as i16 {', 'if subs != 0 {'),
]
