# A Liga dos Campeões: o sorteio dos grupos da segunda fase e a ordem dos classificados (european_cup.cpp):
# - 0x585ab0 (EuropeanCup::draw_phase2_groups, 2175 bytes, quadro SEH só para o new da fase: sem destrutor no caminho
#   normal): os 16 classificados da primeira fase (o primeiro e o segundo de cada grupo, 0x6826d0) com a conta dos da
#   mesma nação, ordenados (0x5871e0), os cartões zerados (0x7e9180), os pares do mesmo grupo, a escolha do grupo de cada
#   um (a ordem dos 4 grupos embaralhada com 0x90cfc0(4); o grupo com menos times, com vaga na metade dele e sem time
#   do mesmo grupo da primeira fase nem da mesma nação), o sorteio de novo (até 20 vezes) se sobrar clube, os pontos do
#   coeficiente, o prêmio de 1.200.000 e a notícia, as fases 8..11 (League_Stage), os terceiros na Copa da UEFA e os
#   quartos sem competição europeia. Roda uma vez na carreira automática até 2002-06-01 (6/11/2001, a "cpy_5 fase
#   seguinte" 2; o mesmo dia em que o contador da cobertura posto no meio dela derrubou o jogo, ver o relatório).
# - 0x5871e0 (european_cup_compare_phase2_draw_entries, 61 bytes): a comparação da qsort dos classificados (os da mesma
#   nação, o word +0x8e da nação e o segundo lugar): passageira (só pela qsort do sorteio).
# Iguais no cm0102-1080.exe (as duas inteiras). Chamam originais não portadas: 0x945501 (calloc), 0x944e46 (malloc),
# 0x9452ca (free), 0x944cf1 (new), 0x68b100 (o construtor da fase), 0x9058b0, 0x48e3e0, 0x7e9180, a tabela das rodadas
# (método +0x3c, 0x583c50), as posições previstas da UEFA (método +0x5c dela) e a caixa; o resto já em Rust (0x6826d0,
# 0x945333, 0x90cfc0, 0x5a0590, 0x5999a0, 0x684230).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x585ab0: 0x5851d0 (push 0; mov ecx, esi; a cópia Rust proxima_fase_da_liga_dos_campeoes chama
#   thiscall(this, 0)) e ela mesma (push ebx = tentativa + 1; mov ecx, this) -> thiscall, ret 4; a tentativa é lida como
#   byte (mov bl, byte ptr [esp+0x45c]).
# - quem chama a 0x5871e0: a qsort pelo ponteiro (push b; push a; add esp, 8) -> cdecl.
# - dentro da 0x585ab0: 0x945501 push 7; push 0x10; add esp, 8 -> cdecl; 0x6826d0 push i; mov ecx, fase -> thiscall,
#   ret 4; 0x945333 push 0x5871e0; push 7; push 0x10; push lista; add esp, 0x10 -> cdecl; 0x7e9180 push clube; mov ecx,
#   [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4; 0x90cfc0 push 4; add esp (acumulado, 8) -> cdecl; 0x944e46 push
#   0x40; add esp, 4 -> cdecl; 0x9058b0 push 1; push nação; mov ecx, [0xde1f58] -> thiscall, ret 8; 0x5a0590 push
#   clube; mov ecx, [0xae2a38] -> thiscall, ret 4 (o push 0x124f80 antes dela fica para a 0x5999a0, mov ecx, finanças
#   -> thiscall, ret 4); 0x48e3e0 push −2; push 0; push 0x44a; push −1; push 0; push 0x124f80; push clube; mov ecx, this
#   -> thiscall, ret 0x1c; o método +0x3c push 0; push &id; push &rodadas; push h + 8; mov ecx, this -> thiscall, ret
#   0x10; 0x944cf1 push 0xee; add esp, 4 -> cdecl; 0x68b100 (22 push; mov ecx, novo) -> thiscall, ret 0x58 (o ano no cx
#   com o resto de um endereço de pilha nos bytes altos, lido como word pelo construtor: a cópia passa o ano limpo);
#   0x684230 mov ecx, fase -> thiscall; 0x9452ca push; add esp -> cdecl; o método +0x5c da UEFA mov ecx -> thiscall.
# Comparadores: a 0x585ab0 só a original (sorteia, paga, aloca e se chama de novo), conferindo o plano na chamada de fora:
# as fases 8..11, +0x30 = 11 e os 16 classificados nas tabelas delas; a 0x5871e0 pela devolução.
# Determinismo: as saídas da tabela das rodadas começam em 0 na cópia; as listas são as da original (zeradas pelo
# calloc); nada lê pilha, heap ou endereço como valor.
# Passageira vinda da cqg_5 (02/11):
# - 0x4b01d0 (comp_util_get_parent_stage_id, 261 bytes, cdecl): o grupo de uma fase pelo id (as faixas fixas da
#   original), pela primeira fase do grupo e pela fase atual (0x48d2b0, 0x48cec0, Rust: push id; add esp, 4 -> cdecl).
#   Comparador pela devolução. Não rodou na carreira automática até 2002-06-01 (o contador "cqg_6 grupo 0xa").
SAVE = False
ROTINAS = [('campeoes_sortear_segunda_fase', 0x585ab0, [0x6a, 0xff, 0x68, 0xfe, 0x8a, 0x95, 0x00]),
           ('campeoes_ordem_da_segunda_fase', 0x5871e0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('grupo_da_fase', 0x4b01d0, [0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões: o sorteio dos grupos da segunda fase (pares do mesmo grupo, metades, nações, prêmio, fases, '
 'terceiros para a Copa da UEFA) e a ordem dos classificados; o grupo de uma fase pelo id',
 'the Champions League: the phase 2 group draw (same-group pairs, halves, nations, prize, stages, third places to '
 "the UEFA Cup) and the qualifiers' order; the group of a stage by id",
 [('campeoes_sortear_segunda_fase',
   'EuropeanCup::draw_phase2_groups',
   '0x585ab0',
   'european_cup.cpp',
   'thiscall, ret 4',
   'os 16 classificados ordenados, os pares do mesmo grupo, a escolha do grupo de cada um (ordem embaralhada, menos '
   'times, sem o mesmo grupo nem a mesma nação), o sorteio de novo até 20 vezes, o coeficiente, o prêmio de '
   '1.200.000 e a notícia, as fases 8 a 11 e os terceiros na Copa da UEFA',
   "the 16 qualifiers sorted, the same-group pairs, each one's group choice (shuffled order, fewest teams, no same "
   'group nor nation), the redraw up to 20 times, the coefficient, the 1,200,000 prize and its news, stages 8 to 11 '
   'and the third places into the UEFA Cup'),
  ('campeoes_ordem_da_segunda_fase',
   'european_cup_compare_phase2_draw_entries',
   '0x5871e0',
   'european_cup.cpp',
   'cdecl',
   'a comparação dos classificados: mais clubes da mesma nação primeiro, depois o word +0x8e da nação maior, depois '
   'o vencedor do grupo antes do segundo',
   'the qualifiers comparison: more same-nation clubs first, then the higher nation word +0x8e, then the group '
   'winner before the runner-up'),
  ('grupo_da_fase',
   'comp_util_get_parent_stage_id',
   '0x4b01d0',
   'comp_util.cpp',
   'cdecl',
   'o grupo (a fase que contém) de uma fase ou rodada pelo id, pelas faixas fixas; −1 sem grupo',
   'the group (the containing stage) of a stage or round by id, by the fixed ranges; −1 without a group')])
CAMPOS = [('liga dos campeões +4 id, +0xc as fases (0..7 a primeira fase, 8..11 a segunda), +0x30 fase atual, +0x40 ano, '
  '+0x50 índice do órgão (char); valor e ponteiro; lê e grava (campeoes_sortear_segunda_fase)',
  'champions league +4 id, +0xc the stages (0..7 phase 1, 8..11 phase 2), +0x30 current stage, +0x40 year, +0x50 '
  'ruling body index (char); value and pointer; reads and writes (campeoes_sortear_segunda_fase)'),
 ('entrada do sorteio da segunda fase (7 bytes): +0 clube, +4 quantos da mesma nação, +5 grupo da primeira fase, +6 '
  '0 vencedor / 1 segundo; valor; grava (campeoes_sortear_segunda_fase), lê (campeoes_ordem_da_segunda_fase)',
  'phase 2 draw entry (7 bytes): +0 club, +4 same-nation count, +5 phase 1 group, +6 0 winner / 1 runner-up; value; '
  'writes (campeoes_sortear_segunda_fase), reads (campeoes_ordem_da_segunda_fase)'),
 ('copa da UEFA +0x14 tabela (situação 5), +0x36 quantos, +0x3e times, +0xa7 tabela; clube +0x53 nação, +0x1db '
  'competição europeia (id; −1 sem); nação +0x8e (word) [inferência: a força da nação no coeficiente]; valor, id e '
  'ponteiro; lê e grava (campeoes_sortear_segunda_fase), lê (campeoes_ordem_da_segunda_fase)',
  'UEFA cup +0x14 table (status 5), +0x36 count, +0x3e teams, +0xa7 table; club +0x53 nation, +0x1db European '
  "competition (id; −1 none); nation +0x8e (word) [inference: the nation's coefficient strength]; value, id and "
  'pointer; reads and writes (campeoes_sortear_segunda_fase), reads (campeoes_ordem_da_segunda_fase)'),
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
