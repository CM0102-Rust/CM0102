# As fases de liga (league_stage.cpp; agente C da frente das competições, 06/11/2026). Na carreira de 8 ligas até
# 2002-06-01 (docs/cobertura_competicoes.txt): o construtor de League_Stage roda ~120 vezes (os turnos dos estaduais e da
# J1, cqr_1; os grupos da Série C, cqs_3; as conferências da MLS; as fases de grupos das copas e das eliminatórias) e o
# destrutor ~40 (nas viradas).
# - 0x68b100 League_Stage::League_Stage (964 bytes, 22 argumentos);
# - 0x68b6a0 League_Stage::~League_Stage (123 bytes; passageira, sem mutação: só libera memória).
# A 0x88c6d0 SubLeague::SubLeague (a minitabela, sub_league.cpp) saiu do item: portada e comparada sem divergência, mas a
# mutação (sem os jogos entre os clubes) escapou: as tabelas dos segundos colocados das eliminatórias de março de 2002
# só decidem depois de junho.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x68b100: 22 pushes; mov ecx, novo; call (ex. 0x401fb9, 0x405a94; e o turno da cqr_1) -> thiscall, ret 0x58;
# - 0x68b6a0: do destrutor que libera (0x68b4d0: push esi; mov esi, ecx; call) -> thiscall, ret 0;
# - 0x682200 (League::League) e 0x682300 (League::~League): mov ecx, esi/ebp; call -> thiscall, ret 0;
# - 0x944e46 (malloc), 0x9452ca (free): push; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push i (dword com i no byte de baixo e sobra da pilha
#   em cima; a rotina lê o byte: a cópia passa i limpo); mov ecx, this -> thiscall, ret 0x10;
# - 0x6835c0 (create_fixtures): mov ecx, ebp -> thiscall, ret 0; 0x6827d0 (sort_table): push 0 -> thiscall, ret 4;
# - 0x687970 (League::free_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original nas duas (alocam ou liberam, e a fase sorteia os jogos), com o plano: na fase, os campos
# gravados dos argumentos que nenhuma rotina de dentro reescreve e os clubes da tabela como conjunto (a ordem os
# reordena); no destrutor, o +0x18 zerado.
# Determinismo: só os argumentos, as datas do jogo e o sorteio dos jogos (0x6835c0).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_league_stage', 0x68b100, [0x6a, 0xff, 0x68, 0xeb, 0xca, 0x95, 0x00]),
           ('destroy_league_stage', 0x68b6a0, [0x6a, 0xff, 0x68, 0x28, 0xcb, 0x95, 0x00])]
DOCS = ('as fases de liga: o construtor e o destrutor de League_Stage',
        'league stages: the League_Stage constructor and destructor',
        [('construct_league_stage', 'League_Stage::League_Stage', '0x68b100', 'league_stage.cpp', 'thiscall, ret 0x58',
          'Uma fase de pontos corridos dentro de uma competição (um turno de estadual ou da J1, um grupo da Série C, uma conferência da MLS, um grupo de copa ou de eliminatória): os clubes, os turnos, os pontos por vitória e por empate, a categoria dos jogos, os critérios de desempate e as vagas da fase vêm de quem a cria; a tabela recebe os clubes na ordem dada; com a tabela de rodadas, os jogos são sorteados; sem ela, a fase tem uma rodada só, hoje; por fim a tabela é ordenada',
          'A round-robin stage inside a competition (a state league or J1 half, a third division group, an MLS conference, a cup or qualifier group): the clubs, the rounds, the points for a win and a draw, the match category, the tie-breakers and the stage places come from the creator; the table gets the clubs in the given order; with the round table, the fixtures are drawn; without it, the stage has a single round, today; finally the table is sorted'),
         ('destroy_league_stage', 'League_Stage::~League_Stage', '0x68b6a0', 'league_stage.cpp', 'thiscall, ret 0',
          'O fim de uma fase de pontos corridos (na virada): a tabela, as rodadas e o vetor +0x18 liberados, e o fim da liga',
          'The end of a round-robin stage (at the turnover): the table, the rounds and the +0x18 array freed, and the league teardown')])
CAMPOS = [('fase +0xc2 / +0xc3: os pontos por vitória / por empate; valor; grava', 'stage +0xc2 / +0xc3: the points for a win / a draw; value; writes'),
          ('fase +0xc5..+0xc8: os critérios de desempate; valores; grava', 'stage +0xc5..+0xc8: the tie-breakers; values; writes'),
          ('fase +0xbe..+0xc1: as vagas da fase; valores; grava', 'stage +0xbe..+0xc1: the stage places; values; writes'),
          ('fase +0xba: as rodadas (registros de 0x41 bytes); ponteiro; grava', 'stage +0xba: the rounds (0x41-byte records); pointer; writes'),
          ('fase +0xb1: a tabela (linhas de 0x3b bytes); ponteiro; grava', 'stage +0xb1: the table (0x3b-byte rows); pointer; writes')]
INTERESSA = ['CMRTE']
