# Mutacoes (mwf_21): a Supercopa da Coreia uma semana depois (17/3 em vez de 10/3; o comparador lado a lado pega); o
# campeao da copa no lugar do da K League (outro clube na Supercopa).
MUTACOES = [
    ('mwf21_jogo_depois',
     'cql12r((1, 1, 0, 5), (0xa, 2, 0, 6, 1, 0), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwf21: the Korean Super Cup match',
     'cql12r((1, 1, 0, 5), (0x11, 2, 0, 6, 1, 0), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutation: 17 March',
     'mwf_21 korean super cup rounds'),
    ('mwf21_sem_campeao_da_liga',
     'select_league_and_cup_winners(this, KEY_CLUB_COMP_KOREAN_LEAGUE, KEY_CLUB_COMP_KOREAN_FA_CUP, KEY_CLUB_COMP_KOREAN_FA_CUP, KOR_SUPER_CUP_FILE, [0xc1, 0xef]) // mwf21: champion and cup winner',
     'select_league_and_cup_winners(this, KEY_CLUB_COMP_KOREAN_LEAGUE_CUP, KEY_CLUB_COMP_KOREAN_FA_CUP, KEY_CLUB_COMP_KOREAN_FA_CUP, KOR_SUPER_CUP_FILE, [0xc1, 0xef]) // mutation: the League Cup winner',
     'mwf_21 korean super cup clubs'),
]
