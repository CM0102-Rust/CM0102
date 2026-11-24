# Mutacoes (sx21b_10). Os caminhos rodam desde a partida do jogo (dados, preferencias, arquivos temporarios da
# carreira); caem pelos comparadores (a pasta devolvida e o buffer do caminho).
# - a pasta padrao de CM3_PREFS vira "data": a primeira leitura das preferencias ja diverge no comparador.
MUTACOES = [
    ('sx21b_10_prefs_em_data',
     '    (0x9cd36c, 0xad9c64), // CM3_PREFS: ""',
     '    (0x9cd36c, 0x9c3824), // mutation: CM3_PREFS in "data"',
     'sx21b_10 default folder'),
]
