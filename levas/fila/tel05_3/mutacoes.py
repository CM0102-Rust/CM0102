# Telas do clube (5c): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (caminho: noticia de pedido dos numeros do elenco ou
# do elenco europeu > abrir; ou tela do clube humano antes de uma partida com numeros fixos):
# - o numero livre da grade ligando para 6 em vez de 5;
# - o botao Submit com a ligacao 0xd.
MUTACOES = [
    ('tel05_3_numero_livre',
     '[0x46, 0, 0, 0, 0, col, 0, 0x30, c3150, c3150, 0xc, 1, c3184, 0xde1f64, 0, 5, n, area]',
     '[0x46, 0, 0, 0, 0, col, 0, 0x30, c3150, c3150, 0xc, 1, c3184, 0xde1f64, 0, 6, n, area]'),
    ('tel05_3_submit',
     'tel26_area_button([bar, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xc, 0]);',
     'tel26_area_button([bar, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xd, 0]);'),
]
