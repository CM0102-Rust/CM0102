# A troca com compensacao, a ordem das vagas da lista de compras e o jogador sob oferta (shortlist_manager.cpp):
# - 0x84b6f0 (shortlist_manager_is_compensation_deal): pela vaga da lista de compras (0x842e10, 0x840000), pelo
#   relatorio do olheiro (0x7b62a0) e pelo lance (0x8e5d60); ~27 milhoes de chamadas na carreira automatica ate
#   2002-06-01 (simular.py --cobertura, motor ligado, 01/11/2026). Decide se o clube leva o jogador pagando so a
#   compensacao: a vaga e os lances da IA dependem dela.
# - 0x83b6c0 (shortlist_compare_slots_by_index): o criterio do qsort das 40 vagas na avaliacao do elenco (0x83cb40,
#   17 mil chamadas; o endereco vai empurrado ao qsort 0x945333).
# - 0x84b580 (Shortlist_Manager::mark_player_under_offer): passageira (0 chamadas ate 2002-06-01: so pela negociacao
#   acrescentada 0x8be9d0 e pelo lance aceito 0x8c0080 de humano).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x84b6f0 thiscall (mov ecx, [esp+0x1c] / [ecx+0x856]; 4 pushes: vaga, contrato, clube, &taxa; ret
#   0x10; o this nao e lido), 0x83b6c0 cdecl (o qsort chama com 2 argumentos e tira; ret simples), 0x84b580 thiscall
#   (mov ecx, [ebx+0x856]; push troca; push comprador; push jogador; ret 0xc).
#   Na 0x84b6f0: 0x868210 thiscall (push 0; push 0; push clube; mov ecx, contrato; ret 0xc), 0x8d0780 (push 0 ou 1;
#   push clube; push pessoa; mov ecx, [0xde1f54]; ret 0xc: o porte e stdcall, a copia passa o ecx como a original),
#   0x8d0a70 (push pessoa; mov ecx, [0xde1f54]; ret 4), 0x867d30 thiscall (mov ecx, contrato; ret simples).
#   Na 0x84b580: 0x68d020/0x68d040/0x68d1b0 thiscall (lea ecx, lista / mov ecx, troca), 0x842180 thiscall (push 0;
#   push 1; push 0; push lista; push jogador; mov ecx, ebx; ret 0x14), 0x600a50 cdecl (5 pushes; add esp, 0x14:
#   (clube, 0, 1, 0, 0)), 0x8c61e0 thiscall (push id; mov ecx, [0xde1f54]; ret 4), 0x849ce0 thiscall (push 0; push
#   0/1; push jogador; push clube; mov ecx, ebx; ret 0x10).
# Comparadores: 0x84b6f0 lado a lado (leve) pela taxa e a devolucao; 0x83b6c0 a devolucao; 0x84b580 lado a lado pelas
# listas de 40 vagas dos clubes interessados e pelo cursor da lista da troca. Iguais no cm0102-1080.exe (as tres
# inteiras). Deterministicas: o x87 da 0x84b6f0 e uma multiplicacao e uma comparacao em precisao dupla (fild exato), sem
# leitura de pilha sem valor (o ebp da 0x84b580 que vale entre os clubes e reproduzido).
SAVE = False
ROTINAS = [('troca_com_compensacao', 0x84b6f0, [0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('ordem_das_vagas', 0x83b6c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('marcar_sob_oferta', 0x84b580, [0x6a, 0xff, 0x68, 0xe8, 0x25, 0x96, 0x00])]
DOCS = ('a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta',
        'the compensation deal, the order of the shortlist slots and the player under offer',
        [('troca_com_compensacao', 'shortlist_manager_is_compensation_deal', '0x84b6f0', 'shortlist_manager.cpp', 'thiscall',
          'o clube leva o jogador pagando a compensação: fora do período protegido da FIFA, se a taxa (0x8d0780) é menor que o valor dele; dentro, só com lei Bosman e a taxa até o valor, pela faixa do vencimento do contrato (0 não; 1-8: o +3 da vaga × 0,5 a 0,9 acima do preço; 9-14 sim)',
          'the club takes the player paying the compensation: outside the FIFA protected period, if the fee (0x8d0780) is below his value; inside it, only with the Bosman rule and the fee up to the value, by the contract expiry band (0 no; 1-8: the slot\'s +3 × 0.5 to 0.9 above the price; 9-14 yes)'),
         ('ordem_das_vagas', 'shortlist_compare_slots_by_index', '0x83b6c0', 'shortlist_manager.cpp', 'cdecl',
          'o critério do qsort das 40 vagas da lista de compras: a diferença dos índices (+0x23); iguais, 1',
          'the qsort criterion of the 40 shortlist slots: the difference of the indices (+0x23); equal, 1'),
         ('marcar_sob_oferta', 'Shortlist_Manager::mark_player_under_offer', '0x84b580', 'shortlist_manager.cpp', 'thiscall',
          'o jogador sob oferta: a vaga dele nas listas dos clubes interessados (fora os de humano, o comprador ou os clubes das ofertas da troca) ganha o bit 0x20, que dobra o preço do lance da IA',
          'the player under offer: his slot in the interested clubs\' lists (except human clubs, the buyer or the clubs of the exchange offers) gets bit 0x20, which doubles the AI bid price')])
CAMPOS = [('vaga da lista de compras (0x25 bytes) +3 verba, +0xb preço, +0x23 índice, +0x24 marcas (bit 0x20 sob oferta); valor; lê (troca_com_compensacao, ordem_das_vagas) e grava o bit (marcar_sob_oferta)',
           'shortlist slot (0x25 bytes) +3 budget, +0xb price, +0x23 index, +0x24 flags (bit 0x20 under offer); value; reads (troca_com_compensacao, ordem_das_vagas) and writes the bit (marcar_sob_oferta)'),
          ('contrato +0 id da pessoa, +0x49 byte que dispensa a conta; id/valor; lê (troca_com_compensacao)',
           'contract +0 person id, +0x49 byte that skips the check; id/value; reads (troca_com_compensacao)'),
          ('pessoa ([0xae23bc] + 110 × id) +0x52 valor; valor; lê (troca_com_compensacao)',
           'person ([0xae23bc] + 110 × id) +0x52 value; value; reads (troca_com_compensacao)'),
          ('Shortlist_Manager +0xdc tabela dos clubes (9 bytes por clube; +0 as 40 vagas); ponteiro; lê (marcar_sob_oferta)',
           'Shortlist_Manager +0xdc club table (9 bytes per club; +0 the 40 slots); pointer; reads (marcar_sob_oferta)')]
INTERESSA = ['CMScout', 'CMRTE']
