# Mutacoes (sbF_8), carreira com a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - a copa com 5 reservas no banco e nao 7: o comparador do construtor acusa (os campos fixos); a carreira, nas
#   escalacoes dos jogos da copa.
# A virada (start_new_season) nao tem mutacao propria: o que ela monta (a copa vazia do ano seguinte) so e usado na
# montagem de agosto de 2002, depois da data da cadeia (ver COMO_ESCREVER_UM_ITEM, "virada de temporada"); o
# comparador confere o ano.
MUTACOES = [
    ('sbF_8_cinco_reservas',
     '    this.set_substitutes_named(7);\n    this.set_discipline_rule(discipline_rule::HOLLAND);',
     '    this.set_substitutes_named(5); // mutation\n    this.set_discipline_rule(discipline_rule::HOLLAND);',
     'sbF_8 dutch cup new'),
]
