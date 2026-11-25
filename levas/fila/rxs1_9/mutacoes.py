# Mutações (rxs1_9), nos jogadores falsos: os jogos pela seleção que sobram; as datas de nascimento que ficam; as
# habilidades trocadas sem a potencial. Só rodam na criação com "Use Real Players: No": a cadeia normal não as acusa
# (explicado no PRONTO.txt); numa carreira com jogadores falsos mudam a base inteira.
MUTACOES = [
    ('rxs1_9_jogos',
     'caps = caps.wrapping_sub(random(caps as i32) as u8); // rxs1_9: fewer caps',
     'caps = caps.wrapping_sub(random(caps as i32 + 1) as u8); // mutation',
     'rxs1_9 fake player keeps fewer caps'),
    ('rxs1_9_nascimento',
     'let born = read_u64(a.addr() + staff::DATE_OF_BIRTH); // rxs1_9: the dates of birth stay',
     'let born = read_u64(b.addr() + staff::DATE_OF_BIRTH); // mutation',
     'rxs1_9 player records swapped'),
    ('rxs1_9_habilidades',
     'let (current, potential) = (pa.current_ability(), pa.potential_ability()); // rxs1_9: the abilities swapped',
     'let (current, potential) = (pa.current_ability(), pb.potential_ability()); // mutation',
     'rxs1_9 squad mates swap abilities'),
]
