# Mutacoes (sx01b_1). O texto nao volta para a simulacao: acusadas pela comparacao (o comparador da rotina,
# compare_text_writer, compara o texto, os vinculos e o gerador), como as de texto aceitas em 12/11/2026. As tres
# primeiras rodam em toda chamada (manchete e corpo, diretoria e torcida); a quarta em toda noticia da diretoria com
# presidente.
# - o vinculo do nome curto do tecnico com o tipo de clube;
# - a resposta do tecnico lida da vaga do sorteio;
# - o vinculo do nome do clube com o tipo de competicao;
# - o presidente pelo nome com a regra invertida.
MUTACOES = [
    ('sx01b_vinculo_do_tecnico',
     "exe::text_add_link(out, s.manager_short.as_ptr(), 0x7d7, s.manager.addr()); // sx01b_1: the manager's links",
     "exe::text_add_link(out, s.manager_short.as_ptr(), 0x7d5, s.manager.addr()); // mutation: the manager linked as a club",
     'sx01b_1 reaction links'),
    ('sx01b_resposta_do_sorteio',
     '    let variant = slot(2) as i32;\n',
     '    let variant = slot(3) as i32; // mutation: the answer read from the draw\n',
     'sx01b_1 reaction links'),
    ('sx01b_vinculo_do_clube',
     'exe::text_add_link(out, c.name_addr() as *const u8, 0x7d5, c.addr());',
     'exe::text_add_link(out, c.name_addr() as *const u8, 0x7d0, c.addr()); // mutation: the club linked as a competition',
     'sx01b_1 reaction links'),
    ('sx01b_presidente_pelo_nome',
     'sheet.interference() as i32 % 20 >= s.confidence % 5',
     'sheet.interference() as i32 % 20 < s.confidence % 5 // mutation: the chairman named by the opposite rule',
     'sx01b_1 reaction board'),
]
