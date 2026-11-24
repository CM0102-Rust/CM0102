# Os destrutores das copas (nir_charity.cpp, nir_cup.cpp, nor_cup.cpp, gre_cup.cpp, hol_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp, ita_cup.cpp, kor_league_cup.cpp, nir_lge_cup.cpp, eng_auto_cup.cpp, ire_super_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Item de TELAS: rodam quando o jogo
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
# Usa drop_national_cup e delete_comp_stats do cx01_14 (integrar depois dele); drop_group_cup e
# delete_stages_and_vector ficam aqui.
SAVE = False
TELAS = True
ROTINAS = [('drop_nir_charity', 0x789fd0, [0x6a, 0xff, 0x68, 0x28, 0xe3, 0x95, 0x00]),
           ('drop_nir_cup', 0x78a7a0, [0x6a, 0xff, 0x68, 0x78, 0xe3, 0x95, 0x00]),
           ('drop_nor_cup', 0x78f210, [0x6a, 0xff, 0x68, 0x18, 0xe5, 0x95, 0x00]),
           ('drop_gre_cup', 0x5e8b10, [0x6a, 0xff, 0x68, 0x68, 0xa9, 0x95, 0x00]),
           ('drop_hol_cup', 0x5f3600, [0x6a, 0xff, 0x68, 0xb8, 0xac, 0x95, 0x00]),
           ('drop_ire_lge_cup', 0x637040, [0x6a, 0xff, 0x68, 0x98, 0xb4, 0x95, 0x00]),
           ('drop_ita_c_cup', 0x63ceb0, [0x6a, 0xff, 0x68, 0x48, 0xb8, 0x95, 0x00]),
           ('drop_ita_cup', 0x63eb70, [0x6a, 0xff, 0x68, 0xe8, 0xb8, 0x95, 0x00]),
           ('drop_kor_league_cup', 0x66c160, [0x6a, 0xff, 0x68, 0x48, 0xc7, 0x95, 0x00]),
           ('drop_nir_lge_cup', 0x78c9f0, [0x6a, 0xff, 0x68, 0x38, 0xe4, 0x95, 0x00]),
           ('drop_eng_auto_cup', 0x56acc0, [0x6a, 0xff, 0x68, 0x38, 0x81, 0x95, 0x00]),
           ('drop_ire_super_cup', 0x63aea0, [0x6a, 0xff, 0x68, 0x48, 0xb6, 0x95, 0x00])]
DOCS = ('os destrutores das copas (3)',
        'the cup destructors (3)',
        [('drop_nir_charity', 'NirCharity::~NirCharity', '0x789fd0', 'nir_charity.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Irlanda do Norte: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Northern Irish Charity Shield: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_nir_cup', 'NirCup::~NirCup', '0x78a7a0', 'nir_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Irlanda do Norte: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Northern Irish Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_nor_cup', 'NorCup::~NorCup', '0x78f210', 'nor_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Noruega: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Norwegian Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_gre_cup', 'GreCup::~GreCup', '0x5e8b10', 'gre_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Grécia: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Greek Cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_hol_cup', 'HolCup::~HolCup', '0x5f3600', 'hol_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Holanda: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Dutch Cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_ire_lge_cup', 'IreLgeCup::~IreLgeCup', '0x637040', 'ire_lge_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga irlandesa: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Irish League Cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_ita_c_cup', 'ItaCCup::~ItaCCup', '0x63ceb0', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Série C: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Serie C cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_ita_cup', 'ItaCup::~ItaCup', '0x63eb70', 'ita_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Itália: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Coppa Italia: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_kor_league_cup', 'KorLeagueCup::~KorLeagueCup', '0x66c160', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga coreana: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Korean League Cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_nir_lge_cup', 'NirLgeCup::~NirLgeCup', '0x78c9f0', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga da Irlanda do Norte: libera a tabela, as datas das rodadas, a lista de inscritos, as fases e as estatísticas e roda o destrutor da liga',
          'destroys the Northern Irish League Cup: frees the table, the round dates, the entrant list, the stages and the statistics and runs the league destructor'),
         ('drop_eng_auto_cup', 'EngAutoCup::~EngAutoCup', '0x56acc0', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'desmonta o Troféu das Divisões Inferiores (Auto Windscreens Shield): libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada, as fases (seções norte e sul e a final), as estatísticas e a lista dos clubes escolhidos e roda o destrutor da copa',
          'destroys the Auto Windscreens Shield: frees the team table, the round table, each round\'s ties, the stages (the north and south sections and the final), the statistics and the chosen clubs list and runs the cup destructor'),
         ('drop_ire_super_cup', 'IreSuperCup::~IreSuperCup', '0x63aea0', 'ire_super_cup.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Irlanda: libera a tabela, as datas das rodadas, as fases e as estatísticas e roda o destrutor da liga (sem lista de inscritos)',
          'destroys the Irish Super Cup: frees the table, the round dates, the stages and the statistics and runs the league destructor (no entrant list)')])
CAMPOS = []
INTERESSA = []
