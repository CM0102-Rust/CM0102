# Mutacoes (sx15_7, item de TELAS: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - a doenca e a lesao trocadas no texto do jogador cortado: acusada pelo compare_text_writer (texto);
# - os vinculos dos adversarios do pedido de convocacao com o codigo de clube fixo: acusada pelo compare_text_writer
#   (vinculos) quando o pedido e de um segundo time; no time principal o codigo ja e 0x7d5 (escapa).
MUTACOES = [
    ('sx15_7_doenca_lesao',
     'write_player_squad_news_text(news, body, out, (0xa5c80c, 0xa5c790), 0xa5c73c); // sx15_7: illness, injury',
     'write_player_squad_news_text(news, body, out, (0xa5c790, 0xa5c80c), 0xa5c73c); // mutation: swapped',
     'sx15_7 withdrawn text'),
    ('sx15_7_vinculo_adversario',
     'exe::text_add_link(out, (o.addr() + club::SHORT_NAME) as *const u8, squad_link_kind(variant), *o); // sx15_7: the opponent links',
     'exe::text_add_link(out, (o.addr() + club::SHORT_NAME) as *const u8, 0x7d5, *o); // mutation: always the club link',
     'sx15_7 squad request text'),
]
