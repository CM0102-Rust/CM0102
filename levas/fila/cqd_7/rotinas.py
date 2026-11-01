# A árvore dos confrontos da copa, a primeira fase do grupo e a fase atual (Cup.cpp, comp.cpp):
# - 0x518790 (Cup::map_fixture_tree, 3832 bytes): os vetores dos confrontos de cada rodada, os confrontos vazios, a
#   vaga de cada confronto na rodada seguinte (pelo par ou sorteada, o lado pela moeda) e os times da rodada nos
#   confrontos (sorteados, o lado pela moeda); pela criação das copas (62 construtores): 22 chamadas na carreira
#   automática até 2002-06-01 (docs/cobertura_simulacao.txt, 17/10). Os cruzamentos e os mandos das copas mudam a
#   carreira.
# - 0x48d2b0 (Comp::get_first_stage_of_group, 352 bytes): o começo do grupo de uma fase, pelas telas das competições
#   (0x494afd): passageira.
# - 0x48cec0 (Comp::get_current_stage, 832 bytes): a fase em andamento, pelas telas das competições (0x490773):
#   passageira.
# Iguais no cm0102-1080.exe (as três inteiras). A 0x518790 só chama a CRT (0x944e46 malloc, 0x9452ca free), o sorteio
# 0x90cfc0 e as caixas; a 0x48cec0 chama o método 0x10 das fases (original) e a 0x48d200 (Rust).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x518790: 0x40acc1 e 0x56ac0d (mov ecx, esi; sem push) e outros 60 construtores -> thiscall, ret 0
#   (ela termina em ret; devolve 1 ou -1 no eax).
# - quem chama a 0x48d2b0: 0x494afd (push eax; mov ecx, [esp+0xa2c]) -> thiscall, ret 4.
# - quem chama a 0x48cec0: 0x490773 (mov ecx, ebp; sem push) -> thiscall, ret 0.
# - dentro da 0x518790: 0x944e46 push tamanho; add esp, 4 -> cdecl; 0x90cfc0 push n; add esp, 4 (ou 8 depois de dois)
#   -> cdecl; 0x9452ca push vetor; add esp, 4 -> cdecl; a caixa "Memory Allocation Error" (0x944cff, 0x90d130 com o
#   texto, 0x944c9f com 7 argumentos, 0x66f4e0 duas vezes, 0x5e8290, 0x5cc7d0, 0x5a8f60, 0x945934) = cpq11_sem_memoria;
#   a caixa "Error" com -1 (aviso_em) ou com o jogo parado (erro_em).
# - dentro da 0x48d2b0: 0x4b01d0 push id; add esp, 4 -> cdecl; a caixa (aviso_em).
# - dentro da 0x48cec0: o método 0x10 da fase (push 1; push 0; mov ecx, fase; call [vt+0x10]) -> thiscall; 0x4b01d0
#   como acima; 0x48d200 push k; mov ecx, ebp -> thiscall, ret 4; a caixa (aviso_em).
# Comparadores: a 0x518790 só a original (aloca e sorteia), conferindo o que ela garante: cada confronto das rodadas
# mapeadas aponta uma vaga (uma ou duas rodadas adiante) que aponta de volta para ele; a 0x48d2b0 as duas com o índice
# válido (só leem; o mesmo índice), senão só a original (a caixa); a 0x48cec0 só a original (o método 0x10 das fases
# pode refazer a tabela).
# Determinismo: os vetores dos confrontos vêm do malloc da original (0x944e46); a palavra +4 de cada confronto a
# original não grava aqui (fica o que o malloc deu) e a cópia também não: nenhuma das duas a lê. Nada lê pilha ou usa
# endereço como valor.
SAVE = False
ROTINAS = [('copa_arvore_dos_confrontos', 0x518790, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('competicao_primeira_fase_do_grupo', 0x48d2b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('competicao_fase_atual', 0x48cec0, [0x81, 0xec, 0x00, 0x04, 0x00, 0x00])]
DOCS = ('a copa e as fases: a árvore dos confrontos da copa (as vagas na rodada seguinte e os times da rodada, sorteados), o começo do grupo de uma fase e a fase em andamento',
        'the cup and the stages: the cup fixture tree (the slots in the next round and the round teams, drawn), the first stage of a group and the current stage',
        [('copa_arvore_dos_confrontos', 'Cup::map_fixture_tree', '0x518790', 'Cup.cpp', 'thiscall, ret 0',
          'aloca os confrontos de cada rodada, zera-os, liga cada confronto a uma vaga na rodada seguinte (pelo par ou sorteada, o lado pela moeda; sem vaga, duas rodadas adiante), põe os times de cada rodada nos confrontos dela (sorteados, o time da nação da copa como mandante) e numera o primeiro confronto de cada rodada',
          'allocates the ties of each round, clears them, links each tie to a slot in the next round (by pair or drawn, the side by a coin; with no slot, two rounds ahead), puts each round\'s teams into its ties (drawn, the cup nation\'s club at home) and numbers the first tie of each round'),
         ('competicao_primeira_fase_do_grupo', 'Comp::get_first_stage_of_group', '0x48d2b0', 'comp.cpp', 'thiscall, ret 4',
          'da fase pedida para trás, o índice da primeira fase do mesmo grupo',
          'from the given stage backwards, the index of the first stage of the same group'),
         ('competicao_fase_atual', 'Comp::get_current_stage', '0x48cec0', 'comp.cpp', 'thiscall, ret 0',
          'a primeira fase que não acabou (numa copa que não começou, o começo do grupo dela); todas acabadas, o começo do grupo da última',
          'the first stage not finished (in a cup not yet started, the start of its group); all finished, the start of the last one\'s group')])
CAMPOS = [('copa +0x3c rodadas, +0x3e times, +0x45 clubes, +0xa3 rodadas (0x68 bytes: +7 tipo 0x8c, +0xd bits 0x200 pelo par e 0x400 vaga toda livre, +0x18, +0x1a confrontos, +0x1c times que entram, +0x1e primeiro time, +0x5b primeiro confronto), +0xa7 times (6 bytes: +0 clube, +5 marca), +0xab/+0xac, +0xad vetores dos confrontos, +0xb1 sem confrontos, +0x24 nação; valor e ponteiro; lê e grava (copa_arvore_dos_confrontos)',
           'cup +0x3c rounds, +0x3e teams, +0x45 clubs, +0xa3 rounds (0x68 bytes: +7 type 0x8c, +0xd bits 0x200 by pair and 0x400 fully free slot, +0x18, +0x1a ties, +0x1c entering teams, +0x1e first team, +0x5b first tie), +0xa7 teams (6 bytes: +0 club, +5 flag), +0xab/+0xac, +0xad tie arrays, +0xb1 no ties, +0x24 nation; value and pointer; reads and writes (copa_arvore_dos_confrontos)'),
          ('confronto (0x35 bytes) +0 mandante, +2 visitante, +6 vaga na rodada de destino, +8 lado (1 mandante), +0xa/+0xc confrontos de origem, +0xe/+0x10 rodadas de origem, +0x12 rodada de destino, +0x14..+0x31, +0x32..+0x34; valor; grava (copa_arvore_dos_confrontos)',
           'tie (0x35 bytes) +0 home, +2 away, +6 slot in the target round, +8 side (1 home), +0xa/+0xc source ties, +0xe/+0x10 source rounds, +0x12 target round, +0x14..+0x31, +0x32..+0x34; value; writes (copa_arvore_dos_confrontos)'),
          ('competição +0xc fases, +0x30 última fase, +0x3a id (grupo pela 0x4b01d0), +0x43 copa, +0xab começou; valor e ponteiro; lê (competicao_primeira_fase_do_grupo, competicao_fase_atual)',
           'competition +0xc stages, +0x30 last stage, +0x3a id (group by 0x4b01d0), +0x43 cup, +0xab started; value and pointer; reads (competicao_primeira_fase_do_grupo, competicao_fase_atual)')]
INTERESSA = ['GPF2']
