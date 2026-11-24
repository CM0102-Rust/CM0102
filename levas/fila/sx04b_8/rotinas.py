# O jogador do ano pelas estatisticas da temporada e as ordens dos candidatos (nation_awards.cpp; leitor sx04b, rodada
# 5, 23/11/2026). 0x769380 e chamada pelos premios de nove nacoes (Finlandia 0x5a973f.., Alemanha 0x5dfdce.., 0x5ecdb9,
# Coreia 0x66e545.., Noruega 0x79431f.., Finlandia antiga 0x79db8f.., Russia 0x7ec657, Escocia 0x7f5e73.., Suecia
# 0x893cb5.., 0x902027); na carreira com LIGAS = 'ENG SWE FIN NOR POR' roda no dia dos premios da Suecia (1o de
# dezembro de 2001, sx05b_12) e da Noruega (20 de outubro, se for dentro da janela); as ordens 0x76a480/0x76a4b0 so
# pelo qsort dela (0x7696b2/0x76969b). Mesmas ligas do sx05b_12, para entrar na mesma leva.
# Convencoes conferidas na desmontagem:
# - 0x769380: thiscall(hoje, premio, nacao do clube, nacao, nacao excluida, nacao propria ou do clube, idade (byte),
#   posicoes (word), modo (byte)), ret 0x24; ecx sem uso; os ids de nacao > -1 viram registros (G_NATION_TABLE +
#   id x 0x122); malloc 0x944e46 cdecl([0xae2364] x 10); PlayerStats::get_stat 0x7c1e30 thiscall([0xb74350]; ficha,
#   categoria (o dword com o byte 2 ou 6 e o resto da pilha), modo, f32 6,5) -> st0 (Rust): fst f32; no modo 2 o st0
#   comparado com f32 0,0 (0x968970); os jogos por _ftol do get_stat(ficha, categoria, 1, 6,5), guardados em word; no
#   modo 3, fild jogos / fdivr da nota f32 -> f32 (0 sem jogos); fora do modo 2, a nota x (peso / 2) (fild, fmul, fstp
#   f32), o peso (reputacao do clube / 500 + reputacao da divisao x 4) / 5 em inteiros com sinal truncados, ou 1;
#   qsort 0x945333(bloco, n, 10, 0x76a480 no modo 3 ou 0x76a4b0); as passadas com fracao f32 0,66 (0,01 nos modos 2
#   e 5), minimo _ftol(fracao x f32 25,0 (0x96af88)) comparado em 32 bits, fsub f64 0,1 guardado em f32, o teste
#   fracao > 0 no comeco de cada passada; staff_get_natural_position_mask 0x5481a0 cdecl -> ax (com 4: fora quem tem o
#   bit 2); no modo 2 a nota _ftol vai para o registro; YEAR_AWARD::record_winners 0x9302f0 (&registro, 1) ret 8.
# - 0x76a480 e 0x76a4b0: cdecl(a, b) do qsort; fcomp da nota f32 (+4): C0 (menor ou sem ordem) e C0|C3.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe (os das ordens com 8 bytes: duas instrucoes de 4).
# Defeitos do original (0x769380): sem nenhum candidato o bloco nao e liberado; fora do modo 2 o dword da nota de cada
# vencedor no registro nunca e escrito (lixo da pilha no historico).
# Depende do sx04_2 (YEAR_RECORD_*, year_record_entry, record_year_award, compare_award_record, NATION_AWARDS_CPP,
# YEAR_AWARD_HISTORY_*): integrar depois dele.
# Comparadores: 0x769380 em ensaio sem o registro, depois a original; o registro mais novo do historico tem de ser o
# planejado (as notas so no modo 2); as ordens por compare_order!.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('nation_awards_award_player_of_year_by_stats', 0x769380, [0x81, 0xec, 0x90, 0x02, 0x00, 0x00]),
           ('nation_awards_compare_score_ascending', 0x76a480, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('nation_awards_compare_score_descending', 0x76a4b0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('o jogador do ano pelas estatísticas da temporada e as ordens dos candidatos',
        "a nation's player of the year by the season statistics and the orders of its candidates",
        [('nation_awards_award_player_of_year_by_stats', 'nation_awards_award_player_of_year_by_stats', '0x769380', 'nation_awards.cpp', 'thiscall, ret 0x24',
          'regra: cada jogador cujo clube (se houver) joga numa competição em andamento e que passa nos filtros de nação (a do clube, a própria, a excluída, a própria ou a do clube) recebe uma nota pela estatística do modo (categoria 2 no modo 2, senão 6) e os seus jogos; no modo 2 nota 0 fica de fora; no modo 3 a nota é por jogo; fora do modo 2 ela é multiplicada por metade do peso da reputação ((reputação do clube / 500 + reputação da divisão x 4) / 5, ou 1). Ordenados pela nota (menor primeiro no modo 3), os três primeiros que jogam numa das posições (zagueiro que também é lateral não conta), têm jogos bastantes (fração de 25 jogos: 0,66, ou 0,01 nos modos 2 e 5, baixando 0,1) e passam na regra da idade vão para o histórico do prêmio. Defeitos: sem candidato o bloco não é liberado; fora do modo 2 a nota do registro fica com o lixo da pilha',
          "rule: each player whose club (if any) plays in a running competition and who passes the nation filters (the club's, his own, the excluded one, his own or his club's) gets a score from the mode's statistic (category 2 in mode 2, else 6) and his games; in mode 2 a score of 0 leaves him out; in mode 3 the score is per game; outside mode 2 it is multiplied by half the reputation weight ((club reputation / 500 + division reputation x 4) / 5, or 1). Sorted by score (lowest first in mode 3), the first three who play one of the positions (a centre back who also plays full back does not count), have enough games (a share of 25 games: 0.66, or 0.01 in modes 2 and 5, down by 0.1) and pass the age rule go to the award history. Defects: without a candidate the block is not freed; outside mode 2 the record's score keeps stack garbage"),
         ('nation_awards_compare_score_ascending', 'nation_awards_compare_score_ascending', '0x76a480', 'nation_awards.cpp', 'cdecl',
          'regra: a ordem dos candidatos pela nota, a menor primeiro (sem ordem conta como menor)',
          'rule: the candidates by score, the lowest first (unordered counts as lower)'),
         ('nation_awards_compare_score_descending', 'nation_awards_compare_score_descending', '0x76a4b0', 'nation_awards.cpp', 'cdecl',
          'regra: a ordem dos candidatos pela nota, a maior primeiro (sem ordem conta como menor)',
          'rule: the candidates by score, the highest first (unordered counts as lower)')])
CAMPOS = [('candidato do prêmio pelas estatísticas (10 bytes): +0 pessoa, +4 nota (f32), +8 jogos (word); valores; grava e lê (nation_awards_award_player_of_year_by_stats e as ordens)',
           'statistics award candidate (10 bytes): +0 person, +4 score (f32), +8 games (word); values; writes and reads (nation_awards_award_player_of_year_by_stats and the orders)')]
INTERESSA = []
