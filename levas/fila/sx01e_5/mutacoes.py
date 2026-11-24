# Mutacoes (sx01e_5). O texto nao volta para a simulacao: acusadas pela comparacao do escritor, com o modo
# CM_TEXTO_NOTICIAS na carreira automatica ou abrindo a noticia jogando comparando:
# - contratacao: o vinculo do nome longo do clube com o tipo de pessoa (todo corpo);
# - titulo: o vinculo da competicao na manchete perdido;
# - agradecimento: a manchete "agradece" com o nome trocado pelo texto vazio (todo assunto comum).
MUTACOES = [
    ('sx01e_contratacao_vinculo',
     '    t.link(long, 0x7d5, s.club.addr());\n    if s.who == 2 && s.spokesman.matches() {',
     '    t.link(long, 0x7d7, s.club.addr()); // mutation\n    if s.who == 2 && s.spokesman.matches() {',
     'sx01e_5 signing text fans'),
    ('sx01e_titulo_manchete',
     '    t.link(comp_short, 0x7d0, s.comp);',
     '    // mutation: no competition link',
     'sx01e_5 trophy text headline'),
    ('sx01e_agradecimento_manchete',
     '        _ => media_text!(t, 0xa242f0, p), // "... thanks manager"',
     '        _ => media_text!(t, 0xa242f0, 0xad9c64), // mutation',
     'sx01e_5 gratitude text headline'),
]
