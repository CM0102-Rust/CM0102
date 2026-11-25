# Mutacoes para o comparador (TELAS: a cadeia nao roda; a prova e o Lucas jogando comparando):
# - rx08_8_notas_da_nova_para_a_velha: as notas na ordem inversa: o comparador das ultimas notas acusa (ficha/elenco).
# - rx08_8_titulos_trocados: o titulo do campo 1 vira o do 2 ("Gls" no lugar de "Apps"): o comparador acusa.
MUTACOES = [
    ('rx08_8_notas_da_nova_para_a_velha',
     'for rating in [a & 0xf, a >> 4, b & 0xf, b >> 4, c & 0xf] { // rx08_8: oldest first',
     'for rating in [c & 0xf, b >> 4, b & 0xf, a >> 4, a & 0xf] { // rx08_8 mutation: newest first',
     'rx08_8 recent ratings'),
    ('rx08_8_titulos_trocados',
     'player_stats_write_label(&PLAYER_STATS_FIELD_LABELS, field, buf, size, 0x5bc, 0x63f) // rx08_8: the field headings',
     'player_stats_write_label(&PLAYER_STATS_FIELD_LABELS[1..], field, buf, size, 0x5bc, 0x63f) // rx08_8 mutation',
     'rx08_8 field label'),
]
