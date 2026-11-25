# Mutacoes (rx11b_1), pela carreira ENG ate 2002-06-01 com o motor de lances: mudam quem joga e onde.
# - os reservas jogam no campo do mandante do jogo principal (sem a troca de mando): outros estadios, publico e resultados
#   dos jogos reservas (a forma e a condicao dos jogadores que jogam neles);
# - o jogo reserva 7 dias depois (9 em vez de 2): outros dias, outros conflitos com o calendario e outro descanso;
# - o jogo B ou sub-21 tambem entre selecoes de reputacao 1500 (em vez de 2500): o plano da copia passa a pedir jogos
#   ligados que a original nao marca, e o comparador do B/sub-21 acusa (a carreira so pelos jogadores convocados).
MUTACOES = [
    ('rx11b_1_sem_troca_de_mando',
     'if read_i32(game.competition().nation().addr()) != read_i32(KEY_NATION_TURKEY) { // rx11b_1: the reserves play at the other ground',
     'if false && read_i32(game.competition().nation().addr()) != read_i32(KEY_NATION_TURKEY) { // mutation',
     'rx11b_1 reserve fixture added'),
    ('rx11b_1_reserva_7_dias_depois',
     '        exe::date_add_days(date, t.as_mut_ptr() as usize, 2)\n',
     '        exe::date_add_days(date, t.as_mut_ptr() as usize, 9) // mutation\n',
     'rx11b_1 reserve fixture added'),
    ('rx11b_1_reputacao_1500',
     'if (home.reputation() as i16) < 0x9c4 || (away.reputation() as i16) < 0x9c4 {',
     'if (home.reputation() as i16) < 0x5dc || (away.reputation() as i16) < 0x5dc { // mutation',
     'rx11b_1 Under 21 international'),
]
