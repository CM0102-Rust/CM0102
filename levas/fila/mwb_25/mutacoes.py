# Mutacoes (mwb_25): o corte trocado (7 com o detentor abaixo do 9o, 8 nos outros casos: outro clube da Liga de Gales
# entra); faixas de 4 nas posicoes previstas (o comparador lado a lado pega).
MUTACOES = [
    ('mwb25_corte_trocado', '{ 8i8 } else { 7 }; // mwb25: the cut-off place', '{ 7i8 } else { 8 }; // mutation: the cut-off swapped', 'mwb_25 welsh premier cup entrants'),
    ('mwb25_faixas_de_4', 'italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 2, ', 'italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 4, ', 'mwb_25 welsh premier cup predicted'),
]
