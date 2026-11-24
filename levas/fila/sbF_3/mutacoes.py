# Mutacoes (sbF_3), carreira com a Grecia ate 2002-08-01. Nenhuma abre caixa.
# - a copa com 5 reservas no banco e nao 7: o comparador do construtor acusa (os campos fixos); a carreira, nas
#   escalacoes da copa;
# - os primeiros e os vices trocados no vetor do mata-mata: quem joga a 1a rodada (dezembro) e quem entra na 2a muda
#   (a carreira acusa; o comparador confere so o conjunto).
MUTACOES = [
    ('sbF_3_cinco_reservas',
     '    this.set_substitutes_named(7);\n    this.set_discipline_rule(discipline_rule::GREECE);',
     '    this.set_substitutes_named(5); // mutation\n    this.set_discipline_rule(discipline_rule::GREECE);',
     'sbF_3 greek cup new'),
    ('sbF_3_primeiros_e_vices_trocados',
     'for (half, row) in [(0usize, 1i32), (GREEK_CUP_GROUPS, 0)] {',
     'for (half, row) in [(0usize, 0i32), (GREEK_CUP_GROUPS, 1)] { // mutation',
     'sbF_3 greek cup knockout'),
]
