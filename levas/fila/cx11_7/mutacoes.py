# Mutacoes (cx11_7), com a Italia e a base NORMAL ate 2002-09-30: a repescagem e o play-out sao criados uma vez em
# cada grupo da C2 (o auxiliar serve aos dois), no fim da liga, e jogados em maio e junho de 2002.
# - as posicoes da repescagem (que o construtor da fase usa para o mando) trocadas: outro clube joga em casa (a mesma
#   mutacao do cql_5 na C1, acusada pela carreira);
# - o play-out com o 13o no lugar do 14o (outro clube joga o play-out e pode cair).
MUTACOES = [
    ('cx11_7_posicoes_trocadas',
     'let positions: [u8; 4] = [4, 1, 3, 2]; // cx11_7: the play-off positions',
     'let positions: [u8; 4] = [1, 4, 2, 3]; // mutation: the positions swapped',
     'cx11_7 serie c2 play-off and play-out'),
    ('cx11_7_play_out_com_o_decimo_terceiro',
     '    for (k, &i) in PLAYOUT_ROWS.iter().enumerate() {\n        write_usize(list + k * 4, ita_ser_c1a_league_row_club(this, i));\n    }\n    let positions: [u8; 4] = [0x10, 0xd, 0xf, 0xe]; // cx11_7: the play-out positions',
     '    for (k, &i) in PLAYOUT_ROWS.iter().enumerate() {\n        write_usize(list + k * 4, ita_ser_c1a_league_row_club(this, if i == 13 { 12 } else { i })); // mutation: the 13th\n    }\n    let positions: [u8; 4] = [0x10, 0xd, 0xf, 0xe]; // cx11_7: the play-out positions',
     'cx11_7 serie c2 play-off and play-out'),
]
