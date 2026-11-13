# Mutacoes (mwa_13), na First Division da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - a virada sobe o ano em 2 (a temporada nova comeca em agosto de 2002: se a carreira nao acusar, o comparador da
#   virada acusa);
# - a repescagem nao espera a Premier terminar (monta antes, com o clube errado da Premier ou sem ele);
# - a repescagem procura o adversario na propria First Division em vez da Premier.
MUTACOES = [
    ('mwa13_virada_ano_mais_dois',
     'if !rebuild_season(this, 0x78bc50, 0xa6965c, [0x1f5, 0x1fc, 0x202, 0x209], 1) { // mwa13: the next season of the Northern Irish First',
     'if !rebuild_season(this, 0x78bc50, 0xa6965c, [0x1f5, 0x1fc, 0x202, 0x209], 2) { // mutation: two years on',
     'mwa_13 northern irish first turnover'),
    ('mwa13_sem_esperar_a_premier',
     'if complete(p, 0, 1) as u8 == 0 {',
     'if false && complete(p, 0, 1) as u8 == 0 { // mutation: does not wait',
     'mwa_13 irish first division waits for the premier'),
    ('mwa13_adversario_da_propria_divisao',
     'create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, [0x252, 0x262, 0x283], "mwa_13 northern irish promotion play-off")',
     'create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, 0xa6965c, [0x252, 0x262, 0x283], "mwa_13 northern irish promotion play-off")',
     'mwa_13 northern irish promotion play-off'),
]
