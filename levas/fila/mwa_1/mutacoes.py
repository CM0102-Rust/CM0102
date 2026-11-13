# Mutacoes (mwa_1), na Copa da Irlanda (FAI Senior Challenge Cup), carreira ENG IRL NIR ate 2002-08-01:
# - a primeira rodada jogada uma semana depois (28/11 -> 5/12): as datas e os adversarios mudam;
# - Munster e Leinster trocam de vagas no sorteio dos regionais (os confrontos da primeira rodada mudam);
# - a virada sobe o ano em 2 (a edicao nova sai da janela).
MUTACOES = [
    ('mwa1_primeira_rodada_semana_depois',
     'cql10r((1, 0xa, 0, 0), (0x1c, 0xa, 0, 6, 1, 0), 0xa, (0, 0, 3), 4, (0x14, 0xa, 0x14, 0), (1, 1, 0), (0, 0xffffffff, 0)), // mwa1: the first round of the Irish Challenge Cup',
     'cql10r((1, 0xa, 0, 0), (5, 0xb, 0, 6, 1, 0), 0xa, (0, 0, 3), 4, (0x14, 0xa, 0x14, 0), (1, 1, 0), (0, 0xffffffff, 0)), // mutation: a week later',
     'mwa_1 irish challenge cup rounds'),
    ('mwa1_regionais_trocados',
     'let order = [KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE];',
     'let order = [KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE]; // mutation',
     'mwa_1 irish challenge cup entrants'),
    ('mwa1_virada_ano_mais_dois',
     'start_new_cup_season(Cup(this), 0x6347d0, 0x9e9e94, [0x1c4, 0x1cb, 0x1d2], 1, "mwa_1 irish challenge cup turnover")',
     'start_new_cup_season(Cup(this), 0x6347d0, 0x9e9e94, [0x1c4, 0x1cb, 0x1d2], 2, "mwa_1 irish challenge cup turnover")',
     'mwa_1 irish challenge cup turnover'),
]
