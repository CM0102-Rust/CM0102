# Mutacoes (sx16_7) pela prova do save (mutacoes.py --prova-save N; CARGA). Sem o 4o campo: a base da prova roda com
# as rotinas desligadas.
# - o ultimo id de noticia lido do save fica 0: as noticias novas depois da carga recebem ids repetidos (a impressao
#   do save gravado depois da carga muda);
# - o armazem novo comeca no id 1 (e nao 0): os ids das noticias da criacao do jogo da prova (o .qst gravado) mudam.
MUTACOES = [
    ('sx16_7_ultimo_id_zero',
     '    if exe::zread(this + news_fields::LAST_ID, 4, 1, z) != 1 {\n        news_fatal(0x254a);\n    }',
     '    if exe::zread(this + news_fields::LAST_ID, 4, 1, z) != 1 {\n        news_fatal(0x254a);\n    }\n    write_u32(this + news_fields::LAST_ID, 0); // mutation'),
    ('sx16_7_primeiro_id',
     '    write_u32(this + news_fields::LAST_ID, 0);\n    this\n',
     '    write_u32(this + news_fields::LAST_ID, 1); // mutation\n    this\n'),
]
