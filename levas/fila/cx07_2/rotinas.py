# O mata-mata e os codigos de progresso da Copa America (goldcup.cpp; leitor cx07, rodada 5, 24/11/2026). Simulacao com
# as 8 ligas: a Copa America do jogo e a de julho de 2001 (o mata-mata a 16 de julho; 1 chamada na cobertura das 8
# ligas ate 2002-06-01).
# - 0x5e18a0 GoldCup::create_knockout_stage thiscall() ret 0 (do advance_stage na fase 2, pelo endereco): a fase
#   temporaria das terceiras (operator new 0x944cf1 cdecl; League_Stage::League_Stage 0x68b100, thiscall, ret 0x58, 22
#   argumentos; o new nao e conferido), as linhas 2 dos grupos copiadas (0x3b bytes, +0x44 da linha zerado = o
#   historico de posicoes), League::sort_table 0x6827d0 (push 0 -> thiscall, ret 4), malloc 0x944e46 (cdecl),
#   League::get_table_row 0x6826d0 (push k -> thiscall, ret 4), as virtuais +0x68 (5 pushes) da copa e +0x94 (sem
#   argumentos) e +0x40 (5 pushes) de cada grupo, o destrutor escalar da temporaria (virtual 0 com 1),
#   comp_util_shuffle_pointer_array 0x4afb40 (cdecl, 2 pushes), a virtual +0x3c (4 pushes), Cup_Stage::Cup_Stage
#   0x522e00 (thiscall, ret 0x3c, 15 argumentos), free 0x9452ca (cdecl), Cup::set_up_round 0x51c800 (push 0 ->
#   thiscall, ret 4). Prologo push -1; push imm32 (7 bytes).
# - 0x5e2500 GoldCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual +0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (thiscall em [this+8]); as tabelas de salto 0x5e25b0 e 0x5e25c0 (1 a 4) e a
#   conta do melhor (1 a 3 -> 1, 4 -> 10); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8]
#   (7 bytes).
# Defeitos do original: (1) o new da fase temporaria nao e conferido (le a tabela dela mesmo nula); (2) sem memoria para
# o vetor das 8 (linha 0x30b) a fase temporaria nao e destruida; (3) o grupo passado ao anuncio e um dword cujo byte
# baixo e o grupo e os altos sao lixo da pilha (o anuncio le o byte).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Depende do cx04_5 (group_championship_progress, compare_group_championship_progress) e do cx05_3 (table_row_club):
# integrar depois deles. O nations_knockout_from e o check_nations_knockout daqui sao usados pelo cx07_4 e pelo cx07_6.
# Comparadores: o mata-mata so a original, com o plano (a fase em [+0xc][2] e +0x30 = 2); o progresso lado a lado nos
# 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('goldcup_create_knockout_stage', 0x5e18a0, [0x6a, 0xff, 0x68, 0xbc, 0xa8, 0x95, 0x00]),
           ('goldcup_set_club_progress', 0x5e2500, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o mata-mata e os códigos de progresso da Copa América',
        'the Copa América knockout and progress codes',
        [('goldcup_create_knockout_stage', 'GoldCup::create_knockout_stage', '0x5e18a0', 'goldcup.cpp', 'thiscall, ret 0',
          'monta as quartas de final da Copa América: os primeiros e segundos dos três grupos e as duas melhores terceiras (as terceiras ordenadas numa fase provisória pelos critérios do grupo); a pior terceira sai das estatísticas, cada grupo anuncia a terceira que passa (mais uma vaga no grupo) ou sai e a quarta que sai; as oito são embaralhadas e a primeira rodada é sorteada',
          'builds the Copa América quarter-finals: the winners and runners-up of the three groups and the two best thirds (ranked in a temporary stage by the group tie-breaks); the worst third leaves the statistics, each group announces its third going through (one more place in the group) or out and its fourth out; the eight are shuffled and the first round drawn'),
         ('goldcup_set_club_progress', 'GoldCup::set_club_progress', '0x5e2500', 'goldcup.cpp', 'thiscall, ret 0x14',
          'grava o progresso de uma seleção nas estatísticas da Copa América: nos grupos (fase abaixo de 2) a atual 1 e 2 viram 5, 3 vira 9 e 4 vira 10; a melhor 1 a 3 vira 1 e 4 vira 10; a pior como a atual; no mata-mata os códigos como vêm',
          "stores a nation's progress in the Copa América statistics: in the groups (stage below 2) current 1 and 2 become 5, 3 becomes 9 and 4 becomes 10; best 1 to 3 becomes 1 and 4 becomes 10; worst as current; in the knockout the codes as they come")])
CAMPOS = []
INTERESSA = []
