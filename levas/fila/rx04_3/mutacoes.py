# Mutacoes (rx04_3). A carga roda na abertura (a lingua do game.cfg, portugues no jogo de teste): acusadas pelo
# comparador da carga (o estado inteiro num hash) e, sem a comparacao, pelos textos (a carreira nao le o texto).
# - as chaves ficam com "{" e "}" (os textos com marcas de genero nao sao achados e saem em ingles);
# - a tabela sem ordenar (a busca binaria se perde).
# A preparacao e o fim ficam sem mutacao propria: a preparacao so escolhe a lingua do teclado (trocada logo depois pela
# do game.cfg) e guarda o caminho e a flag (uma flag 0 faz cada texto ler os 204 MB do arquivo: a carreira trava); o
# fim so roda na saida do programa. Os dois sao provados pelos comparadores (o estado depois da copia e da original).
MUTACOES = [
    ('rx043_chaves_ficam',
     "        if c != b'{' && c != b'}' {\n            write_u8(key + w, c);",
     "        if true { // mutation: the braces stay\n            write_u8(key + w, c);",
     'rx04_3 language table'),
    ('rx043_sem_ordenar',
     '    exe::crt_qsort(read_usize(G_LANGLIB_TABLE), read_u32(G_LANGLIB_ENTRY_COUNT), LANGLIB_ENTRY_SIZE as u32, LANGLIB_KEY_ORDER);',
     '    // mutation: the table not sorted',
     'rx04_3 language table'),
]
