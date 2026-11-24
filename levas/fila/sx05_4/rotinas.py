# Os coletores das notas do ano (award/year_ratings.cpp; leitor sx05, rodada 5, 23/11/2026): o esvaziamento do ano
# (YEAR_RATINGS::reset_if_year 0x931aa0), os melhores jogadores com filtros (collect_best_players 0x931ae0, chamado
# por NationAwards::award_player_of_year_from_ratings 0x7683c0) e os candidatos por continente e posicao
# (collect_candidates 0x931d50, chamado por nation_awards_award_top_three_from_ratings 0x768720). Usam o YearEntry
# do sx05_2 e chamam as ordens dos sx05_2/sx05_3 pelo endereco original (o qsort passa pelos desvios): integrar junto.
# Convencoes conferidas na desmontagem:
# - 0x931aa0 thiscall(this, &ano (short)) ret 4; o ano da tabela em +0xc (word, gravado pelo construtor 0x931360);
#   free 0x9452ca; chamada por 0x413588/0x413595 (AWARD_MANAGER, a virada do ano);
# - 0x931ae0 thiscall(this, saida, max (word), nacao, nacao do clube, nacao excluida, nacao de casa, divisao, modo
#   (byte), idade maxima (byte, 0xff nenhuma)) ret 0x24, devolve ax; o modo e o contador do laco no mesmo slot;
# - 0x931d50 thiscall(this, saida, max (word), continente, mascara de posicoes (word), modo (byte), lista excluida)
#   ret 0x18, devolve ax; 0x5481a0 staff_get_natural_position_mask cdecl(pessoa); 0x68d1b0 LLIST::get_next thiscall;
#   o iterador da lista rebobinado com [lista+0xa] = [lista+2]; KEY_CONTINENT_SOUTH_AMERICA (0x9cfa1c);
# - qsort 0x945333 cdecl(tabela, conta, 0x15, ordem). O ramo `indice >= conta` no topo do laco (0x931bd4, 0x931e0f,
#   devolve a conta) nao e alcancavel: o laco ja para antes; a copia nao o tem.
# Os premios de jogador do ano rodam no fim da temporada: LIGAS = 'ENG' e ATE = '2002-08-01', como o mwg_5.
# Comparadores: os coletores lado a lado (a tabela ordenada, as max vagas da saida, o iterador da lista) e a conta;
# reset_if_year so a original (libera a tabela) e o cabecalho conferido contra o plano.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_ratings_reset_if_year', 0x931aa0, [0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08]),
           ('year_ratings_collect_best_players', 0x931ae0, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('year_ratings_collect_candidates', 0x931d50, [0x83, 0xec, 0x08, 0x8a, 0x44, 0x24, 0x1c])]
_COLETA_PT = 'até max entradas na ordem, em passadas do mínimo de partidas para baixo de 5 em 5 (enquanto acima de 0), cada uma pegando as entradas com partidas suficientes que ainda não estão nas max vagas da saída e passam nos filtros, até max; devolve quantas. Defeito do original: a conferência de "já pega" lê as max vagas da saída, não só as preenchidas'
_COLETA_EN = 'up to max entries in order, in passes from the minimum matches down in steps of 5 (while above 0), each taking the entries with enough matches that are not yet among the max output slots and pass the filters, until max; returns how many. Defect of the original: the "already taken" check reads all max output slots, not only the filled ones'
DOCS = ('os coletores das notas do ano',
        'the year ratings collectors',
        [('year_ratings_reset_if_year', 'YEAR_RATINGS::reset_if_year', '0x931aa0', 'award/year_ratings.cpp', 'thiscall, ret 4',
          'libera as entradas e zera a conta e a capacidade quando o ano dado é o da tabela',
          'frees the entries and zeroes the count and capacity when the given year is the table\'s'),
         ('year_ratings_collect_best_players', 'YEAR_RATINGS::collect_best_players', '0x931ae0', 'award/year_ratings.cpp', 'thiscall, ret 0x24',
          'ordena a tabela pelo modo (0 a 5) e coleta ' + _COLETA_PT + '. Mínimo de 25 partidas, 15 no ano civil de 2001; filtros (−1 ou 0xff nenhum): a nacionalidade; a nação do clube; a nacionalidade excluída; a nação do clube e a nacionalidade iguais à de casa; a idade máxima; a divisão do clube',
          'sorts the table by mode (0 to 5) and collects ' + _COLETA_EN + '. Minimum 25 matches, 15 in calendar year 2001; filters (−1 or 0xff none): the nationality; the club\'s nation; the excluded nationality; the club\'s nation and the nationality both the home nation; the maximum age; the club\'s division'),
         ('year_ratings_collect_candidates', 'YEAR_RATINGS::collect_candidates', '0x931d50', 'award/year_ratings.cpp', 'thiscall, ret 0x18',
          'ordena a tabela pelo modo e coleta ' + _COLETA_PT + '. Mínimo de 25 partidas; com continente, a nação da pessoa nele (sem nacionalidade passa), e na América do Sul também o clube numa nação sul-americana; com máscara de posições, quem tem ficha precisa de uma posição natural nela; quem está na lista excluída (pelo id) fica de fora',
          'sorts the table by mode and collects ' + _COLETA_EN + '. Minimum 25 matches; with a continent, the person\'s nation in it (no nationality passes), and for South America also a club of a South American nation; with a position mask, someone with a sheet needs a natural position in it; anyone in the excluded list (by id) is left out')])
CAMPOS = []
INTERESSA = []
