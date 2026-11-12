# Mutacoes (cqx_5), nas posicoes previstas dos estaduais (rodam na criacao, no primeiro dia, e na virada). Nenhuma muda a
# carreira ate junho (so a media de publico do fim da temporada le a posicao prevista): sao para o comparador lado a
# lado, pela confirmacao no modo comparando ate 2001-08-20 (a copia reescreve outros lugares que os da original).
MUTACOES = [
    ('cqx5_nordeste_1o_e_2o_trocados', 'places: &[1, 2, 3, 3], label: "cqx_5 northeast predicted"', 'places: &[2, 1, 3, 3], label: "cqx_5 northeast predicted" /* mutation: 1st and 2nd swapped */', 'cqx_5 northeast predicted'),
    ('cqx5_paranaense_dois_primeiros', 'places: &[1, 2], label: "cqx_5 paraná predicted"', 'places: &[1, 1], label: "cqx_5 paraná predicted" /* mutation: two 1sts */', 'cqx_5 paraná predicted'),
    ('cqx5_pernambucano_sem_terceiros_divididos', 'places: &[1, 2, 3, 3], label: "cqx_5 pernambuco predicted"', 'places: &[1, 2, 3, 4], label: "cqx_5 pernambuco predicted" /* mutation: no shared 3rd */', 'cqx_5 pernambuco predicted'),
]
