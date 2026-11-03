# A Copa do Mundo: a tabela do grupo 1, as fases dos grupos 2 a 8 e as posições previstas (world_cup.cpp), todas do
# sorteio dos grupos (0x92e7f0, uma vez na carreira automática até 2002-06-01, em 27/12/2001):
# - 0x92d0f0 (WorldCup::create_group_table, 264 bytes, sem SEH): a própria Copa vira o grupo 1 (4 times, a tabela
#   [+0xb1] de 0xec bytes) com os times do grupo 1 da lista, na ordem (0x687430), e a notícia dos grupos (0x684230).
# - 0x92d540 (WorldCup::create_group_stage, 560 bytes, quadro SEH só para o new, sem destrutor no caminho normal): a fase
#   do grupo g+2 (League_Stage, 0x944cf1(0xee) + 0x68b100 com 22 argumentos) com os times dele, a tabela das rodadas do
#   método +0x3c (0x92c290), em [+0xc][g]; +0x30 = g e a notícia do grupo.
# - 0x92f680 (WorldCup::set_predicted_positions, 864 bytes, método +0x5c): as 32 nações ordenadas pela reputação
#   (qsort 0x945333 com 0x4b07d0), a posição prevista de cada uma na estatística (0x4a2540: 1, 2, 3, 4, 5 ×4, 9 ×8,
#   17 ×8, 25 ×8) e a Copa como competição atual da nação de cada clube (0x775220).
# Iguais no cm0102-1080.exe (as três inteiras). Chamam originais não portadas: 0x944e46 (malloc), 0x944cf1 (new),
# 0x9452ca (free), 0x68b100 (o construtor da fase), a tabela das rodadas (0x92c290, método +0x3c) e a caixa; o resto já
# em Rust (0x687430, 0x684230, 0x945333, 0x4b07d0, 0x4a2540, 0x775220).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x92d0f0: 0x92e804 (push lista; mov ecx, esi) -> thiscall, ret 4; devolve 1 ou 0 no eax.
# - quem chama a 0x92d540: 0x92e901 (push lista; push g; mov ecx, esi) -> thiscall, ret 8; devolve 1 ou 0.
# - quem chama a 0x92f680: 0x92e919 (o método +0x5c da vtable 0x9712f0, mov ecx, esi; sem push) -> thiscall, ret 0.
# - dentro da 0x92d0f0: 0x944e46 push 0xec; add esp, 4 -> cdecl; 0x687430 push 0; push 0; push clube; push linha; mov
#   ecx, esi -> thiscall, ret 0x10 (a linha no cl, com lixo nos bytes altos do ecx que a 0x687430 não lê: ela usa o byte;
#   a cópia passa o byte limpo); 0x684230 mov ecx, esi -> thiscall, ret 0.
# - dentro da 0x92d540: 0x944e46 push (+0x3e << 2); add esp -> cdecl; o método +0x3c push 0; push &id; push &rodadas;
#   push g; mov ecx, esi -> thiscall, ret 0x10 (a 0x92c290 grava as duas saídas nas fases 0..6); 0x944cf1 push 0xee; add
#   esp, 4 -> cdecl; 0x68b100 (22 push; mov ecx, novo) -> thiscall, ret 0x58: os words +0xd9, as rodadas, o id, o ano e
#   os bytes +0xc2..+0xc4, +0x51 e g vão com lixo nos bytes altos (o cx e o dx de antes, o resto da pilha nas duas
#   saídas); a leitura do construtor (mov ax/cx/dx, word ptr [esp + 0x22c..0x274]; mov cl/dl/al, byte ptr) não os usa:
#   a cópia passa os valores limpos; 0x9452ca push; add esp -> cdecl (duas vezes); 0x684230 mov ecx, fase -> thiscall.
# - dentro da 0x92f680: 0x944e46 push; add esp -> cdecl; 0x945333 push 0x4b07d0; push 4; push n; push vetor; add esp,
#   0x10 -> cdecl; 0x4a2540 push posição; push clube; mov ecx, [+8] -> thiscall, ret 8; 0x775220 push [+4]; push [clube
#   +0x53]; mov ecx, [0xb74318] -> thiscall, ret 8; 0x9452ca push; add esp -> cdecl.
# Comparadores: as três só a original (alocam, mandam notícias, acrescentam registros à estatística e avisam o
# gerenciador dos técnicos), conferindo o plano feito antes com a lista: as linhas do grupo 1 com os clubes dele, na
# ordem; a fase do grupo g+2 em [+0xc][g] com +0x30 = g, os times e o ano da Copa e os clubes do grupo nas linhas (em
# qualquer ordem: as linhas da fase não seguem a ordem da lista); e
# a posição prevista (+0x72 do registro achado por 0x4a4850) das 32 nações na ordem da mesma qsort feita numa cópia.
# Determinismo: as duas saídas da tabela das rodadas começam em 0 na cópia (a 0x92c290 as grava sempre nas fases 0..6;
# na original são sobra de pilha até lá); nada lê pilha, heap ou endereço como valor.
SAVE = False
ROTINAS = [('mundial_tabela_do_grupo_um', 0x92d0f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('mundial_fase_do_grupo', 0x92d540, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('mundial_posicoes_previstas', 0x92f680, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa do Mundo: a tabela do grupo 1, as fases dos grupos 2 a 8 e as posições previstas',
        'the World Cup: the group 1 table, the group 2 to 8 stages and the predicted positions',
        [('mundial_tabela_do_grupo_um', 'WorldCup::create_group_table', '0x92d0f0', 'world_cup.cpp', 'thiscall, ret 4',
          'a própria Copa vira o grupo 1: quatro times, a tabela alocada, os times do grupo 1 da lista nas linhas e a notícia dos grupos',
          'the World Cup itself becomes group 1: four teams, the allocated table, the group 1 teams of the list in the rows and the group draw news'),
         ('mundial_fase_do_grupo', 'WorldCup::create_group_stage', '0x92d540', 'world_cup.cpp', 'thiscall, ret 8',
          'a fase de um dos grupos 2 a 8: os times dele, a tabela das rodadas, o construtor da fase, a fase atual e a notícia do grupo',
          'the stage of one of groups 2 to 8: its teams, the round schedule, the stage constructor, the current stage and the group news'),
         ('mundial_posicoes_previstas', 'WorldCup::set_predicted_positions', '0x92f680', 'world_cup.cpp', 'thiscall, ret 0',
          'as 32 nações ordenadas pela reputação, a posição prevista de cada uma (1, 2, 3, 4, 5, 9, 17, 25 pelas faixas das fases) e a Copa como competição atual da nação',
          'the 32 nations sorted by reputation, each one\'s predicted position (1, 2, 3, 4, 5, 9, 17, 25 by the stage bands) and the World Cup as the nation\'s current competition')])
CAMPOS = [('copa do mundo +4 id da competição (repassado), +8 estatística (Comp_Stats), +0xc as fases, +0x14 tabela das nações (6 bytes: +0 clube), +0x30 fase atual, +0x36 quantas nações (word), +0x3e times (word), +0x40 ano, +0x51, +0xb1 tabela do grupo 1 (linhas de 0x3b bytes, +0 clube), +0xbe/+0xc2..+0xc5 regras repassadas à fase, +0xd9 (word, repassado); valor e ponteiro; lê e grava (mundial_tabela_do_grupo_um, mundial_fase_do_grupo, mundial_posicoes_previstas)',
           'world cup +4 competition id (passed on), +8 statistics (Comp_Stats), +0xc the stages, +0x14 nation table (6 bytes: +0 club), +0x30 current stage, +0x36 nation count (word), +0x3e teams (word), +0x40 year, +0x51, +0xb1 group 1 table (0x3b-byte rows, +0 club), +0xbe/+0xc2..+0xc5 rules passed to the stage, +0xd9 (word, passed on); value and pointer; reads and writes (mundial_tabela_do_grupo_um, mundial_fase_do_grupo, mundial_posicoes_previstas)'),
          ('clube +0x53 nação; ponteiro; lê (mundial_posicoes_previstas)', 'club +0x53 nation; pointer; reads (mundial_posicoes_previstas)'),
          ('registro da estatística da competição +0x72 posição (byte); valor; grava pela 0x4a2540 (mundial_posicoes_previstas)',
           'competition statistics record +0x72 position (byte); value; writes through 0x4a2540 (mundial_posicoes_previstas)')]
INTERESSA = ['CMRTE']
