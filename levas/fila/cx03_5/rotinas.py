# Cinco destrutores escalares de copas (leitor cx03, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e
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
ROTINAS = [('delete_wel_lge_cup', 0x9260e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_wel_prm_cup', 0x927a30, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_con_merc_cup', 0x4c1880, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_conmebol_liber', 0x4c30e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_conmebol_merc', 0x4c4850, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('cinco destrutores escalares de copas',
        'five cup scalar deleting destructors',
        [('delete_wel_lge_cup', 'WelLgeCup::scalar_deleting_destructor', '0x9260e0', 'wel_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga galesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Welsh League Cup and, with bit 0, deletes the object'),
         ('delete_wel_prm_cup', 'WelPrmCup::scalar_deleting_destructor', '0x927a30', 'wel_prm_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Premier galesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Welsh Premier Cup and, with bit 0, deletes the object'),
         ('delete_con_merc_cup', 'ConMercCup::scalar_deleting_destructor', '0x4c1880', 'con_merc_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa Merconorte e, com o bit 0, apaga o objeto',
          'runs the destructor of the Copa Merconorte and, with bit 0, deletes the object'),
         ('delete_conmebol_liber', 'ConmebolLiber::scalar_deleting_destructor', '0x4c30e0', 'conmebol_liber.cpp', 'thiscall, ret 4',
          'roda o destrutor da Libertadores e, com o bit 0, apaga o objeto',
          'runs the destructor of the Copa Libertadores and, with bit 0, deletes the object'),
         ('delete_conmebol_merc', 'ConmebolMerc::scalar_deleting_destructor', '0x4c4850', 'conmebol_merc.cpp', 'thiscall, ret 4',
          'roda o destrutor da Mercosul e, com o bit 0, apaga o objeto',
          'runs the destructor of the Copa Mercosur and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
