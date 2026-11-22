# Mutacoes (tel29_2): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva; estas provam
# os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - o cursor da barra de rolagem com altura minima de 20 (o comparador lado a lado de setup_scrollbar pega em qualquer
#   lista longa com poucas linhas a mostra, como o elenco);
# - itens de mesma linha entram antes dos que ja estao (o comparador lado a lado de add_widget pega ao montar uma
#   tela com dois itens na mesma linha).
MUTACOES = [
    ('tel29_2_cursor_minimo',
     "let thumb = if thumb > 10 { thumb } else { 10 }; // tel29_2: the thumb at least 10 pixels",
     "let thumb = if thumb > 20 { thumb } else { 20 }; // mutation: the thumb at least 20 pixels",
     'tel29_2 scroll bar set up'),
    ('tel29_2_ordem_estavel',
     "if row >= read_i32(area_widget(area, other) + s::WIDGET_ROW) { // tel29_2: sorted by row, stable",
     "if row > read_i32(area_widget(area, other) + s::WIDGET_ROW) { // mutation: equal rows go first",
     'tel29_2 widget added'),
]
