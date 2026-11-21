# Mutations (gra_21): the tours run in July 2001 (the two pre-season planning days); other tours change the friendlies
# played, the gate receipts and every draw from then on, which the ENG career to 2002-06-01 catches. The tournament half
# only runs from the pre-season of 2002 (after the chain's date): no mutation there.
MUTACOES = [
    ('gra21_menos_amistosos',
     'let how_many = (friendly_random(2) as u8).wrapping_add(3) as i8; // gra21: up to 3 or 4 friendlies',
     'let how_many = (friendly_random(2) as u8).wrapping_add(2) as i8; // mutation: one friendly fewer',
     'gra_21 tour planned'),
    ('gra21_outro_sorteio_de_quem_viaja',
     'if read_i8(cand + 4) > 4 || friendly_random(3) == 0 { // gra21: one in three stays home',
     'if read_i8(cand + 4) > 4 || friendly_random(3) == 1 { // mutation: another third stays home',
     'gra_21 tour planned'),
    ('gra21_excursao_so_com_tres',
     'if m > 1 { // gra21: a tour needs two opponents',
     'if m > 2 { // mutation: a tour needs three opponents',
     'gra_21 tour scheduled'),
]
