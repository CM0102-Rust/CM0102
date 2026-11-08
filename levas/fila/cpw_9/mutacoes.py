# Mutacoes, todas nas lesoes no treino do dia das lesoes (0x62b7b0, todo dia de semana da carreira): cada uma muda a
# contagem de sorteios do dia (mais sorteios de uma pessoa, mais jogadores passando pelo risco ate o sorteio(3), um dia
# a mais de treino na semana), alem das lesoes que saem dali (a escalacao e o motor das partidas leem). O risco sem
# recorrencia e o caminho de quase todo jogador (+0x12 negativo).
MUTACOES = [
    ('cpw9_sorteios_por_140',
     'let mut quantos = (cpw9_i(0xae2364) / 150) as i16; // cpw9: os sorteios do treino',
     'let mut quantos = (cpw9_i(0xae2364) / 140) as i16; // mutacao: um sorteio a cada 140',
     'cpw_9 training day'),
    ('cpw9_risco_triplo',
     'let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(physical_record_by_index(this, idx) + 0x1e) as i32) * 2; // cpw9: o risco sem recorrência',
     'let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(physical_record_by_index(this, idx) + 0x1e) as i32) * 3; // mutacao: risco triplo',
     'cpw_9 risk without recurrence'),
    ('cpw9_treino_no_sabado',
     'if semana(0xae2c90) as i16 != 6 && semana(0xae2c90) as i16 != 5 { // cpw9: sábado e domingo sem treino',
     'if semana(0xae2c90) as i16 != 6 { // mutacao: treino num dos dias do fim de semana',
     'cpw_9 training day'),
]
