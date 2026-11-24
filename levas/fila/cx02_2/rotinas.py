# Os destrutores das copas (hol_super.cpp, ire_chal_cup.cpp, ire_leinster_cup.cpp, ire_munster_cup.cpp, ire_pres_cup.cpp, ita_c1_super.cpp, ita_super.cpp, jap_emp_cup.cpp, jap_j_cup.cpp, jap_super.cpp, kor_fa_cup.cpp, kor_super_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Item de TELAS: rodam quando o jogo
# e desmontado (carregar outro jogo com um jogo aberto, ou sair da carreira); caminho no jogo: com uma carreira aberta
# que tenha essas copas (as inglesas com a Inglaterra; as outras pedem a liga da nacao), carregar outro save.
# Convencoes conferidas: thiscall, ret 0 (chamados pelos destrutores escalares, mov ecx -> call); dentro, o free do CRT
# 0x9452ca (push; add esp, 4 -> cdecl), Comp_Stats::~Comp_Stats 0x49f450 (mov ecx -> thiscall) e o operator delete
# 0x944c94 (cdecl), Cup::~Cup 0x518690 / League::~League 0x682300 (mov ecx -> thiscall, ret 0), League::free_table
# 0x687970 (push 0 -> thiscall, ret 4), o destrutor escalar de cada fase (slot 0 com 1). O quadro SEH so cobre o
# desenrolar de excecao (nao portado). Prologo push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe.
# Ficam de fora os destrutores escalares da lista (32 bytes cada: push esi; mov esi, ecx; call ~X): o prologo tem 3
# bytes antes de um call rel32; entrariam pelas RELATIVAS do detours.rs (endereco, 3), como no sx18_29.
# Comparadores: so a original (liberam memoria).
# Usa drop_national_cup do cx01_14 (o molde dos destrutores das copas; integrar depois dele).
SAVE = False
TELAS = True
ROTINAS = [('drop_hol_super', 0x5f8a00, [0x6a, 0xff, 0x68, 0x58, 0xae, 0x95, 0x00]),
           ('drop_ire_chal_cup', 0x634290, [0x6a, 0xff, 0x68, 0x58, 0xb3, 0x95, 0x00]),
           ('drop_ire_leinster_cup', 0x636690, [0x6a, 0xff, 0x68, 0x48, 0xb4, 0x95, 0x00]),
           ('drop_ire_munster_cup', 0x638990, [0x6a, 0xff, 0x68, 0x28, 0xb5, 0x95, 0x00]),
           ('drop_ire_pres_cup', 0x639460, [0x6a, 0xff, 0x68, 0x78, 0xb5, 0x95, 0x00]),
           ('drop_ita_c1_super', 0x63c7c0, [0x6a, 0xff, 0x68, 0xf8, 0xb7, 0x95, 0x00]),
           ('drop_ita_super', 0x65f850, [0x6a, 0xff, 0x68, 0xd8, 0xbd, 0x95, 0x00]),
           ('drop_jap_emp_cup', 0x661f20, [0x6a, 0xff, 0x68, 0xc8, 0xc0, 0x95, 0x00]),
           ('drop_jap_j_cup', 0x665570, [0x6a, 0xff, 0x68, 0xf8, 0xc1, 0x95, 0x00]),
           ('drop_jap_super', 0x665f50, [0x6a, 0xff, 0x68, 0x48, 0xc2, 0x95, 0x00]),
           ('drop_kor_fa_cup', 0x66a650, [0x6a, 0xff, 0x68, 0xa8, 0xc6, 0x95, 0x00]),
           ('drop_kor_super_cup', 0x66da00, [0x6a, 0xff, 0x68, 0xd8, 0xc7, 0x95, 0x00])]
DOCS = ('os destrutores das copas (2)',
        'the cup destructors (2)',
        [('drop_hol_super', 'HolSuper::~HolSuper', '0x5f8a00', 'hol_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Holanda: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Dutch Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ire_chal_cup', 'IreChalCup::~IreChalCup', '0x634290', 'ire_chal_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Irlanda (Challenge Cup): libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Irish Challenge Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ire_leinster_cup', 'IreLeinsterCup::~IreLeinsterCup', '0x636690', 'ire_leinster_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa de Leinster: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Leinster Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ire_munster_cup', 'IreMunsterCup::~IreMunsterCup', '0x638990', 'ire_munster_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa de Munster: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Munster Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ire_pres_cup', 'IrePresCup::~IrePresCup', '0x639460', 'ire_pres_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa do Presidente irlandesa: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Irish President Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ita_c1_super', 'ItaC1Super::~ItaC1Super', '0x63c7c0', 'ita_c1_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Série C1: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Serie C1 Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ita_super', 'ItaSuper::~ItaSuper', '0x65f850', 'ita_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Itália: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Italian Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_jap_emp_cup', 'JapEmpCup::~JapEmpCup', '0x661f20', 'jap_emp_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa do Imperador: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Emperor Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_jap_j_cup', 'JapJCup::~JapJCup', '0x665570', 'jap_j_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da J League: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the J League Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_jap_super', 'JapSuper::~JapSuper', '0x665f50', 'jap_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa do Japão: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Japanese Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_kor_fa_cup', 'KorFaCup::~KorFaCup', '0x66a650', 'kor_fa_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Coreia: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Korean FA Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_kor_super_cup', 'KorSuperCup::~KorSuperCup', '0x66da00', 'kor_super_cup.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Coreia: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Korean Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor')])
CAMPOS = []
INTERESSA = []
