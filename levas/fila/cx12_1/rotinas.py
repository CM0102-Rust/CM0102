# Os destrutores da Serie C2/C italiana, das ligas japonesas, da coreana e das norte-irlandesas (ita_ser_c2c.cpp,
# jap_j1.cpp, jap_j2.cpp, kor_league.cpp, nir_first.cpp, nir_prm.cpp; leitor cx12, rodada 5, 24/11/2026). Item de TELAS,
# como os destrutores do cx08_1/cx10_2: rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou
# sair da carreira). Caminho no jogo: com uma carreira aberta que tenha essas ligas carregadas (a Serie C2 so com a base
# NORMAL ou MAXIMA), carregar outro save, jogando comparando.
# - 0x65b730 ItaSerC2c::~ItaSerC2c: drop_stage_league_unchecked do cx10_4 (igual ao 0x6530e0 da C2/A, cx11_1), vtable 0x96c804.
# - 0x662d70 JapJ1::~JapJ1: drop_argentine_league do cx08_1 (igual ao 0x4044b0, ArgPrm: a fase atual volta a -1), vtable 0x96ca40.
# - 0x6646f0 JapJ2::~JapJ2, 0x66b0a0 KorLeague::~KorLeague, 0x78e2a0 NirPrm::~NirPrm: drop_plain_league do cx10_1 (igual ao
#   0x639e00, IrePrm), vtables 0x96caf4, 0x96cdd0, 0x96dccc.
# - 0x78b250 NirFirst::~NirFirst: drop_stats_league do cx04_1 (igual ao 0x4110e0, AusNsl), vtable 0x96db64.
# - 0x65b710, 0x662d50, 0x6646d0, 0x66b080, 0x78b230, 0x78e280: os destrutores escalares (push esi; mov esi, ecx; call
#   rel32 ~X; operator delete 0x944c94 cdecl com o bit 0), thiscall(flags) ret 4 -> this: delete_through do cx03_3 e o
#   prologo pelas RELATIVAS (ver INTEGRAR_ANTES.txt).
# Conferidos instrucao a instrucao contra os moldes (so a vtable e o manipulador do SEH mudam); prologo dos destrutores
# push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats), cx03_3 (delete_through), cx04_1 (drop_stats_league), cx08_1
# (drop_argentine_league), cx10_1 (drop_plain_league) e cx10_4 (drop_stage_league_unchecked): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_ita_ser_c2c', 0x65b730, [0x6a, 0xff, 0x68, 0x38, 0xbd, 0x95, 0x00]),
           ('delete_ita_ser_c2c', 0x65b710, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_jap_j1', 0x662d70, [0x6a, 0xff, 0x68, 0x18, 0xc1, 0x95, 0x00]),
           ('delete_jap_j1', 0x662d50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_jap_j2', 0x6646f0, [0x6a, 0xff, 0x68, 0xa8, 0xc1, 0x95, 0x00]),
           ('delete_jap_j2', 0x6646d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_kor_league', 0x66b0a0, [0x6a, 0xff, 0x68, 0xf8, 0xc6, 0x95, 0x00]),
           ('delete_kor_league', 0x66b080, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_nir_first', 0x78b250, [0x6a, 0xff, 0x68, 0xc8, 0xe3, 0x95, 0x00]),
           ('delete_nir_first', 0x78b230, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_nir_prm', 0x78e2a0, [0x6a, 0xff, 0x68, 0xc8, 0xe4, 0x95, 0x00]),
           ('delete_nir_prm', 0x78e280, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da Série C2/C italiana, das ligas japonesas, da coreana e das norte-irlandesas',
        'the destructors of the Italian Serie C2/C, the Japanese, Korean and Northern Irish leagues',
        [('drop_ita_ser_c2c', 'ItaSerC2c::~ItaSerC2c', '0x65b730', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'desmonta a Série C2/C italiana: libera a tabela, as datas das rodadas, as fases até a atual, o vetor das fases (sem teste nem zerar) e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie C2/C: frees the table, the round dates, every stage up to the current one, the stage vector (not tested nor cleared) and the statistics and runs the league destructor'),
         ('delete_ita_ser_c2c', 'ItaSerC2c::scalar_deleting_destructor', '0x65b710', 'ita_ser_c2c.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C2/C italiana e, com o bit 0, libera o objeto',
          'runs the Italian Serie C2/C destructor and, with bit 0, frees the object'),
         ('drop_jap_j1', 'JapJ1::~JapJ1', '0x662d70', 'jap_j1.cpp', 'thiscall, ret 0',
          'desmonta a J1 League japonesa: libera a tabela, as datas das rodadas, as fases até a atual (a fase atual volta a −1), o vetor das fases (sem teste nem zerar) e as estatísticas e roda o destrutor da liga',
          'destroys the Japanese J1 League: frees the table, the round dates, every stage up to the current one, the current stage back to −1, the stage vector (not tested nor cleared) and the statistics and runs the league destructor'),
         ('delete_jap_j1', 'JapJ1::scalar_deleting_destructor', '0x662d50', 'jap_j1.cpp', 'thiscall, ret 4',
          'roda o destrutor da J1 League japonesa e, com o bit 0, libera o objeto',
          'runs the Japanese J1 League destructor and, with bit 0, frees the object'),
         ('drop_jap_j2', 'JapJ2::~JapJ2', '0x6646f0', 'jap_j2.cpp', 'thiscall, ret 0',
          'desmonta a J2 League japonesa: libera a tabela, as datas das rodadas e as estatísticas (sem fases) e roda o destrutor da liga',
          'destroys the Japanese J2 League: frees the table, the round dates and the statistics (no stages) and runs the league destructor'),
         ('delete_jap_j2', 'JapJ2::scalar_deleting_destructor', '0x6646d0', 'jap_j2.cpp', 'thiscall, ret 4',
          'roda o destrutor da J2 League japonesa e, com o bit 0, libera o objeto',
          'runs the Japanese J2 League destructor and, with bit 0, frees the object'),
         ('drop_kor_league', 'KorLeague::~KorLeague', '0x66b0a0', 'kor_league.cpp', 'thiscall, ret 0',
          'desmonta a liga coreana: libera a tabela, as datas das rodadas e as estatísticas (sem fases) e roda o destrutor da liga',
          'destroys the Korean league: frees the table, the round dates and the statistics (no stages) and runs the league destructor'),
         ('delete_kor_league', 'KorLeague::scalar_deleting_destructor', '0x66b080', 'kor_league.cpp', 'thiscall, ret 4',
          'roda o destrutor da liga coreana e, com o bit 0, libera o objeto',
          'runs the Korean league destructor and, with bit 0, frees the object'),
         ('drop_nir_first', 'NirFirst::~NirFirst', '0x78b250', 'nir_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão norte-irlandesa: libera a tabela, as datas das rodadas, as fases até a atual, o vetor das fases (liberado e zerado) e as estatísticas e roda o destrutor da liga',
          'destroys the Northern Irish First Division: frees the table, the round dates, every stage up to the current one, the stage vector (freed and cleared) and the statistics and runs the league destructor'),
         ('delete_nir_first', 'NirFirst::scalar_deleting_destructor', '0x78b230', 'nir_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão norte-irlandesa e, com o bit 0, libera o objeto',
          'runs the Northern Irish First Division destructor and, with bit 0, frees the object'),
         ('drop_nir_prm', 'NirPrm::~NirPrm', '0x78e2a0', 'nir_prm.cpp', 'thiscall, ret 0',
          'desmonta a Premier Division norte-irlandesa: libera a tabela, as datas das rodadas e as estatísticas (sem fases) e roda o destrutor da liga',
          'destroys the Northern Irish Premier Division: frees the table, the round dates and the statistics (no stages) and runs the league destructor'),
         ('delete_nir_prm', 'NirPrm::scalar_deleting_destructor', '0x78e280', 'nir_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Premier Division norte-irlandesa e, com o bit 0, libera o objeto',
          'runs the Northern Irish Premier Division destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
