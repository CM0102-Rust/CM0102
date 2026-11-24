# Os destrutores (swe_second.cpp, tur_first.cpp, tur_second.cpp, tur_second_b.cpp; leitor cx14, rodada 5, 24/11/2026). Item de TELAS, como os
# destrutores do cx13_1: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da
# carreira). Caminho no jogo: com uma carreira aberta que tenha as ligas do item carregadas, carregar outro save,
# jogando comparando.
# - 0x8919a0 SweSecond::~SweSecond (vtable 0x9702a0): drop_stats_league (cx04_1), conferido instrucao a instrucao.
# - 0x8fd190 TurFirst::~TurFirst (vtable 0x97052c): drop_plain_league (cx10_1), conferido instrucao a instrucao.
# - 0x8fe920 TurSecond::~TurSecond (vtable 0x9705e0): drop_plain_league (cx10_1), conferido instrucao a instrucao.
# - 0x8ff720 TurSecondB::~TurSecondB (vtable 0x970694): drop_stats_league (cx04_1), conferido instrucao a instrucao.
# - os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0),
#   thiscall(flags) ret 4 -> this: delete_through (cx03_3), prologo pelas RELATIVAS (endereco, 3).
# Prologo dos destrutores: push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a
# original (liberam memoria).
# A Segunda Divisao B espanhola e a Segunda Divisao sueca so existem com a base NORMAL ou maior.
SAVE = False
TELAS = True
ROTINAS = [('drop_swe_second', 0x8919a0, [0x6a, 0xff, 0x68, 0xd8, 0x31, 0x96, 0x00]),
           ('delete_swe_second', 0x891980, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_tur_first', 0x8fd190, [0x6a, 0xff, 0x68, 0xe8, 0x4f, 0x96, 0x00]),
           ('delete_tur_first', 0x8fd170, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_tur_second', 0x8fe920, [0x6a, 0xff, 0x68, 0x38, 0x50, 0x96, 0x00]),
           ('delete_tur_second', 0x8fe900, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_tur_second_b', 0x8ff720, [0x6a, 0xff, 0x68, 0x88, 0x50, 0x96, 0x00]),
           ('delete_tur_second_b', 0x8ff700, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da Segunda Divisão sueca, da Primeira Divisão turca, da Segunda Divisão turca, da Segunda Divisão turca categoria B',
        'the destructors of the Swedish Second Division, the Turkish First Division, the Turkish Second Division, the Turkish Second Division category B',
        [('drop_swe_second', 'SweSecond::~SweSecond', '0x8919a0', 'swe_second.cpp', 'thiscall, ret 0', 'desmonta a Segunda Divisão sueca: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga', 'destroys the Swedish Second Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_swe_second', 'SweSecond::scalar_deleting_destructor', '0x891980', 'swe_second.cpp', 'thiscall, ret 4', 'roda o destrutor da Segunda Divisão sueca e, com o bit 0, libera o objeto', 'runs the Swedish Second Division destructor and, with bit 0, frees the object'),
         ('drop_tur_first', 'TurFirst::~TurFirst', '0x8fd190', 'tur_first.cpp', 'thiscall, ret 0', 'desmonta a Primeira Divisão turca: uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga', 'destroys the Turkish First Division: a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_tur_first', 'TurFirst::scalar_deleting_destructor', '0x8fd170', 'tur_first.cpp', 'thiscall, ret 4', 'roda o destrutor da Primeira Divisão turca e, com o bit 0, libera o objeto', 'runs the Turkish First Division destructor and, with bit 0, frees the object'),
         ('drop_tur_second', 'TurSecond::~TurSecond', '0x8fe920', 'tur_second.cpp', 'thiscall, ret 0', 'desmonta a Segunda Divisão turca (categoria A): uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga', 'destroys the Turkish Second Division (category A): a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_tur_second', 'TurSecond::scalar_deleting_destructor', '0x8fe900', 'tur_second.cpp', 'thiscall, ret 4', 'roda o destrutor da Segunda Divisão turca (categoria A) e, com o bit 0, libera o objeto', 'runs the Turkish Second Division (category A) destructor and, with bit 0, frees the object'),
         ('drop_tur_second_b', 'TurSecondB::~TurSecondB', '0x8ff720', 'tur_second_b.cpp', 'thiscall, ret 0', 'desmonta a Segunda Divisão turca categoria B: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga', 'destroys the Turkish Second Division category B: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_tur_second_b', 'TurSecondB::scalar_deleting_destructor', '0x8ff700', 'tur_second_b.cpp', 'thiscall, ret 4', 'roda o destrutor da Segunda Divisão turca categoria B e, com o bit 0, libera o objeto', 'runs the Turkish Second Division category B destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
