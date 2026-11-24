# Mutacoes (sxB_5). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando. Cada uma
# num trecho que roda em toda manchete ou todo corpo do seu escritor:
# - pedido a diretoria: o vinculo do clube com o tipo de pessoa (roda em toda manchete e todo corpo);
# - ultimato: a manchete "recusa" vira "concorda" (frase trocada, a condicao invertida).
MUTACOES = [
    ('sxB_pedido_vinculo',
     '    exe::text_add_link(out, long as *const u8, 0x7d5, club);\n}\n\n/// Comparer for `news_board_request_reply_text`',
     '    exe::text_add_link(out, long as *const u8, 0x7da, club); // mutation\n}\n\n/// Comparer for `news_board_request_reply_text`',
     'sxB_5 board request headline'),
    ('sxB_ultimato_manchete',
     '        } else if matches!(answer, 2 | 3 | 4) {',
     '        } else if !matches!(answer, 2 | 3 | 4) { // mutation',
     'sxB_5 ultimatum headline'),
]
