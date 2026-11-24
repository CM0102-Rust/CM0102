# Mutacoes (sx17_8), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando (um olheiro com tarefa e um observando o proximo adversario):
# - a ordem por nota atual lendo a potencial (o comparador da ordem acusa quando as duas notas diferem);
# - as escolhas por posicao recusando o id 0 (o comparador acusa so se o jogador de id 0 aparecer: pode escapar).
MUTACOES = [
    ('sx17_8_nota_trocada',
     'player_search_order_by_field_desc(a, b, 0x21) // sx17_8: the current rating',
     'player_search_order_by_field_desc(a, b, 0x25) // mutation: the potential rating'),
    ('sx17_8_id_zero',
     'if id < 0 || id > read_i32(G_STAFF_COUNT) { // sx17_8: the staff id range',
     'if id < 1 || id > read_i32(G_STAFF_COUNT) { // mutation: id 0 refused'),
]
