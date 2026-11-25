# O relatorio do olheiro sobre um jogador (player_search.cpp; leitor rx06, rodada 6, 25/11/2026):
# PLAYER_SEARCH::report_on_player (0x7b62a0). Rotina inteira (0x7b62a0..0x7b7489 ret 0x1c; 0x7b748c.. enchimento; a
# 0x7b7490 e outra rotina, scout_covers_nation, ja em Rust).
# DEPENDE DO rx06_2, DO sx17_2 E DO sx17_5 (integrar junto ou depois).
# Quem chama: o jogo observado pelo olheiro (player_search_scout_match, ja em Rust, modos 0/1/2 com o jogo e o
# resultado), os pedidos de observacao do dia do SCOUT_MANAGER (ja em Rust: -1000, modo 1, sem jogo) e a lista de
# observacao (shortlist_manager). A carreira automatica (tecnico de ferias, sem olheiros) nao passa: contador 0.
# TELAS: a prova e o Lucas jogando comparando: um olheiro com tarefa observando jogos (relatorios e noticias de
# recomendacao), um pedido de observacao de um jogador ("scout player") ate o relatorio.
# Convencoes conferidas na desmontagem:
# - 0x7b62a0 thiscall(busca, jogador, registro, desvio: i32, modo: byte, jogo, resultado, lado: byte) ret 0x1c, sem
#   retorno (eax sem valor);
# - 0x53e990 pelo get_scout (inline); 0x7bca90 cdecl(clube, 0, 0); 0x4cf5a0 get_contract (duas vezes no boato, a
#   segunda sem gravar 0xae1080); 0x7f9520 record_scout_report thiscall([0xb74448], jogador, clube) ret 8;
#   0x5afde0/0x5b09d0 fog of war (com 0x9cc270); 0x5ac7a0 find_stored_fixture thiscall([0xae2a58], jogo) (so jogo da
#   competicao [0x9cf684], o de reservas; a categoria +0x3a vira 2);
# - 0x6e2f20 match_man_score_player_for_role com 15 argumentos (0x1e80 para o atual, 0x1f00 para o futuro), cada um
#   entre dois random(500): desvio - r1 + nota + r2; 0x945640 _ftol; 0x846920 interesse (1, 0, 1, 0) -> st0;
# - 0x946464 bsearch(&entrada, resultados, n, 0x2d, 0x7bce50); 0x945667 realloc (+250, a caixa fatal 0x1131 pelo
#   player_search_append_result); 0x945333 qsort;
# - o boato: 0x5a0590 get_club_finance thiscall([0xae2a38], clube); 0x53b900 cdecl; 0x600a50 cdecl(clube, 0, 1, 0, 0);
#   0x598800 get_transfer_budget thiscall(financas, jogador, 1, 0, 1, 0) -> edx:eax; 0x84b6f0 is_compensation_deal
#   thiscall([[0xde1f54]+0x856], &vaga local, contrato, clube, &taxa) (a vaga: so a data +0x1b construida, o preco +0xb
#   e a avaliacao +3); 0x4d42d0 / 0x4d1600 thiscall([0xae1050], ...); 0x754740 add_transfer_interest thiscall([0xb6b7c8],
#   jogador, clube, preco, interesse double) ret 0x14;
# - 0x8c7560 / 0x8c7950 permissao de trabalho; 0x5b7d20 cdecl(ficha) -> ax; 0x7e4920 fill_staff_name_ref; 0x7e4750
#   fill_match_record_from_fixture cdecl(jogo, clube do lado, saida); 0x77bbe0 NewsItem(0x138b, 0); 0x785790 set_slot;
#   0x7e5160 / 0x7e4de0 escrevem referencia de nome e jogo na noticia; 0x7bd9d0 traits; 0x546ab0 cdecl(jogador,
#   clube); 0x785d40 deliver_to_manager thiscall([0xb7431c], item, tecnico, 0).
# Defeitos do original: com o olheiro da lista sumido, o clube dele e lido no endereco 0x39 (o jogo cai); o byte +0x2b
# do resultado novo vem da pilha (a copia escreve 0 e o comparador o ignora); no boato, a avaliacao sorteada
# random(40000) e guardada em 16 bits (acima de 32767 fica negativa); o byte de bandeiras do resultado achado leva um
# "ou" com 0 (nao muda); o clube das bandeiras de forca da noticia passa a ser o vendedor do boato (nenhum num contrato
# de tipo 5) quando ha boato.
TELAS = True
ROTINAS = [('player_search_report_on_player', 0x7b62a0, [0x6a, 0xff, 0x68, 0xc2, 0xf5, 0x95, 0x00])]
DOCS = ('o relatorio do olheiro sobre um jogador',
        'the scout report on one player',
        [('player_search_report_on_player', 'PLAYER_SEARCH::report_on_player', '0x7b62a0', 'player_search.cpp', 'thiscall, ret 0x1c',
          'julga um jogador que o olheiro viu: olheiro sem clube, a caixa; o registro pedido comeca com o modo 0xff (sem relatorio); a meta do clube do olheiro (1750 a 35000). Jogador sem ficha: nada. Conta mais um relatorio no pedido, o tecnico passa a conhecer o jogador (com mascara de atributos) e o jogo de reservas guardado vira categoria 2. Os dois valores levam o desvio pedido e dois sorteios de 500 em volta da nota da funcao (0x1e80 o atual, 0x1f00 o futuro abaixo de 25 anos) e as mesmas correcoes da lista do olheiro; fora do modo 1, nada mais se os dois nao passam de 0,6 x a meta ou o interesse em vir fica abaixo de 3,15. O resultado do jogador e atualizado (hoje, valores, notas de 0 a 5, nota do caderno) ou acrescentado, com nota acima de 0 e tarefa de nacao, competicao, base ou regiao. Nota acima de 2 pode virar boato de transferencia (1 vez em 37 - reputacao / 300; com reputacao baixa so de clube humano; com interesse 3,15, o preco pedido pelo vendedor e, se o orcamento ou o saldo cobrem 0,75 dele, a imprensa fica sabendo). Com modo e registro, o registro e preenchido (modo 2 so com nota 4; jogo, lado, permissao de trabalho ou segunda nacionalidade); senao, para quem nao tinha 5, fora do proximo adversario e com nota 5, a noticia de recomendacao 0x138b. Por fim zera a marca do relatorio',
          'judges a player the scout saw: a scout without a club, the box; the asked record starts with mode 0xff (no report); the scout club target (1750 to 35000). A player without a sheet: nothing. One report more on the request, the manager comes to know the player (with attribute masking) and the stored reserve fixture becomes category 2. Both worths take the given bias and two draws of 500 around the role rating (0x1e80 the current one, 0x1f00 the future one under 25) and the same corrections as the scout list; outside mode 1, nothing more when neither passes 0.6 x the target or the interest in coming is below 3.15. The player result is updated (today, worths, 0..5 grades, note rating) or added, with a grade above 0 and a nation, competition, youth or region assignment. A grade above 2 may become a transfer rumour (1 time in 37 - reputation / 300; with a low reputation only from a human club; with interest 3.15, the seller asking price and, when the budget or the balance covers 0.75 of it, the media hears of it). With a mode and a record, the record is filled (mode 2 only with a 4; match, side, work permit or second nationality); otherwise, for who had no 5, outside opposition scouting and with a 5, the recommendation news 0x138b. Last it clears the report mark')])
