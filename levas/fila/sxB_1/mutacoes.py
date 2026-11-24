# Mutacoes (sxB_1). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando. Cada uma
# num trecho que roda em toda manchete ou em todo texto do escritor:
# - a manchete anunciada vira a da surpresa (frase trocada);
# - o vinculo do nome com o tipo de clube (vinculo trocado; roda em toda manchete e todo corpo quando os nomes conferem).
MUTACOES = [
    ('sxB_aposentadoria_manchete',
     '        } else if r.announced != 0 {\n            0xa6e224 // "... retires"',
     '        } else if r.announced == 0 { // mutation\n            0xa6e224 // "... retires"',
     'sxB_1 retirement headline'),
    ('sxB_aposentadoria_vinculo',
     '        exe::text_add_link(out, name.as_ptr(), 0x7d7, r.person());\n    }\n}\n\n/// Comparer for `player_regen_write_retirement_news_text`',
     '        exe::text_add_link(out, name.as_ptr(), 0x7d5, r.person()); // mutation\n    }\n}\n\n/// Comparer for `player_regen_write_retirement_news_text`',
     'sxB_1 retirement headline'),
]
