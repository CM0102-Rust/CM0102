# Os destrutores das ligas da Argentina, da Australia e da Belgica (arg_prm.cpp, arg_second.cpp, aus_nsl.cpp,
# bel_first.cpp, bel_second.cpp; leitor cx08, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_1/cx06_1:
# rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com
# uma carreira aberta que tenha a Argentina, a Australia ou a Belgica, carregar outro save, jogando comparando.
# - 0x4044b0 ArgPrm::~ArgPrm e 0x406e40 ArgSecond::~ArgSecond thiscall() ret 0: a vtable (0x967324, 0x9673d8),
#   League::free_table 0x687970 (push 0), free(+0xba) e zera, as fases 0..=+0x30 (slot 0 com 1), +0x30 = -1 e free(+0xc)
#   sem testar nem zerar, as estatisticas (delete_comp_stats do cx01_14), League::~League 0x682300. Diferente do molde
#   das copas (drop_stats_league do cx04_1): o +0x30 volta a -1 e o vetor nao e testado nem zerado.
# - 0x4110e0 AusNsl::~AusNsl e 0x41f280 BelSecond::~BelSecond: o molde drop_stats_league do cx04_1 (conferidos instrucao a
#   instrucao) com as vtables 0x967768 e 0x9679f4.
# - 0x41df80 BelFirst::~BelFirst: sem fases (a tabela, as datas, as estatisticas e League::~League).
# - 0x404490, 0x406e20, 0x4110c0, 0x41df60, 0x41f260: os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X;
#   operator delete 0x944c94 cdecl com o bit 0), thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o
#   prologo pelas RELATIVAS (0x404490, 3), (0x406e20, 3), (0x4110c0, 3), (0x41df60, 3), (0x41f260, 3) do
#   rust/src/detours.rs (ver INTEGRAR_ANTES.txt).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), Comp_Stats::~Comp_Stats 0x49f450 (mov ecx),
# os destrutores thiscall, ret 0. Prologo dos destrutores push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats), do cx03_3 (delete_through) e do cx04_1 (drop_stats_league): integrar depois.
SAVE = False
TELAS = True
ROTINAS = [('drop_arg_prm', 0x4044b0, [0x6a, 0xff, 0x68, 0x88, 0x35, 0x95, 0x00]),
           ('delete_arg_prm', 0x404490, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_arg_second', 0x406e40, [0x6a, 0xff, 0x68, 0x08, 0x36, 0x95, 0x00]),
           ('delete_arg_second', 0x406e20, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_aus_nsl', 0x4110e0, [0x6a, 0xff, 0x68, 0xf8, 0x39, 0x95, 0x00]),
           ('delete_aus_nsl', 0x4110c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_bel_first', 0x41df80, [0x6a, 0xff, 0x68, 0xc8, 0x3f, 0x95, 0x00]),
           ('delete_bel_first', 0x41df60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_bel_second', 0x41f280, [0x6a, 0xff, 0x68, 0x18, 0x40, 0x95, 0x00]),
           ('delete_bel_second', 0x41f260, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas da Argentina, da Austrália e da Bélgica',
        'the destructors of the Argentine, Australian and Belgian leagues',
        [('drop_arg_prm', 'ArgPrm::~ArgPrm', '0x4044b0', 'arg_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão argentina: libera a tabela, a tabela de rodadas, as fases até a atual (a fase atual volta a −1), o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Argentine Premier Division: frees the table, the round table, the stages up to the current one (the current stage back to −1), the stage vector and the statistics and runs the league destructor'),
         ('delete_arg_prm', 'ArgPrm::scalar_deleting_destructor', '0x404490', 'arg_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão argentina e, com o bit 0, libera o objeto',
          'runs the Argentine Premier Division destructor and, with bit 0, frees the object'),
         ('drop_arg_second', 'ArgSecond::~ArgSecond', '0x406e40', 'arg_second.cpp', 'thiscall, ret 0',
          'o mesmo para a Segunda Divisão argentina (Nacional B)',
          'the same for the Argentine Second Division (Nacional B)'),
         ('delete_arg_second', 'ArgSecond::scalar_deleting_destructor', '0x406e20', 'arg_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão argentina e, com o bit 0, libera o objeto',
          'runs the Argentine Second Division destructor and, with bit 0, frees the object'),
         ('drop_aus_nsl', 'AusNsl::~AusNsl', '0x4110e0', 'aus_nsl.cpp', 'thiscall, ret 0',
          'desmonta a liga australiana: libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Australian league: frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_aus_nsl', 'AusNsl::scalar_deleting_destructor', '0x4110c0', 'aus_nsl.cpp', 'thiscall, ret 4',
          'roda o destrutor da liga australiana e, com o bit 0, libera o objeto',
          'runs the Australian league destructor and, with bit 0, frees the object'),
         ('drop_bel_first', 'BelFirst::~BelFirst', '0x41df80', 'bel_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão belga (sem fases): libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Belgian First Division (no stages): frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_bel_first', 'BelFirst::scalar_deleting_destructor', '0x41df60', 'bel_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão belga e, com o bit 0, libera o objeto',
          'runs the Belgian First Division destructor and, with bit 0, frees the object'),
         ('drop_bel_second', 'BelSecond::~BelSecond', '0x41f280', 'bel_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão belga: libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Belgian Second Division: frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_bel_second', 'BelSecond::scalar_deleting_destructor', '0x41f260', 'bel_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão belga e, com o bit 0, libera o objeto',
          'runs the Belgian Second Division destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
