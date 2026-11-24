# Mutacoes (sx12_6). Rodam na criacao do jogo:
# - a velocidade da narracao 0x7e: o comparador do registro (lado a lado sobre os 0xa6ef bytes) acusa nas 17 chamadas;
# - a temporada de cada nacao comecando na data do fim: o comparador da criacao usa o mesmo auxiliar e nao acusa; a
#   carreira sim, pela forma de pre-temporada e pelos dias de folga (key_nation_club_in_season na
#   atualizacao da forma de cada jogador, 0x62c680) e pelo ano da temporada.
MUTACOES = [
    ('sx12_narracao_padrao',
     'write_u16(record + 0x9ad2, 0x7d); // sx12_6: the commentary speed, Slow',
     'write_u16(record + 0x9ad2, 0x7e); // mutation: another commentary speed',
     'sx12_6 reset human record'),
    ('sx12_temporada_comeca_no_fim',
     'for (date, parts) in [(key_nation_record::START_DATE, 0x32), (key_nation_record::END_DATE, 0x41)] {',
     'for (date, parts) in [(key_nation_record::START_DATE, 0x41), (key_nation_record::END_DATE, 0x41)] { // mutation: the season starts at its end',
     'sx12_6 key nation competitions'),
]
