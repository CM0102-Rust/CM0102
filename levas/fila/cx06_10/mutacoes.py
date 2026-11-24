# Mutacoes (cx06_10), com as 8 ligas e ATE 2003-08-15 (os grupos das eliminatorias de set/2002 em diante):
# - o resultado entra na liga sem atualizar as estatisticas (o segundo argumento 0): as estatisticas das selecoes e dos
#   jogadores mudam;
# - as posicoes e situacoes da liga nao sao atualizadas (sem League::update_positions_and_status): as situacoes e as
#   noticias dos grupos mudam;
# - o vice de grupo fica com o melhor codigo 11 no lugar de 10: o comparador do progresso acusa.
MUTACOES = [
    ('cx06_10_sem_estatisticas',
     '    exe::league_add_result(this, fixture, update_stats)\n}',
     '    exe::league_add_result(this, fixture, 0 * update_stats) // mutation\n}',
     'cx06_10 group result kept'),
    ('cx06_10_sem_posicoes',
     '    update(this);\n    if read_u32(this + 0xfb) != 0 { // already marked',
     '    let _ = update; // mutation\n    if read_u32(this + 0xfb) != 0 { // already marked',
     'cx06_10 qualifiers positions'),
    ('cx06_10_progresso_errado',
     'const EURO_QUAL_GROUP_BEST: [u8; 6] = [1, 0xa, 0x13, 0x1c, 0x25, 0x2e];',
     'const EURO_QUAL_GROUP_BEST: [u8; 6] = [1, 0xb, 0x13, 0x1c, 0x25, 0x2e]; // mutation',
     'cx06_10 qualifiers group progress'),
]
