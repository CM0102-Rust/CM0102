# Os destrutores das copas (den_cup.cpp, eng_cc_cup.cpp, eng_charity.cpp, eng_fa_cup.cpp, eng_fa_trophy.cpp, fin_cup.cpp, fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp, ger_cup.cpp, ger_lge_cup.cpp, gre_super.cpp; leitor cx02, rodada 5, 24/11/2026). Item de TELAS: rodam quando o jogo
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
ROTINAS = [('drop_den_cup', 0x550ba0, [0x6a, 0xff, 0x68, 0xa8, 0x7a, 0x95, 0x00]),
           ('drop_eng_cc_cup', 0x56c4a0, [0x6a, 0xff, 0x68, 0xc8, 0x81, 0x95, 0x00]),
           ('drop_eng_charity', 0x56d570, [0x6a, 0xff, 0x68, 0x18, 0x82, 0x95, 0x00]),
           ('drop_eng_fa_cup', 0x56f250, [0x6a, 0xff, 0x68, 0xb8, 0x82, 0x95, 0x00]),
           ('drop_eng_fa_trophy', 0x570df0, [0x6a, 0xff, 0x68, 0x28, 0x83, 0x95, 0x00]),
           ('drop_fin_cup', 0x592440, [0x6a, 0xff, 0x68, 0xd8, 0x8e, 0x95, 0x00]),
           ('drop_fra_cup', 0x5ba320, [0x6a, 0xff, 0x68, 0x18, 0x99, 0x95, 0x00]),
           ('drop_fra_lge_cup', 0x5bcb70, [0x6a, 0xff, 0x68, 0x18, 0x9a, 0x95, 0x00]),
           ('drop_fra_super', 0x5bf500, [0x6a, 0xff, 0x68, 0xf8, 0x9a, 0x95, 0x00]),
           ('drop_ger_cup', 0x5d8e10, [0x6a, 0xff, 0x68, 0x58, 0xa5, 0x95, 0x00]),
           ('drop_ger_lge_cup', 0x5dc2d0, [0x6a, 0xff, 0x68, 0xf8, 0xa5, 0x95, 0x00]),
           ('drop_gre_super', 0x5ec220, [0x6a, 0xff, 0x68, 0x98, 0xaa, 0x95, 0x00])]
DOCS = ('os destrutores das copas (1)',
        'the cup destructors (1)',
        [('drop_den_cup', 'DenCup::~DenCup', '0x550ba0', 'den_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Dinamarca: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Danish Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_eng_cc_cup', 'EngCcCup::~EngCcCup', '0x56c4a0', 'eng_cc_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga inglesa: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the League Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_eng_charity', 'EngCharity::~EngCharity', '0x56d570', 'eng_charity.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa inglesa (Charity Shield): libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Charity Shield: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_eng_fa_cup', 'EngFaCup::~EngFaCup', '0x56f250', 'eng_fa_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Inglaterra (FA Cup): libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the FA Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_eng_fa_trophy', 'EngFaTrophy::~EngFaTrophy', '0x570df0', 'eng_fa_trophy.cpp', 'thiscall, ret 0',
          'desmonta o FA Trophy: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the FA Trophy: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_fin_cup', 'FinCup::~FinCup', '0x592440', 'fin_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Finlândia: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Finnish Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_fra_cup', 'FraCup::~FraCup', '0x5ba320', 'fra_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da França: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Coupe de France: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_fra_lge_cup', 'FraLgeCup::~FraLgeCup', '0x5bcb70', 'fra_lge_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga francesa: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the French League Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_fra_super', 'FraSuper::~FraSuper', '0x5bf500', 'fra_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da França: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the French Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ger_cup', 'GerCup::~GerCup', '0x5d8e10', 'ger_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Alemanha: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the German Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_ger_lge_cup', 'GerLgeCup::~GerLgeCup', '0x5dc2d0', 'ger_lge_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Liga alemã: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the German League Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_gre_super', 'GreSuper::~GreSuper', '0x5ec220', 'gre_super.cpp', 'thiscall, ret 0',
          'desmonta a Supercopa da Grécia: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Greek Super Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor')])
CAMPOS = []
INTERESSA = []
