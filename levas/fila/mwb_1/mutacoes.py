# Mutacoes (mwb_1): a Copa Challenge com 2 substituicoes em vez de 3 (muda os jogos da copa); a final uma semana depois
# (14/10 -> 21/10: outro dia, outro resultado); os inscritos do fim do banco para o comeco (outra ordem, outro sorteio).
MUTACOES = [
    ('mwb1_challenge_duas_substituicoes',
     'bench: Some((5, 3)), // mwb1: the Challenge Cup bench',
     'bench: Some((5, 2)), // mutation: 2 substitutions',
     'mwb_1 new challenge cup'),
    ('mwb1_final_uma_semana_depois',
     'cql10r((2, 8, 0, 3), (0xe, 9, 0, 6, 1, 0x28), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mwb1: the Challenge Cup final',
     'cql10r((2, 8, 0, 3), (0x15, 9, 0, 6, 1, 0x28), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mutation: the final on 21/10',
     'mwb_1 challenge cup rounds'),
    ('mwb1_inscritos_ao_contrario',
     'let order: Vec<i32> = (0..n).collect(); // mwb1: the clubs in database order',
     'let order: Vec<i32> = (0..n).rev().collect(); // mutation: from the end of the database',
     'mwb_1 challenge cup entrants'),
]
