# Mutacoes (cx15_6), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - o pedido da lista de numeros nao e enviado: a noticia some da caixa do humano (a resposta automatica do humano de
#   ferias deixa de rodar);
# - o texto do pedido sem a ligacao do clube: acusado pela comparacao com CM_TEXTOS=1 (o texto nao volta a simulacao);
# - a noticia da inscricao europeia tambem com 25 inscritos: o humano recebe a noticia a mais (so se o clube humano
#   estiver numa copa europeia; pode escapar da carreira de teste, como as demais desta rotina).
MUTACOES = [
    ('cx15_6_pedido_nao_enviado',
     '    exe::newsitem_set_slot(p, 1, 0);\n    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);',
     '    exe::newsitem_set_slot(p, 1, 0); // mutation',
     'cx15_6 squad numbers request'),
    ('cx15_6_texto_sem_ligacao',
     '    exe::text_set_string(text, 0xde1f64 as *const u8);\n    exe::text_add_link(text, name as *const u8, 0x7d5, c);\n    if body != 0 && submitted != 0 {',
     '    exe::text_set_string(text, 0xde1f64 as *const u8); // mutation\n    if body != 0 && submitted != 0 {'),
    ('cx15_6_inscricao_com_25',
     '    if count == 25 && step != 3 {\n        return;\n    }',
     '    if count == 125 && step != 3 { // mutation\n        return;\n    }',
     'cx15_6 euro registration news'),
]
