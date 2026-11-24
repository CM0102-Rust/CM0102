# Os destrutores da competicao, da copa e de cinco copas, e as duas virtuais de base da copa (comp.cpp, Cup.cpp,
# bel_fa_cup.cpp, bel_super.cpp, bra_champ_cup.cpp, bra_cup.cpp, cro_cup.cpp; leitor cx01, rodada 5, 24/11/2026). Item
# de TELAS, como os destrutores do sx18_29: rodam quando o jogo e desmontado (carregar outro save ou sair da carreira);
# Comp::~Comp e Cup::~Cup rodam tambem na carreira, quando as fases das copas e das ligas sao apagadas (Cup_Stage e
# League::~League). Caminho no jogo: com um jogo aberto que tenha as ligas da Belgica, do Brasil e da Croacia
# carregadas, carregar outro save.
# - 0x48c650 Comp::~Comp thiscall() ret 0: a vtable 0x9689e8, free(+0x14), free(+0x18) e zera.
# - 0x48c600 Comp::scalar_deleting_destructor thiscall(flags) ret 4 -> this: o mesmo e, com o bit 0, operator delete.
# - 0x518690 Cup::~Cup thiscall() ret 0: a vtable 0x9693c8 e jmp Comp::~Comp (a copia chama pelo desvio).
# - 0x41d240 BelFaCup::~BelFaCup, 0x4210e0 BelSuper::~BelSuper, 0x427630 BraCup::~BraCup, 0x517670 CroCup::~CroCup
#   thiscall() ret 0 (SEH so para a base): o mesmo molde com a vtable de cada (0x9678a0, 0x967aa8, 0x967cf8, 0x9692e0):
#   free(+0xa7), free(+0xa3), os confrontos de cada rodada e o vetor (+0xad, +0x3c rodadas), Comp_Stats::~Comp_Stats
#   0x49f450 e operator delete, Cup::~Cup 0x518690.
# - 0x425b50 BraChampCup::~BraChampCup thiscall() ret 0: a vtable 0x967c44, League::free_table 0x687970 (push 0),
#   free(+0xba) e zera, o slot 0 das fases (push 1), free(+0xc) e zera, as estatisticas, League::~League 0x682300.
# - 0x5186a0 Cup::create_round_table thiscall(fase, &rodadas, &extra, &liga) ret 0x10 -> 0 e 0x518710 Cup::set_up_rounds
#   thiscall() ret 0 -> -1: a caixa do Cup.cpp (linhas 0x54 e 0x60). Toda copa derivada substitui as duas: nao ha
#   caminho no jogo (so rodariam numa copa sem a sua).
# Ficam de fora os destrutores escalares das copas (0x518670, 0x41d220, 0x4210c0, 0x425b30, 0x427610, 0x517650): o
# prologo tem 3 bytes antes de um call rel32 (push esi; mov esi, ecx; call ~X); entrariam pelas RELATIVAS (k = 3).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria ou abrem a caixa).
SAVE = False
TELAS = True
ROTINAS = [('drop_comp', 0x48c650, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x14]),
           ('delete_comp', 0x48c600, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x14]),
           ('drop_cup', 0x518690, [0xc7, 0x01, 0xc8, 0x93, 0x96, 0x00]),
           ('drop_bel_fa_cup', 0x41d240, [0x6a, 0xff, 0x68, 0x78, 0x3f, 0x95, 0x00]),
           ('drop_bel_super', 0x4210e0, [0x6a, 0xff, 0x68, 0xc8, 0x40, 0x95, 0x00]),
           ('drop_bra_cup', 0x427630, [0x6a, 0xff, 0x68, 0xf8, 0x43, 0x95, 0x00]),
           ('drop_cro_cup', 0x517670, [0x6a, 0xff, 0x68, 0xe8, 0x74, 0x95, 0x00]),
           ('drop_bra_champ_cup', 0x425b50, [0x6a, 0xff, 0x68, 0x58, 0x43, 0x95, 0x00]),
           ('cup_create_round_table', 0x5186a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cup_set_up_rounds', 0x518710, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os destrutores da competição, da copa e de cinco copas, e as virtuais de base da copa',
        "the destructors of the competition, the cup and five cups, and the cup's base virtuals",
        [('drop_comp', 'Comp::~Comp', '0x48c650', 'comp.cpp', 'thiscall',
          'desmonta a base da competição: libera o livro dos inscritos e o segundo buffer (zerado)',
          "destroys the competition base: frees the entrant book and the second buffer (cleared)"),
         ('delete_comp', 'Comp::scalar_deleting_destructor', '0x48c600', 'comp.cpp', 'thiscall, ret 4',
          'o mesmo e, com o bit 0, apaga o objeto',
          'the same and, with bit 0, deletes the object'),
         ('drop_cup', 'Cup::~Cup', '0x518690', 'Cup.cpp', 'thiscall',
          'desmonta a base da copa: a tabela virtual da copa e o destrutor da competição',
          'destroys the cup base: the cup vtable and the competition destructor'),
         ('drop_bel_fa_cup', 'BelFaCup::~BelFaCup', '0x41d240', 'bel_fa_cup.cpp', 'thiscall',
          'desmonta a Copa da Bélgica: libera os times, as rodadas, os confrontos e as estatísticas',
          'destroys the Belgian Cup: frees the teams, the rounds, the ties and the statistics'),
         ('drop_bel_super', 'BelSuper::~BelSuper', '0x4210e0', 'bel_super.cpp', 'thiscall',
          'desmonta a Supercopa da Bélgica, como a copa',
          'destroys the Belgian Super Cup, like the cup'),
         ('drop_bra_cup', 'BraCup::~BraCup', '0x427630', 'bra_cup.cpp', 'thiscall',
          'desmonta a Copa do Brasil, como a da Bélgica',
          'destroys the Brazilian Cup, like the Belgian one'),
         ('drop_cro_cup', 'CroCup::~CroCup', '0x517670', 'cro_cup.cpp', 'thiscall',
          'desmonta a Copa da Croácia, como a da Bélgica',
          'destroys the Croatian Cup, like the Belgian one'),
         ('drop_bra_champ_cup', 'BraChampCup::~BraChampCup', '0x425b50', 'bra_champ_cup.cpp', 'thiscall',
          'desmonta a Copa dos Campeões: a tabela, as datas, as fases e as estatísticas, e o destrutor da liga',
          'destroys the Champions Cup: the table, the dates, the stages and the statistics, and the league destructor'),
         ('cup_create_round_table', 'Cup::create_round_table', '0x5186a0', 'Cup.cpp', 'thiscall, ret 0x10',
          'a versão de base da tabela de rodadas, que toda copa substitui: a caixa de erro e nenhuma tabela',
          'the base version of the round table, which every cup overrides: the error box and no table'),
         ('cup_set_up_rounds', 'Cup::set_up_rounds', '0x518710', 'Cup.cpp', 'thiscall',
          'a versão de base da montagem das rodadas, que toda copa substitui: a caixa de erro e −1',
          'the base version of the round set-up, which every cup overrides: the error box and −1')])
CAMPOS = []
INTERESSA = []
