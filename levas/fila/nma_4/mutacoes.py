# Mutacoes (nma_4). A rotina roda 191 vezes na carreira ENG ate 2002-06-01 (manchete e corpo das historias de elogio
# a jogadores); o texto nao entra no save, entao quem acusa e o comparador da rotina (texto e vinculos, original x
# Rust), nos pedacos que rodam em toda chamada (nao se sabe de antemao quais tipos a carreira escreve):
# - o corpo junta a resposta antes do primeiro paragrafo;
# - o veiculo escrito como estrangeiro (com o adjetivo da nacao) quando e nacional, e vice-versa;
# - a manchete "Praise for ..." trocada pelas outras (a condicao invertida);
# - o nome do jogador ligado como clube (0x7d5) em vez de pessoa (0x7d7).
MUTACOES = [
    ('nma4_corpo_invertido',
     '        sprintf(PRAISE_TEXT_BUFFER, 0x991948, p.head(), p.reply()); // "%s\\n\\n%s"',
     '        sprintf(PRAISE_TEXT_BUFFER, 0x991948, p.reply(), p.head()); // mutation: the reply first',
     'nma_4 praise body'),
    ('nma4_veiculo_trocado',
     '    if p.home {\n        f(out, home);',
     '    if !p.home { // mutation: national and foreign swapped\n        f(out, home);',
     'nma_4 praise outlet type'),
    ('nma4_manchete_invertida',
     '    if praise_rem(p.variant, 8) == praise_rem(p.player, 7) {',
     '    if praise_rem(p.variant, 8) != praise_rem(p.player, 7) { // mutation: the headline condition reversed',
     'nma_4 praise headline'),
    ('nma4_vinculo_do_jogador',
     '        link(p.full(), 0x7d7, staff_of(p.player));',
     '        link(p.full(), 0x7d5, staff_of(p.player)); // mutation: the player linked as a club',
     'nma_4 praise links'),
]
