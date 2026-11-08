# Mutacoes (cpw_5), nas notas do mes que escolhem o jogador e o jovem do mes de cada divisao inglesa (na virada do mes,
# pelo 0x767150; o vencedor ganha injury_word_0c pela 0x412f00, gravada no registro do jogador, que o hash do banco le todo
# dia): so o lado da casa entra nas notas (os jogos fora somem: medias e contas de jogos mudam, e com elas quem passa
# dos 3 jogos), o registro novo ja comeca com um jogo (todas as medias caem por soma/(n+1) e o corte dos 3 jogos muda)
# e o melhor do mes sem ordenar a tabela (o primeiro inscrito da liga em vez do de maior media). As tres trocam o
# premiado de algum mes ate 2002-06-01 e com ele a injury_word_0c gravada. Podem escapar so se o premiado de todos os meses
# sair igual (improvavel na primeira e na terceira).
MUTACOES = [
    ('cpw5_so_a_casa', '    for l in 0..2 { // cpw5: os dois lados',
     '    for l in 0..1 { // mutacao: so a casa',
     'cpw_5 notas do visitante'),
    ('cpw5_comeca_com_um_jogo', '    *((r + 0xa) as *mut u8) = 0; // cpw5: começa sem jogos',
     '    *((r + 0xa) as *mut u8) = 1; // mutacao: comeca com um jogo',
     'cpw_5 jogador novo no mes'),
    ('cpw5_sem_ordenar', '    ordenar(cpw5_d(this), cpw5_d(this + 4) as u32, 0xc, 0x768200); // cpw5: ordena pela média',
     '    let _ = ordenar; // mutacao: sem ordenar',
     'cpw_5 ordenou as notas'),
]
