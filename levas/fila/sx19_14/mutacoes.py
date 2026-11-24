# Mutacoes (sx19_14), para o Lucas provar o comparador na sessao dele (a carreira nao abre telas): a tela do elenco de
# um clube da Noruega ou de Portugal, com jogadores com cartoes.
MUTACOES = [
    ('sx19_14_noruega_terceiro',
     '(n == 2 || n > 3) as u32 // sx19_14: the Norwegian warning',
     '(n == 3 || n > 3) as u32 // mutation: one booking late'),
    ('sx19_14_portugal_passo',
     '{ 3 } else { 5 }; // sx19_14: the Portuguese steps',
     '{ 3 } else { 4 }; // mutation: a step of 4'),
]
