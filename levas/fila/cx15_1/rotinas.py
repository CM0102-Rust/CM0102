# Os destrutores da MLS, da Liga do Pais de Gales, das eliminatorias africana e asiatica da Copa do Mundo, dos estadios e
# do ranking da UEFA (usa_mls.cpp, wel_first.cpp, wc_african_cup.cpp, wc_asia_league.cpp, stadium.cpp, uefa_seeding.cpp;
# leitor cx15, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx13_1: rodam quando o jogo e desmontado
# (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que tenha os EUA
# e o Pais de Gales carregados (as eliminatorias, os estadios e o ranking existem em todo jogo), carregar outro save,
# jogando comparando.
# - 0x907210 UsaMls::~UsaMls (vtable 0x970844): drop_stats_league (cx04_1), conferido instrucao a instrucao.
# - 0x925060 WelFirst::~WelFirst (vtable 0x970f80): drop_plain_league (cx10_1), conferido instrucao a instrucao.
# - 0x90f4f0 WcAfricanCup::~WcAfricanCup (vtable 0x970ad0): o corpo do EuropeanCup::~EuropeanCup do cx04_1 (as tabelas
#   de times e de rodadas, os confrontos de cada rodada, cada fase ate a atual, o vetor das fases testado e zerado, as
#   estatisticas) e, antes do Cup::~Cup 0x518690, free(+0xb2) da lista dos inscritos, sem teste (free(0) nao faz nada).
# - 0x911e10 WcAsiaLeague::~WcAsiaLeague (vtable 0x970b70): o corpo do drop_stats_league com free(+0xef), a lista das
#   selecoes, depois das datas das rodadas (com teste, sem zerar) e free(+0xf7), o sorteio dos grupos, depois das
#   estatisticas (com teste, sem zerar).
# - os destrutores escalares das quatro (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o
#   bit 0), thiscall(flags) ret 4 -> this: delete_through (cx03_3), prologo pelas RELATIVAS (endereco, 3).
# - 0x85b7f0 Stadium::~Stadium thiscall() ret 0: free das listas de reservas (+0) e de sedes (+0xc), cada uma so quando
#   existe, sem zerar. Chamado na desmontagem do jogo (0x830194). Prologo push esi; mov esi, ecx; mov eax, [esi] (5).
# - 0x903570 UefaSeeding::~UefaSeeding thiscall() ret 0: free da lista dos voluntarios da Intertoto (+0x43b) quando
#   existe, sem zerar. Chamado na desmontagem do jogo (0x83020a). Prologo mov eax, [ecx+0x43b] (6 bytes).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), os destrutores thiscall, ret 0 (mov ecx ->
# call). O quadro SEH so cobre o desenrolar de excecao. Prologo dos destrutores das competicoes: push -1; push imm32 (7
# bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_usa_mls', 0x907210, [0x6a, 0xff, 0x68, 0x98, 0x53, 0x96, 0x00]),
           ('delete_usa_mls', 0x9071f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_wel_first', 0x925060, [0x6a, 0xff, 0x68, 0x08, 0x5d, 0x96, 0x00]),
           ('delete_wel_first', 0x925040, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_wc_african_cup', 0x90f4f0, [0x6a, 0xff, 0x68, 0x98, 0x56, 0x96, 0x00]),
           ('delete_wc_african_cup', 0x90f4d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_wc_asia_league', 0x911e10, [0x6a, 0xff, 0x68, 0x28, 0x57, 0x96, 0x00]),
           ('delete_wc_asia_league', 0x911df0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_stadium', 0x85b7f0, [0x56, 0x8b, 0xf1, 0x8b, 0x06]),
           ('drop_uefa_seeding', 0x903570, [0x8b, 0x81, 0x3b, 0x04, 0x00, 0x00])]
DOCS = ('os destrutores da MLS, da Liga do País de Gales, das eliminatórias africana e asiática, dos estádios e do ranking da UEFA',
        'the destructors of the MLS, the League of Wales, the African and Asian qualifiers, the stadiums and the UEFA seeding',
        [('drop_usa_mls', 'UsaMls::~UsaMls', '0x907210', 'usa_mls.cpp', 'thiscall, ret 0',
          'desmonta a MLS: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the MLS: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_usa_mls', 'UsaMls::scalar_deleting_destructor', '0x9071f0', 'usa_mls.cpp', 'thiscall, ret 4',
          'roda o destrutor da MLS e, com o bit 0, libera o objeto',
          'runs the MLS destructor and, with bit 0, frees the object'),
         ('drop_wel_first', 'WelFirst::~WelFirst', '0x925060', 'wel_first.cpp', 'thiscall, ret 0',
          'desmonta a Liga do País de Gales, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the League of Wales, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_wel_first', 'WelFirst::scalar_deleting_destructor', '0x925040', 'wel_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Liga do País de Gales e, com o bit 0, libera o objeto',
          'runs the League of Wales destructor and, with bit 0, frees the object'),
         ('drop_wc_african_cup', 'WcAfricanCup::~WcAfricanCup', '0x90f4f0', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'desmonta a eliminatória africana: libera as tabelas de times e de rodadas, os confrontos de cada rodada, cada fase até a atual, o vetor das fases, as estatísticas e a lista dos inscritos e roda o destrutor da copa',
          'destroys the African qualifiers: frees the team and round tables, the ties of each round, every stage up to the current one, the stage vector, the statistics and the entrant list and runs the cup destructor'),
         ('delete_wc_african_cup', 'WcAfricanCup::scalar_deleting_destructor', '0x90f4d0', 'wc_african_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da eliminatória africana e, com o bit 0, libera o objeto',
          'runs the African qualifiers destructor and, with bit 0, frees the object'),
         ('drop_wc_asia_league', 'WcAsiaLeague::~WcAsiaLeague', '0x911e10', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'desmonta a eliminatória asiática: libera a tabela, a tabela de rodadas, a lista das seleções, cada fase até a atual, o vetor das fases, as estatísticas e o sorteio dos grupos e roda o destrutor da liga',
          'destroys the Asian qualifiers: frees the table, the round table, the national team list, every stage up to the current one, the stage vector, the statistics and the group draw and runs the league destructor'),
         ('delete_wc_asia_league', 'WcAsiaLeague::scalar_deleting_destructor', '0x911df0', 'wc_asia_league.cpp', 'thiscall, ret 4',
          'roda o destrutor da eliminatória asiática e, com o bit 0, libera o objeto',
          'runs the Asian qualifiers destructor and, with bit 0, frees the object'),
         ('drop_stadium', 'Stadium::~Stadium', '0x85b7f0', 'stadium.cpp', 'thiscall, ret 0',
          'desmonta o objeto dos estádios: libera a lista das reservas e a das sedes neutras, quando existem',
          'destroys the stadium object: frees the booking list and the neutral venue list, when they exist'),
         ('drop_uefa_seeding', 'UefaSeeding::~UefaSeeding', '0x903570', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'desmonta o ranking da UEFA: libera a lista dos voluntários da Intertoto, quando existe',
          'destroys the UEFA seeding: frees the Intertoto volunteer list, when there is one')])
CAMPOS = []
INTERESSA = []
