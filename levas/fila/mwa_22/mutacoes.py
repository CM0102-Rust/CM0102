# Mutacoes (mwa_22), na Supercopa da Irlanda, carreira ENG IRL NIR ate 2002-08-01 (a montagem diaria roda quando a
# classificacao da UEFA chega ao ano da copa; se isso nao acontecer antes de agosto, so os comparadores acusam):
# - a tabela sem os clubes da Liga dos Campeoes (outros clubes);
# - a Supercopa nunca e montada (nao e jogada);
# - a virada sobe o ano em 2.
MUTACOES = [
    ('mwa22_sem_a_liga_dos_campeoes',
     'for key in [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP] { // mwa22: the European entrants first',
     'for key in [KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP] { // mutation: no Champions League',
     'mwa_22 irish super cup table'),
    ('mwa22_nunca_montada',
     'if irish_super_cup_due(l, flag) { // mwa22: the Super Cup is set up once the seeding reaches its year',
     'if false && irish_super_cup_due(l, flag) { // mutation: never set up',
     'mwa_22 irish super cup set up'),
    ('mwa22_virada_ano_mais_dois',
     'start_irish_super_cup_season(League(this), 1) // mwa22: the Irish Super Cup turnover',
     'start_irish_super_cup_season(League(this), 2) // mutation: two years on',
     'mwa_22 irish super cup turnover'),
]
