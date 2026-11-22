# Mutacoes (tel39_5). Acusadas pelos comparadores (lado a lado); a da comparacao de datas pode mudar a carreira
# (contratos e noticias que esperam um dia certo).
# - a segunda data do recorde de idade copiada da primeira (o fim de temporada da carreira);
# - o publico do recorde copiado mais um;
# - a comparacao de datas so pelo ano.
MUTACOES = [
    ('tel39_5_segunda_data',
     'copy_record_date(this + 0x18, source + 0x18);',
     'copy_record_date(this + 0x18, source);',
     'tel39_5 player age record copy'),
    ('tel39_5_publico',
     '// tel39_5: the attendance\n    write_u32(this, read_u32(source));',
     '// tel39_5: the attendance\n    write_u32(this, read_u32(source).wrapping_add(1));',
     'tel39_5 attendance record copy'),
    ('tel39_5_so_o_ano',
     '((day_year >> 16) as u16 == read_u16(this + 2) && day_year as u16 == read_u16(this)) as u32',
     '((day_year >> 16) as u16 == read_u16(this + 2)) as u32',
     'tel39_5 date equals'),
]
