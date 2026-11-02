# A Liga dos Campeões: as posições previstas dos inscritos; o grupo de uma fase (european_cup.cpp, comp_util.cpp):
# - 0x586900 (EuropeanCup::rank_teams, 768 bytes, método +0x5c da Liga dos Campeões): os inscritos (+0x14) ordenados pela
#   força (0x4c08f0) e a posição prevista de cada um na estatística (0x4a2540): o mata-mata da final para trás (o campeão
#   1, o vice 2, os das semifinais 3...), 2 × 4 da segunda fase, 2 × 8 da primeira e as preliminares da última para a
#   primeira. Pela criação da Liga dos Campeões (0x5837c0) na carga do jogo.
# - 0x4b01d0 (comp_util_get_parent_stage_id, 261 bytes, cdecl): o grupo de uma fase pelo id (as faixas fixas da
#   original), pela primeira fase do grupo e pela fase atual (0x48d2b0, 0x48cec0, Rust) e outras: passageira.
# Iguais no cm0102-1080.exe (as duas inteiras). Chamam originais não portadas: 0x944e46 (malloc), 0x9452ca (free), a
# tabela das rodadas (método +0x3c, 0x583c50) e a caixa; o resto já em Rust (0x4c08f0, 0x4a2540).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x586900: o método +0x5c da vtable 0x96a1d0 (mov ecx, copa; sem push) -> thiscall, ret 0.
# - quem chama a 0x4b01d0: 0x48d2b0 e 0x48cec0 (push id; add esp, 4) -> cdecl.
# - dentro da 0x586900: 0x944e46 push (+0x36 << 2); add esp, 4 -> cdecl; 0x4c08f0 push n; push vetor; add esp, 8 ->
#   cdecl; o método +0x3c push 0; push &id; push &rodadas; push 0xc; mov ecx, edi -> thiscall, ret 0x10; 0x4a2540
#   push [esp+0x18]; push clube; mov ecx, [+8] -> thiscall, ret 8 (o dword empurrado tem a posição no byte de baixo e
#   sobra de pilha nos outros; a 0x4a2540 grava só o byte em +0x72: a cópia passa o byte limpo); 0x9452ca push; add
#   esp, 4 -> cdecl (a tabela do mata-mata depois das posições dele, o vetor no fim).
# Comparadores: a 0x586900 só a original (acrescenta registros à estatística), conferindo o plano feito antes (o vetor
# ordenado numa cópia pela mesma 0x4c08f0, a tabela do mata-mata pedida ao método +0x3c e liberada): a posição (+0x72 do
# registro achado por 0x4a4850) de cada clube; a 0x4b01d0 pela devolução.
# Determinismo: as saídas da tabela das rodadas começam em 0 na cópia; nada lê pilha, heap ou endereço como valor.
SAVE = False
ROTINAS = [('campeoes_posicoes_previstas', 0x586900, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('grupo_da_fase', 0x4b01d0, [0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões: as posições previstas dos inscritos; o grupo de uma fase pelo id',
        'the Champions League: the entrants\' predicted positions; the group of a stage by id',
        [('campeoes_posicoes_previstas', 'EuropeanCup::rank_teams', '0x586900', 'european_cup.cpp', 'thiscall, ret 0',
          'os inscritos ordenados pela força e a posição prevista de cada um: o mata-mata da final para trás, a segunda e a primeira fase de grupos e as preliminares',
          'the entrants sorted by strength and each one\'s predicted position: the knockout from the final back, the second and first group phases and the qualifying rounds'),
         ('grupo_da_fase', 'comp_util_get_parent_stage_id', '0x4b01d0', 'comp_util.cpp', 'cdecl',
          'o grupo (a fase que contém) de uma fase ou rodada pelo id, pelas faixas fixas; −1 sem grupo',
          'the group (the containing stage) of a stage or round by id, by the fixed ranges; −1 without a group')])
CAMPOS = [('liga dos campeões +8 estatística (Comp_Stats), +0x14 inscritos (6 bytes: +0 clube), +0x36 quantos (word), +0x3c rodadas preliminares (word), +0xa3 tabela das preliminares (0x68 bytes, +0x1a confrontos); ponteiro e valor; lê (campeoes_posicoes_previstas)',
           'champions league +8 statistics (Comp_Stats), +0x14 entrants (6 bytes: +0 club), +0x36 count (word), +0x3c qualifying rounds (word), +0xa3 qualifying round table (0x68 bytes, +0x1a ties); pointer and value; reads (campeoes_posicoes_previstas)'),
          ('registro da estatística da competição +0x72 posição (byte); valor; grava pela 0x4a2540 (campeoes_posicoes_previstas)',
           'competition statistics record +0x72 position (byte); value; writes through 0x4a2540 (campeoes_posicoes_previstas)')]
INTERESSA = ['CMRTE']
