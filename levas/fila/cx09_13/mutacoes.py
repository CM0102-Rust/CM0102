# Mutacoes (cx09_13) pela carreira com a Belgica ate 2002-08-01 (o jogo e criado do zero: construtor e grupo B rodam no
# inicio e o grupo B de novo na virada):
# - o grupo B com 1 ponto por vitoria (o byte do empate no lugar do da vitoria): a classificacao do grupo B muda (o
#   molde serve tambem ao grupo sul finlandes do cx09_20);
# - a Terceira Divisao com a regra de disciplina 5 (e nao a belga): as suspensoes mudam (o comparador do construtor
#   tambem acusa a regra).
MUTACOES = [
    ('cx09_13_grupo_b_um_ponto',
     'list, legs, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,',
     'list, legs, l.competition().addr(), table, rounds as u16 as u32, l.draw_points() as u32, l.draw_points() as u32, // mutation\n'),
    ('cx09_13_outra_disciplina',
     '    l.set_discipline_rule(discipline_rule::BELGIUM_LEAGUE);\n    if exe::league_load_from_temp_file(this, 1) != 0 {\n        return this;',
     '    l.set_discipline_rule(discipline_rule::BELGIUM_LEAGUE + 1); // mutation\n    if exe::league_load_from_temp_file(this, 1) != 0 {\n        return this;'),
]
