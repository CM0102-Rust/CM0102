# Mutacoes (sx17_2), item de TELAS: a carreira automatica nao passa por estas rotinas; a cadeia nao as roda. Servem
# para provar o comparador numa sessao do Lucas jogando comparando (pôr jogadores numa lista de busca, um olheiro
# trabalhando):
# - a capacidade cresce 200 em vez de 250 (o comparador do add_result confere o plano da realocacao: acusa);
# - o proprio elenco reconhecido pelo bit 4 em vez do 2 (o comparador lado a lado do remove_own_squad acusa a lista);
# - a nota da anotacao vira 0 (o comparador do add_player acusa o +0x2c).
MUTACOES = [
    ('sx17_2_crescimento',
     'write_i32(capacity, read_i32(capacity) + 250); // sx17_2: the growth step',
     'write_i32(capacity, read_i32(capacity) + 200); // mutation: a smaller growth step'),
    ('sx17_2_bit_do_elenco',
     'if read_u8(entry + search_result::FLAGS) & 2 != 0 { // sx17_2: the own squad flag',
     'if read_u8(entry + search_result::FLAGS) & 4 != 0 { // mutation: the wrong flag'),
    ('sx17_2_nota_zero',
     'write_u8(e + 0x2c, if note != 0 { read_u8(note + 0x113) } else { 0xff }); // sx17_2: the note rating',
     'write_u8(e + 0x2c, 0); // mutation: no note rating'),
]
