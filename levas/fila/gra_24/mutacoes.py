# Mutacoes (gra_24). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - o vinculo do adversario trocado pelo do proprio clube: no fim comum, toda chamada (qualquer ramo) diverge.
MUTACOES = [
    ('gra24_vinculo_do_adversario',
     "exe::text_add_link(text, (opponent.addr() + club::SHORT_NAME) as *const u8, 0x7d5, opponent); // gra24: the opponent's link",
     "exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club); // mutation: the club's link in place of the opponent's",
     'gra_24 cup tie news'),
]
