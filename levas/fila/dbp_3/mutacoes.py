# Mutacoes da rejeitada e das favoritas. A da rejeitada tira das favoritas (relacoes do elenco, diaria, e volta da
# lesao): a carreira ate 2002-06-01 deve acusar. A das favoritas repete a segunda no lugar da terceira: so roda na
# opiniao e na reacao da diretoria (MANAGER_MANAGER), e so muda algo com tres favoritas; PODE ESCAPAR da carreira (o
# comparando acusa na primeira pessoa com tres favoritas consultada).
MUTACOES = [
    ('dbp_tirar_rejeitada', '        let reg = pref + 0x28; // as pessoas rejeitadas', '        let reg = pref + 0x1c; // mutacao: das pessoas favoritas'),
    ('dbp_pessoas_favoritas', '[0x1cusize, 0x20, 0x24].into_iter().enumerate() { // as três favoritas', '[0x1cusize, 0x20, 0x20].into_iter().enumerate() { // mutacao: a segunda no lugar da terceira'),
]
