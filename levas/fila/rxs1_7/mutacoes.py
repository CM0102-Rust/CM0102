# Mutação (rxs1_7): o nome da competição continental trocado pelo nome curto. Acusada pelo comparador (lado a lado do
# texto) no primeiro jogador que recusa por ela; contagem 0 até 2002-08-01: escapa da cadeia por não rodar (explicado
# no PRONTO.txt).
MUTACOES = [
    ('rxs1_7_continental',
     'fmt(reasons[n].as_mut_ptr() as usize, 0xa9df68, g, g, c.name_addr()); // rxs1_7: continental competition',
     'fmt(reasons[n].as_mut_ptr() as usize, 0xa9df68, g, g, c.short_name_addr()); // mutation',
     'rxs1_7 refusal for the continental competition'),
]
