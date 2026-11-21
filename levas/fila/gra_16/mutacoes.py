# Mutacoes (gra_16). O texto nao volta para a simulacao: acusadas pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - a suspensao de um jogo por cartoes amarelos (a mais comum na carreira ENG) com o modelo de "mais uma" suspensao;
# - o vinculo do clube do jogador apontando o adversario (suspensao com jogo).
MUTACOES = [
    ('gra16_um_jogo_por_amarelos',
     '    [0x9bed88, 0x9bee20, 0x9becb8], // gra16: one match',
     '    [0x9bee20, 0x9bee20, 0x9becb8], // mutation: the further template',
     'gra_16 ban template'),
    ('gra16_vinculo_do_clube',
     'exe::text_add_link(out, (c1 + club::SHORT_NAME) as *const u8, 0x7d5, c1);',
     'exe::text_add_link(out, (c1 + club::SHORT_NAME) as *const u8, 0x7d5, c2); // mutation: the opponent',
     'gra_16 ban with a match'),
]
