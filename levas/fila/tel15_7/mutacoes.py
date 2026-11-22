# Mutations (tel15_7). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): in a live match, Tactics, make 4 substitutions and Ok (the return text);
# any tactics screen (the pitch) and the tactics overview (the goalkeeper's circle).
MUTACOES = [
    ('tel15_7_subs_words', '    if made > 3 {',
     '    if made > 2 {'),
    ('tel15_7_penalty_spot', 'let spot = pitch_ftol(*out_left as f64 + r12);',
     'let spot = pitch_ftol(*out_left as f64 + r6);'),
    ('tel15_7_keeper_point', '        *out_y = bottom - 10;',
     '        *out_y = bottom - 12;'),
]
