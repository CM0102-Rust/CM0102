# Mutações (cqp_1): a cadeia roda uma vez até 2002-06-01 na carreira de 8 ligas (o fim da temporada de uma liga com
# divisão de baixo [inferência: a J1/J2 do Japão, em dezembro]); os clubes que sobem e descem mudam de competição
# (+0x57), o que muda os jogos da temporada seguinte e o banco (hash diário).
# - sem a troca entre as duas ligas (ninguém sobe nem desce) (contador "cqp_1 cadeia");
# - a troca com as duas ligas invertidas (a de baixo no lugar da de cima) (contador "cqp_1 cadeia"). A primeira
#   tentativa (`aos_pares` invertido) escapou: com o mesmo número de rebaixados e promovidos, trocar aos pares ou em
#   bloco dá o mesmo resultado;
# - sem fechar a liga de baixo antes da troca (contador "cqp_1 cadeia").
# As notícias do prêmio e da TV só chegam à caixa do técnico humano (a carreira de férias não as lê): sem mutação.
MUTACOES = [
    ('cqp1_sem_troca', 'trocar(this, acima, abaixo, 1, aos_pares, u32::MAX, u32::MAX); // cqp1: sobem e descem', 'let _ = trocar; // mutacao: ninguém sobe nem desce', 'cqp_1 cadeia'),
    ('cqp1_ligas_trocadas', 'trocar(this, acima, abaixo, 1, aos_pares, u32::MAX, u32::MAX); // cqp1: sobem e descem', 'trocar(this, abaixo, acima, 1, aos_pares, u32::MAX, u32::MAX); // mutacao: a de baixo no lugar da de cima', 'cqp_1 cadeia'),
    ('cqp1_sem_fechar_a_de_baixo', 'finalize_league_table(abaixo); // cqp1: fecha a de baixo', 'let _ = abaixo; // mutacao: sem fechar a de baixo', 'cqp_1 cadeia'),
]
