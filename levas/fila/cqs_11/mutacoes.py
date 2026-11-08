# Mutacoes (cqs_11), nos convidados da Merconorte de 2001 (a liga americana jogavel): a band sorteada do Mexico com 1
# (so os 3 de cima embaralhados, em vez de 12); o vice da MLS na vaga do campeao; a lista final embaralhada so nas 15
# primeiras. Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqs11_faixa_do_mexico_menor',
     'ordenar(mexico.clubes.as_mut_ptr() as usize, mexico.n as u32, 4); // cqs11: os 4 do México',
     'ordenar(mexico.clubes.as_mut_ptr() as usize, mexico.n as u32, 1); // mutacao: a band de 1',
     'cqs_11 merconorte guests'),
    ('cqs11_vice_da_mls_no_lugar_do_campeao',
     'vaga(0, colocado(registro, 0)); // cqs11: o campeão da liga americana',
     'vaga(0, colocado(registro, 1)); // mutacao: o vice',
     'cqs_11 american league champion'),
    ('cqs11_embaralha_15',
     'embaralhar(cqs11_d(this + 0xee), 0x10);',
     'embaralhar(cqs11_d(this + 0xee), 0xf); // mutacao: 15',
     'cqs_11 merconorte guests'),
]
