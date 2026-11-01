# Mutacoes (cqb_4). As tres nos amistosos dos clubes (0x5c4770), que roda em cada plano da pre-temporada da IA (2 vezes
# na base, com dezenas de amistosos planejados): a faixa com um a mais (os adversarios achados pela 0x5c4cb0 mudam), um
# amistoso a menos por clube (a conta de sorteio(3) + 3) e o mandante trocado (o de faixa menor em casa: outra renda,
# outro deslocamento). Cada uma muda os amistosos marcados (jogos, sorteios, renda), e a carreira. Os contadores ficam
# depois dos filtros do clube e do adversario achado, colados no efeito.
MUTACOES = [
    ('cqb4_faixa_maior',
     'let mut b = (cqb4_sortear(3) as u8).wrapping_add(1); // cqb4: o acréscimo da faixa',
     'let mut b = (cqb4_sortear(3) as u8).wrapping_add(2); // mutacao: a faixa com um a mais',
     'cqb_4 clube planejado'),
    ('cqb4_um_amistoso_a_menos',
     'let quantos = (cqb4_sortear(3) as u8).wrapping_add(4) as i8; // cqb4: quantos amistosos',
     'let quantos = (cqb4_sortear(3) as u8).wrapping_add(3) as i8; // mutacao: um amistoso a menos',
     'cqb_4 clube planejado'),
    ('cqb4_mandante_trocado',
     'if cqb4_c(cand + 5) > cqb4_c(opp + 5) { // cqb4: o de faixa maior em casa',
     'if cqb4_c(cand + 5) <= cqb4_c(opp + 5) { // mutacao: o de faixa menor em casa',
     'cqb_4 adversario achado'),
]
