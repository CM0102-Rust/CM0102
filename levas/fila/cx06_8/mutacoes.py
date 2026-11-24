# Mutacoes (cx06_8), com as 8 ligas e ATE 2003-08-15 (a Copa das Confederacoes de 2003):
# - as semifinais so na fase 2: a copa para depois dos grupos (sem os jogos de 06 a 09/08/2003);
# - quem passa do grupo ganha o registro de eliminado (que encerra as convocacoes) e quem cai o de classificado: as
#   convocacoes dos clubes mudam [duvida: se a carreira acusa antes de 2003-08-15];
# - o segundo do grupo fica com o codigo atual 4 no lugar de 3: o comparador do progresso acusa.
MUTACOES = [
    ('cx06_8_semifinal_atrasada',
     '    if stage == 1 {\n        covered!("cx06_8 confed knockout stage");',
     '    if stage == 2 { // mutation\n        covered!("cx06_8 confed knockout stage");',
     'cx06_8 confed knockout stage'),
    ('cx06_8_registros_trocados',
     '            6 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x19),\n            0xfe => european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19),',
     '            0xfe => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x19), // mutation\n            6 => european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19),',
     'cx06_8 confed group records'),
    ('cx06_8_progresso_errado',
     '[[1, 3, 5, 7], [1, 3, 5, 7], [2, 3, 5, 7]]',
     '[[1, 4, 5, 7], [1, 3, 5, 7], [2, 3, 5, 7]] /* mutation */',
     'cx06_8 confed club progress'),
]
