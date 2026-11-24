# Os destrutores da liga irlandesa e as italianas (ire_prm.cpp, ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp; leitor cx10, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do
# cx08_1: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no
# jogo: com uma carreira aberta que tenha essas ligas carregadas, carregar outro save, jogando comparando.
# - 0x639e00 IrePrm::~IrePrm thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96c014; 0x639de0 o destrutor escalar.
# - 0x640920 ItaSerA::~ItaSerA thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96c3cc; 0x640900 o destrutor escalar.
# - 0x645a60 ItaSerB::~ItaSerB thiscall() ret 0: liga sem fases (drop_plain_league do cx10_1, o molde do BelFirst), vtable 0x96c480; 0x645a40 o destrutor escalar.
# - 0x64acc0 ItaSerC1a::~ItaSerC1a thiscall() ret 0: drop_stage_league_unchecked (aqui): as fases ate a atual e o vetor solto sem teste nem zerar, vtable 0x96c534; 0x64aca0 o destrutor escalar.
# - 0x64ee70 ItaSerC1b::~ItaSerC1b thiscall() ret 0: drop_stage_league_unchecked (aqui): as fases ate a atual e o vetor solto sem teste nem zerar, vtable 0x96c5e8; 0x64ee50 o destrutor escalar.
# Conferidos instrucao a instrucao contra o molde (a vtable e o manipulador do SEH a parte). Prologo dos destrutores
# push -1; push imm32 (7 bytes); os escalares push esi; mov esi, ecx; call rel32 ~X (operator delete 0x944c94 cdecl com
# o bit 0), thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (ver
# INTEGRAR_ANTES.txt). Free 0x9452ca e operator delete 0x944c94 cdecl; Comp_Stats::~Comp_Stats 0x49f450 (mov ecx).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats), cx03_3 (delete_through), cx04_1 (drop_stats_league) e cx10_1 (drop_plain_league): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_ire_prm', 0x639e00, [0x6a, 0xff, 0x68, 0xc8, 0xb5, 0x95, 0x00]),
           ('delete_ire_prm', 0x639de0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ita_ser_a', 0x640920, [0x6a, 0xff, 0x68, 0x78, 0xb9, 0x95, 0x00]),
           ('delete_ita_ser_a', 0x640900, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ita_ser_b', 0x645a60, [0x6a, 0xff, 0x68, 0x28, 0xba, 0x95, 0x00]),
           ('delete_ita_ser_b', 0x645a40, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ita_ser_c1a', 0x64acc0, [0x6a, 0xff, 0x68, 0xb8, 0xba, 0x95, 0x00]),
           ('delete_ita_ser_c1a', 0x64aca0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ita_ser_c1b', 0x64ee70, [0x6a, 0xff, 0x68, 0x58, 0xbb, 0x95, 0x00]),
           ('delete_ita_ser_c1b', 0x64ee50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da liga irlandesa e as italianas',
        'the destructors of the Irish and Italian leagues',
        [
         ('drop_ire_prm', 'IrePrm::~IrePrm', '0x639e00', 'ire_prm.cpp', 'thiscall, ret 0',
          'desmonta a Divisão Principal irlandesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Irish Premier Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_ire_prm', 'IrePrm::scalar_deleting_destructor', '0x639de0', 'ire_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Divisão Principal irlandesa e, com o bit 0, libera o objeto',
          'runs the destructor of the Irish Premier Division and, with bit 0, frees the object'),
         ('drop_ita_ser_a', 'ItaSerA::~ItaSerA', '0x640920', 'ita_ser_a.cpp', 'thiscall, ret 0',
          'desmonta a Série A italiana, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie A, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_ita_ser_a', 'ItaSerA::scalar_deleting_destructor', '0x640900', 'ita_ser_a.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série A italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie A and, with bit 0, frees the object'),
         ('drop_ita_ser_b', 'ItaSerB::~ItaSerB', '0x645a60', 'ita_ser_b.cpp', 'thiscall, ret 0',
          'desmonta a Série B italiana, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie B, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_ita_ser_b', 'ItaSerB::scalar_deleting_destructor', '0x645a40', 'ita_ser_b.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série B italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie B and, with bit 0, frees the object'),
         ('drop_ita_ser_c1a', 'ItaSerC1a::~ItaSerC1a', '0x64acc0', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
          'desmonta a Série C1/A italiana, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases (sem testar nem zerar) e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie C1/A, frees the table, the round table, the stages up to the current one, the stage vector (unchecked and not cleared) and the statistics and runs the league destructor'),
         ('delete_ita_ser_c1a', 'ItaSerC1a::scalar_deleting_destructor', '0x64aca0', 'ita_ser_c1a.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C1/A italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie C1/A and, with bit 0, frees the object'),
         ('drop_ita_ser_c1b', 'ItaSerC1b::~ItaSerC1b', '0x64ee70', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
          'desmonta a Série C1/B italiana, libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases (sem testar nem zerar) e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie C1/B, frees the table, the round table, the stages up to the current one, the stage vector (unchecked and not cleared) and the statistics and runs the league destructor'),
         ('delete_ita_ser_c1b', 'ItaSerC1b::scalar_deleting_destructor', '0x64ee50', 'ita_ser_c1b.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C1/B italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie C1/B and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
