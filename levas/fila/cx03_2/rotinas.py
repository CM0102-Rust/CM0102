# Os destrutores de onze copas, nacionais, continentais e por fases (leitor cx03, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e
# desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que
# tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao; as continentais, as ligas do
# continente), carregar outro save, jogando comparando.
# - US Open Cup, Copa de Gales, Supercopa da Asia, Copa dos Campeoes da CONCACAF e Supercopa da Europa: o molde
#   drop_national_cup do cx01_14.
# - Russia, Suecia e as duas asiaticas (copas por fases): drop_cup_with_stages (aqui), com free_season_tables (HEAD),
#   delete_comp_stats (cx01_14) e delete_stages_and_vector (cx02_3); a lista +0xb2 da Suecia sai entre as
#   estatisticas e as fases, a das asiaticas no fim, depois das estatisticas (ordem conferida na desmontagem).
# - As duas copas galesas (League): drop_group_cup do cx02_3 com a lista de inscritos +0xee.
# Integrar depois do cx01_14 e do cx02_3. Prologo push -1; push imm32 (7 bytes).
# Convencoes conferidas: destrutores thiscall, ret 0 (mov ecx -> call pelo destrutor escalar); o free do CRT 0x9452ca
# (cdecl), Comp_Stats::~Comp_Stats 0x49f450 (thiscall), operator delete 0x944c94 (cdecl), Cup::~Cup 0x518690 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (push 0, thiscall, ret 4), o destrutor
# escalar de cada fase (slot 0 com 1). O quadro SEH so cobre o desenrolar de excecao (nao portado).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_usa_open_cup', 0x90b0a0, [0x6a, 0xff, 0x68, 0x18, 0x55, 0x96, 0x00]),
           ('drop_wel_cup', 0x924610, [0x6a, 0xff, 0x68, 0xb8, 0x5c, 0x96, 0x00]),
           ('drop_asia_super_cup', 0x410840, [0x6a, 0xff, 0x68, 0xa8, 0x39, 0x95, 0x00]),
           ('drop_con_champ', 0x4c0e30, [0x6a, 0xff, 0x68, 0x58, 0x69, 0x95, 0x00]),
           ('drop_eur_super_cup', 0x57a2f0, [0x6a, 0xff, 0x68, 0x58, 0x87, 0x95, 0x00]),
           ('drop_rus_cup', 0x7e9420, [0x6a, 0xff, 0x68, 0x18, 0x09, 0x96, 0x00]),
           ('drop_swe_cup', 0x88ce20, [0x6a, 0xff, 0x68, 0x88, 0x30, 0x96, 0x00]),
           ('drop_asia_club_champ', 0x40ad70, [0x6a, 0xff, 0x68, 0x78, 0x37, 0x95, 0x00]),
           ('drop_asia_cup_winner', 0x40ce70, [0x6a, 0xff, 0x68, 0x38, 0x38, 0x95, 0x00]),
           ('drop_wel_lge_cup', 0x926100, [0x6a, 0xff, 0x68, 0x58, 0x5d, 0x96, 0x00]),
           ('drop_wel_prm_cup', 0x927a50, [0x6a, 0xff, 0x68, 0xf8, 0x5d, 0x96, 0x00])]
DOCS = ('os destrutores de onze copas, nacionais, continentais e por fases',
        'the destructors of eleven national, continental and staged cups',
        [('drop_usa_open_cup', 'UsaOpenCup::~UsaOpenCup', '0x90b0a0', 'usa_open_cup.cpp', 'thiscall',
          'desmonta a US Open Cup: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the US Open Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_wel_cup', 'WelCup::~WelCup', '0x924610', 'wel_cup.cpp', 'thiscall',
          'desmonta a Copa do País de Gales: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Welsh Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_asia_super_cup', 'AsiaSuperCup::~AsiaSuperCup', '0x410840', 'asia_super_cup.cpp', 'thiscall',
          'desmonta a Supercopa da Ásia: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Asian Super Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_con_champ', 'ConChamp::~ConChamp', '0x4c0e30', 'con_champ.cpp', 'thiscall',
          'desmonta a Copa dos Campeões da CONCACAF: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the CONCACAF Champions' Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_eur_super_cup', 'EurSuperCup::~EurSuperCup', '0x57a2f0', 'eur_super_cup.cpp', 'thiscall',
          'desmonta a Supercopa da Europa: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the European Super Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_rus_cup', 'RusCup::~RusCup', '0x7e9420', 'rus_cup.cpp', 'thiscall',
          'desmonta a Copa da Rússia: libera os times, as rodadas, os confrontos e as estatísticas, apaga as fases e o vetor delas, e roda o destrutor da copa',
          'destroys the Russian Cup: frees the teams, the rounds, the ties and the statistics, destroys the stages and their vector, and runs the cup destructor'),
         ('drop_swe_cup', 'SweCup::~SweCup', '0x88ce20', 'swe_cup.cpp', 'thiscall',
          'desmonta a Copa da Suécia: libera os times, as rodadas, os confrontos, as estatísticas e a reserva de clubes, apaga as fases e o vetor delas, e roda o destrutor da copa',
          'destroys the Swedish Cup: frees the teams, the rounds, the ties, the statistics and the club pool, destroys the stages and their vector, and runs the cup destructor'),
         ('drop_asia_club_champ', 'AsiaClubChamp::~AsiaClubChamp', '0x40ad70', 'asia_club_champ.cpp', 'thiscall',
          'desmonta a Liga dos Campeões da Ásia: libera os times, as rodadas e os confrontos, apaga as fases e o vetor delas, libera as estatísticas e a lista de clubes, e roda o destrutor da copa',
          'destroys the Asian Club Championship: frees the teams, the rounds and the ties, destroys the stages and their vector, frees the statistics and the club list, and runs the cup destructor'),
         ('drop_asia_cup_winner', 'AsiaCupWinner::~AsiaCupWinner', '0x40ce70', 'asia_cup_winner.cpp', 'thiscall',
          'desmonta a Recopa da Ásia: libera os times, as rodadas e os confrontos, apaga as fases e o vetor delas, libera as estatísticas e a lista de clubes, e roda o destrutor da copa',
          "destroys the Asian Cup Winners' Cup: frees the teams, the rounds and the ties, destroys the stages and their vector, frees the statistics and the club list, and runs the cup destructor"),
         ('drop_wel_lge_cup', 'WelLgeCup::~WelLgeCup', '0x926100', 'wel_lge_cup.cpp', 'thiscall',
          'desmonta a Copa da Liga galesa: libera a tabela, as datas e os inscritos, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Welsh League Cup: frees the table, the dates and the entrants, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_wel_prm_cup', 'WelPrmCup::~WelPrmCup', '0x927a50', 'wel_prm_cup.cpp', 'thiscall',
          'desmonta a Copa da Premier galesa: libera a tabela, as datas e os inscritos, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Welsh Premier Cup: frees the table, the dates and the entrants, destroys the stages and their vector, frees the statistics and runs the league destructor')])
CAMPOS = []
INTERESSA = []
