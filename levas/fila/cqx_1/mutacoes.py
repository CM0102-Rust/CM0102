# Mutacoes (cqx_1), na Liga dos Campeoes asiatica montada no comeco do jogo (as fases jogam na temporada 2001/02,
# com os clubes japoneses): 20 escolhas no embaralhamento (os sorteios mudam e os escolhidos de cada nacao tambem;
# acusada na cqu_10); os 16 da zona oeste de tras para a frente (os confrontos da fase 0 mudam; acusada na cqu_10); o
# primeiro clube da tabela da copa com a posicao prevista 2 (nenhuma faixa tem 2: so o comparador lado a lado da
# rank_teams a pega, pela confirmacao no modo comparando ate 2001-08-20).
MUTACOES = [
    ('cqx1_20_escolhas',
     '0x28); // cqx1: 40 picks in the shuffle',
     '0x14); // mutation: 20 picks in the shuffle',
     'cqx_1 club draw'),
    ('cqx1_zona_oeste_de_tras_para_frente',
     '    asia_cup_winner_zone_knockout_stage(this, 0x987a40, (0x32a, 0x341), "cqx_1 west zone team"); // cqx1: the west zone knockout',
     '    { let l = read_usize(this + 0xb2); for i in 0..8usize { let (a, b) = (read_usize(l + 0x40 + i * 4), read_usize(l + 0x7c - i * 4)); write_usize(l + 0x40 + i * 4, b); write_usize(l + 0x7c - i * 4, a); } } // mutation: the west zone backwards\n    asia_cup_winner_zone_knockout_stage(this, 0x987a40, (0x32a, 0x341), "cqx_1 west zone team");',
     'cqx_1 west zone team'),
    ('cqx1_primeiro_previsto_em_2',
     '    asia_cup_winner_predicted_positions(this, 0x987a40, (0x45a, 0x462)); // cqx1: the predicted positions of the two zones',
     '    asia_cup_winner_predicted_positions(this, 0x987a40, (0x45a, 0x462));\n    exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(read_usize(this + cup::TEAM_TABLE)), 2); // mutation: the first club predicted 2nd',
     'cqx_1 predicted positions'),
]
