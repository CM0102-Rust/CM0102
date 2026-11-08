# Mutacoes das regras do contrato. A do clube em temporada inverte a resposta (o contrato padrao soma ao salario
# com o clube em temporada, 0x865640): muda os contratos gerados na criacao, que vao para o .qst. A do clube numa
# grande nacao tira a Holanda (os contratos padrao dos clubes holandeses mudam de band, 0x8650f2). A da nacao grande
# tira a Holanda tambem, mas a 0x549ea0 so roda na regeneracao dos jovens (temporada): ESCAPA da prova do save
# (lacuna); o comparador dela roda no jogo real comparando da cadeia.
MUTACOES = [
    ('dbl_clube_em_temporada', '    temporada(clube, 0, 0) // a temporada da nação do clube', '    (temporada(clube, 0, 0) == 0) as u32 // mutacao: a resposta invertida'),
    ('dbl_clube_em_nacao_grande', '0x9cf488, 0x9cf338]; // Holanda (clube)', '0x9cf488, 0x9cf488]; // mutacao: sem a Holanda (clube)'),
    ('dbl_nacao_grande', '0x9cf488, 0x9cf338]; // Holanda (nação)', '0x9cf488, 0x9cf488]; // mutacao: sem a Holanda (nação)'),
]
