# Mutacoes (o comparando roda a original: so o modo Rust e a carreira acusam). A transferencia passada sem a conta
# subir (~950 chamadas: cada uma sobrescreve a anterior; o historico de transferencias e lido nas negociacoes e nas
# noticias). A permissao de trabalho sem a marca 0x20 (~100: a busca 0x8c7260 nao acha a permissao e o estrangeiro
# volta a precisar dela; escalacao e transferencias mudam). O premio por jogo sem a conta subir (~14: o pagamento por
# jogo some, as financas mudam devagar: PODE ESCAPAR).
MUTACOES = [
    ('cpl_passada_conta', 'cpl3_por(this + 0x86a, n.wrapping_add(1)); // a conta das transferências passadas',
     'cpl3_por(this + 0x86a, n); // mutacao: a conta nao sobe', 'cpl_3 past transfer'),
    ('cpl_permissao_marca', '*marca |= 0x20; // a marca da permissão',
     '*marca |= 0; // mutacao: sem a marca', 'cpl_3 work permit'),
    ('cpl_premio_conta', 'cpl3_por(this + 0x894, n.wrapping_add(1)); // mais um prêmio',
     'cpl3_por(this + 0x894, n); // mutacao: a conta nao sobe', 'cpl_3 appearance bonus'),
]
