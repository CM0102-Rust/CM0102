# Os destrutores das ligas polonesas (Primeira e Segunda Divisao) e portuguesas (Primeira Liga, Segunda Liga e Segunda
# Divisao B) (pol_first.cpp, pol_second.cpp, por_prm.cpp, por_second.cpp, por_second_b.cpp; leitor cx13, rodada 5,
# 24/11/2026). Item de TELAS, como os destrutores do cx10_1: rodam quando o jogo e desmontado (carregar outro save com
# um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que tenha a Polonia e Portugal
# carregados (a Segunda Divisao B so com a base NORMAL ou maior), carregar outro save, jogando comparando.
# - 0x7c8d50 PolFirst::~PolFirst: o corpo da liga sem fases com, antes da tabela de rodadas, free(+0xee), a lista dos
#   16 clubes do set_predicted_positions, sem zerar; o vetor das fases nao e liberado aqui.
# - 0x7cb410 PolSecond::~PolSecond e 0x7ce950 PorPrm::~PorPrm: drop_plain_league (cx10_1), conferidos instrucao a
#   instrucao.
# - 0x7d0b80 PorSecond::~PorSecond e 0x7d1b80 PorSecondB::~PorSecondB: drop_stats_league (cx04_1; fases testadas e
#   zeradas).
# - os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0),
#   thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (endereco, 3).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), os destrutores thiscall, ret 0 (mov ecx ->
# call pelo destrutor escalar). O quadro SEH so cobre o desenrolar de excecao. Prologo dos destrutores: push -1; push
# imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_pol_first', 0x7c8d50, [0x6a, 0xff, 0x68, 0x58, 0xf7, 0x95, 0x00]),
           ('delete_pol_first', 0x7c8d30, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_pol_second', 0x7cb410, [0x6a, 0xff, 0x68, 0x58, 0xf8, 0x95, 0x00]),
           ('delete_pol_second', 0x7cb3f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_por_prm', 0x7ce950, [0x6a, 0xff, 0x68, 0x48, 0xfa, 0x95, 0x00]),
           ('delete_por_prm', 0x7ce930, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_por_second', 0x7d0b80, [0x6a, 0xff, 0x68, 0x98, 0xfa, 0x95, 0x00]),
           ('delete_por_second', 0x7d0b60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_por_second_b', 0x7d1b80, [0x6a, 0xff, 0x68, 0xe8, 0xfa, 0x95, 0x00]),
           ('delete_por_second_b', 0x7d1b60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas polonesas e portuguesas',
        'the destructors of the Polish and Portuguese leagues',
        [('drop_pol_first', 'PolFirst::~PolFirst', '0x7c8d50', 'pol_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão polonesa: libera a tabela, a lista dos 16 clubes (sem zerar), a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Polish First Division: frees the table, the list of the 16 clubs (not cleared), the round table and the statistics and runs the league destructor'),
         ('delete_pol_first', 'PolFirst::scalar_deleting_destructor', '0x7c8d30', 'pol_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão polonesa e, com o bit 0, libera o objeto',
          'runs the Polish First Division destructor and, with bit 0, frees the object'),
         ('drop_pol_second', 'PolSecond::~PolSecond', '0x7cb410', 'pol_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão polonesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Polish Second Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_pol_second', 'PolSecond::scalar_deleting_destructor', '0x7cb3f0', 'pol_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão polonesa e, com o bit 0, libera o objeto',
          'runs the Polish Second Division destructor and, with bit 0, frees the object'),
         ('drop_por_prm', 'PorPrm::~PorPrm', '0x7ce950', 'por_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Liga portuguesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Portuguese Premier Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_por_prm', 'PorPrm::scalar_deleting_destructor', '0x7ce930', 'por_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Liga portuguesa e, com o bit 0, libera o objeto',
          'runs the Portuguese Premier Division destructor and, with bit 0, frees the object'),
         ('drop_por_second', 'PorSecond::~PorSecond', '0x7d0b80', 'por_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Liga portuguesa: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Portuguese Second Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_por_second', 'PorSecond::scalar_deleting_destructor', '0x7d0b60', 'por_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Liga portuguesa e, com o bit 0, libera o objeto',
          'runs the Portuguese Second Division destructor and, with bit 0, frees the object'),
         ('drop_por_second_b', 'PorSecondB::~PorSecondB', '0x7d1b80', 'por_second_b.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão B portuguesa: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Portuguese Second Division B: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_por_second_b', 'PorSecondB::scalar_deleting_destructor', '0x7d1b60', 'por_second_b.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão B portuguesa e, com o bit 0, libera o objeto',
          'runs the Portuguese Second Division B destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
