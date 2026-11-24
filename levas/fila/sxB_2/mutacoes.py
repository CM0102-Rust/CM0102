# Mutacoes (sxB_2). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_media_text_writer:
# o texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando. Cada uma
# num trecho que roda em toda manchete ou todo corpo do seu escritor:
# - quer ser titular: o vinculo do nome longo do clube com o tipo de pessoa (roda em todo corpo);
# - desabafo: a manchete "desanimado" vira "desabafa contra o clube" (frase trocada; o caso comum);
# - desabafo: o vinculo do nome curto da selecao perdido (roda em todo corpo).
MUTACOES = [
    ('sxB_titular_vinculo_longo',
     '    t.link(long, 0x7d5, w.club.addr());',
     '    t.link(long, 0x7d7, w.club.addr()); // mutation',
     'sxB_2 wants first team body'),
    ('sxB_desabafo_manchete',
     '        story_lang!(t.buf(), 0xa27350, name.ptr()); // "... disheartened"',
     '        story_lang!(t.buf(), 0xa27380, 0x64, name.ptr()); // mutation',
     'sxB_2 outburst headline'),
    ('sxB_desabafo_selecao',
     '    t.link(nation.addr() + nation::SHORT_NAME, 0x7d5, exe::database_get_national_team(nation));\n',
     '    // mutation: no short name link\n',
     'sxB_2 outburst body'),
]
