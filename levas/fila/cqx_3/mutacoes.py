# Mutacoes (cqx_3), nas posicoes previstas dos estaduais (rodam na criacao, no primeiro dia, e na virada). Nenhuma muda a
# carreira ate junho (so a media de publico do fim da temporada le a posicao prevista): sao para o comparador lado a
# lado, pela confirmacao no modo comparando ate 2001-08-20 (a copia reescreve outros lugares que os da original).
MUTACOES = [
    ('cqx3_baiano_1o_e_2o_trocados', 'places: &[1, 2, 3, 3], label: "cqx_3 bahia predicted"', 'places: &[2, 1, 3, 3], label: "cqx_3 bahia predicted" /* mutation: 1st and 2nd swapped */', 'cqx_3 bahia predicted'),
    ('cqx3_centro_oeste_sem_os_quintos', 'places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_3 centre-west predicted"', 'places: &[1, 2, 3, 3, 6, 6, 6, 6], label: "cqx_3 centre-west predicted" /* mutation: no 5th places */', 'cqx_3 centre-west predicted'),
    ('cqx3_sem_ordenar_pela_forca', 'exe::comp_util_sort_clubs_by_strength(v, l.club_count() as i32); // cqx3: the order by strength', '// mutation: not sorted by strength', 'cqx_3 rio grande do sul predicted'),
]
