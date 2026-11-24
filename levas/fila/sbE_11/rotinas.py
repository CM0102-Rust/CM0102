# Os passos da Regionalliga na virada alema (ger_first.cpp; leitor sbE, rodada 5, 24/11/2026). So rodam com a
# Regionalliga carregada (GerFirst::start_new_season 0x5db4a0, mwc_21: com o objeto de 0x9cf564), o que pede a base
# NORMAL (a Regionalliga e divisao inferior; o simular.py fixa MINIMUM, como nos itens de C2 do cx11): 0 chamadas na
# cobertura do mundo ate 2002-08-01. BASE = 'NORMAL', que nenhuma ferramenta le ainda: passar a chave ao fila.py, ao
# cadeia.sh e ao simular.py antes de integrar.
# - 0x5db980 ger_first_exchange_regional_lower_division_clubs thiscall() ret 0 (chamada direta, ecx = a Bundesliga): os
#   clubes alemaes (+0x53 = 0x9cf314) sem divisao ou na divisao inferior (0x9cf764), fora os de parceiro (0x540a50, a
#   marca 0) sem divisao, na Regionalliga ou na inferior, ate 200, numa lista na pilha; as linhas de status 3 da
#   Regionalliga e da sua fase 0 caem para a inferior (0x6831a0) com a divisao reserva zerada, contadas (uma a menos com
#   19 clubes na Regionalliga); poucos candidatos: a caixa 0x3d8; senao comp_util_sort_and_shuffle_top_clubs 0x4afbd0 e
#   os primeiros sobem (0x6830b0). Prologo sub esp, 0x530 (6 bytes).
# - 0x5dbf50 ger_first_relegate_b_teams_of_relegated_clubs thiscall() ret 0 (ecx nao lido): cada linha da 2. Bundesliga
#   com parceiro (marca diferente de 0) na Regionalliga: a linha dele na secao Norte (a Regionalliga) ou Sul (a fase 0)
#   pela divisao reserva (League::find_club_row 0x68ab70); outra secao mantem a linha da volta anterior (no comeco o
#   proprio objeto da fase 0: defeito do original); sem linha, a caixa 0x4d2; o clube principal com status 3 e o parceiro
#   nao: o parceiro 3 e a noticia (0x48ece0 com ecx = a Regionalliga). Prologo sub esp, 0x210 (6 bytes).
# Sem dependencias da fila (usa o injury_club_count_without_nations ja publicado).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (movem clubes e mandam noticias).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('ger_first_exchange_regional_lower_division_clubs', 0x5db980, [0x81, 0xec, 0x30, 0x05, 0x00, 0x00]),
           ('ger_first_relegate_b_teams_of_relegated_clubs', 0x5dbf50, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('os passos da Regionalliga na virada alemã', 'the Regionalliga steps of the German turnover',
        [('ger_first_exchange_regional_lower_division_clubs', 'ger_first_exchange_regional_lower_division_clubs', '0x5db980', 'ger_first.cpp', 'thiscall, ret 0',
          'com a Regionalliga carregada, os rebaixados das duas seções caem para a divisão inferior e o mesmo número de clubes alemães da inferior (um a menos com 19 na Regionalliga), pela força com os melhores sorteados, sobe',
          'with the Regionalliga loaded, the relegated clubs of both sections go down to the lower division and as many German lower division clubs (one fewer with 19 in the Regionalliga), by strength with the best ones shuffled, go up'),
         ('ger_first_relegate_b_teams_of_relegated_clubs', 'ger_first_relegate_b_teams_of_relegated_clubs', '0x5dbf50', 'ger_first.cpp', 'thiscall, ret 0',
          'o time B da Regionalliga cujo clube principal cai da 2. Bundesliga também é rebaixado (com a notícia)',
          'a Regionalliga B team whose senior club goes down from the 2. Bundesliga is relegated too (with the news)')])
CAMPOS = []
INTERESSA = []
