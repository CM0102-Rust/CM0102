# Mutacoes (cx05_9), carreira de 8 ligas ate 2004-07-15 (as quartas-de-final da Eurocopa de 2004 em 22 de junho, a
# final em 30 de junho e as honras logo depois).
# - a primeira vaga das quartas e o segundo do grupo 2 e nao o primeiro: o vencedor do grupo 2 fica fora e o segundo
#   joga duas vezes;
# - as honras da temporada sao as do grupo 4 (a fase 2) e nao as do mata-mata: o historico da Eurocopa muda.
MUTACOES = [
    ('cx05_9_segundo_do_grupo_2',
     "    write_usize(teams, club(1, 0)); // cx05_9: group 2's winner opens the draw",
     "    write_usize(teams, club(1, 1)); // mutation"),
    ('cx05_9_honras_do_grupo_4',
     '    let knockout = read_usize(league.stages() + 0xc); // cx05_9: the knockout is the fourth stage',
     '    let knockout = read_usize(league.stages() + 8); // mutation'),
]
