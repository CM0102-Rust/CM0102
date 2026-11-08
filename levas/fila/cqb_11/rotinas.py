# Os sorteios da copa (Cup.cpp):
# - 0x51d2d0 (Cup::sort_round_teams): o sorteio pela semente (regra 0x400 da rodada), pelo sorteio da rodada 0x51c410
#   (copa_sortear_rodada, cqa_3, ramo "cqa_3 sorteio pela semente"); 5 chamadas na carreira automatica ate 2002-06-01
#   (docs/cobertura_simulacao.txt, 17/10). Os confrontos sorteados sao a tabela da copa (quem joga com quem e onde).
# - 0x51d9f0 (Cup::check_round_teams_for_nation): o sorteio que separa as nacoes (regra 4 sem 0x400), pela mesma 0x51c410;
#   3 chamadas na mesma carreira.
# - 0x518780 (Cup::start_new_season, a da classe base): so zera +0xb1 (passageira; vtable, nao roda ate 2002-06-01).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x51d2d0 e 0x51d9f0 thiscall, 1 arg (push ebp = rodada; mov ecx, esi = a copa em 0x51c4dd/0x51c5db),
#   ret 4; 0x518780 thiscall (ecx = a copa: mov byte [ecx + 0xb1], 0), 0 args, ret.
#   0x944e46 malloc: cdecl (push n; add esp, 4 em 0x51d319 e 0x51daf0), 1 arg. 0x945501 calloc: cdecl (push 1; push
#   [0xae2350]; add esp, 8 em 0x51da13), 2 args. 0x9452ca free: cdecl, 1 arg (add esp, 4 em 0x51d9d1; em 0x51ddcb o
#   add esp, 0x14 cobre o free e o qsort). 0x945333 qsort: cdecl (4 pushes: comparacao 0x51ebc0, 7, n, vetor; add esp,
#   0x10 em 0x51d54b). 0x90cfc0 sorteio: cdecl (push n; add esp, 4 em 0x51d590/0x51d644/0x51de40/0x51dfd5), 1 arg.
#   As caixas: 0x944cff, 0x90d130, 0x944c9f, 0x66f4e0, 0x5e8290, 0x5cc7d0, 0x5a8f60, 0x945934 pelo cpq11_sem_memoria e
#   pelo aviso_em (cdecl, ja no motor.rs).
# Comparadores: as duas lado a lado pelo estado do sorteador (STATE_REGIONS) e pelos confrontos de todas as rodadas
# da copa (cpq11_confrontos, com 0xb67a34), a copia em without_counting (os vetores sao temporarios: malloc e free nas duas);
# a 0x518780 lado a lado no byte +0xb1. Sem SEH. Iguais no cm0102-1080.exe (as tres inteiras).
SAVE = False
ROTINAS = [('copa_sortear_pela_semente', 0x51d2d0, [0x81, 0xec, 0x2c, 0x02, 0x00, 0x00]),
           ('copa_sortear_separando_nacoes', 0x51d9f0, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('copa_comeco_da_temporada', 0x518780, [0xc6, 0x81, 0xb1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os sorteios da copa: pela semente e separando as nações, e o começo da temporada da copa base',
        'the cup draws: by seeding and keeping nations apart, and the base cup season start',
        [('copa_sortear_pela_semente', 'Cup::sort_round_teams', '0x51d2d0', 'Cup.cpp', 'thiscall',
          'o sorteio da rodada pela semente: os times saem dos confrontos, ordenados pela semente (a maior primeiro), e cada um vai a um confronto sorteado; na primeira volta só confrontos vazios (com as duas vagas livres, o lado sorteado), depois qualquer vaga livre, com a regra 4 separando os clubes da mesma nação até não haver jeito',
          'the round draw by seeding: the teams leave the ties, sorted by seed (highest first), and each goes to a drawn tie; on the first pass only empty ties (with both slots free, the side is drawn), then any free slot, with rule 4 keeping clubs of one nation apart until there is no way'),
         ('copa_sortear_separando_nacoes', 'Cup::check_round_teams_for_nation', '0x51d9f0', 'Cup.cpp', 'thiscall',
          'o sorteio da rodada que separa as nações: o nível de cada time vira quantos times da nação dele há na rodada (as nações maiores saem primeiro) e, da segunda volta em diante, um time não cai num confronto com outro da mesma nação, enquanto der',
          "the round draw that keeps nations apart: each team's level becomes how many teams of its nation are in the round (the bigger nations go first) and, from the second pass on, a team does not land in a tie with another of its nation while possible"),
         ('copa_comeco_da_temporada', 'Cup::start_new_season', '0x518780', 'Cup.cpp', 'thiscall',
          'o começo da temporada da copa base: zera o byte +0xb1',
          'the base cup season start: clears byte +0xb1')])
CAMPOS = [('copa +0xa3 rodadas (0x68 bytes: +0xb regra 4, +0xd regra 0x400, +0x18 vagas, +0x1a confrontos); ponteiro/valores; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)',
           'cup +0xa3 rounds (0x68 bytes: +0xb rule 4, +0xd rule 0x400, +0x18 slots, +0x1a ties); pointer/values; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)'),
          ('copa +0xad confrontos por rodada (0x35 bytes: +0/+2 times, +0xe/+0x10 e +0xa/+0xc vínculos, +6/+8/+0x12 o confronto anterior); valores; lê e grava (copa_sortear_pela_semente, copa_sortear_separando_nacoes)',
           'cup +0xad ties per round (0x35 bytes: +0/+2 teams, +0xe/+0x10 and +0xa/+0xc links, +6/+8/+0x12 the previous tie); values; reads and writes (copa_sortear_pela_semente, copa_sortear_separando_nacoes)'),
          ('copa +0xa7 times (6 bytes: +0 clube, +4 semente); ponteiro/valor; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)',
           'cup +0xa7 teams (6 bytes: +0 club, +4 seed); pointer/value; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)'),
          ('clube +0x53 nação (+0 id); ponteiro; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)',
           'club +0x53 nation (+0 id); pointer; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)'),
          ('[0xae2350] quantas nações; valor; lê (copa_sortear_separando_nacoes)', '[0xae2350] nation count; value; reads (copa_sortear_separando_nacoes)'),
          ('copa +0xb1; valor; grava 0 (copa_comeco_da_temporada)', 'cup +0xb1; value; writes 0 (copa_comeco_da_temporada)')]
INTERESSA = ['GPF2']
