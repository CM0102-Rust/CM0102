# Bloco novo: competições (Cup.cpp). Os mandantes da rodada (0x51e870: o sorteio da rodada 0x51c410 e o processamento
# da partida 0x51a150; 133 chamadas na carreira) e o prêmio da rodada (0x522240: 3 chamadas no processamento da
# partida; 574). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH,
# sem caixa (a 0x5a0590 só tem caixa no erro), sem sorteio. A 0x51e870 só grava os confrontos (comparador lado a lado
# na faixa de confrontos; a posição 0x90 da vtable, club_in_higher_division 0x522360, é consulta pura e roda nos dois
# lados, pelo mesmo ponteiro). A 0x522240 grava o fundo (+0x28) e chama a notícia do prêmio (0x48e3e0, aloca) e o
# dinheiro (0x5999a0): o comparador escuta essas chamadas (escuta do item cpb_1, que vem antes na fila). Variável
# local: a 0x522240 grava só o byte do desfecho num dword da pilha e empurra o dword inteiro para a 0x48e3e0, que lê
# só o byte (`movsx ecx, byte [esp+0x114]`): os bits altos de lixo nunca são usados; a cópia passa o byte e a fita
# compara só o byte. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('copa_mandantes_da_rodada', 0x51e870, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x08]),
           ('copa_premio_da_rodada', 0x522240, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c])]
DOCS = ('a copa: os mandantes da rodada e o prêmio da rodada',
        'the cup: the home teams of the round and the round prize money',
        [('copa_mandantes_da_rodada', 'Cup::set_home_teams_for_round', '0x51e870', 'Cup.cpp', 'thiscall',
          'inverte mandante e visitante num confronto ou em todos da rodada pela regra da rodada: o clube de divisão mais alta, o nível do time ou a nação da copa',
          'swaps home and away in one tie or in every tie of the round by the round rule: the club in the higher division, the team level or the cup nation'),
         ('copa_premio_da_rodada', 'Cup::pay_round_prize_money', '0x522240', 'Cup.cpp', 'thiscall',
          'paga o prêmio da rodada ao vencedor (com a notícia) e ao perdedor, do valor da rodada ou de uma parte do fundo, e tira do fundo',
          'pays the round prize money to the winner (with the news item) and to the loser, from the round value or from a share of the fund, and takes it from the fund')])
CAMPOS = [('Cup +0xad: confrontos da rodada (0x35 bytes: +0/+2 times, +0xa/+0xc, +0xe/+0x10); lê e grava (copa_mandantes_da_rodada)',
           'Cup +0xad: ties of the round (0x35 bytes: +0/+2 teams, +0xa/+0xc, +0xe/+0x10); reads and writes (copa_mandantes_da_rodada)'),
          ('Cup +0x24 (nação), rodada +0xb (regra), time +4 (nível); lê (copa_mandantes_da_rodada)',
           'Cup +0x24 (nation), round +0xb (rule), team +4 (level); reads (copa_mandantes_da_rodada)'),
          ('Cup +0x28: fundo de prêmios; lê e grava. Rodada +0x60/+0x64 (prêmios), +0x6f; lê (copa_premio_da_rodada)',
           'Cup +0x28: prize fund; reads and writes. Round +0x60/+0x64 (prizes), +0x6f; reads (copa_premio_da_rodada)')]
INTERESSA = ['SGE', 'CMRTE']
