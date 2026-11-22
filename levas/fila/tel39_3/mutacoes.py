# Mutacoes (tel39_3). As classificacoes do ano so viram premio no fim da temporada e o registro do tecnico humano
# nao volta para a simulacao: acusadas pelos comparadores (lado a lado), como as de texto.
# - o limite de reputacao 2000 vira 1000 (classificacao mundial: jogadores entre 1000 e 2000 passam a contar);
# - o jogador frances deixa de contar pela nacao (os franceses da Premier League saem da segunda classificacao);
# - a posicao entre os 16 registros desloca um (o registro do tecnico humano errado).
MUTACOES = [
    ('tel39_3_reputacao',
     'read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 2000) as u32',
     'read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 1000) as u32',
     'tel39_3 year ratings filter'),
    ('tel39_3_franca',
     '|| own(KEY_NATION_FRANCE) || comp(KEY_NATION_FRANCE)',
     '|| comp(KEY_NATION_FRANCE)',
     'tel39_3 year ratings filter'),
    ('tel39_3_registro',
     'let k = read_i32(m).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16);',
     'let k = read_i32(m).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(15);',
     'tel39_3 human record'),
]
