# Mutacoes (com o ramo de cada uma; a nota e comparada bit a bit no comparando, e a carreira muda o jogador escolhido
# para as historias da midia). Com menos de 12 partidas, a nota escalada por s1/6 em vez de s1/12 (o comeco da
# temporada). A nota negativa tackle pela razao das reputacoes em vez de multiplicada. O peso da historia por 1/30 em
# vez de 1/60 (todas as chamadas).
MUTACOES = [
    ('cpn_poucas_partidas', 'r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // menos de 12 partidas',
     'r = r * s1f as f64 * (1.0f32 / 6.0) as f64; // mutacao: s1/6',
     'cpn_3 poucas partidas'),
    ('cpn_nota_negativa', 'r = r * cpn3_razao_das_reputacoes(jogador, clube); // a negativa pela reputação',
     'r = r / cpn3_razao_das_reputacoes(jogador, clube); // mutacao: tackle',
     'cpn_3 nota negativa'),
    ('cpn_peso_da_historia', 'peso as f64 * (1.0f32 / 60.0) as f64; // o peso da história',
     'peso as f64 * (1.0f32 / 30.0) as f64; // mutacao: 1/30',
     'cpn_3 nota do jogador'),
]
