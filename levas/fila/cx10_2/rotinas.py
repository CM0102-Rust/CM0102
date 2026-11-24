# Os destrutores das ligas francesas e alemãs (fra_second.cpp, fra_third.cpp, ger_first.cpp, ger_regional.cpp, ger_second.cpp; leitor cx10, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do
# cx08_1: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no
# jogo: com uma carreira aberta que tenha essas ligas carregadas, carregar outro save, jogando comparando.
# - 0x5be560 FraSecond::~FraSecond thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96ac2c; 0x5be540 o destrutor escalar.
# - 0x5bfd70 FraThird::~FraThird thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96ad80; 0x5bfd50 o destrutor escalar.
# - 0x5d9be0 GerFirst::~GerFirst thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96b0e4; 0x5d9bc0 o destrutor escalar.
# - 0x5dce30 GerRegional::~GerRegional thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96b250; 0x5dce10 o destrutor escalar.
# - 0x5de510 GerSecond::~GerSecond thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96b304; 0x5de4f0 o destrutor escalar.
# Conferidos instrucao a instrucao contra o molde (a vtable e o manipulador do SEH a parte). Prologo dos destrutores
# push -1; push imm32 (7 bytes); os escalares push esi; mov esi, ecx; call rel32 ~X (operator delete 0x944c94 cdecl com
# o bit 0), thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (ver
# INTEGRAR_ANTES.txt). Free 0x9452ca e operator delete 0x944c94 cdecl; Comp_Stats::~Comp_Stats 0x49f450 (mov ecx).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats), cx03_3 (delete_through), cx04_1 (drop_stats_league) e cx10_1 (drop_plain_league): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_fra_second', 0x5be560, [0x6a, 0xff, 0x68, 0xa8, 0x9a, 0x95, 0x00]),
           ('delete_fra_second', 0x5be540, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_fra_third', 0x5bfd70, [0x6a, 0xff, 0x68, 0x48, 0x9b, 0x95, 0x00]),
           ('delete_fra_third', 0x5bfd50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ger_first', 0x5d9be0, [0x6a, 0xff, 0x68, 0xa8, 0xa5, 0x95, 0x00]),
           ('delete_ger_first', 0x5d9bc0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ger_regional', 0x5dce30, [0x6a, 0xff, 0x68, 0x48, 0xa6, 0x95, 0x00]),
           ('delete_ger_regional', 0x5dce10, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ger_second', 0x5de510, [0x6a, 0xff, 0x68, 0xb8, 0xa6, 0x95, 0x00]),
           ('delete_ger_second', 0x5de4f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas francesas e alemãs',
        'the destructors of the French and German leagues',
        [
         ('drop_fra_second', 'FraSecond::~FraSecond', '0x5be560', 'fra_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão francesa, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the French Second Division, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_fra_second', 'FraSecond::scalar_deleting_destructor', '0x5be540', 'fra_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão francesa e, com o bit 0, libera o objeto',
          'runs the destructor of the French Second Division and, with bit 0, frees the object'),
         ('drop_fra_third', 'FraThird::~FraThird', '0x5bfd70', 'fra_third.cpp', 'thiscall, ret 0',
          'desmonta a Terceira Divisão francesa, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the French Third Division, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_fra_third', 'FraThird::scalar_deleting_destructor', '0x5bfd50', 'fra_third.cpp', 'thiscall, ret 4',
          'roda o destrutor da Terceira Divisão francesa e, com o bit 0, libera o objeto',
          'runs the destructor of the French Third Division and, with bit 0, frees the object'),
         ('drop_ger_first', 'GerFirst::~GerFirst', '0x5d9be0', 'ger_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão alemã, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the German First Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_ger_first', 'GerFirst::scalar_deleting_destructor', '0x5d9bc0', 'ger_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão alemã e, com o bit 0, libera o objeto',
          'runs the destructor of the German First Division and, with bit 0, frees the object'),
         ('drop_ger_regional', 'GerRegional::~GerRegional', '0x5dce30', 'ger_regional.cpp', 'thiscall, ret 0',
          'desmonta a Regionalliga alemã, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the German Regional League, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_ger_regional', 'GerRegional::scalar_deleting_destructor', '0x5dce10', 'ger_regional.cpp', 'thiscall, ret 4',
          'roda o destrutor da Regionalliga alemã e, com o bit 0, libera o objeto',
          'runs the destructor of the German Regional League and, with bit 0, frees the object'),
         ('drop_ger_second', 'GerSecond::~GerSecond', '0x5de510', 'ger_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão alemã, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the German Second Division, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_ger_second', 'GerSecond::scalar_deleting_destructor', '0x5de4f0', 'ger_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão alemã e, com o bit 0, libera o objeto',
          'runs the destructor of the German Second Division and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
