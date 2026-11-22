# Mutacoes (tel02_1): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva; estas provam
# os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a linha "No player restrictions"/regras invertida (a aba Next Match de qualquer clube com jogo marcado);
# - a lista de jogos pula um dia sim, outro nao (a aba Fixtures de um clube, depois da primeira abertura, que cria a
#   lista; na primeira so roda a original);
# - o escudo nunca acha o nome (a aba Info > General Info de um clube espanhol cujo escudo esteja em CM3_PICTURES).
MUTACOES = [
    ('tel02_1_regra_limite',
     "if row() == first_rule {",
     "if row() != first_rule { // mutation: the no-restrictions row inverted",
     'tel02_1 next match tab'),
    ('tel02_1_dia_seguinte',
     "let t = exe::date_add_days(d1, tmp, 1);",
     "let t = exe::date_add_days(d1, tmp, 2); // mutation: every other day",
     'tel02_1 fixtures tab'),
    ('tel02_1_escudo',
     "if badge_names_equal(entry_name, name) {",
     "if badge_names_equal(entry_name, name) && false { // mutation: no badge found",
     'tel02_1 badge picture of a Spanish club'),
]
