# Mutacoes (com o ramo de cada uma). As tres mexem na confianca e nos humores da diretoria (+6/+0x14/+0x16) ou nas
# pessoas favoritas do presidente, que o jogo le depois (demissoes, propostas). O comprador e o vendedor com a taxa
# dentro do limit (o caso comum das transferencias da maquina) ganham outra marca: a confianca muda em toda
# transferencia. O jogador fica nas favoritas do presidente do vendedor. Se a tabela da diretoria nao entrar na
# impressao da carreira e nenhuma demissao mudar ate 2002-06-01, as duas primeiras PODEM ESCAPAR (comparando, acusa).
MUTACOES = [
    ('cpo_comprador_no_limite', 'or_byte(f1, 4); // o comprador: taxa dentro do limit',
     'or_byte(f1, 8); // mutacao: o comprador ganha a marca da barganha',
     'cpo_1 comprador no limit'),
    ('cpo_vendedor_no_limite', 'or_byte(f2, 4); // o vendedor: taxa dentro do limit',
     'or_byte(f2, 2); // mutacao: o vendedor perde confianca',
     'cpo_1 vendedor no limit'),
    ('cpo_tirar_favorito', 'tirar(cpo1_d(cpo1_d(pessoa + 0x39) + 0xbf), pessoa); // o presidente do vendedor',
     'let _ = tirar; // mutacao: o jogador fica nas favoritas do presidente',
     'cpo_1 tirar favorito'),
]
