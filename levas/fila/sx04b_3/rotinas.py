# As partidas de uma nacao nas notas do mes e o jogador do ano pelas notas do ano (nation_awards.cpp; leitor sx04b,
# rodada 5, 23/11/2026). Rodam na carreira ENG: 0x7683a0 a cada partida da liga inglesa (pela virtual +8 da Inglaterra,
# 0x5a96b0, item sx03_4); 0x7683c0 em 6 de maio de 2002 pela diaria internacional (0x632e87, sx03_4: o jogador do ano
# mundial, premio 0, com lista curta: a lista e preenchida e anunciada). O ramo sem lista curta (o registro dos tres
# vencedores) so roda nas nacoes (Escocia, Irlanda do Norte, Coreia... com LIGAS); o comparador o confere quando roda.
# Convencoes conferidas na desmontagem:
# - 0x7683a0: thiscall(partida, registro), ret 8; MONTH_RATINGS::record_match_ratings 0x767ee0 thiscall(partida,
#   registro) ret 8 (ja Rust, cpw_5).
# - 0x7683c0: thiscall(hoje, premio, notas do ano, nacao, nacao do clube, nacao excluida, nacao dos dois, divisao, modo,
#   idade maxima), ret 0x28; malloc 0x944e46 cdecl(n * 4); YEAR_RATINGS::collect_best_players 0x931ae0 thiscall(bloco,
#   n (o dword: word da capacidade com a metade alta 0, ou 3), os 7 filtros) ret 0x24 -> ax (comparado com sinal);
#   AWARD_SHORTLIST::clear 0x418320 thiscall; add_entry 0x4178a0 thiscall(pessoa, clube, nota f32) ret 0xc; announce
#   0x417a20 thiscall(hoje, [this+0x13], [premio+0x11]) ret 0xc; YEAR_AWARD::record_winners 0x9302f0 thiscall(&registro,
#   1) ret 8; free 0x9452ca. A nota da lista: fild/fidiv/fstp f32 (a soma do modo 0, 1 ou 2, senao 2 x gols +
#   assistencias, sobre os jogos, byte com sinal); f64 arredondado a f32 da o mesmo.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe.
# Defeitos do original (0x7683c0): sem notas do ano (a caixa da linha 178) o bloco nao e liberado; no registro sem
# lista curta, o dword da nota de cada vencedor nunca e escrito (o historico guarda o lixo da pilha; como no sx04_2).
# Depende do sx04_2 (YEAR_RECORD_*, year_record_entry, record_year_award, compare_award_record, NATION_AWARDS_CPP,
# YEAR_AWARD_HISTORY_*), do sx05_4 (YearEntry) e do sx05b_5 (nation_awards_fields, award_base): integrar depois deles.
# Comparadores: 0x7683a0 so a original; 0x7683c0 com lista curta: ensaio da copia sem anunciar, a lista guardada e
# reposta, a original e o conteudo comparado (contagem, votos, entradas em qualquer ordem: o anuncio as ordena pelo
# nome); sem lista curta, como o sx04_2 (o registro mais novo do historico, sem as notas).
SAVE = False
ROTINAS = [('nation_awards_record_match_ratings', 0x7683a0, [0x8b, 0x49, 0x0a, 0x85, 0xc9]),
           ('nation_awards_award_player_of_year_from_ratings', 0x7683c0, [0x81, 0xec, 0x60, 0x02, 0x00, 0x00])]
DOCS = ('as partidas de uma nação nas notas do mês e o jogador do ano pelas notas do ano',
        "a nation's matches in its month ratings and the player of the year from the year ratings",
        [('nation_awards_record_match_ratings', 'NationAwards::record_match_ratings', '0x7683a0', 'nation_awards.cpp', 'thiscall, ret 8',
          'regra: passa cada partida da liga às notas do mês da nação, quando ela as tem',
          "rule: passes each league match to the nation's month ratings, when it has them"),
         ('nation_awards_award_player_of_year_from_ratings', 'NationAwards::award_player_of_year_from_ratings', '0x7683c0', 'nation_awards.cpp', 'thiscall, ret 0x28',
          'regra: os melhores das notas do ano pelos filtros (nação, nação do clube, nação excluída, nação dos dois, divisão, modo, idade máxima), tantos quanto cabem na lista curta do prêmio (ou 3 sem ela); se achar menos, nada. Com lista curta, ela é limpa, recebe cada candidato com a nota (a soma do modo, ou 2 x gols + assistências, por jogo) e é anunciada; sem ela, os três vencedores vão para o histórico do prêmio do ano. Defeitos: sem notas do ano o bloco não é liberado; a nota de cada vencedor no registro fica com o lixo da pilha',
          "rule: the best of the year ratings by the filters (nation, club nation, excluded nation, home nation, division, mode, maximum age), as many as the award's shortlist holds (or 3 without one); fewer found, nothing. With a shortlist, it is cleared, gets each candidate with the score (the mode's sum, or 2 x goals + assists, per match) and is announced; without one, the three winners go to the year award history. Defects: without year ratings the block is not freed; each winner's score in the record keeps stack garbage")])
CAMPOS = [('prêmios da nação +0x13 id da nação (o do anúncio da lista curta); valor; lê (nation_awards_award_player_of_year_from_ratings)',
           "nation awards +0x13 nation id (the shortlist announcement's); value; reads (nation_awards_award_player_of_year_from_ratings)")]
INTERESSA = []
