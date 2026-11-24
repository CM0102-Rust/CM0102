# Mutações (sx11_2).
# - a ordem das habilidades decrescente: as tabelas do dia saem invertidas e o desenvolvimento dos jogadores muda (o
#   comparador acusa em cada chamada do qsort; a carreira nos dias de desenvolvimento);
# - a data estática do jogo com outro ano: só o comparador acusa (na partida do programa; a data de verdade vem da
#   criação do jogo, então a carreira não muda).
MUTACOES = [
    ('sx11_2_ordem_invertida',
     '    read_i16(a as usize) as i32 - read_i16(b as usize) as i32\n}',
     '    read_i16(b as usize) as i32 - read_i16(a as usize) as i32 // mutation: descending\n}'),
    ('sx11_2_data_estatica',
     '    exe::date_date_54a110(date);\n}',
     '    exe::date_date_54a110(date);\n    write_u16(date + 2, 0x7b5); // mutation: 1973\n}'),
]
