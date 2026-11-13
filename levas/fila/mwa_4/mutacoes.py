# Mutacoes (mwa_4), na Copa do Presidente (Irlanda), carreira ENG IRL NIR ate 2002-08-01 (a edicao de 2001 e jogada
# em 4/8/2001):
# - o jogo uma semana depois (4/8 -> 11/8);
# - os dois clubes mais bem colocados fora de Leinster (outros clubes jogam);
# - a virada sobe o ano em 2 (a edicao de 2002 e depois da data final: so o comparador da virada acusa).
MUTACOES = [
    ('mwa4_final_semana_depois',
     'cql10r((5, 6, 0, 5), (4, 7, 0, 4, 2, 0x400), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa4: the President\'s Cup final',
     'cql10r((5, 6, 0, 5), (0xb, 7, 0, 4, 2, 0x400), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutation: a week later',
     'mwa_4 presidents cup rounds'),
    ('mwa4_clubes_fora_de_leinster',
     'if c != 0 && ire_club_is_in_leinster(Club(c)) {',
     'if c != 0 && !ire_club_is_in_leinster(Club(c)) { // mutation',
     'mwa_4 presidents cup clubs'),
    ('mwa4_virada_ano_mais_dois',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x639670, 0x9ea000, [0x14d, 0x154, 0x15b], 1, false, "mwa_4 presidents cup turnover")',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x639670, 0x9ea000, [0x14d, 0x154, 0x15b], 2, false, "mwa_4 presidents cup turnover")',
     'mwa_4 presidents cup turnover'),
]
