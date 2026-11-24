# Mutacoes (sbE_2), com a Finlandia e Portugal ate 2002-08-01:
# - os grupos da Ykkonen nao reunidos (a Ykkonen nasce sem clubes: a tabela da Ykkonen e a carreira mudam; o comparador
#   do construtor tambem acusa);
# - os rebaixados da Liga de Honra para a zona Sul em vez da Norte (acusada pelo comparador; e a carreira, em julho de
#   2002, com os grupos da zona).
MUTACOES = [
    ('sbE_2_grupos_nao_reunidos',
     'move_group_clubs_to_division(&[KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH], KEY_CLUB_COMP_FINNISH_FIRST_DIVISION); // sbE_2: the Ykkonen groups gathered',
     'move_group_clubs_to_division(&[KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH], KEY_CLUB_COMP_FINNISH_FIRST_DIVISION); // mutation: the South group left out',
     'sbE_2 new veikkausliiga'),
    ('sbE_2_rebaixados_para_o_sul',
     'comp_record_of(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH), 1); // sbE_2: down to the North zone',
     'comp_record_of(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH), 1); // mutation: down to the South zone',
     'sbE_2 portuguese zones up'),
]
