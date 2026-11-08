# Mutacoes (cqm_6), na divisao de baixo espanhola e na tabela da Segunda (a criacao da carreira de 8 ligas).
# - a divisao de baixo sem os times B da Primera (os lugares vao para os candidatos sorteados);
# - o embaralhamento dos candidatos com uma escolha a mais (3 trocas sorteadas a mais);
# - a tabela da Segunda com os clubes na ordem inversa do banco (outra grade de jogos).
MUTACOES = [
    ('cqm6_sem_os_times_b_da_primera',
     'for g in [0x9cf624usize, 0x9cf628] { // cqm6: os times B da Primera e da Segunda',
     'for g in [0x9cf628usize] { // mutacao: sem os da Primera',
     'cqm_6 lower division fixtures'),
    ('cqm6_uma_escolha_a_mais',
     'let faltam = (cqm6_w(this + 0x3e) as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam',
     'let faltam = (cqm6_w(this + 0x3e) as u16).wrapping_sub(k as i8 as i16 as u16).wrapping_add(1) as u32; // mutacao',
     'cqm_6 lower division fixtures'),
    ('cqm6_segunda_na_ordem_inversa',
     'let c = base + (i as u16 as usize) * 0x245; // cqm6: o clube i do banco',
     'let c = base + ((n - 1 - i as i32) as usize) * 0x245; // mutacao: a ordem inversa',
     'cqm_6 second division fixtures'),
]
