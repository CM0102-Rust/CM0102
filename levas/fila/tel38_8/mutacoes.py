# A base da interface (8): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat, no grafico da posicao na liga (a tela do
# tel23_2: a liga, o grafico de progresso de um clube):
# - a segunda linha do grafico um pixel abaixo em vez de acima (draw_line_graph lado a lado);
# - o lado de cima da moldura na cor do de baixo (draw_axes lado a lado).
MUTACOES = [
    ('tel38_8_linha_grossa_abaixo',
     "plot_line(px, py - 1, x, y - 1, colour); // tel38_8: the line a pixel thicker",
     "plot_line(px, py + 1, x, y + 1, colour); // mutation: the second line below"),
    ('tel38_8_topo_preto',
     "plot_line(x0, y0, x1, y0, blue); // tel38_8: the top side",
     "plot_line(x0, y0, x1, y0, black); // mutation: the top side black"),
]
