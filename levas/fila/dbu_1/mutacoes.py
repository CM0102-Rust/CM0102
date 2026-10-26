# Mutacoes do nome pelos textos, do nome do cargo e do nome do idioma. As tres so mudam texto e PODEM ESCAPAR da
# carreira ate 2002-06-01 (a impressao so pega o texto que acaba no banco ou muda um sorteio): a do nome com o
# sobrenome primeiro (forma 1 com nome e sobrenome e sem nome comum, pela 0x53e140: noticias, contratos), a do
# cargo de tecnico inteiro trocado pelo de auxiliar (midia 0x744f80, transferencias 0x8dc370) e a do idioma
# seguinte (so telas: busca e tela da pessoa; ESCAPA). O comparando acusa na primeira chamada de cada uma (o
# comparador confere o texto).
MUTACOES = [
    ('dbu_nome_textos', '                    1 => { sprintf(texto, 0x98f0b8, a, b); } // "%s %s" (dbu)',
     '                    1 => { sprintf(texto, 0x98f0b8, b, a); } // mutacao: sobrenome primeiro'),
    ('dbu_cargo', '    (0x9b6964, 0x9b6988, 0x9b695c, 0x9b6440, 4), // manager (dbu)',
     '    (0x9b6964, 0x9b6988, 0x9b68f8, 0x9b642c, 4), // mutacao: manager vira assistant manager'),
    ('dbu_idioma', '{ DBU1_IDIOMAS[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)',
     '{ DBU1_IDIOMAS[x as usize % 64] } else { 0x9b6b60 }; // mutacao: o idioma seguinte'),
]
