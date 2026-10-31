# Mutacoes, todas na suspensao do cartao vermelho (cartoes vermelhos em toda rodada; o orgao ingles, por exemplo, pede
# a suspensao em jogos, 0x7dd772 com por_jogos = 1). O registro volta para o orgao do pais, que o copia para a
# disciplina do jogo: um jogo a mais de suspensao tira o jogador de mais uma escalacao; devolver 0 faz o orgao nao
# aplicar a suspensao; ler o codigo da falta do outro campo (+0x2f com o tipo 0) muda os jogos ou tira a suspensao
# (PODE ESCAPAR se os dois campos coincidirem sempre no tipo 0). As escalacoes mudam e os sorteios das partidas tambem.
MUTACOES = [
    ('cpu10_um_jogo_a_mais', '*(r as *mut u8) = jogos; // cpu10: os jogos da suspensão',
     '*(r as *mut u8) = jogos.wrapping_add(1); // mutacao: um jogo a mais',
     'cpu_10 punicao'),
    ('cpu10_sem_punicao', '1 // cpu10: a punição feita',
     '0 // mutacao: diz que nao ha suspensao',
     'cpu_10 punicao'),
    ('cpu10_falta_do_outro_campo', '0 => *((j + 0x2e) as *const i8) as i32, // cpu10: a falta do primeiro tipo',
     '0 => *((j + 0x2f) as *const i8) as i32, // mutacao: o codigo do outro campo',
     'cpu_10 punicao'),
]
