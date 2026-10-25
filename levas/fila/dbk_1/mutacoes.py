# Mutacoes da criacao do jogo (os times A e B e o indice de nomes). A prova e o .qst da prova do save: a criacao
# roda uma vez antes da gravacao e o que ela muda (elencos, comissao, pessoas; nomes do pessoal criado pela 0x527240
# a partir do indice) vai para o save. A do indice pode escapar se a 0x527240 nao criar pessoal na prova (ver rotinas.py).
MUTACOES = [
    ('dbk1_cortar_elencos', '                while k >= 0 && total > 48 { // o elenco conjunto a 48', '                while k >= 0 && total > 47 { // mutacao: o elenco conjunto a 47'),
    ('dbk1_juntar_comissao', '                        if n < k as i32 { // a comissão do B vai para o A', '                        if n < 0 { // mutacao: a comissao do B nao vai para o A'),
    ('dbk1_indice_de_nomes', '                        *((e + d_quantos) as *mut i32) = quantos; // quantidade do grupo', '                        *((e + d_quantos) as *mut i32) = quantos + 1; // mutacao: a quantidade do grupo + 1'),
]
