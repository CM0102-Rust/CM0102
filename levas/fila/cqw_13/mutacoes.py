# Mutacoes (cqw_13), nas copas asiaticas de clubes montadas no comeco do jogo: cada uma um ano depois (as datas das
# rodadas saem da janela; os clubes japoneses delas nao jogam).
MUTACOES = [
    ('cqw13_liga_um_ano_depois',
     'let ano = ano as i16; // cqw13: a Liga dos Campeões asiática do ano do jogo',
     'let ano = (ano as i16).wrapping_add(1); // mutacao: um ano depois',
     'cqw_13 liga dos campeoes asiatica'),
    ('cqw13_recopa_um_ano_depois',
     'let ano = ano as i16; // cqw13: a Recopa asiática do ano do jogo',
     'let ano = (ano as i16).wrapping_add(1); // mutacao: um ano depois',
     'cqw_13 recopa asiatica'),
]
