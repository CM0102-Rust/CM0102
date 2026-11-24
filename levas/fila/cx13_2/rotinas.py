# Os destrutores das ligas russas (Primeira Liga e Primeira Divisao) e escocesas (Premier, First, Second e Third)
# (rus_first.cpp, rus_prm.cpp, sco_first.cpp, sco_prm.cpp, sco_second.cpp, sco_third.cpp; leitor cx13, rodada 5,
# 24/11/2026). Item de TELAS, como os destrutores do cx10_1. Caminho no jogo: com uma carreira aberta que tenha a Russia
# e a Escocia carregadas, carregar outro save, jogando comparando.
# - 0x7ea160 RusFirst::~RusFirst: drop_stage_league_unchecked (cx10_4; o vetor das fases liberado sem teste e sem zerar).
# - 0x7eacc0 RusPrm::~RusPrm e 0x7f0df0 ScoPrm::~ScoPrm: drop_plain_league (cx10_1).
# - 0x7ee220 ScoFirst::~ScoFirst, 0x7f3480 ScoSecond::~ScoSecond e 0x7f4420 ScoThird::~ScoThird: drop_stats_league
#   (cx04_1).
# - os destrutores escalares (push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94 cdecl com o bit 0),
#   thiscall(flags) ret 4 -> this: o corpo delete_through do cx03_3 e o prologo pelas RELATIVAS (endereco, 3).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), os destrutores thiscall, ret 0 (mov ecx ->
# call pelo destrutor escalar). O quadro SEH so cobre o desenrolar de excecao. Prologo dos destrutores: push -1; push
# imm32 (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_rus_first', 0x7ea160, [0x6a, 0xff, 0x68, 0x68, 0x09, 0x96, 0x00]),
           ('delete_rus_first', 0x7ea140, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_rus_prm', 0x7eacc0, [0x6a, 0xff, 0x68, 0xb8, 0x09, 0x96, 0x00]),
           ('delete_rus_prm', 0x7eaca0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_sco_first', 0x7ee220, [0x6a, 0xff, 0x68, 0x78, 0x0b, 0x96, 0x00]),
           ('delete_sco_first', 0x7ee200, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_sco_prm', 0x7f0df0, [0x6a, 0xff, 0x68, 0x38, 0x0c, 0x96, 0x00]),
           ('delete_sco_prm', 0x7f0dd0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_sco_second', 0x7f3480, [0x6a, 0xff, 0x68, 0xb8, 0x0c, 0x96, 0x00]),
           ('delete_sco_second', 0x7f3460, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_sco_third', 0x7f4420, [0x6a, 0xff, 0x68, 0x08, 0x0d, 0x96, 0x00]),
           ('delete_sco_third', 0x7f4400, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das ligas russas e escocesas',
        'the destructors of the Russian and Scottish leagues',
        [('drop_rus_first', 'RusFirst::~RusFirst', '0x7ea160', 'rus_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão russa: libera a tabela, a tabela de rodadas, cada fase até a atual e o vetor das fases (sem testar nem zerar), as estatísticas, e roda o destrutor da liga',
          'destroys the Russian First Division: frees the table, the round table, every stage up to the current one and the stage vector (untested, not cleared), the statistics, and runs the league destructor'),
         ('delete_rus_first', 'RusFirst::scalar_deleting_destructor', '0x7ea140', 'rus_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão russa e, com o bit 0, libera o objeto',
          'runs the Russian First Division destructor and, with bit 0, frees the object'),
         ('drop_rus_prm', 'RusPrm::~RusPrm', '0x7eacc0', 'rus_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Liga russa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Russian Premier League, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_rus_prm', 'RusPrm::scalar_deleting_destructor', '0x7eaca0', 'rus_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Liga russa e, com o bit 0, libera o objeto',
          'runs the Russian Premier League destructor and, with bit 0, frees the object'),
         ('drop_sco_first', 'ScoFirst::~ScoFirst', '0x7ee220', 'sco_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão escocesa: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Scottish First Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_sco_first', 'ScoFirst::scalar_deleting_destructor', '0x7ee200', 'sco_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão escocesa e, com o bit 0, libera o objeto',
          'runs the Scottish First Division destructor and, with bit 0, frees the object'),
         ('drop_sco_prm', 'ScoPrm::~ScoPrm', '0x7f0df0', 'sco_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Liga escocesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Scottish Premier Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_sco_prm', 'ScoPrm::scalar_deleting_destructor', '0x7f0dd0', 'sco_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Liga escocesa e, com o bit 0, libera o objeto',
          'runs the Scottish Premier Division destructor and, with bit 0, frees the object'),
         ('drop_sco_second', 'ScoSecond::~ScoSecond', '0x7f3480', 'sco_second.cpp', 'thiscall, ret 0',
          'desmonta a Segunda Divisão escocesa: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Scottish Second Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_sco_second', 'ScoSecond::scalar_deleting_destructor', '0x7f3460', 'sco_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão escocesa e, com o bit 0, libera o objeto',
          'runs the Scottish Second Division destructor and, with bit 0, frees the object'),
         ('drop_sco_third', 'ScoThird::~ScoThird', '0x7f4420', 'sco_third.cpp', 'thiscall, ret 0',
          'desmonta a Terceira Divisão escocesa: libera a tabela, a tabela de rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Scottish Third Division: frees the table, the round table, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_sco_third', 'ScoThird::scalar_deleting_destructor', '0x7f4400', 'sco_third.cpp', 'thiscall, ret 4',
          'roda o destrutor da Terceira Divisão escocesa e, com o bit 0, libera o objeto',
          'runs the Scottish Third Division destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
