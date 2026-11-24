# Os codigos de progresso e a noticia do vencedor de periodo da Primeira Divisao holandesa (hol_first.cpp; leitor cx10,
# rodada 5, 24/11/2026). Competicao de outra nacao: o progresso a cada rodada da Eerste Divisie (e nos grupos da
# repescagem, fases 4 e 5, no fim da temporada); a noticia quando um periodo termina (update_positions_and_status
# 0x5f6d20, original; os periodos da Eerste Divisie fecham ao longo da temporada).
# - 0x5f73d0 HolFirst::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (slot SET_CLUB_PROGRESS):
#   Comp_Stats::find_club_entry 0x4a4850 (mov ecx, [this+8]; push clube -> thiscall, ret 4); a situacao do clube na
#   tabela (+0xb1, +0x3e linhas de 0x3b bytes, situacao em +0x37; sem linha, 0xff). Fase -1: vencedor de periodo
#   (situacao 1) nao grava; melhor 2..7 vira 2; pior 2..7 vira 7. Fases 4 e 5: clube da Eredivisie (divisao do clube
#   +0x57 = [0x9cf568]) grava na entrada das estatisticas da Eredivisie ([0xadadfc][id], +8) com 1 -> 15 e 2..4 -> 18;
#   clube da Eerste Divisie soma 1 a cada codigo. Outras fases: como vem. Prologo push ebx; push ebp; push esi; push
#   edi; mov edi, [esp+0x14] (8 bytes).
# - 0x5f7330 HolFirst::add_period_winner_news thiscall(clube, periodo) ret 8: noticia 0x12 com o id do clube, o periodo
#   (byte com sinal) e o id da competicao, aos tecnicos da divisao (News::send_to_division_managers 0x7862d0: ecx =
#   [0xb7431c]; push comp; push &noticia -> thiscall, ret 8). Prologo push -1; push imm32 (7 bytes; o quadro SEH so
#   cobre o desenrolar de excecao do NewsItem local).
# Iguais no cm0102-1080.exe (inteiras).
# Comparadores: o progresso lado a lado sobre os tres bytes da entrada desta liga e, para clube da Eredivisie, da
# entrada dela; a noticia so a original (envia; nao pode rodar duas vezes).
# Sem dependencias na fila: usa so o que ja e Rust (CompStatsClub, exe::comp_stats_find_club_entry).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ROTINAS = [('hol_first_set_club_progress', 0x5f73d0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('hol_first_add_period_winner_news', 0x5f7330, [0x6a, 0xff, 0x68, 0xbb, 0xad, 0x95, 0x00])]
DOCS = ('os códigos de progresso e a notícia do vencedor de período da Primeira Divisão holandesa',
        'the Dutch First Division\'s progress codes and period winner news',
        [('hol_first_set_club_progress', 'HolFirst::set_club_progress', '0x5f73d0', 'hol_first.cpp', 'thiscall, ret 0x14',
          'guarda os três códigos de progresso do clube pela fase: na liga, o vencedor de período não grava, o melhor de 2 a 7 vira 2 e o pior de 2 a 7 vira 7; nos grupos da repescagem, o clube da Eredivisie grava nas estatísticas dela (1 vira 15, 2 a 4 viram 18) e o da Eerste Divisie soma 1 a cada código',
          'stores the club\'s three progress codes by the stage: in the league, a period winner is skipped, the best 2 to 7 becomes 2 and the worst 2 to 7 becomes 7; in the play-off groups, a Premier Division club stores them in its statistics (1 becomes 15, 2 to 4 become 18) and a First Division club adds 1 to each code'),
         ('hol_first_add_period_winner_news', 'HolFirst::add_period_winner_news', '0x5f7330', 'hol_first.cpp', 'thiscall, ret 8',
          'envia aos técnicos da divisão a notícia do clube que venceu um período, com o período e a competição',
          'sends the managers of the division the news of the club that won a period, with the period and the competition')])
CAMPOS = []
INTERESSA = []
