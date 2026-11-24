# Doze destrutores escalares de copas (leitor cx03, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e
# desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que
# tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao; as continentais, as ligas do
# continente), carregar outro save, jogando comparando.
# Os destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): o prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (endereco, 3) do
# rust/src/detours.rs, ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo desvio (delete_through do cx03_3; integrar depois dele).
# Convencoes conferidas: destrutores thiscall, ret 0 (mov ecx -> call pelo destrutor escalar); o free do CRT 0x9452ca
# (cdecl), Comp_Stats::~Comp_Stats 0x49f450 (thiscall), operator delete 0x944c94 (cdecl), Cup::~Cup 0x518690 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (push 0, thiscall, ret 4), o destrutor
# escalar de cada fase (slot 0 com 1). O quadro SEH so cobre o desenrolar de excecao (nao portado).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('delete_spa_super', 0x858870, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_tur_cup', 0x8fc580, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_usa_mls_all_stars', 0x90a310, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_usa_open_cup', 0x90b080, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_wel_cup', 0x9245f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_asia_super_cup', 0x410820, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_con_champ', 0x4c0e10, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eur_super_cup', 0x57a2d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_rus_cup', 0x7e9400, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_swe_cup', 0x88ce00, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_asia_club_champ', 0x40ad50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_asia_cup_winner', 0x40ce50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('doze destrutores escalares de copas',
        'twelve cup scalar deleting destructors',
        [('delete_spa_super', 'SpaSuper::scalar_deleting_destructor', '0x858870', 'spa_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Espanha e, com o bit 0, apaga o objeto',
          'runs the destructor of the Spanish Super Cup and, with bit 0, deletes the object'),
         ('delete_tur_cup', 'TurCup::scalar_deleting_destructor', '0x8fc580', 'tur_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Turquia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Turkish Cup and, with bit 0, deletes the object'),
         ('delete_usa_mls_all_stars', 'UsaMlsAllStars::scalar_deleting_destructor', '0x90a310', 'usa_mls_all_stars.cpp', 'thiscall, ret 4',
          'roda o destrutor do Jogo das Estrelas da MLS e, com o bit 0, apaga o objeto',
          'runs the destructor of the MLS All-Star Game and, with bit 0, deletes the object'),
         ('delete_usa_open_cup', 'UsaOpenCup::scalar_deleting_destructor', '0x90b080', 'usa_open_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da US Open Cup e, com o bit 0, apaga o objeto',
          'runs the destructor of the US Open Cup and, with bit 0, deletes the object'),
         ('delete_wel_cup', 'WelCup::scalar_deleting_destructor', '0x9245f0', 'wel_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa do País de Gales e, com o bit 0, apaga o objeto',
          'runs the destructor of the Welsh Cup and, with bit 0, deletes the object'),
         ('delete_asia_super_cup', 'AsiaSuperCup::scalar_deleting_destructor', '0x410820', 'asia_super_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Ásia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Asian Super Cup and, with bit 0, deletes the object'),
         ('delete_con_champ', 'ConChamp::scalar_deleting_destructor', '0x4c0e10', 'con_champ.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa dos Campeões da CONCACAF e, com o bit 0, apaga o objeto',
          "runs the destructor of the CONCACAF Champions' Cup and, with bit 0, deletes the object"),
         ('delete_eur_super_cup', 'EurSuperCup::scalar_deleting_destructor', '0x57a2d0', 'eur_super_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Europa e, com o bit 0, apaga o objeto',
          'runs the destructor of the European Super Cup and, with bit 0, deletes the object'),
         ('delete_rus_cup', 'RusCup::scalar_deleting_destructor', '0x7e9400', 'rus_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Rússia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Russian Cup and, with bit 0, deletes the object'),
         ('delete_swe_cup', 'SweCup::scalar_deleting_destructor', '0x88ce00', 'swe_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Suécia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Swedish Cup and, with bit 0, deletes the object'),
         ('delete_asia_club_champ', 'AsiaClubChamp::scalar_deleting_destructor', '0x40ad50', 'asia_club_champ.cpp', 'thiscall, ret 4',
          'roda o destrutor da Liga dos Campeões da Ásia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Asian Club Championship and, with bit 0, deletes the object'),
         ('delete_asia_cup_winner', 'AsiaCupWinner::scalar_deleting_destructor', '0x40ce50', 'asia_cup_winner.cpp', 'thiscall, ret 4',
          'roda o destrutor da Recopa da Ásia e, com o bit 0, apaga o objeto',
          "runs the destructor of the Asian Cup Winners' Cup and, with bit 0, deletes the object")])
CAMPOS = []
INTERESSA = []
