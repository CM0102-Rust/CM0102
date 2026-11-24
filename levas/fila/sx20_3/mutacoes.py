# Mutacoes (sx20_3). As tres rodam na carreira ENG: o nome na iniciacao dos jogadores do banco (criacao do jogo) e no
# motor de partida; a ordem das vagas a cada divisao do orcamento de transferencias; a ordem dos pares nas buscas do
# gerente de listas curtas:
# - o resultado invertido (todo outro nome casa: a formacao pelo nome muda);
# - as vagas abertas pelo menor preco primeiro (o orcamento vai para outras vagas);
# - o primeiro id -1 deixa de casar (as buscas por segundo id falham).
MUTACOES = [
    ('sx20_3_nome_invertido',
     'guio_same_c_string(name.as_ptr() as usize, text) as u32 // sx20_3: the names match',
     '!guio_same_c_string(name.as_ptr() as usize, text) as u32 // mutation: every other name matches',
     'sx20_3 staff name compared'),
    ('sx20_3_vagas_menor_preco',
     'let by_price = read_i32(b + shortlist_slot::PRICE).wrapping_sub(read_i32(a + shortlist_slot::PRICE)); // sx20_3: highest price first',
     'let by_price = read_i32(a + shortlist_slot::PRICE).wrapping_sub(read_i32(b + shortlist_slot::PRICE)); // mutation: lowest first',
     'sx20_3 open slots by price'),
    ('sx20_3_menos_um_nao_casa',
     'if read_i32(a) == -1 || read_i32(b) == -1 { // sx20_3: -1 matches any first id',
     'if false { // mutation: -1 does not match',
     'sx20_3 same second id'),
]
