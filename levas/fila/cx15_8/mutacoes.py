# Mutacoes (cx15_8), carreira ENG ate 2002-06-01 (a noticia sai em 1/1/2002). Nenhuma abre caixa.
# - a noticia so para a ultima nacao: as outras nacoes nao recebem (a caixa do humano ingles muda se a Inglaterra nao
#   for a ultima; pode escapar da carreira se nada mais ler a noticia, e o comparador so roda a original);
# - o texto sem a ligacao do clube: acusado pela comparacao com CM_TEXTOS=1.
MUTACOES = [
    ('cx15_8_so_a_ultima_nacao',
     '                covered!("cx15_8 qualifiers of a nation sent");\n                send_nation_qualifiers(news, nation, next);',
     '                covered!("cx15_8 qualifiers of a nation sent"); // mutation',
     'cx15_8 qualifiers of a nation sent'),
    ('cx15_8_texto_sem_ligacao_do_clube',
     '            exe::text_add_link(text, name as *const u8, 0x7d5, c);\n            k = k.wrapping_add(1);',
     '            k = k.wrapping_add(1); // mutation',
     'cx15_8 last qualified club'),
]
