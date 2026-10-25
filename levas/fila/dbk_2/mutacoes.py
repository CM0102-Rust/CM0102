# Mutacoes de tirar a pessoa do clube e dos olheiros do clube; a prova e o .qst da prova do save (a criacao roda
# antes da gravacao). A de tirar deixa o clube na pessoa depois de sair (os cortes da criacao, 0x541e60/0x524d70,
# tiram milhares): aparece nas pessoas gravadas. A dos olheiros ESCAPA da prova do save: a criacao do .qst nao chama
# 0x53cd60 (so a 0x7f7270, nas listas dos olheiros, na carreira); fica para o jogo real comparando (lacuna).
MUTACOES = [
    ('dbk2_tirar_pessoa', '            core::ptr::write_unaligned((pessoa + 0x39) as *mut usize, 0); // desliga a pessoa do clube', '            // mutacao: a pessoa fica com o clube (+0x39) depois de sair'),
    ('dbk2_olheiros', '        let o = rd(p + 0x1b3 + 4 * j); // os olheiros do parceiro', '        let o = rd(clube + 0x1b3 + 4 * j); // mutacao: os olheiros do proprio clube no lugar dos do parceiro'),
]
