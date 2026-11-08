# Mutacoes das ordens: a dos clubes inverte o lado de menor injury_word_0c (o vetor de clubes vai para o save); a do
# elenco vira crescente (o corte do conjunto A+B a 48 tira os melhores; so pega se algum conjunto passa de 48 na
# criacao: pode escapar); a da forca do elenco vira crescente (0x549a30 so roda na temporada: ESCAPA da prova do
# save, lacuna; o comparador dela roda no jogo real comparando da cadeia).
MUTACOES = [
    ('dbi_ordem_dos_clubes_pela_reputacao', '        1 // a de menor reputação depois', '        -1 // mutacao: a de menor reputação antes'),
    ('dbi_ordem_do_elenco_pela_habilidade', '    habilidade(pb) as i32 - habilidade(pa) as i32 // a maior primeiro', '    habilidade(pa) as i32 - habilidade(pb) as i32 // mutacao: a menor primeiro'),
    ('dbi_ordem_do_elenco_pela_habilidade_atual', '        palavra(jb, 5) - palavra(ja, 5) // a habilidade atual maior primeiro', '        palavra(ja, 5) - palavra(jb, 5) // mutacao: a menor primeiro'),
]
