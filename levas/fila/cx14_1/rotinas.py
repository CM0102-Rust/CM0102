# Os destrutores (spa_second_b.cpp, sub_league.cpp, swe_first.cpp, swe_prm.cpp; leitor cx14, rodada 5, 24/11/2026). Item de TELAS, como os
# destrutores do cx13_1: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da
# carreira). Caminho no jogo: com uma carreira aberta que tenha as ligas do item carregadas, carregar outro save,
# jogando comparando.
# - 0x8561a0 SpaSecondB::~SpaSecondB (vtable 0x96fc84): drop_stats_league (cx04_1), conferido instrucao a instrucao.
# - 0x88c990 SubLeague::~SubLeague (vtable 0x96ff90): so a vtable, League::free_table 0x687970 (push 0) e League::~League 0x682300.
# - 0x88e820 SweFirst::~SweFirst (vtable 0x9700e4): o corpo do drop_stats_league, mas o vetor das fases e liberado sem teste e sem zerar.
# - 0x88fe20 SwePrm::~SwePrm (vtable 0x970198): drop_plain_league (cx10_1), conferido instrucao a instrucao.
# - os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0),
#   thiscall(flags) ret 4 -> this: delete_through (cx03_3), prologo pelas RELATIVAS (endereco, 3).
# Prologo dos destrutores: push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a
# original (liberam memoria).
# A Segunda Divisao B espanhola e a Segunda Divisao sueca so existem com a base NORMAL ou maior.
SAVE = False
TELAS = True
ROTINAS = [('drop_spa_second_b', 0x8561a0, [0x6a, 0xff, 0x68, 0xc8, 0x27, 0x96, 0x00]),
           ('delete_spa_second_b', 0x856180, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_sub_league', 0x88c990, [0x6a, 0xff, 0x68, 0x18, 0x30, 0x96, 0x00]),
           ('delete_sub_league', 0x88c970, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_swe_first', 0x88e820, [0x6a, 0xff, 0x68, 0x18, 0x31, 0x96, 0x00]),
           ('delete_swe_first', 0x88e800, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_swe_prm', 0x88fe20, [0x6a, 0xff, 0x68, 0x88, 0x31, 0x96, 0x00]),
           ('delete_swe_prm', 0x88fe00, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da Segunda Divisão B espanhola, da subliga, da Superettan sueca, da Allsvenskan sueca',
        'the destructors of the Spanish Second Division B, the sub league, the Swedish Superettan, the Swedish Allsvenskan',
        [('drop_spa_second_b', 'SpaSecondB::~SpaSecondB', '0x8561a0', 'spa_second_b.cpp', 'thiscall, ret 0', 'desmonta a Segunda Divisão B espanhola: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga', 'destroys the Spanish Second Division B: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_spa_second_b', 'SpaSecondB::scalar_deleting_destructor', '0x856180', 'spa_second_b.cpp', 'thiscall, ret 4', 'roda o destrutor da Segunda Divisão B espanhola e, com o bit 0, libera o objeto', 'runs the Spanish Second Division B destructor and, with bit 0, frees the object'),
         ('drop_sub_league', 'SubLeague::~SubLeague', '0x88c990', 'sub_league.cpp', 'thiscall, ret 0', 'desmonta a subliga (a tabela de um grupo de clubes): libera a tabela e roda o destrutor da liga (sem tabela de rodadas nem estatísticas)', 'destroys the sub league (the table of a group of clubs): frees the table and runs the league destructor (no round table nor statistics)'),
         ('delete_sub_league', 'SubLeague::scalar_deleting_destructor', '0x88c970', 'sub_league.cpp', 'thiscall, ret 4', 'roda o destrutor da subliga (a tabela de um grupo de clubes) e, com o bit 0, libera o objeto', 'runs the sub league (the table of a group of clubs) destructor and, with bit 0, frees the object'),
         ('drop_swe_first', 'SweFirst::~SweFirst', '0x88e820', 'swe_first.cpp', 'thiscall, ret 0', 'desmonta a Superettan sueca: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases (sem conferir nem zerar o ponteiro) e as estatísticas e roda o destrutor da liga', 'destroys the Swedish Superettan: frees the table, the round table, every stage up to the current one, the stage vector (the pointer neither checked nor cleared) and the statistics and runs the league destructor'),
         ('delete_swe_first', 'SweFirst::scalar_deleting_destructor', '0x88e800', 'swe_first.cpp', 'thiscall, ret 4', 'roda o destrutor da Superettan sueca e, com o bit 0, libera o objeto', 'runs the Swedish Superettan destructor and, with bit 0, frees the object'),
         ('drop_swe_prm', 'SwePrm::~SwePrm', '0x88fe20', 'swe_prm.cpp', 'thiscall, ret 0', 'desmonta a Allsvenskan sueca: uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga', 'destroys the Swedish Allsvenskan: a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_swe_prm', 'SwePrm::scalar_deleting_destructor', '0x88fe00', 'swe_prm.cpp', 'thiscall, ret 4', 'roda o destrutor da Allsvenskan sueca e, com o bit 0, libera o objeto', 'runs the Swedish Allsvenskan destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
