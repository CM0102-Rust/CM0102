# Mutacoes (mwd_7): o fim da liga croata (maio de 2002) sempre sem jogo-desempate: sem a fase 0, a tabela final e o
# acesso mudam. So pega se houver clubes empatados em pontos (na Primeira ou no grupo sul); senao a mutacao nao muda nada.
MUTACOES = [
    ('mwd7_sem_desempate',
     'if pairs == 0 { // mwd7: no level clubs',
     'if true { // mutation: never a tie-break',
     'mwd_7 croatian tie-break play-off'),
]
