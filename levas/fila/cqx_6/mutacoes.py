# Mutacoes (cqx_6), nas posicoes previstas dos estaduais (rodam na criacao, no primeiro dia, e na virada). Nenhuma muda a
# carreira ate junho (so a media de publico do fim da temporada le a posicao prevista): sao para o comparador lado a
# lado, pela confirmacao no modo comparando ate 2001-08-20 (a copia reescreve outros lugares que os da original).
MUTACOES = [
    ('cqx6_carioca_1o_e_2o_trocados', 'places: &[1, 2], label: "cqx_6 rio predicted"', 'places: &[2, 1], label: "cqx_6 rio predicted" /* mutation: 1st and 2nd swapped */', 'cqx_6 rio predicted'),
    ('cqx6_catarinense_quartos_divididos', 'places: &[1, 2, 3, 3], label: "cqx_6 santa catarina predicted"', 'places: &[1, 2, 4, 4], label: "cqx_6 santa catarina predicted" /* mutation: shared 4th */', 'cqx_6 santa catarina predicted'),
    ('cqx6_paulista_1o_e_2o_trocados', 'places: &[1, 2, 3, 3], label: "cqx_6 são paulo predicted"', 'places: &[2, 1, 3, 3], label: "cqx_6 são paulo predicted" /* mutation: 1st and 2nd swapped */', 'cqx_6 são paulo predicted'),
]
