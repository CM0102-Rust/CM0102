# A Liga dos Campeões: o sorteio dos grupos da primeira fase e a ordem dos potes (european_cup.cpp):
# - 0x585290 (EuropeanCup::draw_phase1_groups, 2066 bytes, quadro SEH só para o new da fase: sem destrutor no caminho
#   normal): os 32 clubes (os 16 diretos da tabela [+0x14] e os vencedores da terceira preliminar em +0xa7), ordenados
#   pelo pote, os parceiros da mesma nação (em metades opostas), o sorteio dos 8 grupos pote a pote (o clube, a metade e
#   o primeiro grupo livre da metade), a troca dos clubes da mesma nação no grupo, os pontos do coeficiente, o prêmio de
#   1.800.000 e a notícia, as 8 fases (League_Stage) e os perdedores da terceira preliminar na Copa da UEFA. Roda uma
#   vez na carreira automática até 2002-06-01 (fim de agosto de 2001), pela fase seguinte (0x5851d0, Rust: o contador
#   "cpy_5 next stage" das cadeias 261 a 268).
# - 0x586f60 (european_cup_compare_clubs_by_seed, 49 bytes): a comparação da qsort dos potes (a semente europeia +0x1df
#   e a reputação): passageira (só pela qsort do sorteio).
# Iguais no cm0102-1080.exe (as duas inteiras). Chamam originais não portadas: 0x945501 (calloc), 0x9452ca (free),
# 0x944cf1 (new), 0x68b100 (o construtor da fase), 0x9058b0 (o coeficiente), 0x48e3e0 (a notícia do prêmio), a tabela
# das rodadas (método +0x3c, 0x583c50), as posições previstas da UEFA (método +0x5c dela) e a caixa; o resto já em
# Rust (0x945333, 0x90cfc0, 0x5a0590, 0x5999a0, 0x684230).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x585290: 0x5851d0 (mov ecx, esi; sem push; a cópia Rust proxima_fase_da_liga_dos_campeoes chama
#   thiscall(this)) -> thiscall, ret 0.
# - quem chama a 0x586f60: a qsort (0x945333) pelo ponteiro (push b; push a; add esp, 8) -> cdecl.
# - dentro da 0x585290: 0x945501 push 9 (ou 4); push 0x20; add esp, 8 -> cdecl; 0x945333 push 0x586f60; push 9; push
#   0x20; push lista; add esp, 0x10 -> cdecl; 0x90cfc0 push n; add esp, 4 -> cdecl; 0x9452ca push; add esp -> cdecl;
#   0x9058b0 push 1; push nação; mov ecx, [0xde1f58] -> thiscall, ret 8; 0x5a0590 push clube; mov ecx, [0xae2a38] ->
#   thiscall, ret 4 (o push 0x1b7740 antes dela fica para a 0x5999a0: push valor; mov ecx, finanças -> thiscall,
#   ret 4); 0x48e3e0 push −2; push 0; push 0x449; push −1; push 0; push 0x1b7740; push clube; mov ecx, this ->
#   thiscall, ret 0x1c; o método +0x3c push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10;
#   0x944cf1 push 0xee; add esp, 4 -> cdecl; 0x68b100 (22 push; mov ecx, novo) -> thiscall, ret 0x58 (o ano no dx com
#   lixo nos bytes altos, lido como word pelo construtor: a cópia passa o ano limpo); 0x684230 mov ecx, fase ->
#   thiscall; o método +0x5c da UEFA mov ecx, uefa -> thiscall; a caixa (aviso_em) e a parada sem memória (erro_em:
#   0x5cc7d0, 0x5a8f60, 0x945934(−1)).
# Comparadores: a 0x585290 só a original (sorteia, paga prêmios, aloca as fases e manda notícias), conferindo o plano
# feito antes: as 8 fases, +0x30 = 7 e os 32 clubes do plano nas tabelas das fases; a 0x586f60 pela devolução.
# Determinismo: as duas saídas da tabela das rodadas começam em 0 na cópia (a 0x583c50 as grava nas fases de grupo);
# os contadores das metades e os vetores alocados são os da original (zerados pelo calloc); nada lê pilha, heap ou
# endereço como valor (a ordem dos clubes é a da qsort pelos valores dos clubes).
# Passageira vinda da cqg_5 (02/11, a mutação dela escapava sozinha: as posições previstas só alimentam a estatística):
# - 0x586900 (EuropeanCup::rank_teams, 768 bytes, método +0x5c da Liga dos Campeões): os inscritos ordenados pela força
#   (0x4c08f0) e a posição prevista de cada um (0x4a2540): o mata-mata da final para trás, 2 × 4 da segunda fase, 2 × 8
#   da primeira e as preliminares. Pela criação da Liga dos Campeões na carga do jogo. Quem chama: o método +0x5c (mov
#   ecx, copa; sem push) -> thiscall, ret 0; dentro: 0x944e46 cdecl; 0x4c08f0 push n; push vetor; add esp, 8 -> cdecl;
#   o método +0x3c push 0; push &id; push &rodadas; push 0xc; mov ecx -> thiscall, ret 0x10; 0x4a2540 push [esp+0x18];
#   push clube; mov ecx, [+8] -> thiscall, ret 8 (o dword com a posição no byte de baixo e sobra de pilha nos outros; a
#   0x4a2540 grava só o byte: a cópia passa o byte limpo); 0x9452ca cdecl (a tabela do mata-mata depois das posições
#   dele, o vetor no fim). Comparador: só a original, conferindo o plano (a posição +0x72 de cada clube, pelo vetor
#   ordenado numa cópia e a tabela do mata-mata pedida ao método +0x3c).
SAVE = False
ROTINAS = [('campeoes_sortear_grupos', 0x585290, [0x6a, 0xff, 0x68, 0xde, 0x8a, 0x95, 0x00]),
           ('campeoes_ordem_dos_potes', 0x586f60, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
           ('campeoes_posicoes_previstas', 0x586900, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões: o sorteio dos grupos da primeira fase (parceiros da mesma nação, metades, potes, prêmio, '
 'fases e os perdedores para a Copa da UEFA), a ordem dos potes e as posições previstas',
 'the Champions League: the phase 1 group draw (same-nation partners, halves, pots, prize, stages and the losers to '
 'the UEFA Cup), the pot order and the predicted positions',
 [('campeoes_sortear_grupos',
   'EuropeanCup::draw_phase1_groups',
   '0x585290',
   'european_cup.cpp',
   'thiscall, ret 0',
   'os 32 clubes ordenados pelo pote, os parceiros da mesma nação em metades opostas, o sorteio dos 8 grupos pote a '
   'pote, a troca dos clubes da mesma nação no grupo, o coeficiente, o prêmio de 1.800.000 e a notícia, as 8 fases e '
   'os perdedores da terceira preliminar na Copa da UEFA',
   'the 32 clubs sorted by pot, the same-nation partners in opposite halves, the draw of the 8 groups pot by pot, '
   'the swap of same-nation clubs in a group, the coefficient, the 1,800,000 prize and its news, the 8 stages and '
   'the third qualifying round losers into the UEFA Cup'),
  ('campeoes_ordem_dos_potes',
   'european_cup_compare_clubs_by_seed',
   '0x586f60',
   'european_cup.cpp',
   'cdecl',
   'a comparação dos potes: a semente europeia do clube (+0x1df), e no empate a reputação maior primeiro',
   "the pot comparison: the club's European seed (+0x1df), and on a tie the higher reputation first"),
  ('campeoes_posicoes_previstas',
   'EuropeanCup::rank_teams',
   '0x586900',
   'european_cup.cpp',
   'thiscall, ret 0',
   'os inscritos ordenados pela força e a posição prevista de cada um: o mata-mata da final para trás, a segunda e a '
   'primeira fase de grupos e as preliminares',
   "the entrants sorted by strength and each one's predicted position: the knockout from the final back, the second "
   'and first group phases and the qualifying rounds')])
CAMPOS = [('liga dos campeões +4 id, +0xc as fases, +0x14 tabela (6 bytes: +0 clube), +0x30 fase atual, +0x3e times da '
  'preliminar (word), +0x40 ano, +0xa7 tabela da preliminar (6 bytes: +0 clube, +5 1 vencedor / 2 perdedor da '
  'terceira), +0xb1 (byte zerado sem os 32); valor e ponteiro; lê e grava (campeoes_sortear_grupos)',
  'champions league +4 id, +0xc the stages, +0x14 table (6 bytes: +0 club), +0x30 current stage, +0x3e qualifying '
  'teams (word), +0x40 year, +0xa7 qualifying table (6 bytes: +0 club, +5 1 winner / 2 loser of the third round), '
  '+0xb1 (byte cleared without 32); value and pointer; reads and writes (campeoes_sortear_grupos)'),
 ('copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]) +0x14 tabela (situação 5), +0x36 quantos (word), +0x3e times (word), '
  '+0xa7 tabela (+4/+5 zerados); valor e ponteiro; grava (campeoes_sortear_grupos)',
  'UEFA cup ([[0xadadfc] + 4 × [0x9cf6f0]]) +0x14 table (status 5), +0x36 count (word), +0x3e teams (word), +0xa7 '
  'table (+4/+5 cleared); value and pointer; writes (campeoes_sortear_grupos)'),
 ('clube +0x53 nação, +0x80 reputação (word), +0x1db competição europeia (id), +0x1df semente europeia (char); '
  'valor, id e ponteiro; lê (campeoes_sortear_grupos, campeoes_ordem_dos_potes), grava +0x1db '
  '(campeoes_sortear_grupos)',
  'club +0x53 nation, +0x80 reputation (word), +0x1db European competition (id), +0x1df European seed (char); value, '
  'id and pointer; reads (campeoes_sortear_grupos, campeoes_ordem_dos_potes), writes +0x1db '
  '(campeoes_sortear_grupos)'),
 ('liga dos campeões +8 estatística (Comp_Stats), +0x14 inscritos (6 bytes: +0 clube), +0x36 quantos (word), +0x3c '
  'rodadas preliminares (word), +0xa3 tabela das preliminares (0x68 bytes, +0x1a confrontos); ponteiro e valor; lê '
  '(campeoes_posicoes_previstas)',
  'champions league +8 statistics (Comp_Stats), +0x14 entrants (6 bytes: +0 club), +0x36 count (word), +0x3c '
  'qualifying rounds (word), +0xa3 qualifying round table (0x68 bytes, +0x1a ties); pointer and value; reads '
  '(campeoes_posicoes_previstas)'),
 ('registro da estatística da competição +0x72 posição (byte); valor; grava pela 0x4a2540 '
  '(campeoes_posicoes_previstas)',
  'competition statistics record +0x72 position (byte); value; writes through 0x4a2540 '
  '(campeoes_posicoes_previstas)')]
INTERESSA = ['CMRTE', 'GPF2']
