# Mutacoes (sx21_5). Rodam na carreira ENG.
# - quatro de fora da UE em vez de tres: escalacoes inglesas mudam (comparador e carreira);
# - o inscrito por outro clube joga (0 em vez de 1): comparador e carreira (copas inglesas);
# - a data de janela sem andar para o dia da semana (0xffff): as datas inglesas (2 de junho para a segunda, 26 de
#   marco para a quinta) mudam (comparador; carreira pela janela). Nao mexe no dia nem no mes (dia 32 ou mes 12 abrem
#   a caixa do Date.cpp).
MUTACOES = [
    ('sx21_5_quatro_fora_ue',
     'fill_squad_restrictions_with(out, &[(5, 3)]); // sx21_5: three non-EU players',
     'fill_squad_restrictions_with(out, &[(5, 4)]); // mutation: four',
     'sx21_5 england squad limits'),
    ('sx21_5_inscrito_joga',
     'return 1; // sx21_5: cup-tied',
     'return 0; // mutation: cup-tied may play',
     'sx21_5 england cup-tied'),
    ('sx21_5_sem_dia_da_semana',
     'year, read_u8(this + 2) as i8 as i16 as u16 as u32); // sx21_5: the day and the month',
     'year, 0xffff); // mutation: the date does not move to its weekday',
     'sx21_5 window date'),
]
