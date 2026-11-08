# Mutacoes (cpq_13). Limpar a vaga seguinte em vez da pedida (a escalacao da selecao na NationalTeams::select_squad
# 0x76d261/0x76d403 perde outro jogador e guarda o que devia sair; o comparando acusa pela linha da escalacao, a
# carreira pelos jogos da selecao inglesa). O batedor tirado continua na celula (a 0x896c60, que a escalacao do
# computador chama, tira o batedor que deixou de jogar: ele seguiria batendo; PODE ESCAPAR se o caso nao ocorrer ate
# 2002-06-01: o ramo diz).
MUTACOES = [
    ('cpq_limpar_outra_vaga', '    core::ptr::write_unaligned(tactics_cell(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga',
     '    core::ptr::write_unaligned(tactics_cell(this, k, 2 + 4 * (v as usize + 1)) as *mut i32, -1); // mutacao: a vaga seguinte',
     'cpq_13 clear slot'),
    ('cpq_batedor_fica', '    core::ptr::write_unaligned(celula(e), -1); // sem batedor',
     '    // mutacao: o batedor tirado fica na celula',
     'cpq_13 taker cleared'),
]
