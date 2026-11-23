# Mutações da 325a leva (itens gra_21, gra_23). Uso: python mutacoes.py mutacoes\leva325.py --carreira 2002-06-01
MUTACOES = [
    ('gra21_menos_amistosos', 'let how_many = (friendly_random(2) as u8).wrapping_add(3) as i8; // gra21: up to 3 or 4 friendlies', 'let how_many = (friendly_random(2) as u8).wrapping_add(2) as i8; // mutation: one friendly fewer', 'gra_21 tour planned'),
    ('gra21_outro_sorteio_de_quem_viaja', 'if read_i8(cand + 4) > 4 || friendly_random(3) == 0 { // gra21: one in three stays home', 'if read_i8(cand + 4) > 4 || friendly_random(3) == 1 { // mutation: another third stays home', 'gra_21 tour planned'),
    ('gra21_excursao_so_com_tres', 'if m > 1 { // gra21: a tour needs two opponents', 'if m > 2 { // mutation: a tour needs three opponents', 'gra_21 tour scheduled'),
    ('gra23_outra_formacao_da_ia', '    (0xa8b8fc, 0x72), // 442_v1.pct', '    (0xa8b8f0, 0x72), // mutation: 532_v1.pct in the slot of 442_v1.pct', 'gra_23 tactics from the files'),
    ('gra23_outra_predefinida', '    (0xa8b784, 0xa782b0, 0x9d), // 442_default.pct, "4-4-2"', '    (0xa8b694, 0xa782b0, 0x9d), // mutation: 433_default.pct under the name "4-4-2"', 'gra_23 tactics from the files'),
]
