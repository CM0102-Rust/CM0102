# Mutacoes. As duas ordens dos cabecas de chave invertidas no nivel (consistentes: o qsort termina): os potes do
# sorteio da Copa da UEFA e da copa da liga inglesa mudam, entao os confrontos, os jogos e as rendas mudam. O teto da
# divisao sem gravar (o byte +0x64 do clube fica como estava no fim da temporada; o ramo diz se o fim de temporada caiu
# antes da data da carreira: se nao caiu, e lacuna medida). As ordens sao quentes: sem ramo.
MUTACOES = [
    ('cpp_cabecas_decrescente', 'entry_level(b) - entry_level(a); // o maior nível primeiro',
     'entry_level(a) - entry_level(b); // mutacao: o menor primeiro'),
    ('cpp_cabecas_crescente', 'entry_level(a) - entry_level(b); // o menor nível primeiro',
     'entry_level(b) - entry_level(a); // mutacao: o maior primeiro'),
    ('cpp_teto_da_divisao', '*p = valor; // o teto novo',
     '// mutacao: o teto fica',
     'cpp_13 teto da divisao'),
]
