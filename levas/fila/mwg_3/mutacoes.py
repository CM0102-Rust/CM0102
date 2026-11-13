# Mutacoes (mwg_3), na criacao de cada jogador virtual (milhoes de chamadas; os clubes ingleses escalam jovens
# virtuais e os estrangeiros das copas europeias jogam com elencos virtuais):
# - a capacidade do virtual de clube sobe 20 pontos (reputacao / 25 - 80): outros resultados;
# - o molde de goleiro aceita nota 15 ou mais (antes so 20): outros goleiros virtuais;
# - o jovem nao perde capacidade (so a reserva perde).
MUTACOES = [
    ('mwg3_capacidade_maior',
     'rep / 25 - 100 // mwg3: the ability of a club\'s virtual player',
     'rep / 25 - 80 // mutation: stronger virtual players',
     'mwg_3 youngster'),
    ('mwg3_goleiro_qualquer',
     '0 => t.goalkeeper() == 20, // mwg3: a template goalkeeper is a perfect one',
     '0 => t.goalkeeper() >= 15, // mutation: any good goalkeeper is a template',
     'mwg_3 youngster'),
    ('mwg3_jovem_sem_perda',
     'ability = ability.wrapping_add(-30 - scaled_crt_rand(41)); // mwg3: the youngster loses ability',
     'let _ = scaled_crt_rand(41); // mutation: the youngster keeps his ability',
     'mwg_3 youngster'),
]
