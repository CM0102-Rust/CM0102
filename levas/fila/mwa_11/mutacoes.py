# Mutacoes (mwa_11), na Premier da Irlanda, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a liga do ano seguinte;
# - 2 pontos por vitoria em vez de 3 (a classificacao muda; os rebaixados das regras a tabela de doze reescreve);
# - com doze clubes, dois rebaixados em vez de tres (a virada de 2002 muda as divisoes).
MUTACOES = [
    ('mwa11_premier_ano_seguinte',
     'construct_irish_league(League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, "mwa_11 irish premier new")',
     'construct_irish_league(League(this), year + 1, record, &IRISH_PREMIER_CONSTRUCTOR, "mwa_11 irish premier new")',
     'mwa_11 irish premier new'),
    ('mwa11_dois_pontos_por_vitoria',
     'writes: &[(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3),',
     'writes: &[(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 2), /* mutation: two points for a win */',
     'mwa_11 irish premier rules'),
    ('mwa11_doze_com_dois_rebaixados',
     'this.set_relegated_count(3); // mwa11: three go down from a twelve-club Premier',
     'this.set_relegated_count(2); // mutation: two go down',
     'mwa_11 irish premier of twelve'),
]
