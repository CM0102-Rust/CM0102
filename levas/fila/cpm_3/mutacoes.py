# Mutacoes (com o ramo de cada uma). A subida sem a obra do estadio minimo da liga (so o modo Rust e a carreira: o
# comparando roda a original; as financas do clube promovido mudam no fim da temporada: depois de 2002-06-01 PODE
# ESCAPAR). Os melhores embaralhados com uma troca a menos e o vetor de ponteiros com metade das trocas (o comparando
# acusa pelo estado do sorteio; a carreira pela contagem de sorteios na primeira montagem de competicao).
MUTACOES = [
    ('cpm_estadio_obra', 'obra(f, capacidade as i32, lugares as i32, id, 0); // a obra do estádio',
     'let _ = (obra, f, capacidade); // mutacao: sem a obra',
     'cpm_3 division stadium'),
    ('cpm_melhores_trocas', 'for _ in 0..m as i32 { // m trocas',
     'for _ in 1..m as i32 { // mutacao: uma troca a menos',
     'cpm_3 shuffle the best'),
    ('cpm_ponteiros_trocas', 'k = k.wrapping_add(1); // mais uma troca',
     'k = k.wrapping_add(2); // mutacao: metade das trocas',
     'cpm_3 shuffle pointers'),
]
