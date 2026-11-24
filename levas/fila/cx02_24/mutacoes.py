# Mutacoes (cx02_24), carreira com a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - o primeiro grupo com os clubes girados uma posicao (o parametro rotate do molde): a ordem da tabela do grupo muda
#   e o plano conferido acusa.
# As honrarias so repassam a chamada a fase 19 (um mutante que nao repassa deixaria a copa sem campeao no historico,
# depois da data): sem mutacao propria.
MUTACOES = [
    ('cx02_24_grupo_girado',
     '    welsh_cup_group_table(League(this), 4, 1, 0x9cedd4, 0x196, 0, "cx02_24 dutch cup first group")',
     '    welsh_cup_group_table(League(this), 4, 1, 0x9cedd4, 0x196, 1, "cx02_24 dutch cup first group") // mutation'),
]
