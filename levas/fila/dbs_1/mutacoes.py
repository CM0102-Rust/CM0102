# Mutacoes do nome pelos ids, do nome do oficial e do texto da mascara. As tres so mudam texto e PODEM ESCAPAR da
# carreira ate 2002-06-01 (a impressao so pega o texto que acaba no banco ou muda um sorteio): a do nome comum pela
# tabela dos sobrenomes (todo nome com comum, ~126 chamadores, notícias e contratos), a do oficial com o sobrenome
# primeiro (forma 1, eventos e painel da partida; a carreira de ferias nao narra partidas) e a da mascara com "right
# back" no lugar de "left back" (midia 0x72d270, que a carreira percorre 103 vezes). O comparando acusa na primeira
# chamada de cada uma (o comparador confere o texto).
MUTACOES = [
    ('dbs_nome_comum', '    let c = terceiro.unwrap_or_else(|| registro(0xae23f8, comum)); // o nome comum',
     '    let c = terceiro.unwrap_or_else(|| registro(0xae23f4, comum)); // mutacao: o comum pela tabela dos sobrenomes'),
    ('dbs_nome_oficial', '            1 => { sprintf(texto, 0x98f0b8, a, b); } // "%s %s"',
     '            1 => { sprintf(texto, 0x98f0b8, b, a); } // mutacao: sobrenome primeiro'),
    ('dbs_mascara_texto', '        ((0x9b82fc, 0x9b82d0), 2, 1) // left back',
     '        ((0x9b82a4, 0x9b8278), 2, 1) // mutacao: right back no lugar de left back'),
]
