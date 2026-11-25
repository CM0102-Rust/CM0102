# Mutacao pela prova do save (CARGA):
# - rx08_3_carga_perdida: a carga responde 0 no fim: o construtor (construct_player_stats) joga fora o que foi lido e
#   monta as estatisticas de um jogo novo: o jogo carregado fica sem as estatisticas da temporada (acusada pela
#   impressao da carga; senao pela ficha de qualquer jogador do jogo carregado, as partidas da temporada zeradas).
# O destrutor so libera: sem mutacao propria (provado pela carga do save de ferias, que destroi o objeto antes).
MUTACOES = [
    ('rx08_3_carga_perdida',
     '    1 // rx08_3: the statistics loaded',
     '    0 // rx08_3: mutation, the statistics lost',
     'rx08_3 load player stats'),
]
