# Mutacoes (rx04_2). O texto nao volta a simulacao: acusadas pelo comparador lado a lado do langlib_sprintf (o guia
# conta como acusada); na cadeia, roda-las com a comparacao ligada. O jogo de teste esta em portugues.
# - os dois bytes de genero de cada pedaco trocados (o modificador de gramatica concorda errado: "o/a", "do/da");
# - os espacos do comeco do formato perdidos (a saida comeca no lugar errado);
# - a marca depois da palavra ("palavra{s}") aplicada sem o caractere e o espaco antes dela.
MUTACOES = [
    ('rx042_generos_trocados',
     'let (g0, g1) = (read_u8((genders as isize + 2 * k) as usize), read_u8((genders as isize + 2 * k + 1) as usize));',
     'let (g1, g0) = (read_u8((genders as isize + 2 * k) as usize), read_u8((genders as isize + 2 * k + 1) as usize)); // mutation',
     'rx04_2 piece through the grammar'),
    ('rx042_sem_espacos_iniciais',
     "    while read_u8(format + n) == b' ' {",
     "    while false && read_u8(format + n) == b' ' { // mutation",
     'rx04_2 english'),
    ('rx042_palavra_sem_o_antes',
     '            let mut q = l + k - 1;',
     '            let mut q = l + k + 1; // mutation',
     'rx04_2 mark after a word'),
]
