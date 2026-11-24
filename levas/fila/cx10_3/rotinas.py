# Os destrutores das ligas grega, holandesa e irlandesa (gre_prm.cpp, gre_second.cpp, hol_first.cpp, hol_prm.cpp, ire_first.cpp; leitor cx10, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do
# cx08_1: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no
# jogo: com uma carreira aberta que tenha essas ligas carregadas, carregar outro save, jogando comparando.
# - 0x5ea640 GrePrm::~GrePrm thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96b568; 0x5ea620 o destrutor escalar.
# - 0x5eb4e0 GreSecond::~GreSecond thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96b61c; 0x5eb4c0 o destrutor escalar.
# - 0x5f56d0 HolFirst::~HolFirst thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96b86c; 0x5f56b0 o destrutor escalar.
# - 0x5f7840 HolPrm::~HolPrm thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96b920; 0x5f7820 o destrutor escalar.
# - 0x634e80 IreFirst::~IreFirst thiscall() ret 0: o molde drop_stats_league do cx04_1, vtable 0x96bccc; 0x634e60 o destrutor escalar.
# Conferidos instrucao a instrucao contra o molde (a vtable e o manipulador do SEH a parte). Prologo dos destrutores
# push -1; push imm32 (7 bytes); os escalares push esi; mov esi, ecx; call rel32 ~X (operator delete 0x944c94 cdecl com
# o bit 0), thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (ver
# INTEGRAR_ANTES.txt). Free 0x9452ca e operator delete 0x944c94 cdecl; Comp_Stats::~Comp_Stats 0x49f450 (mov ecx).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats), cx03_3 (delete_through), cx04_1 (drop_stats_league) e cx10_1 (drop_plain_league): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_gre_prm', 0x5ea640, [0x6a, 0xff, 0x68, 0xf8, 0xa9, 0x95, 0x00]),
           ('delete_gre_prm', 0x5ea620, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_gre_second', 0x5eb4e0, [0x6a, 0xff, 0x68, 0x48, 0xaa, 0x95, 0x00]),
           ('delete_gre_second', 0x5eb4c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_hol_first', 0x5f56d0, [0x6a, 0xff, 0x68, 0x48, 0xad, 0x95, 0x00]),
           ('delete_hol_first', 0x5f56b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_hol_prm', 0x5f7840, [0x6a, 0xff, 0x68, 0x08, 0xae, 0x95, 0x00]),
           ('delete_hol_prm', 0x5f7820, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ire_first', 0x634e80, [0x6a, 0xff, 0x68, 0xa8, 0xb3, 0x95, 0x00]),
           ('delete_ire_first', 0x634e60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas grega, holandesa e irlandesa',
        'the destructors of the Greek, Dutch and Irish leagues',
        [
         ('drop_gre_prm', 'GrePrm::~GrePrm', '0x5ea640', 'gre_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão grega, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Greek First Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_gre_prm', 'GrePrm::scalar_deleting_destructor', '0x5ea620', 'gre_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão grega e, com o bit 0, libera o objeto',
          'runs the destructor of the Greek First Division and, with bit 0, frees the object'),
         ('drop_gre_second', 'GreSecond::~GreSecond', '0x5eb4e0', 'gre_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão grega, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Greek Second Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_gre_second', 'GreSecond::scalar_deleting_destructor', '0x5eb4c0', 'gre_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão grega e, com o bit 0, libera o objeto',
          'runs the destructor of the Greek Second Division and, with bit 0, frees the object'),
         ('drop_hol_first', 'HolFirst::~HolFirst', '0x5f56d0', 'hol_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão holandesa, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Dutch First Division, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_hol_first', 'HolFirst::scalar_deleting_destructor', '0x5f56b0', 'hol_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão holandesa e, com o bit 0, libera o objeto',
          'runs the destructor of the Dutch First Division and, with bit 0, frees the object'),
         ('drop_hol_prm', 'HolPrm::~HolPrm', '0x5f7840', 'hol_prm.cpp', 'thiscall, ret 0',
          'desmonta a Divisão Principal holandesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Dutch Premier Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_hol_prm', 'HolPrm::scalar_deleting_destructor', '0x5f7820', 'hol_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Divisão Principal holandesa e, com o bit 0, libera o objeto',
          'runs the destructor of the Dutch Premier Division and, with bit 0, frees the object'),
         ('drop_ire_first', 'IreFirst::~IreFirst', '0x634e80', 'ire_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão irlandesa, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Irish First Division, frees the table, the round table, the stages up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_ire_first', 'IreFirst::scalar_deleting_destructor', '0x634e60', 'ire_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão irlandesa e, com o bit 0, libera o objeto',
          'runs the destructor of the Irish First Division and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
