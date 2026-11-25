# Os alvos da lista do olheiro (player_search.cpp; leitor rx06, rodada 6, 25/11/2026): PLAYER_SEARCH::build_scout_targets
# (0x7b0050). Rotina inteira (0x7b0050..0x7b0e30 ret; o resto ate 0x7b0e40 e enchimento).
# DEPENDE DO sx17_2 E DO sx17_5 (integrar junto ou depois): usa player_search_append_result e player_search_note_rating
# (sx17_5) e player_search_sort_or_free_results (sx17_2).
# Quem chama: start_scout_assignment (0x7aee5b, sx17_4) ao dar uma tarefa a um olheiro (nacao, regiao, competicao,
# base, proximo adversario). A carreira automatica (tecnico de ferias, sem olheiros) nao passa: contador 0.
# TELAS: a prova e o Lucas jogando comparando: dar tarefas a um olheiro (uma de cada tipo, inclusive "proximo
# adversario" e "base") e abrir a lista dele.
# Convencoes conferidas na desmontagem:
# - 0x7b0050 thiscall(busca) ret -> 1 ou 0 (moldura SEH so para as datas locais e a LLIST);
# - 0x7bca90 player_search_get_club_target_rating cdecl(clube, 0, 0) (ja em Rust); 0x53e990 pelo get_scout (inline no
#   original); a caixa player_search.cpp 0x4bc/0x4c6 (show_error_at) e a fatal 0x60c (realloc, pelo
#   player_search_append_result do sx17_5);
# - 0x54c5b0 Date::add_days / 0x54c770 Date::add_weeks thiscall(0xae2c90, &saida, n) ret 8 (as 8 semanas sao pedidas
#   duas vezes no original, uma para o ano e outra para o dia; a copia pede uma); 0x5acbe0 find_fixtures
#   thiscall([0xae2a58], amanha (dword cru e +2), dia e ano de +8 semanas, id do clube, LLIST, -1, -2, 1) ret 0x24;
#   0x68d020/0x68d040/0x68d1b0 LLIST;
# - 0x4cf5a0 get_contract (o resultado em 0xae1080); 0x7a7eb0 retirement_status thiscall([0xb7434c], pessoa) -> al;
#   0x8cca10 get_future_transfer thiscall([0xde1f54], pessoa, 0); 0x6e2f20 match_man_score_player_for_role cdecl com
#   15 argumentos (jogador, ficha, 0, 0, 0x1e82 ou 0x1f00, 0, 0, olheiro, -1, 0 x 6), antes dele random(500) duas vezes;
#   0x945640 _ftol (x87: 1.2, 1.1, 1.05, 0.95, 0.9, 0.85, 0.75 de .rdata); 0x846920 staff_interest_in_joining_club
#   thiscall([[0xde1f54]+0x856], pessoa, clube, 1, 0, 1, 0) e (pessoa, clube, 0, 0, 5, 0) -> st0;
#   0x7e4920 fill_staff_name_ref cdecl; 0x795750 Notes::find_note; 0x5407e0 cdecl(pessoa, olheiro, 0) -> ax;
#   0x945333 qsort com 0x7bce80, 0x7bceb0 e 0x7bce50; 0x9452ca free; 0x90cfc0 utils_random.
# Defeitos do original: o byte +0x2b de cada resultado vem da pilha sem ser iniciado (a copia escreve 0 e o comparador
# o ignora); o teste "cargo 0x10" depois de exigir cargo 11, 12, 13 ou 15 e morto; dois contadores locais (notas > 0) sao
# contados e nunca lidos; as notas atuais sao julgadas com a reputacao do clube + 10750 e as potenciais com + 10500.
TELAS = True
ROTINAS = [('player_search_build_scout_targets', 0x7b0050, [0x6a, 0xff, 0x68, 0x12, 0xf4, 0x95, 0x00])]
DOCS = ('os alvos da lista do olheiro',
        'the scout list targets',
        [('player_search_build_scout_targets', 'PLAYER_SEARCH::build_scout_targets', '0x7b0050', 'player_search.cpp', 'thiscall',
          'enche a lista de um olheiro com os jogadores que a tarefa aponta: sem olheiro (ou sem clube dele), a caixa e 0; a meta e a do clube do olheiro, presa entre 1750 e 35000; libera os resultados antigos e poe as dez marcas em i32::MAX. Seguindo o proximo adversario, o adversario e o do primeiro jogo nao jogado do clube nas proximas 8 semanas (nenhum: 0). Cada jogador (cargo de jogador, fora do clube do olheiro, com ficha, sem aposentar, sem transferencia futura e, com adversario, desse clube) recebe dois valores: o atual (sorteio de -500 a 500 mais a nota da funcao 0x1e82, bonus de atleta ate x1,2, descontos por pouca velocidade, pouco fisico e mais de 34 anos) e, abaixo de 25 anos, o futuro (o mesmo com 0x1f00; na tarefa de base, (valor + 100 x potencial) x 2/3 e descontos se o atual e baixo). Sem adversario, so entra quem passa de 0,6 x a meta num dos dois e tem interesse de pelo menos 3,15 em vir; cada um ganha as notas de 0 a 5 contra a meta e a nota do caderno do tecnico. Depois ordena pelo atual e pelo futuro; sem adversario, tira quem nao e da nacao ou regiao da tarefa (na base: menor de 20 da regiao do clube ou com futuro acima de 33333, livre ou com interesse 3,5) e quem o olheiro ja conhece (4000 de reputacao, da nacao do clube); corta ao acaso ate 50 + sorteio(habilidade do olheiro / 3 + 2 x profissionalismo + 20) e ordena por pessoa (vazia: libera). Devolve 1',
          'fills a scout list with the players its assignment points to: without a scout (or without his club), the box and 0; the target is the scout club one, held between 1750 and 35000; frees the old results and sets the ten marks to i32::MAX. Following the next opposition, the opponent is that of the club first unplayed fixture in the next 8 weeks (none: 0). Each player (a playing job, outside the scout club, with a sheet, not retiring, without a future transfer and, with an opponent, at that club) gets two worths: the current one (a draw from -500 to 500 plus the role rating 0x1e82, an athlete bonus up to x1.2, discounts for little speed, little physique and over 34 years) and, under 25, the future one (the same with 0x1f00; on a youth assignment, (worth + 100 x potential) x 2/3 and discounts when the current one is low). Without an opponent, only who passes 0.6 x the target in one of them and is at least 3.15 interested in coming goes in; each gets the 0..5 grades against the target and the manager note rating. Then it sorts by current and by future worth; without an opponent, drops who is not from the assignment nation or region (youth: under 20 from the club region or with a future worth above 33333, free or 3.5 interested) and whom the scout already knows (4000 reputation, from the club nation); cuts at random to 50 + random(scout ability / 3 + 2 x professionalism + 20) and sorts by person (empty: frees). Returns 1')])
