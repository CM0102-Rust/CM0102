# Mutacoes (sx01e_4). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (o texto, os vinculos
# e o gerador), com o modo CM_TEXTO_NOTICIAS na carreira automatica ou abrindo a noticia jogando comparando:
# - gesto obsceno: o placar com o menor primeiro (todo corpo);
# - insatisfacao com a venda: o vinculo do nome longo do vendedor com o tipo de pessoa (todo corpo);
# - expulsao: a manchete troca as duas frases.
MUTACOES = [
    ('sx01e_gesto_placar',
     'story_crt!(score.as_mut_ptr(), 0x98c114, g1.max(g2) as i32, g1.min(g2) as i32); // "%d-%d"',
     'story_crt!(score.as_mut_ptr(), 0x98c114, g1.min(g2) as i32, g1.max(g2) as i32); // mutation',
     'sx01e_4 obscene gesture text during the match'),
    ('sx01e_venda_vinculo',
     '    t.link(seller_long, 0x7d5, u.seller.addr());',
     '    t.link(seller_long, 0x7d7, u.seller.addr()); // mutation',
     'sx01e_4 sale unrest text body'),
    ('sx01e_expulsao_manchete',
     "let tpl = if exe::newsitem_get_slot(news, 8) != 0 { 0xa56ae4 } else { 0xa56aa4 };",
     "let tpl = if exe::newsitem_get_slot(news, 8) == 0 { 0xa56ae4 } else { 0xa56aa4 }; // mutation",
     'sx01e_4 sending off text headline'),
]
