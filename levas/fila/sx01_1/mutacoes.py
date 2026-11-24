# Mutacoes (sx01_1). O texto nao volta para a simulacao: acusadas pela comparacao (o comparador da rotina compara o
# texto, os vinculos e o gerador), como as de texto aceitas em 12/11/2026. Todas em trechos que rodam em toda chamada
# (manchete, corpo, ponto de vista, vinculos), nao em um tipo so.
# - o vinculo do clube na manchete com o tipo da competicao;
# - o corpo com a resposta antes da historia;
# - "em casa" nunca marcado (o veiculo e os textos de fora para os clubes da nacao do tecnico na tela);
# - o vinculo do nome da competicao com o tipo de clube.
MUTACOES = [
    ('sx01_vinculo_da_manchete',
     "exe::text_add_link(out, TeamText::short(c) as *const u8, 0x7d5, c); // sx01_1: the headline's club link",
     "exe::text_add_link(out, TeamText::short(c) as *const u8, 0x7d0, c); // mutation: the headline's club linked as a competition",
     'sx01_1 team text headline'),
    ('sx01_resposta_antes',
     'story_crt!(joined.as_mut_ptr(), 0x991948, s.text.as_ptr(), s.second.as_ptr()); // "%s\\n\\n%s"',
     'story_crt!(joined.as_mut_ptr(), 0x991948, s.second.as_ptr(), s.text.as_ptr()); // mutation: the reply first',
     'sx01_1 team text body'),
    ('sx01_em_casa',
     '        s.home = true;\n',
     '        s.home = false; // mutation: never at home\n',
     'sx01_1 team text body'),
    ('sx01_vinculo_da_competicao',
     'exe::text_add_link(out, TeamText::long(k) as *const u8, 0x7d0, k);',
     'exe::text_add_link(out, TeamText::long(k) as *const u8, 0x7d5, k); // mutation: the competition linked as a club',
     'sx01_1 team text body'),
]
