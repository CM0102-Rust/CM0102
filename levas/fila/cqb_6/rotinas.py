# A parte dos clubes anteriores e o percentual da próxima venda (transfer_manager.cpp):
# TRANSFER_MANAGER::pay_former_club_fee_cuts (0x8e4f20: pela transferência efetivada com valor, 0x8c9390, já em Rust;
# jogador com mais de 22 anos: (valor / 20) / soma dos pesos × peso de cada clube anterior entra nas vendas das finanças
# do COMPRADOR, como a original faz, e a notícia 0xfc3 vai ao clube anterior quando a parte passa de 11 × reputação −
# 10000), TRANSFER_MANAGER::pay_next_sale_percentage (0x8cb0f0: pela transferência efetivada com a marca 8 da pessoa,
# 0x8c91e0, já em Rust; a cláusula achada pela busca binária paga o percentual ao clube da cláusula; roda pouco na
# carreira: passageira) e TRANSFER_MANAGER::post_sale_percentage_news (0x8db3a0: a notícia 0xfa7 do percentual,
# passageira). Iguais no cm0102-1080.exe; quadros SEH sem destrutor (a notícia local não tem destrutor; o estado só
# passa a 0). Sem x87.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8e4f20: 0x8c94fa push esi; mov ecx, ebp; call -> thiscall(this, oferta), ret 4 (o this não é usado).
# - quem chama a 0x8cb0f0: 0x8c920c push edi; push ebp; mov ecx, esi; call -> thiscall(this, pessoa, oferta), ret 8.
# - quem chama a 0x8db3a0: 0x8cb258 mov ecx, [esp+0x10] (this); push ebp; push edi; push esi; push ebx; call ->
#   thiscall(this, cláusula, parte, oferta, da_oferta), ret 0x10 (o this não é usado).
# - dentro da 0x8e4f20: 0x77bbe0 push 0; push 0xfc3; lea ecx, notícia -> thiscall(notícia, tipo, 0), ret 8; 0x785790
#   push v; push k; lea ecx, notícia -> thiscall(notícia, k, v), ret 8; 0x4cf5a0 mov ecx, [0xae1050]; push pessoa ->
#   thiscall(gerência, pessoa), ret 4; 0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall(tabela, clube), ret 4;
#   0x599800 push edx; push eax (cdq); mov ecx, finanças -> thiscall(fin, baixo, alto), ret 8; 0x7861e0 push clube;
#   push notícia; mov ecx, [0xb7431c] -> thiscall(news, notícia, clube), ret 8.
# - dentro da 0x8cb0f0: 0x4cf5a0 como acima; 0x946464 push 0x8e54d0; push 0xd; push conta; push vetor; push chave; add
#   esp, 0x14 -> cdecl(chave, vetor, conta, 0xd, comparação); 0x68d1b0 mov ecx, lista -> thiscall(lista); 0x4d0c60 mov
#   ecx, [0xae1050]; push 0; push -1; push 0; push clube; push pessoa -> thiscall(gerência, pessoa, clube, 0, -1, 0),
#   ret 0x14; 0x8db3a0 como acima; 0x5a0590 como acima; 0x448ad0 mov ecx, [0xad9dac]; push clube da cláusula; push
#   clube do contrato; push parte -> thiscall(registros, parte, comprador, vendedor), ret 0xc; 0x599800 e 0x599890 push
#   alto; push baixo; mov ecx, finanças -> thiscall(fin, baixo, alto), ret 8.
# - dentro da 0x8db3a0: 0x77bbe0, 0x785790, 0x4cf5a0 e 0x7861e0 como acima.
# Comparadores: a 0x8e4f20 e a 0x8cb0f0 com a cópia em ensaio (sem notícias nem os totais do clube 0x448ad0) e a
# original a partir do mesmo estado, lado a lado nas finanças que recebem e pagam e em [0xae1080] (com entrega de
# notícia ou pagamento feito, só as finanças: a resposta automática do humano de férias pode regravar [0xae1080]); a
# 0x8cb0f0 também no sorteio (a 0x4d0c60 pode sortear). A 0x8db3a0 só a original (entrega a notícia; a cópia leva o
# ramo no modo Rust).
SAVE = False
ROTINAS = [('parte_dos_clubes_anteriores', 0x8e4f20, [0x6a, 0xff, 0x68, 0xab, 0x48, 0x96, 0x00]),
           ('percentual_da_proxima_venda_pago', 0x8cb0f0, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('noticia_do_percentual_da_venda', 0x8db3a0, [0x6a, 0xff, 0x68, 0xdb, 0x44, 0x96, 0x00])]
DOCS = ('a parte dos clubes anteriores na transferência (que a original credita ao comprador), o percentual da próxima venda pago ao clube da cláusula e a notícia dele',
        "the former clubs' cut of a transfer fee (which the original credits to the buyer), the next-sale percentage paid to the clause's club and its news",
        [('parte_dos_clubes_anteriores', 'TRANSFER_MANAGER::pay_former_club_fee_cuts', '0x8e4f20', 'transfer_manager.cpp', 'thiscall, ret 4',
          'jogador com mais de 22 anos: (valor da oferta / 20) / soma dos pesos × peso de cada um dos até cinco clubes anteriores entra nas vendas das finanças do comprador; a notícia 0xfc3 vai ao clube anterior quando a parte passa de 11 × reputação − 10000',
          "player over 22: (offer fee / 20) / sum of weights × weight of each of up to five former clubs goes into the buyer's sales; news 0xfc3 goes to the former club when the cut exceeds 11 × reputation − 10000"),
         ('percentual_da_proxima_venda_pago', 'TRANSFER_MANAGER::pay_next_sale_percentage', '0x8cb0f0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'acha pela busca binária a cláusula da pessoa com o clube do contrato; a parte (percentual × valor da oferta com os da troca, ou o valor da pessoa) de 1000 para cima: notícia, totais de transferência, entra no clube da cláusula e sai do clube do contrato',
          "binary-searches the person's clause with the contract club; the cut (percentage × offer fee plus exchange players, or the person's value) from 1000 up: news, transfer totals, paid into the clause club and out of the contract club"),
         ('noticia_do_percentual_da_venda', 'TRANSFER_MANAGER::post_sale_percentage_news', '0x8db3a0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'a notícia 0xfa7: a pessoa, o percentual, os dois clubes da cláusula, a parte e o comprador (ou o clube do contrato), aos técnicos do clube da cláusula e do vendedor',
          'news 0xfa7: the person, the percentage, the two clause clubs, the cut and the buyer (or the contract club), to the managers of the clause club and the seller')])
CAMPOS = [('oferta +4 / +8 / +0xc / +0x1b: a troca (LLIST de ids), o comprador, a pessoa e o valor; id e valor; lê (as três)',
           'offer +4 / +8 / +0xc / +0x1b: the exchange (LLIST of ids), the buyer, the person and the fee; id and value; reads (all three)'),
          ('pessoa +0x18: a idade; valor; lê (parte_dos_clubes_anteriores)',
           'person +0x18: the age; value; reads (parte_dos_clubes_anteriores)'),
          ('pessoa +0x52: o valor dos jogadores da troca; valor; lê (percentual_da_proxima_venda_pago)',
           'person +0x52: the value of the exchange players; value; reads (percentual_da_proxima_venda_pago)'),
          ('registro de 79 bytes [0xae2d0c] +0xe + 5k / +0x12 + 5k (k = 0..4): os clubes anteriores (−1 = vaga vazia) e o peso de cada um; id e valor; lê (parte_dos_clubes_anteriores)',
           '79-byte record [0xae2d0c] +0xe + 5k / +0x12 + 5k (k = 0..4): the former clubs (−1 = empty slot) and their weight; id and value; reads (parte_dos_clubes_anteriores)'),
          ('clube +0x80: a reputação (o piso da notícia da parte); valor; lê (parte_dos_clubes_anteriores)',
           'club +0x80: the reputation (the floor of the cut news); value; reads (parte_dos_clubes_anteriores)'),
          ('finanças do clube (0x167 bytes) +0 saldo, +0x78/+0x118 vendas, +0x70/+0x110 compras; valores; grava pelas 0x599800/0x599890 (parte_dos_clubes_anteriores no comprador, percentual_da_proxima_venda_pago nos dois clubes)',
           'club finance (0x167 bytes) +0 balance, +0x78/+0x118 sales, +0x70/+0x110 purchases; values; writes through 0x599800/0x599890 (parte_dos_clubes_anteriores on the buyer, percentual_da_proxima_venda_pago on both clubs)'),
          ('[this+0x880] / [this+0x888]: as cláusulas de percentual da próxima venda (0xd bytes: +0 percentual, +1 clube vendedor, +5 clube da cláusula, +9 pessoa) e a conta; ponteiro e valor; lê (percentual_da_proxima_venda_pago, noticia_do_percentual_da_venda)',
           '[this+0x880] / [this+0x888]: the next-sale percentage clauses (0xd bytes: +0 percentage, +1 selling club, +5 clause club, +9 person) and their count; pointer and value; reads (percentual_da_proxima_venda_pago, noticia_do_percentual_da_venda)'),
          ('[0xae1080]: o último contrato consultado; ponteiro; grava (as três)',
           '[0xae1080]: the last contract looked up; pointer; writes (all three)')]
INTERESSA = ['CMRTE', 'SGE']
