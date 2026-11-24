# Os destrutores de doze ligas: Bélgica, Brasil, Inglaterra, Finlândia e Croácia (leitor cx09, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_3 e cx03_3:
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
ROTINAS = [('drop_bel_third', 0x421940, [0x6a, 0xff, 0x68, 0x18, 0x41, 0x95, 0x00]),
           ('drop_bra_nat_first', 0x429a90, [0x6a, 0xff, 0x68, 0x48, 0x44, 0x95, 0x00]),
           ('drop_bra_nat_second', 0x42c080, [0x6a, 0xff, 0x68, 0xb8, 0x44, 0x95, 0x00]),
           ('drop_eng_conf', 0x56dda0, [0x6a, 0xff, 0x68, 0x68, 0x82, 0x95, 0x00]),
           ('drop_eng_first', 0x5718c0, [0x6a, 0xff, 0x68, 0x78, 0x83, 0x95, 0x00]),
           ('drop_eng_second', 0x5756e0, [0x6a, 0xff, 0x68, 0x38, 0x84, 0x95, 0x00]),
           ('drop_eng_third', 0x577020, [0x6a, 0xff, 0x68, 0xa8, 0x84, 0x95, 0x00]),
           ('drop_fin_first', 0x593100, [0x6a, 0xff, 0x68, 0x28, 0x8f, 0x95, 0x00]),
           ('drop_bra_nat_third', 0x42d540, [0x6a, 0xff, 0x68, 0x28, 0x45, 0x95, 0x00]),
           ('drop_cro_a1', 0x514050, [0x6a, 0xff, 0x68, 0x78, 0x73, 0x95, 0x00]),
           ('drop_cro_a2a', 0x515550, [0x6a, 0xff, 0x68, 0x08, 0x74, 0x95, 0x00]),
           ('drop_cro_a2b', 0x516670, [0x6a, 0xff, 0x68, 0x78, 0x74, 0x95, 0x00])]
DOCS = ('os destrutores de doze ligas: Bélgica, Brasil, Inglaterra, Finlândia e Croácia',
        'the destructors of twelve leagues: Belgium, Brazil, England, Finland and Croatia',
        [('drop_bel_third', 'BelThird::~BelThird', '0x421940', 'bel_third.cpp', 'thiscall',
          'desmonta a Terceira Divisão da Bélgica: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Belgian Third Division: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_bra_nat_first', 'BraNatFirst::~BraNatFirst', '0x429a90', 'bra_nat_first.cpp', 'thiscall',
          'desmonta a Série A do Brasileiro: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Brazilian Serie A: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_bra_nat_second', 'BraNatSecond::~BraNatSecond', '0x42c080', 'bra_nat_second.cpp', 'thiscall',
          'desmonta a Série B do Brasileiro: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Brazilian Serie B: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_eng_conf', 'EngConf::~EngConf', '0x56dda0', 'eng_conf.cpp', 'thiscall',
          'desmonta a Conference inglesa: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Football Conference: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_eng_first', 'EngFirst::~EngFirst', '0x5718c0', 'eng_first.cpp', 'thiscall',
          'desmonta a First Division inglesa: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the English First Division: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_eng_second', 'EngSecond::~EngSecond', '0x5756e0', 'eng_second.cpp', 'thiscall',
          'desmonta a Second Division inglesa: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the English Second Division: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_eng_third', 'EngThird::~EngThird', '0x577020', 'eng_third.cpp', 'thiscall',
          'desmonta a Third Division inglesa: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the English Third Division: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_fin_first', 'FinFirst::~FinFirst', '0x593100', 'fin_first.cpp', 'thiscall',
          'desmonta a Primeira Divisão da Finlândia: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Finnish First Division: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_bra_nat_third', 'BraNatThird::~BraNatThird', '0x42d540', 'bra_nat_third.cpp', 'thiscall',
          'desmonta a Série C do Brasileiro: libera o vetor dos clubes da fase seguinte e depois, como as outras, a tabela, as datas, as fases e as estatísticas, e roda o destrutor da liga',
          "destroys the Brazilian Serie C: frees the next stage's club pool, then, like the others, the table, the dates, the stages and the statistics, and runs the league destructor"),
         ('drop_cro_a1', 'CroA1::~CroA1', '0x514050', 'cro_a1.cpp', 'thiscall',
          'desmonta a Primeira Divisão da Croácia: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the Croatian First Division: frees the table and the dates, frees the statistics and runs the league destructor (no stages)'),
         ('drop_cro_a2a', 'CroA2a::~CroA2a', '0x515550', 'cro_a2a.cpp', 'thiscall',
          'desmonta o grupo A da Segunda Divisão da Croácia: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the Croatian Second Division group A: frees the table and the dates, frees the statistics and runs the league destructor (no stages)'),
         ('drop_cro_a2b', 'CroA2b::~CroA2b', '0x516670', 'cro_a2b.cpp', 'thiscall',
          'desmonta o grupo B da Segunda Divisão da Croácia: libera a tabela e as datas, libera as estatísticas e roda o destrutor da liga (sem fases)',
          'destroys the Croatian Second Division group B: frees the table and the dates, frees the statistics and runs the league destructor (no stages)')])
CAMPOS = []
INTERESSA = []
