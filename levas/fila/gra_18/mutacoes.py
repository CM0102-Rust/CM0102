# Mutacoes (gra_18). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - o vinculo do clube com outro tipo (0x7d6): manchete e corpo de todos os tipos passam por ele, toda chamada diverge.
MUTACOES = [
    ('gra18_vinculo_do_clube',
     "exe::text_add_link(out, name as *const u8, 0x7d5, club); // gra18: the club's link",
     "exe::text_add_link(out, name as *const u8, 0x7d6, club); // mutation: the club's link with another type",
     'gra_18 record news'),
]
