# Mutacoes. Sem nenhuma clausula ativa (0x867b50): o valor do jogador 0x4d0c60, o preco pedido 0x4d2070 e o vendedor que
# tem de aceitar 0x8cd240 deixam de pesar a multa (a oferta no valor da multa deixa de ser aceita de cara); muda as
# transferencias dos jogadores com clausula (pode escapar se nenhum jogador das ligas carregadas tiver a clausula 2 em
# +0x1c/+0x1f ou 1 em +0x1d). No periodo protegido (0x868210): sem as regras novas da FIFA ([0xae233c] desligado ate a
# data das regras [0xae2330]) 0 no lugar de 1 muda o dia dos contratos 0x4c7ac0 e os lances 0x8e5d60 do comeco da
# carreira (escapa se as regras ja estiverem ligadas desde o inicio); com as regras, o periodo de 2 anos tambem para
# quem assinou antes dos 28 deixa livres os contratos de 2 a 3 anos (lances e pedidos de transferencia). Sem contador.
MUTACOES = [
    ('cpu4_nenhuma_clausula', 'return 1; // cpu4: com a clausula ativa',
     'return 0; // mutacao: nenhuma clausula ativa'),
    ('cpu4_sem_regras_livre', 'return 1; // cpu4: sem as regras novas',
     'return 0; // mutacao: sem as regras novas, livre'),
    ('cpu4_periodo_de_dois_anos', 'let limite: u8 = if (idade(pessoa, d) as u8 as i8) < 0x1c { 3 } else { 2 }; // cpu4: o periodo pela idade',
     'let limite: u8 = { idade(pessoa, d); 2 }; // mutacao: sempre 2 anos'),
]
