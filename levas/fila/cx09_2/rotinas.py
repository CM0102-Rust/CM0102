# Os destrutores de três ligas sem fases e de nove estaduais do Brasil (leitor cx09, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_3 e cx03_3:
# rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo:
# com uma carreira aberta que tenha as ligas dessas nacoes carregadas (Inglaterra; Belgica, Brasil, Croacia,
# Dinamarca e Finlandia), carregar outro save, jogando comparando.
# Destrutores (thiscall, ret 0; prologo push -1; push imm32, 7 bytes; o quadro SEH so cobre o desenrolar de excecao):
# - forma das ligas com fases (8): a vtable, League::free_table 0x687970 (push 0), free(+0xba) e zera, o slot 0 das
#   fases 0..+0x30 (push 1), free(+0xc) e zera, Comp_Stats::~Comp_Stats 0x49f450 e operator delete, League::~League
#   0x682300: o drop_group_cup do cx02_3 sem lista de inscritos. A Serie C (0x42d540) libera antes o +0xee.
# - forma sem fases (CroA1, CroA2a, CroA2b, DenFirst, DenPrm, EngPrm): sem o laco das fases (drop_plain_league, cx09_1).
# - forma dos estaduais e da DenSecond: o vetor das fases e liberado sem teste de nulo e nao e zerado
#   (drop_regional_league, cx09_2).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), Comp_Stats::~Comp_Stats 0x49f450 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (thiscall, ret 4). Iguais no
# cm0102-1080.exe (todas inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_den_first', 0x551920, [0x6a, 0xff, 0x68, 0xf8, 0x7a, 0x95, 0x00]),
           ('drop_den_prm', 0x552870, [0x6a, 0xff, 0x68, 0x48, 0x7b, 0x95, 0x00]),
           ('drop_eng_prm', 0x5735d0, [0x6a, 0xff, 0x68, 0xe8, 0x83, 0x95, 0x00]),
           ('drop_bra_reg_bahia', 0x42f510, [0x6a, 0xff, 0x68, 0x08, 0x46, 0x95, 0x00]),
           ('drop_bra_reg_central', 0x4306f0, [0x6a, 0xff, 0x68, 0x78, 0x46, 0x95, 0x00]),
           ('drop_bra_reg_gaucho', 0x431a00, [0x6a, 0xff, 0x68, 0xe8, 0x46, 0x95, 0x00]),
           ('drop_bra_reg_goias', 0x432d90, [0x6a, 0xff, 0x68, 0x58, 0x47, 0x95, 0x00]),
           ('drop_bra_reg_minas_gerais', 0x433f90, [0x6a, 0xff, 0x68, 0xc8, 0x47, 0x95, 0x00]),
           ('drop_bra_reg_north', 0x435580, [0x6a, 0xff, 0x68, 0x58, 0x48, 0x95, 0x00]),
           ('drop_bra_reg_northeast', 0x436630, [0x6a, 0xff, 0x68, 0xc8, 0x48, 0x95, 0x00]),
           ('drop_bra_reg_parana', 0x437640, [0x6a, 0xff, 0x68, 0x38, 0x49, 0x95, 0x00]),
           ('drop_bra_reg_pern', 0x438b30, [0x6a, 0xff, 0x68, 0xc8, 0x49, 0x95, 0x00])]
DOCS = ('os destrutores de três ligas sem fases e de nove estaduais do Brasil',
        'the destructors of three leagues without stages and nine Brazilian state championships',
        [('drop_den_first', 'DenFirst::~DenFirst', '0x551920', 'den_first.cpp', 'thiscall',
          'desmonta a Primeira Divisão da Dinamarca: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the Danish First Division: frees the table and the dates, frees the statistics and runs the league destructor (no stages)'),
         ('drop_den_prm', 'DenPrm::~DenPrm', '0x552870', 'den_prm.cpp', 'thiscall',
          'desmonta a Superliga da Dinamarca: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the Danish Superliga: frees the table and the dates, frees the statistics and runs the league destructor (no stages)'),
         ('drop_eng_prm', 'EngPrm::~EngPrm', '0x5735d0', 'eng_prm.cpp', 'thiscall',
          'desmonta a Premier League: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the English Premier League: frees the table and the dates, frees the statistics and runs the league destructor (no stages)'),
         ('drop_bra_reg_bahia', 'BraRegBahia::~BraRegBahia', '0x42f510', 'bra_reg_bahia.cpp', 'thiscall',
          'desmonta o Campeonato Baiano: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Bahia state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_central', 'BraRegCentral::~BraRegCentral', '0x4306f0', 'bra_reg_central.cpp', 'thiscall',
          'desmonta o estadual do Centro-Oeste: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Central regional championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_gaucho', 'BraRegGaucho::~BraRegGaucho', '0x431a00', 'bra_reg_gaucho.cpp', 'thiscall',
          'desmonta o Campeonato Gaúcho: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Rio Grande do Sul state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_goias', 'BraRegGoias::~BraRegGoias', '0x432d90', 'bra_reg_goias.cpp', 'thiscall',
          'desmonta o Campeonato Goiano: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Goias state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_minas_gerais', 'BraRegMinasGerais::~BraRegMinasGerais', '0x433f90', 'bra_reg_minas_gerais.cpp', 'thiscall',
          'desmonta o Campeonato Mineiro: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Minas Gerais state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_north', 'BraRegNorth::~BraRegNorth', '0x435580', 'bra_reg_north.cpp', 'thiscall',
          'desmonta o estadual do Norte: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the North regional championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_northeast', 'BraRegNortheast::~BraRegNortheast', '0x436630', 'bra_reg_northeast.cpp', 'thiscall',
          'desmonta o estadual do Nordeste: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Northeast regional championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_parana', 'BraRegParana::~BraRegParana', '0x437640', 'bra_reg_parana.cpp', 'thiscall',
          'desmonta o Campeonato Paranaense: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Parana state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_pern', 'BraRegPern::~BraRegPern', '0x438b30', 'bra_reg_pern.cpp', 'thiscall',
          'desmonta o Campeonato Pernambucano: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Pernambuco state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor')])
CAMPOS = []
INTERESSA = []
