# Mutacoes das chaves de competicao. Rodam na criacao do jogo (a carreira automatica e a gravacao do
# .qst da prova do save criam o jogo): o comparador confere as globais e a devolucao a cada chamada.
MUTACOES = [
    ('ix2_comp_clube', "*(global as *mut u32) = linha; // chave de competição de clube", "*(global as *mut u32) = linha + 1; // chave de competição de clube"),
    ('ix2_comp_pessoa', "*(global as *mut u32) = linha; // chave de competição de pessoa", "*(global as *mut u32) = linha + 1; // chave de competição de pessoa"),
    ('ix2_devolucao', "r as u32 // o eax do último _stricmp (chave de competição de clube)", "(r != 0) as u32 // o eax do último _stricmp (chave de competição de clube)"),
]
