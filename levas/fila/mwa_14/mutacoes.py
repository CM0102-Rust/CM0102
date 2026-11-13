# Mutacoes (mwa_14), na First Division da Irlanda, carreira ENG IRL NIR ate 2002-08-01:
# - a virada sobe o ano em 2 (a temporada nova comeca em agosto de 2002: se a carreira nao acusar, o comparador da
#   virada acusa);
# - a fase seguinte espera a First Division em vez da Premier (a repescagem sai em outra data ou sem o clube da Premier);
# - a repescagem procura o adversario na propria First Division em vez da Premier.
MUTACOES = [
    ('mwa14_virada_ano_mais_dois',
     'if !rebuild_season(this, 0x635910, 0x9e9ed0, [0x217, 0x21e, 0x224, 0x22b], 1) { // mwa14: the next season of the Irish First',
     'if !rebuild_season(this, 0x635910, 0x9e9ed0, [0x217, 0x21e, 0x224, 0x22b], 2) { // mutation: two years on',
     'mwa_14 irish first turnover'),
    ('mwa14_espera_a_propria_divisao',
     'advance_irish_first_stage(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, 0x248, 0x635dc0)',
     'advance_irish_first_stage(League(this), KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9e9ed0, 0x248, 0x635dc0)',
     'mwa_13 irish first division waits for the premier'),
    ('mwa14_adversario_da_propria_divisao',
     'create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, [0x274, 0x284, 0x2a5], "mwa_14 irish promotion play-off")',
     'create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9e9ed0, [0x274, 0x284, 0x2a5], "mwa_14 irish promotion play-off")',
     'mwa_14 irish promotion play-off'),
]
