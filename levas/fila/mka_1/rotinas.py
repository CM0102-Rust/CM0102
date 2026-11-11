# O lado do dinheiro do mercado (leitor A do mapa do mercado, 11/11/2026; levas/reorganizacao/mercado/secao_a.md):
# a verba de transferências do clube (Finance::get_transfer_budget 0x598800, fin_prm.cpp; 37 chamadores: os lances da
# IA 0x8e5d60 e vizinhos, as listas de observação, a diretoria, os eventos financeiros, as telas) e o preço pedido pelo
# jogador de um clube humano (contract_manager_get_human_asking_price 0x4d42d0, contract_manager.cpp; 9 chamadores,
# entre eles o lance da IA 0x8e64df). Iguais no cm0102-1080.exe. Sem SEH.
# Convenções conferidas nos pontos de chamada:
# - 0x598800: push 0 x4; push alvo; mov ecx, finanças -> thiscall, ret 0x14, devolve i64 (edx:eax); 5 argumentos:
#   jogador, somar os listados, *ajustada, conhecimento do humano, descontar o que está pendente;
# - 0x4d42d0: push ebp (jogador); push comprador; push ebx (vendedor); mov ecx, [0xae1050] -> thiscall, ret 0xc
#   (0x8e64d6..0x8e64df). O mapa da comunidade chama o 1º argumento de buying_club: é o vendedor (0x8e64c3 testa se
#   ebx é humano antes de chamar);
# - 0x8ccb80 e 0x4ca300 (as listas): thiscall(objeto, clube, *saída), ret 8, devolvem a contagem em al;
# - 0x4d1600 (o preço calculado): thiscall, ret 0x18 (vendedor, comprador, jogador, 0, 1, 1);
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): show_error_at.
# Comparadores: lado a lado pelo estado do sorteio (a cópia sem contar), com [0xae1080] e o *ajustada; o preço do
# clube que não é humano (caixa de erro) só na original.
SAVE = False
ROTINAS = [('get_transfer_budget', 0x598800, [0x83, 0xec, 0x38, 0x8b, 0x41, 0x08]),
           ('get_human_asking_price', 0x4d42d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o lado do dinheiro do mercado: a verba de transferências do clube e o preço pedido pelo jogador de um clube humano',
        'the money side of the market: the club\'s transfer budget and the asking price of a human club\'s player',
        [('get_transfer_budget', 'Finance::get_transfer_budget', '0x598800', 'fin_prm.cpp', 'thiscall, ret 0x14',
          'o saldo (mais o valor dos listados do elenco, se pedido), menos o que está pendente no clube humano (compras, vendas de estágio 3 a 6, ofertas de contrato), menos a reserva da diretoria: m × (folha + salários pendentes − ⅔ das parcelas), com m = Negócios do presidente + 6 (menos a Ambição acima de 15; 16 sem presidente; +5 na Alemanha), o estádio, o treino e a reputação; mais o público, a liga e a competição continental; menos o comprometido (taxas e o gasto líquido da temporada acima de 8 milhões); piso de 10% do saldo (até 2,5 milhões), teto suave por reputação² (10% do excesso), as verbas da diretoria, os limites por reputação (5000 e 3500) e por nação; no clube humano com máscara de atributos, o conhecimento do jogador limita a verba (defeito: com saldo negativo e jogador com ficha, a verba cai a um oitavo ou a 0)',
          'the balance (plus the value of the listed squad players, if asked), minus what is pending at a human club (purchases, stage 3 to 6 sales, contract offers), minus the board\'s reserve: m × (wage bill + pending wages − ⅔ of the instalments), with m = the chairman\'s Business + 6 (minus Ambition above 15; 16 without a chairman; +5 in Germany), the ground, training and reputation; plus the crowd, the league and the continental competition; minus the committed money (fees and the season\'s net spending above 8 million); a floor of 10% of the balance (up to 2.5 million), a soft cap by reputation² (10% of the excess), the board\'s funds, the reputation limits (5000 and 3500) and the nation\'s; at a human club with attribute masking, the knowledge of the player limits it (defect: in debt and with a player sheet, it falls to an eighth or to 0)'),
         ('get_human_asking_price', 'contract_manager_get_human_asking_price', '0x4d42d0', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'o valor guardado do jogador (até a multa rescisória ativa) é o piso; o preço calculado (0x4d1600) sobe até o piso e para na multa; listado ou com pedido de saída, ou sem o bit 4 do estado de transferência: o piso; pelo papel no elenco: 0x10 e 0x50 o preço calculado, 0x20 e 0x30 a média, os outros o piso; contrato que não é de jogador ou de saída pela lei Bosman: 0; clube que não é humano: a caixa de erro e o valor',
          'the player\'s stored value (at most the active release fee) is the floor; the computed price (0x4d1600) is raised to the floor and stops at the fee; listed or with a request, or without bit 4 of the transfer status: the floor; by squad status: 0x10 and 0x50 the computed price, 0x20 and 0x30 the mean, the others the floor; a contract that is not a player one or a Bosman leaver: 0; a club that is not human: the error box and the value')])
INTERESSA = ['Rust Tweaks']
