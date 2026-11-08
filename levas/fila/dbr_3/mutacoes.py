# Mutacoes das ordens pelo nome e da conferencia dos nomes. A dos clubes pelo nome curto troca os lados (a ordem
# invertida): roda na semeadura da UEFA 0x906410 e no Setup; se a carreira ate 2002-06-01 nao pass_ball pela semeadura,
# PODE ESCAPAR (as telas so no jogo real comparando). A das pessoas pelo nome completo troca os lados: roda no
# gerente de transferencias 0x8e55f0; PODE ESCAPAR se a lista ordenada nao mudar o que o gerente escolhe (o
# comparando acusa na primeira comparacao). A da conferencia ignora o sobrenome: com ~137 chamadores (regeneracao e
# transferencias na carreira), mais pessoas conferem e a carreira deve acusar.
MUTACOES = [
    ('dbr_clubes_pelo_nome_curto', '    compare_text_nocase(nome(a), nome(b))', '    compare_text_nocase(nome(b), nome(a)) // mutacao: a ordem invertida'),
    ('dbr_pessoas_pelo_nome_completo', '    compare_text_nocase(na.as_ptr(), nb.as_ptr())', '    compare_text_nocase(nb.as_ptr(), na.as_ptr()) // mutacao: a ordem invertida'),
    ('dbr_confere_nomes', '    for (d, id) in [(4, nome), (8, sobrenome), (0xc, comum)] {', '    for (d, id) in [(4, nome), (0xc, comum)] { // mutacao: sem o sobrenome'),
]
