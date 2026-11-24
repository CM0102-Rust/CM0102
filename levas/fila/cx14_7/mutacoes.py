# Mutacoes (cx14_7), com a Turquia ate 2002-08-01:
# - o mata-mata com o segundo de cada grupo de classificacao em vez do primeiro (outros clubes no mata-mata da
#   primavera de 2002 e outro acesso na virada; acusada tambem pelo comparador, que confere os clubes);
# - os grupos regionais de nove em nove (outra divisao dos grupos na virada do verao de 2002; acusada pelo
#   comparador, lado a lado).
MUTACOES = [
    ('cx14_7_mata_mata_segundos',
     'v.push(aus_nsl_table_row_club(League(read_usize(l.stages() + 4 * g)), 0)); // cx14_7: each classification group winner',
     'v.push(aus_nsl_table_row_club(League(read_usize(l.stages() + 4 * g)), 1)); // mutation: the runners-up',
     'cx14_7 category b play-off'),
    ('cx14_7_grupos_de_nove',
     'let g = (k / 10).min(4); // cx14_7: ten clubs a group',
     'let g = (k / 9).min(4); // mutation: nine a group',
     'cx14_7 turkish regional groups'),
]
