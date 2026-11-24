# Mutacoes (cx09_19) pela carreira com a Croacia ate 2002-08-01 (a virada croata roda em julho de 2002):
# - a cadeia nao troca a Primeira e o grupo sul: quem sobe e quem cai entre eles fica;
# - os 16 mais ao norte vao ao grupo sul (e os outros ao norte): os grupos e os calendarios novos mudam.
MUTACOES = [
    ('cx09_19_sem_troca_sul',
     '    for key in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {\n        exe::league_promote_and_relegate(',
     '    for key in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH] { // mutation\n        exe::league_promote_and_relegate('),
    ('cx09_19_grupos_trocados',
     '        let group = if k < 16 { north } else { south };',
     '        let group = if k < 16 { south } else { north }; // mutation'),
]
