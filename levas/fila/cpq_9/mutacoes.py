# Mutacoes. A nota do doador sem o sorteio da idade (um sorteio a menos por pessoa com 31 anos ou mais a cada
# escolha do doador 0x7a6e60, 22 na carreira ate 2002-06-01: a contagem de sorteios muda e o doador escolhido tambem).
# A lista de aposentadorias pelo ano mais tarde primeiro (consistente: o qsort termina; a 0x7a59e0 para na primeira
# entrada futura: as aposentadorias e os regens atrasam). A fila de regens pelo dia mais tarde primeiro no mesmo ano
# (consistente; a 0x7a65d0 cria na ordem da fila: os regens mudam de ordem e de data). As tres sao quentes: sem ramo.
MUTACOES = [
    ('cpq9_sorteio_da_idade', 'let mut nota = sortear((idade - 31) * 150); // a idade',
     'let mut nota = 0i32; // mutacao: sem o sorteio da idade'),
    ('cpq9_ordem_das_aposentadorias', 'return aa as i32 - ab as i32; // o ano mais cedo primeiro',
     'return ab as i32 - aa as i32; // mutacao: o mais tarde primeiro'),
    ('cpq9_ordem_da_fila', 'let (ja, jb) = (cpq9_w(a + 4), cpq9_w(b + 4));',
     'let (jb, ja) = (cpq9_w(a + 4), cpq9_w(b + 4)); // mutacao: o dia mais tarde primeiro'),
]
