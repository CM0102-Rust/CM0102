# Os destrutores das ligas espanholas (Primeira, a divisao inferior generica e Segunda) (spa_first.cpp, spa_lower.cpp,
# spa_second.cpp; leitor cx13, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx10_1. A Espanha esta nas 8
# ligas. Caminho no jogo: com uma carreira aberta que tenha a Espanha carregada, carregar outro save, jogando comparando.
# - 0x84e3b0 SpaFirst::~SpaFirst: o corpo da liga sem fases e, depois das estatisticas, a divisao inferior (+0xee,
#   lower_division) pelo slot 0 com 1, sem zerar (delete_owned_comp do cx10_1).
# - 0x8511b0 SpaLower::~SpaLower: as fases como no drop_stats_league, mas sem estatisticas (a divisao inferior nao tem
#   Comp_Stats).
# - 0x853bd0 SpaSecond::~SpaSecond: drop_stats_league (cx04_1).
# - os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0),
#   thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (endereco, 3).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), os destrutores thiscall, ret 0 (mov ecx ->
# call pelo destrutor escalar). O quadro SEH so cobre o desenrolar de excecao. Prologo dos destrutores: push -1; push
# imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_spa_first', 0x84e3b0, [0x6a, 0xff, 0x68, 0xc8, 0x26, 0x96, 0x00]),
           ('delete_spa_first', 0x84e390, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_spa_lower', 0x8511b0, [0x6a, 0xff, 0x68, 0x28, 0x27, 0x96, 0x00]),
           ('delete_spa_lower', 0x851190, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_spa_second', 0x853bd0, [0x6a, 0xff, 0x68, 0x78, 0x27, 0x96, 0x00]),
           ('delete_spa_second', 0x853bb0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas espanholas',
        'the destructors of the Spanish leagues',
        [('drop_spa_first', 'SpaFirst::~SpaFirst', '0x84e3b0', 'spa_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão espanhola: libera a tabela, a tabela de rodadas e as estatísticas, apaga a divisão inferior espanhola que ela criou e roda o destrutor da liga',
          'destroys the Spanish First Division: frees the table, the round table and the statistics, deletes the Spanish lower division it created and runs the league destructor'),
         ('delete_spa_first', 'SpaFirst::scalar_deleting_destructor', '0x84e390', 'spa_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão espanhola e, com o bit 0, libera o objeto',
          'runs the Spanish First Division destructor and, with bit 0, frees the object'),
         ('drop_spa_lower', 'SpaLower::~SpaLower', '0x8511b0', 'spa_lower.cpp', 'thiscall, ret 0',
          'desmonta a divisão inferior espanhola: libera a tabela, a tabela de rodadas, cada fase até a atual e o vetor das fases e roda o destrutor da liga (ela não tem estatísticas)',
          'destroys the Spanish lower division: frees the table, the round table, every stage up to the current one and the stage vector and runs the league destructor (it has no statistics)'),
         ('delete_spa_lower', 'SpaLower::scalar_deleting_destructor', '0x851190', 'spa_lower.cpp', 'thiscall, ret 4',
          'roda o destrutor da divisão inferior espanhola e, com o bit 0, libera o objeto',
          'runs the Spanish lower division destructor and, with bit 0, frees the object'),
         ('drop_spa_second', 'SpaSecond::~SpaSecond', '0x853bd0', 'spa_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão espanhola: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Spanish Second Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_spa_second', 'SpaSecond::scalar_deleting_destructor', '0x853bb0', 'spa_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão espanhola e, com o bit 0, libera o objeto',
          'runs the Spanish Second Division destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
