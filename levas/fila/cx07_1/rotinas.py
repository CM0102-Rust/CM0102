# Os destrutores da Copa America, da Copa das Nacoes da Oceania, do futebol olimpico e da base League, as tres virtuais
# de base da League e a copia de uma liga para as telas (goldcup.cpp, oceania_nations.cpp, olympics.cpp, league.cpp;
# leitor cx07, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e desmontado (carregar outro save
# com um jogo aberto, ou sair da carreira); League::~League roda tambem na carreira, quando uma fase de liga
# (League_Stage) e apagada; a copia de exibicao roda nas telas de competicao. Caminho no jogo: com uma carreira aberta
# (as tres selecoes em torneio sempre existem), carregar outro save, jogando comparando; a copia, abrindo a tela de uma
# liga (Competicoes > uma liga > tabela).
# - 0x5e0840 GoldCup::~GoldCup, 0x798aa0 OceaniaNations::~OceaniaNations, 0x79f710 Olympics::~Olympics thiscall() ret 0:
#   o molde das selecoes em torneio (drop_nations_tournament do cx05_1) com a vtable de cada (0x96b400, 0x96e0cc,
#   0x96e1d0); conferidos instrucao a instrucao contra o da Copa Africana. Prologo push -1; push imm32 (7 bytes).
# - 0x5e0820, 0x798a80, 0x79f6f0, 0x6822e0 destrutores escalares thiscall(flags) ret 4 -> this (push esi; mov esi, ecx;
#   call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0): RELATIVAS (endereco, 3), ver INTEGRAR_ANTES.txt;
#   delete_through do cx03_3.
# - 0x682300 League::~League thiscall() ret 0: a vtable 0x96d020; com a lista de jogos fixos (+0xea) o vector
#   destructor iterator do CRT 0x9462bb (cdecl, 4 pushes: o bloco, 0x10, a contagem do dword anterior, 0x5a53c0, um
#   corpo vazio: a copia nao o chama) e o operator delete do bloco - 4; Comp::~Comp 0x48c650 (mov ecx -> thiscall, ret 0,
#   pelo desvio do cx01_14). Prologo push -1; push imm32.
# - 0x682650 League::create_schedule thiscall(4 args) ret 0x10 -> 0, 0x6826f0 League::set_up_rules thiscall() ret 0 -> -1,
#   0x682760 League::start_new_season thiscall() ret 0: a caixa do league.cpp (linhas 0xb8, 0xcf, 0xe1; show_error_at).
#   Toda liga derivada substitui as tres: nao ha caminho no jogo (so rodariam numa liga sem a sua). Prologo sub esp,
#   0x200 (6 bytes).
# - 0x682380 League::copy_for_display thiscall(destino, origem) ret 8 -> destino: rep movsd 0xee bytes da origem; malloc
#   0x944e46 (cdecl) e a copia das datas das rodadas; League::copy_table 0x687710 (mov ecx, origem -> thiscall, ret 0);
#   zera +0x18, +0xea, +0x14, +0x38, +0x36; copia campo a campo +4..+0xee para o destino com a vtable 0x9689e8 e depois
#   0x96d020. Prologo push ecx; push ebx; push ebp; mov ebp, [esp+0x14] (7 bytes).
# Defeito do original: League::copy_for_display nao confere o malloc das datas (copia para o nulo sem memoria).
# Iguais no cm0102-1080.exe (todas inteiras).
# Depende do cx03_3 (delete_through), do cx05_1 (drop_nations_tournament, que depende do cx01_14) e do cx07_9
# (LEAGUE_VTABLE, LEAGUE_OBJECT_SIZE): integrar depois deles.
# Comparadores: so a original nos destrutores e nas virtuais de base; a copia de exibicao so a original, com o plano.
SAVE = False
TELAS = True
ROTINAS = [('drop_goldcup', 0x5e0840, [0x6a, 0xff, 0x68, 0x48, 0xa8, 0x95, 0x00]),
           ('delete_goldcup', 0x5e0820, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_oceania_nations', 0x798aa0, [0x6a, 0xff, 0x68, 0x98, 0xeb, 0x95, 0x00]),
           ('delete_oceania_nations', 0x798a80, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_olympics', 0x79f710, [0x6a, 0xff, 0x68, 0xd8, 0xef, 0x95, 0x00]),
           ('delete_olympics', 0x79f6f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_league', 0x682300, [0x6a, 0xff, 0x68, 0x54, 0xc9, 0x95, 0x00]),
           ('delete_league', 0x6822e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('league_create_schedule', 0x682650, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('league_set_up_rules', 0x6826f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('league_start_new_season', 0x682760, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('league_copy_for_display', 0x682380, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14])]
DOCS = ('os destrutores da Copa América, da Copa da Oceania, do futebol olímpico e da liga, as virtuais de base da liga e a cópia de exibição',
        'the destructors of the Copa América, the Oceania Nations Cup, the Olympic football and the league, the league base virtuals and the display copy',
        [('drop_goldcup', 'GoldCup::~GoldCup', '0x5e0840', 'goldcup.cpp', 'thiscall, ret 0',
          'desmonta a Copa América: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the Copa América: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('delete_goldcup', 'GoldCup::scalar_deleting_destructor', '0x5e0820', 'goldcup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa América e, com o bit 0 das opções, libera o objeto',
          'runs the Copa América destructor and, with bit 0 of the flags, frees the object'),
         ('drop_oceania_nations', 'OceaniaNations::~OceaniaNations', '0x798aa0', 'oceania_nations.cpp', 'thiscall, ret 0',
          'desmonta a Copa das Nações da Oceania: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the Oceania Nations Cup: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('delete_oceania_nations', 'OceaniaNations::scalar_deleting_destructor', '0x798a80', 'oceania_nations.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Oceania e, com o bit 0 das opções, libera o objeto',
          'runs the Oceania Nations Cup destructor and, with bit 0 of the flags, frees the object'),
         ('drop_olympics', 'Olympics::~Olympics', '0x79f710', 'olympics.cpp', 'thiscall, ret 0',
          'desmonta o futebol olímpico: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the Olympic football: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('delete_olympics', 'Olympics::scalar_deleting_destructor', '0x79f6f0', 'olympics.cpp', 'thiscall, ret 4',
          'roda o destrutor do futebol olímpico e, com o bit 0 das opções, libera o objeto',
          'runs the Olympic football destructor and, with bit 0 of the flags, frees the object'),
         ('drop_league', 'League::~League', '0x682300', 'league.cpp', 'thiscall, ret 0',
          'desmonta a base da liga: libera a lista de jogos fixos e roda o destrutor da competição',
          'destroys the league base: frees the fixed fixture list and runs the competition destructor'),
         ('delete_league', 'League::scalar_deleting_destructor', '0x6822e0', 'league.cpp', 'thiscall, ret 4',
          'roda o destrutor da liga e, com o bit 0 das opções, libera o objeto',
          'runs the league destructor and, with bit 0 of the flags, frees the object'),
         ('league_create_schedule', 'League::create_schedule', '0x682650', 'league.cpp', 'thiscall, ret 0x10',
          'versão de base do calendário de uma fase, que toda liga substitui: a caixa de erro e nulo',
          'base version of a stage\'s round table, which every league overrides: the error box and null'),
         ('league_set_up_rules', 'League::set_up_rules', '0x6826f0', 'league.cpp', 'thiscall, ret 0',
          'versão de base das regras de uma liga, que toda liga substitui: a caixa de erro e −1',
          'base version of a league\'s rules, which every league overrides: the error box and −1'),
         ('league_start_new_season', 'League::start_new_season', '0x682760', 'league.cpp', 'thiscall, ret 0',
          'versão de base da virada de uma liga, que toda liga substitui: a caixa de erro',
          'base version of a league\'s new season, which every league overrides: the error box'),
         ('league_copy_for_display', 'League::copy_for_display', '0x682380', 'league.cpp', 'thiscall, ret 8',
          'faz deste objeto uma cópia particular da liga de origem para uma tela (as datas das rodadas e a tabela copiadas; sem inscritos, jogos fixos nem tabela de médias) e copia-o para o destino',
          'makes this object a private copy of the source league for a screen (round dates and table copied; no entrants, fixed fixtures or points-average table) and copies it into the destination')])
CAMPOS = []
INTERESSA = []
