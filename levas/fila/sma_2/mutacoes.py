# Mutacoes (sma_2). Na carreira de teste a nomeacao roda uma vez so, na criacao do jogo (0x826f20, Setup.cpp): o
# tecnico humano assume o clube dele (sem emprego antigo; os ramos da vaga no clube antigo e do tecnico-jogador nao
# rodam ate 2002-06-01). As mutacoes ficam no caminho que roda:
# - a diretoria do clube nao recomeca (reset_board_for_new_manager): outra confianca, outras expectativas;
# - os treinos do clube humano nao sao refeitos para ele (outro desenvolvimento dos jogadores);
# - a nacao do clube humano nao ganha a marca 2 da liga do humano.
MUTACOES = [
    ('sma2_sem_diretoria_nova',
     'reset(this, club.addr()); // sma2: the new board',
     'let _ = reset; // mutation: the board does not restart',
     'sma_2 human manager'),
    ('sma2_sem_treino_do_humano',
     "exe::training_manager_reassign_squad_schedules(read_usize(G_TRAINING_MANAGER), p, old, club.addr() as u32); // sma2: the human's training",
     'let _ = old; // mutation: the training schedules stay',
     'sma_2 human manager'),
    ('sma2_sem_marca_da_liga_humana',
     "cn.set_league_selected(cn.league_selected() | 2); // sma2: the human's league mark",
     "let _ = cn; // mutation: no human league mark",
     'sma_2 human manager'),
]
