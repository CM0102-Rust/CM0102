# Mutacoes (cx05_8), carreira de 8 ligas ate 2004-07-15 (os grupos nascem no sorteio de 10 de fevereiro de 2004; os
# jogos em junho).
# - o grupo 1 pega as sorteadas do grupo 2 (o grupo 1 repete o 2);
# - cada fase pega o grupo anterior (o grupo 2 repete o 1, o 3 o 2, o 4 o 3);
# - as finalistas nao passam a ter a Eurocopa como competicao atual (a avaliacao dos tecnicos das selecoes muda).
MUTACOES = [
    ('cx05_8_grupo_1_do_2',
     '        if read_u8(entry + 4) == 1 { // cx05_8: the entries of group 1',
     '        if read_u8(entry + 4) == 2 { // mutation'),
    ('cx05_8_fase_do_grupo_anterior',
     "    let group = g + 2; // cx05_8: the stage's group in the draw list",
     "    let group = g + 1; // mutation"),
    ('cx05_8_sem_competicao_atual',
     "        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr(), league.competition().addr()); // cx05_8: the finalists' current competition",
     "        let _ = team; // mutation"),
]
