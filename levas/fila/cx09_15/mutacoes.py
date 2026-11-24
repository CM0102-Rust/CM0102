# Mutacoes (cx09_15) pela carreira da Inglaterra ate 2002-08-01 (a virada inglesa roda uma vez, no meio de 2002):
# - a cadeia nao troca a First e a Second: quem sobe e quem cai entre elas fica (as tabelas da temporada nova mudam);
# - a virada nao vira a Second Division: ela fica sem tabela nem calendario novos.
# O ramo da Conference (o campeao dela e o estadio) nao roda na carreira (a Conference nao e carregada): sem mutacao.
MUTACOES = [
    ('cx09_15_sem_first_second',
     '    for pair in ladder.windows(2) {\n        exe::league_promote_and_relegate(this, comp_by_global(pair[0]), comp_by_global(pair[1]), 1, arg, -1, -1);',
     '    for pair in ladder.windows(2).filter(|p| p[0] != KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) { // mutation\n        exe::league_promote_and_relegate(this, comp_by_global(pair[0]), comp_by_global(pair[1]), 1, arg, -1, -1);'),
    ('cx09_15_second_sem_virada',
     '    for key in [KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE] {',
     '    for key in [KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE] { // mutation'),
]
