# Mutacoes (gra_2c), pela carreira ENG ate 2002-06-01. O comparador so roda a original, entao cada uma tem de mudar a
# carreira Rust: todas mexem em quem joga (a habilidade e os atributos dos jogadores, tres dias por ano cada um, ou o
# dia do jogador) e na contagem de sorteios.
# - o crescimento do dia de desenvolvimento tres passos acima na tabela;
# - o terceiro dos tres dias do ano 127 dias depois do segundo (e nao 120);
# - o crescimento treinado preso a 2..40 (e nao 2..25);
# - o jogador de 30 a 32 anos perde aceleracao acima de 8 (e nao de 18);
# - o portao do dia do jogador aberto a partir da reputacao 2048 (e nao 4750).
MUTACOES = [
    ('gra2c_crescimento_tres_passos',
     '    d.growth = step(growth_at);\n',
     '    d.growth = step(growth_at + 3); // mutation\n',
     'gra_2c development day'),
    ('gra2c_terceiro_dia',
     '    let third = ((second as i32 + 120) % 365) as i16;',
     '    let third = ((second as i32 + 127) % 365) as i16; // mutation',
     'gra_2c development day'),
    ('gra2c_treino_ate_40',
     '    d.growth = (trained as i16).clamp(2, 25);',
     '    d.growth = (trained as i16).clamp(2, 40); // mutation',
     'gra_2c training factor'),
    ('gra2c_aceleracao_aos_30',
     '        sd_maybe_lose_one(pd, player_sheet::ACCELERATION, 18);',
     '        sd_maybe_lose_one(pd, player_sheet::ACCELERATION, 8); // mutation',
     'gra_2c ageing 30 to 32'),
    ('gra2c_portao_da_reputacao',
     '        && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x128e',
     '        && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x800 // mutation',
     "gra_2c player's day"),
]
