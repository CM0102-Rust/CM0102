# A noticia do campeao de periodo e o progresso dos clubes da Segunda Divisao belga (bel_second.cpp; leitor cx08, rodada
# 5, 24/11/2026). Simulacao com a Belgica (as ligas do cx02_21: 'ENG BEL HOL GER DEN', ate 2002-08-01): a noticia sai
# quando um periodo tem campeao (update_positions_and_status, cx08_16), o progresso a cada atualizacao de status.
# - 0x420ca0 BelSecond::send_period_winner_news thiscall(clube, periodo) ret 8: NewsItem::NewsItem 0x77bbe0 (thiscall,
#   0x11, 0) num local de 0xe0 bytes, NewsItem::set_slot 0x785790 (0: id do clube, 1: periodo com sinal, 2: id da
#   competicao), News::send_to_division_managers 0x7862d0 em [0xb7431c] com o registro +4. O quadro SEH so cobre o
#   desenrolar (nenhum destrutor no caminho normal). Prologo push -1; push imm32.
# - 0x420d40 BelSecond::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: find_club_entry 0x4a4850;
#   o status da linha do clube (0xff sem linha); fase 0xff: status 1 nao grava; melhor 2..5 -> 2, pior 2..5 -> 5; fase 3:
#   +1 nos tres; grava +0x73, +0x74, +0x75. Prologo push ebx; push ebp; mov ebp, [esp+0xc] (6 bytes).
# Comparadores: a noticia so a original; o progresso lado a lado nos 3 bytes (compare_group_championship_progress do
# cx04_5).
# Depende do cx04_5: integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_second_send_period_winner_news', 0x420ca0, [0x6a, 0xff, 0x68, 0x7b, 0x40, 0x95, 0x00]),
           ('bel_second_set_club_progress', 0x420d40, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c])]
DOCS = ('a notícia do campeão de período e o progresso na Segunda Divisão belga', 'the Belgian Second Division period winner news and club progress',
        [('bel_second_send_period_winner_news', 'BelSecond::send_period_winner_news', '0x420ca0', 'bel_second.cpp', 'thiscall, ret 8',
          'o clube que vence um terço da temporada da Segunda Divisão belga ganha a notícia (tipo 0x11) da vaga na repescagem, enviada aos técnicos da divisão',
          'a club that wins a third of the Belgian Second Division season gets the news (type 0x11) of its play-off place, sent to the managers of the division'),
         ('bel_second_set_club_progress', 'BelSecond::set_club_progress', '0x420d40', 'bel_second.cpp', 'thiscall, ret 0x14',
          'o progresso na Segunda Divisão belga: na liga, um campeão de período (marcado 1) fica como está; senão o melhor código de 2 a 5 vira 2 e o pior de 2 a 5 vira 5; no grupo da repescagem cada código sobe um; no resto, como vêm',
          'the Belgian Second Division progress: in the league a period winner (marked 1) keeps its codes; otherwise the best code 2 to 5 becomes 2 and the worst 2 to 5 becomes 5; in the play-off group each code goes up by one; elsewhere as they come')])
CAMPOS = []
INTERESSA = []
