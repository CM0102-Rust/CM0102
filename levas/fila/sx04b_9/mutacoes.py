# Mutacoes (sx04b_9), para o comparador numa sessao do Lucas (a carreira nao roda tela nem desfaz o jogo):
# - a ordem da equipe da semana invertida (compare_order! acusa);
# Os destrutores ficam sem mutacao: o que escrevem (vtables, blocos liberados) nao e lido depois.
MUTACOES = [
    ('sx04b_9_ordem_invertida',
     'read_u8(b as usize + 8) as i32 - read_u8(a as usize + 8) as i32 // sx04b_9: the higher rating first',
     'read_u8(a as usize + 8) as i32 - read_u8(b as usize + 8) as i32 // mutation: the lower first',
     'sx04b_9 team of the week ordered'),
]
