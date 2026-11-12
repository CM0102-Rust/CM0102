# Mutacoes (cqx_14), na minitabela (9 chamadas: as tabelas dos segundos colocados das eliminatorias, marco de 2002; so
# decidem depois de junho): a vitoria vale 1 ponto (os pontos da tabela mudam: o comparador lado a lado pega quando a
# confirmacao no modo comparando chega as chamadas; ver PRONTO.txt). A noticia da Liga dos Campeoes (1 chamada): o
# vinculo do clube com o tipo 0x7d0; so o comparador do texto (compare_text_writer) pega.
MUTACOES = [
    ('cqx14_vitoria_vale_1',
     'write_u8(this + league::WIN_POINTS, win_points as u8); // cqx14: the win points',
     'write_u8(this + league::WIN_POINTS, 1); // mutation: 1 point a win',
     'cqx_14 sub league row'),
    ('cqx14_vinculo_do_clube_trocado',
     'add_link(text, club + 0x38, 0x7d5, club); // cqx14: the club link',
     'add_link(text, club + 0x38, 0x7d0, club); // mutation: the competition link type',
     'cqx_14 champions league qualifying text'),
]
