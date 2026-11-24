# Mutacoes (sxB_4). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando. Cada uma
# num trecho que roda em toda manchete ou todo corpo do seu escritor:
# - boato de cargo: o vinculo do clube do cargo com o tipo de pessoa (roda em todo corpo);
# - expectativas: o vinculo da competicao com o tipo de clube (roda em todo corpo);
# - oferta de emprego: a manchete com o nome longo no lugar do curto (frase trocada).
MUTACOES = [
    ('sxB_boato_vinculo',
     '    exe::text_add_link(out, tgt as *const u8, 0x7d5, target);\n}',
     '    exe::text_add_link(out, tgt as *const u8, 0x7da, target); // mutation\n}',
     'sxB_4 job rumour body'),
    ('sxB_expectativas_vinculo',
     '    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);',
     '    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d5, comp); // mutation',
     'sxB_4 FA expectations body'),
    ('sxB_oferta_manchete',
     '        story_lang!(tp, 0xa0b600, g, g, club.addr() + club::SHORT_NAME);',
     '        story_lang!(tp, 0xa0b600, g, g, club.addr() + club::NAME); // mutation',
     'sxB_4 job offer headline'),
]
