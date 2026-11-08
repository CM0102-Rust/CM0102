# Mutacoes. O clube com oferta por qualquer outro clube (o comparando acusa na primeira chamada com oferta; a carreira,
# porque quem ja tem oferta deixa de fazer outra e as negociacoes mudam). A oferta do clube sendo a primeira de qualquer
# clube (~30 chamadas: PODE ESCAPAR). A pendente sem a situacao 1 (a carreira: o jogador com oferta pendente nao entra
# em outras transferencias).
MUTACOES = [
    ('cpl_clube_tem_oferta', 'if cpl1_d(offer_at(this, cpl1_d(no)) + 8) == cpl1_d(clube) { // a oferta é do clube',
     'if cpl1_d(offer_at(this, cpl1_d(no)) + 8) != cpl1_d(clube) { // mutacao: de outro clube', 'cpl_1 club has offer'),
    ('cpl_oferta_do_clube', 'if cpl1_d(o + 8) == cpl1_d(clube) { // a oferta do clube',
     'if o != 0 { // mutacao: a primeira de qualquer clube', 'cpl_1 club offer'),
    ('cpl_oferta_pendente', 'if matches!(*((o + 0x2c) as *const u8), 0 | 1 | 5 | 6) { // a situação pendente',
     'if matches!(*((o + 0x2c) as *const u8), 0 | 5 | 6) { // mutacao: sem a 1', 'cpl_1 pending offer'),
]
