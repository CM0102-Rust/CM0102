# Mutacoes (cqf_11), no relatorio do jogo da selecao aos clubes (0x7706c0, 808 chamadas ate 2002-06-01): cada noticia
# sorteia (sorteio(2)); as mutacoes mudam o numero de noticias (e de sorteios) ou tiram o sorteio.
MUTACOES = [
    ('cqf11_sem_sorteio',
     'let r = cqf11_sortear(2); // cqf11: o sorteio da notícia',
     'let r = 0; // mutacao: sem o sorteio',
     'cqf_11 noticia do clube'),
    ('cqf11_um_grupo_so',
     'if cqf11_d(outro + 0x39) != cqf11_d(primeiro + 0x39) {',
     'if false && cqf11_d(outro + 0x39) != cqf11_d(primeiro + 0x39) { // mutacao: um grupo so',
     'cqf_11 noticia do clube'),
    ('cqf11_sem_tirar_do_elenco',
     '                    *pj = 0;',
     '                    let _ = pj; // mutacao: o jogador fica no elenco',
     'cqf_11 noticia do clube'),
]
