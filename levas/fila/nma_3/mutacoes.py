# Mutacoes (nma_3). A rotina roda 103 vezes na carreira ENG ate 2002-06-01 (manchete e corpo das historias da midia
# sobre jogadores); o texto nao entra no save, entao quem acusa e o comparador da rotina (texto e vinculos, original x
# Rust) nos pedacos que rodam sempre:
# - o corpo junta a resposta antes da historia (os vinculos e o resto iguais);
# - o veiculo local escrito como estrangeiro (com o adjetivo da nacao) e vice-versa;
# - o tecnico do clube que apoia passa a criticar (a resposta 1 vira a 2);
# - a manchete liga o nome curto do clube com o tipo de pessoa (0x7d7) em vez de clube (0x7d5).
MUTACOES = [
    ('nma3_corpo_invertido',
     'story_crt!(self.long_extra.as_mut_ptr(), 0x991948, self.text.as_ptr(), self.extra.as_ptr());',
     'story_crt!(self.long_extra.as_mut_ptr(), 0x991948, self.extra.as_ptr(), self.text.as_ptr()); // mutation: the reply first',
     'nma_3 links'),
    ('nma3_veiculo_trocado',
     '        if self.local != 0 {\n            story_lang!(out, plain);',
     '        if self.local == 0 { // mutation: local and foreign swapped\n            story_lang!(out, plain);',
     'nma_3 outlet'),
    ('nma3_apoio_vira_critica',
     '                1 => self.club_manager_supports(),',
     '                1 => self.club_manager_criticises(), // mutation: support becomes criticism',
     'nma_3 club manager reply'),
    ('nma3_manchete_vinculo',
     '    exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);\n    exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);\n    let n',
     '    exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d7, c); // mutation: the club linked as a person\n    exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);\n    let n',
     'nma_3 headline'),
]
