# Mutacoes (cqw_13), nas copas asiaticas de clubes montadas no comeco do jogo: cada uma um ano depois (as datas das
# rodadas saem da janela; os clubes japoneses delas nao jogam).
MUTACOES = [
    ('cqw13_liga_um_ano_depois',
     'let year = year as i16; // cqw13: the Asian Club Championship of the game\'s year',
     'let year = (year as i16).wrapping_add(1); // mutation: one year later',
     'cqw_13 asian club championship'),
    ('cqw13_recopa_um_ano_depois',
     'let year = year as i16; // cqw13: the Asian Cup Winners\' Cup of the game\'s year',
     'let year = (year as i16).wrapping_add(1); // mutation: one year later',
     'cqw_13 asian cup winners cup'),
]
