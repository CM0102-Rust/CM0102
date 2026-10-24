# Mutacoes das chaves de clube e de nacao. Rodam na criacao do jogo (a carreira automatica e a gravacao
# do .qst da prova do save criam o jogo): o comparador confere as globais das chaves a cada chamada.
MUTACOES = [
    ('ix1_clube', "*(global as *mut u32) = linha; // chave de clube", "*(global as *mut u32) = linha + 1; // chave de clube"),
    ('ix1_troca', "*(g as *mut u32) = novo; // chave remapeada", "*(g as *mut u32) = antigo; // chave remapeada"),
    ('ix1_nacao', "*(global as *mut u32) = linha; // chave de nação", "*(global as *mut u32) = linha + 1; // chave de nação"),
]
