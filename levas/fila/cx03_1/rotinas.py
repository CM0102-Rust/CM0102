# Os destrutores de doze copas nacionais (leitor cx03, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e
# desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que
# tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao; as continentais, as ligas do
# continente), carregar outro save, jogando comparando.
# Molde das copas nacionais (drop_national_cup do cx01_14; integrar depois dele): a vtable de cada, free(+0xa7),
# free(+0xa3), os confrontos de cada rodada e o vetor (+0xad, +0x3c rodadas), Comp_Stats::~Comp_Stats e operator
# delete, Cup::~Cup. Prologo push -1; push imm32 (7 bytes).
# Convencoes conferidas: destrutores thiscall, ret 0 (mov ecx -> call pelo destrutor escalar); o free do CRT 0x9452ca
# (cdecl), Comp_Stats::~Comp_Stats 0x49f450 (thiscall), operator delete 0x944c94 (cdecl), Cup::~Cup 0x518690 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (push 0, thiscall, ret 4), o destrutor
# escalar de cada fase (slot 0 com 1). O quadro SEH so cobre o desenrolar de excecao (nao portado).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_pol_cup', 0x7c7f70, [0x6a, 0xff, 0x68, 0x08, 0xf7, 0x95, 0x00]),
           ('drop_pol_lge_cup', 0x7ca6d0, [0x6a, 0xff, 0x68, 0x08, 0xf8, 0x95, 0x00]),
           ('drop_pol_super', 0x7ccb00, [0x6a, 0xff, 0x68, 0xc8, 0xf8, 0x95, 0x00]),
           ('drop_por_cup', 0x7cdb70, [0x6a, 0xff, 0x68, 0xf8, 0xf9, 0x95, 0x00]),
           ('drop_por_super', 0x7d3020, [0x6a, 0xff, 0x68, 0x58, 0xfb, 0x95, 0x00]),
           ('drop_sco_chal_cup', 0x7ecb70, [0x6a, 0xff, 0x68, 0xd8, 0x0a, 0x96, 0x00]),
           ('drop_sco_fa_cup', 0x7ed500, [0x6a, 0xff, 0x68, 0x28, 0x0b, 0x96, 0x00]),
           ('drop_sco_lge_cup', 0x7efbe0, [0x6a, 0xff, 0x68, 0xe8, 0x0b, 0x96, 0x00]),
           ('drop_spa_cup', 0x84d150, [0x6a, 0xff, 0x68, 0x68, 0x26, 0x96, 0x00]),
           ('drop_spa_super', 0x858890, [0x6a, 0xff, 0x68, 0x98, 0x28, 0x96, 0x00]),
           ('drop_tur_cup', 0x8fc5a0, [0x6a, 0xff, 0x68, 0x98, 0x4f, 0x96, 0x00]),
           ('drop_usa_mls_all_stars', 0x90a330, [0x6a, 0xff, 0x68, 0x88, 0x54, 0x96, 0x00])]
DOCS = ('os destrutores de doze copas nacionais',
        'the destructors of twelve national cups',
        [('drop_pol_cup', 'PolCup::~PolCup', '0x7c7f70', 'pol_cup.cpp', 'thiscall',
          'desmonta a Copa da Polônia: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Polish Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_pol_lge_cup', 'PolLgeCup::~PolLgeCup', '0x7ca6d0', 'pol_lge_cup.cpp', 'thiscall',
          'desmonta a Copa da Liga polonesa: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Polish League Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_pol_super', 'PolSuper::~PolSuper', '0x7ccb00', 'pol_super.cpp', 'thiscall',
          'desmonta a Supercopa da Polônia: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Polish Super Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_por_cup', 'PorCup::~PorCup', '0x7cdb70', 'por_cup.cpp', 'thiscall',
          'desmonta a Taça de Portugal: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Portuguese Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_por_super', 'PorSuper::~PorSuper', '0x7d3020', 'por_super.cpp', 'thiscall',
          'desmonta a Supertaça de Portugal: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Portuguese Super Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_sco_chal_cup', 'ScoChalCup::~ScoChalCup', '0x7ecb70', 'sco_chal_cup.cpp', 'thiscall',
          'desmonta a Challenge Cup escocesa: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Scottish Challenge Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_sco_fa_cup', 'ScoFaCup::~ScoFaCup', '0x7ed500', 'sco_fa_cup.cpp', 'thiscall',
          'desmonta a Copa da Escócia: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Scottish Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_sco_lge_cup', 'ScoLgeCup::~ScoLgeCup', '0x7efbe0', 'sco_lge_cup.cpp', 'thiscall',
          'desmonta a Copa da Liga escocesa: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Scottish League Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_spa_cup', 'SpaCup::~SpaCup', '0x84d150', 'spa_cup.cpp', 'thiscall',
          'desmonta a Copa do Rei: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Spanish Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_spa_super', 'SpaSuper::~SpaSuper', '0x858890', 'spa_super.cpp', 'thiscall',
          'desmonta a Supercopa da Espanha: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Spanish Super Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_tur_cup', 'TurCup::~TurCup', '0x8fc5a0', 'tur_cup.cpp', 'thiscall',
          'desmonta a Copa da Turquia: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the Turkish Cup: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor"),
         ('drop_usa_mls_all_stars', 'UsaMlsAllStars::~UsaMlsAllStars', '0x90a330', 'usa_mls_all_stars.cpp', 'thiscall',
          'desmonta o Jogo das Estrelas da MLS: libera os times, as rodadas, os confrontos de cada rodada e as estatísticas, e roda o destrutor da copa',
          "destroys the MLS All-Star Game: frees the teams, the rounds, each round's ties and the statistics, and runs the cup destructor")])
CAMPOS = []
INTERESSA = []
