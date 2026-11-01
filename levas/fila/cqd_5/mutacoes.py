# Mutacoes (cqd_5), no dia da liga (0x6847c0, 46522 chamadas ate 2002-06-01) e na criacao dos jogos da temporada
# (0x6835c0, 44 chamadas, na montagem das competicoes):
# - os jogos para a TV sao escolhidos cinco semanas adiante em vez de quatro: as escolhas (sorteio das notas, conflitos
#   na janela) acontecem noutro dia e com outro calendario, e outras partidas vao para a TV;
# - mandante e visitante trocados em todos os jogos criados (as duas formas: o rodizio e a lista propria): os mandos
#   da temporada inteira mudam;
# - a ordem dos clubes nao e sorteada (0x686a70): um sorteio a menos por liga e outra tabela.
# Os contadores ficam colados no efeito: a escolha da TV depois das condicoes; o jogo antes do calendario; o sorteio da
# ordem dentro da condicao da competicao.
MUTACOES = [
    ('cqd5_tv_cinco_semanas',
     'semanas(pd0, px, 4); // cqd5: a TV quatro semanas adiante',
     'semanas(pd0, px, 5); // mutacao: cinco semanas',
     'cqd_5 jogos para a tv'),
    ('cqd5_mando_trocado',
     'let (ea, eb) = (ea, eb); // cqd5: o mandante e o visitante',
     'let (ea, eb) = (eb, ea); // mutacao: o mando trocado',
     'cqd_5 jogo da tabela'),
    ('cqd5_ordem_sem_sorteio',
     'sortear(this, n as u32); // cqd5: a ordem dos clubes',
     '// mutacao: a ordem dos clubes sem sorteio',
     'cqd_5 ordem sorteada'),
]
