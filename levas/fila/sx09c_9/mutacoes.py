# Mutacoes (sx09c_9). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando. Cada uma
# num trecho que roda em toda manchete ou todo corpo do escritor (sem recurso pendente):
# - a manchete de um jogo vira a de N jogos (frase trocada);
# - o vinculo do primeiro clube com o tipo de pessoa (vinculo trocado; roda em todo corpo).
MUTACOES = [
    ('sx09c_suspensao_a_mais_manchete',
     '            if b.count == 1 {\n                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np);',
     '            if b.count != 1 { // mutation\n                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np);',
     'sx09c_9 further ban headline'),
    ('sx09c_suspensao_a_mais_vinculo',
     '    exe::text_add_link(out, (c1.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c1);\n    exe::text_add_link(out, (c2.addr()',
     '    exe::text_add_link(out, (c1.addr() + club::SHORT_NAME) as *const u8, 0x7d7, c1); // mutation\n    exe::text_add_link(out, (c2.addr()',
     'sx09c_9 further ban body'),
]
