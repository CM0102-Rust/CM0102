# Mutacoes (cqx_4), nas posicoes previstas dos estaduais (rodam na criacao, no primeiro dia, e na virada). Nenhuma muda a
# carreira ate junho (so a media de publico do fim da temporada le a posicao prevista): sao para o comparador lado a
# lado, pela confirmacao no modo comparando ate 2001-08-20 (a copia reescreve outros lugares que os da original).
MUTACOES = [
    ('cqx4_goiano_sem_os_quintos', 'places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_4 goiás predicted"', 'places: &[1, 2, 3, 3, 6, 6, 6, 6], label: "cqx_4 goiás predicted" /* mutation: no 5th places */', 'cqx_4 goiás predicted'),
    ('cqx4_mineiro_1o_e_2o_trocados', 'places: &[1, 2], label: "cqx_4 minas gerais predicted"', 'places: &[2, 1], label: "cqx_4 minas gerais predicted" /* mutation: 1st and 2nd swapped */', 'cqx_4 minas gerais predicted'),
    ('cqx4_norte_sem_terceiros_divididos', 'places: &[1, 2, 3, 3], label: "cqx_4 north predicted"', 'places: &[1, 2, 3, 4], label: "cqx_4 north predicted" /* mutation: no shared 3rd */', 'cqx_4 north predicted'),
]
