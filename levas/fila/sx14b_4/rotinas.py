# A visao da diretoria sobre as liberacoes de jogador e as noticias de pedidos e memorandos (manager_manager.cpp;
# leitor sx14b, rodada 5, 24/11/2026). Todas rodam por acao do tecnico humano nas telas (0 chamadas na carreira de
# teste): item de TELAS. Rotinas inteiras conferidas na desmontagem (fim no ultimo ret e a tabela de salto 0x6a87f8 da
# 0x6a8770).
# Convencoes conferidas na desmontagem:
# - 0x69b780 stdcall(clube) ret 4: chama 0x6b1d10 cdecl(clube) (open_resign_job_screen, Rust, pelo endereco);
# - 0x6ae180 stdcall(clube, pedido) ret 8 -> al; 0x6a9e20 e 0x6a8770 stdcall(opcao, buf, tamanho) ret 0xc (tamanho
#   palavra com sinal, menor que 15 nada; 0x66f4e0 langlib cdecl);
# - 0x696940, 0x697240, 0x697430 thiscall(this, jogador, clube) ret 8; 0x4cf5f0 thiscall([0xae1050], jogador, clube);
#   0x569e30 thiscall([0xae2a30], jogador, 0); 0x4d0c60 thiscall([0xae1050], jogador, clube, 1, -1, humano);
#   0x600a50 (clube, 0, 1, 0, 0) e 0x53c590 (clube, 0, 0) cdecl; x87: (reputacao x 0.2)^2 por _ftol (0x945640) e
#   reputacao x 0.8 comparada com a reputacao do jogador (fcompp); 0x6a0990 thiscall(this, jogador, clube, marcas) ret
#   0xc (pelo endereco); 0x62ed70 thiscall([0xb63d28], jogador); 0x547790 cdecl(jogador, clube); 0x547810
#   cdecl(jogador, presidente); a caixa de erro (linhas 0x20c5, 0x20cc, 0x20d1) = show_error_box;
# - 0x6a0990 thiscall(this, jogador, clube, marcas) ret 0xc; 0x7875e0 thiscall([0xb7431c], clube);
# - 0x69c7e0 thiscall(this, pessoa, clube, outro) ret 0xc; 0x7858c0 thiscall(noticia, k, nome, &prox) ret 0xc;
#   0x785d40 thiscall([0xb7431c], noticia, pessoa, 0);
# - 0x6a9350 thiscall(this, clube, pedido, resposta) ret 0xc; 0x5a0590, 0x5986b0 thiscall;
# - 0x6ac060 thiscall(this, jogador, veredito, jogos, suspensao, outros) ret 0x14;
# - noticias: 0x77bbe0 thiscall(noticia, tipo, 0); 0x785790 thiscall(noticia, k, v) ret 8; 0x7861e0 thiscall.
# Prologos (fronteira de instrucao, sem relativo, iguais no 1080): 0x69b780 `mov eax, [esp+4]; push eax` (5);
# 0x6ae180 `cmp byte [esp+8], 2` (5); 0x6a9e20 `mov ecx, [esp+8]; test ecx, ecx` (6); 0x6a8770 `mov eax, [esp+8];
# test eax, eax` (6); 0x696940 `push ebx; mov ebx, [esp+8]` (5); 0x697240 `push ebx; push ebp; push esi; mov esi,
# [esp+0x14]` (7); 0x697430 `sub esp, 0x204` (6); 0x6a0990, 0x6a9350, 0x6ac060 `mov eax, fs:[0]` (6); 0x69c7e0
# `push -1; push 0x95ccbb` (7); 0x6a4f80 `push -1; push 0x95ce8b` (7).
# Comparadores: lado a lado nas tres que so leem ou mexem no registro da diretoria (0x6ae180, os dois rotulos e
# 0x696940, esta sobre o registro de 0x49 bytes do clube); so a original nas que abrem tela ou postam noticia.
# Caminho no jogo (para o Lucas provar comparando):
# - 0x69b780: barra de menu > o clube > "Resign" (a confirmacao da demissao);
# - 0x6a8770, 0x6ae180, 0x6a9350: menu do tecnico > pedidos a diretoria ("Request to board"): a lista das opcoes e,
#   pedindo, a resposta na caixa de noticias;
# - 0x6a9e20: menu do tecnico > pedidos a federacao ("Request to FA"): a lista das opcoes;
# - 0x696940, 0x697240, 0x697430, 0x6a0990: perfil de um jogador do proprio clube > liberar ("Release"/"Free
#   transfer"); com um jogador valorizado a diretoria bloqueia ou reclama (noticia 0x1778);
# - 0x69c7e0: candidatar-se a um cargo vago (perfil do clube > "Apply for job"): o memorando da propria diretoria;
# - 0x6a4f80: chamada pela daily_update (sx14_3) quando o tecnico humano esta sem clube ha 90 dias (a oferta de um
#   clube sem tecnico); nao e tela, mas nao roda na carreira de teste: a prova e o Lucas sem clube por 90 dias;
# - 0x6ac060: apelar de uma suspensao (perfil do jogador suspenso > "Appeal ban"): o resultado dois dias depois.
TELAS = True
ROTINAS = [('open_resign_confirm_dialog', 0x69b780, [0x8b, 0x44, 0x24, 0x04, 0x50]),
           ('is_board_request_available', 0x6ae180, [0x80, 0x7c, 0x24, 0x08, 0x02]),
           ('get_fa_request_option_text', 0x6a9e20, [0x8b, 0x4c, 0x24, 0x08, 0x85, 0xc9]),
           ('get_board_request_option_text', 0x6a8770, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('check_board_blocks_release', 0x696940, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('judge_player_release', 0x697240, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('apply_release_confidence_penalty', 0x697430, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('post_board_release_reaction_news', 0x6a0990, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_board_interest_memo_news', 0x69c7e0, [0x6a, 0xff, 0x68, 0xbb, 0xcc, 0x95, 0x00]),
           ('post_board_request_reply_news', 0x6a9350, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_ban_appeal_result_news', 0x6ac060, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_job_offer_to_human_news', 0x6a4f80, [0x6a, 0xff, 0x68, 0x8b, 0xce, 0x95, 0x00])]
DOCS = ('a visão da diretoria sobre as liberações e as notícias de pedidos e memorandos',
        "the board's view of releases and the request and memo news",
        [('open_resign_confirm_dialog', 'open_resign_confirm_dialog', '0x69b780', 'manager_manager.cpp', 'stdcall, ret 4',
          'abre a confirmação da demissão do técnico humano no clube (open_resign_job_screen)',
          "opens the human manager's resignation confirmation for the club (open_resign_job_screen)"),
         ('is_board_request_available', 'is_board_request_available', '0x6ae180', 'manager_manager.cpp', 'stdcall, ret 8',
          'se um pedido à diretoria está aberto: 0 para o investimento nas categorias de base (2) com as instalações de treino no máximo de 20, senão 1',
          'whether a board request is open: 0 for the youth development request (2) with the training facilities at the maximum of 20, else 1'),
         ('get_fa_request_option_text', 'get_fa_request_option_text', '0x6a9e20', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'escreve o rótulo de uma opção de pedido à federação (1 reclamar do árbitro, 2 adiar a partida) num buffer de pelo menos 15 bytes',
          'writes the label of an FA request option (1 complain about a referee, 2 request a postponement) into a buffer of at least 15 bytes'),
         ('get_board_request_option_text', 'get_board_request_option_text', '0x6a8770', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'escreve o rótulo de uma das seis opções de pedido à diretoria (ampliar o estádio, investir na base, verba de transferências, verba de salários, mais tempo para reconstruir, prêmio pela próxima partida) num buffer de pelo menos 15 bytes',
          'writes the label of one of the six board request options (expand the stadium, invest in youth, transfer funds, wage funds, time to rebuild, win bonus for the next match) into a buffer of at least 15 bytes'),
         ('check_board_blocks_release', 'MANAGER_MANAGER::check_board_blocks_release', '0x696940', 'manager_manager.cpp', 'thiscall, ret 8',
          'decide se a diretoria bloqueia a liberação de um jogador: sem contrato com o clube, contrato de tipo 1 ou 3, jogador em disputa ou infeliz, 0. Valendo mais que (reputação × 0,2)² para o clube: bloqueia (0x200 com menos de 15 no elenco, senão 0x800); valendo menos, bloqueia só com menos de 15 no elenco (0x200), senão 0. Ao bloquear, confiança da diretoria − 1250 (no mínimo 1), humor das transferências − 1250 e humor dos favoritos − 750',
          'decides whether the board blocks a release: no contract with the club, a contract of type 1 or 3, a player in dispute or unhappy, 0. Worth more than (reputation × 0.2)² to the club: blocked (0x200 with fewer than 15 in the squad, else 0x800); worth less, blocked only with fewer than 15 in the squad (0x200), else 0. When blocking, board confidence − 1250 (at least 1), transfer mood − 1250 and favourites mood − 750'),
         ('judge_player_release', 'MANAGER_MANAGER::judge_player_release', '0x697240', 'manager_manager.cpp', 'thiscall, ret 8',
          'avalia como a diretoria vê a liberação do jogador, posta a notícia da reação e devolve o código: 4 (sem notícia) com contrato de tipo 1 ou 5, ou jogador abaixo de 2,5 milhões que o presidente detesta; valendo mais que (reputação × 0,2)², feliz 0x800, senão com elenco pequeno 0x400 e com elenco grande 1; valendo menos, feliz com elenco grande 1 e com pequeno a notícia 0x32 mas o código 0x800; infeliz com reputação acima de 0,8 × a do clube, 1 (elenco pequeno) ou 2; senão 2 (elenco pequeno) ou 4. Defeitos: não confere se há contrato (lê um contrato nulo); o caso feliz com elenco pequeno posta 0x32 e devolve 0x800',
          "rates how the board views the release, posts the reaction news and answers the code: 4 (no news) for a contract of type 1 or 5, or a player under 2.5 million whom the chairman dislikes; worth more than (reputation × 0.2)², happy 0x800, else with a small squad 0x400 and with a big one 1; worth less, happy with a big squad 1 and with a small one the news 0x32 but the code 0x800; unhappy with a reputation above 0.8 × the club's, 1 (small squad) or 2; otherwise 2 (small squad) or 4. Defects: no check that there is a contract (a null contract is read); the happy case with a small squad posts 0x32 and answers 0x800"),
         ('apply_release_confidence_penalty', 'MANAGER_MANAGER::apply_release_confidence_penalty', '0x697430', 'manager_manager.cpp', 'thiscall, ret 8',
          'corta a confiança e os humores da diretoria depois de uma liberação, pela reação (judge_player_release), pela idade e lesão do jogador (mais de 31 anos e mais de 90 na lesão) e pela felicidade dele, nas faixas das reações 1, 2 e 0x400; depois − 500 no humor dos favoritos se o clube o favorece e − 500 se o presidente o favorece; confiança no mínimo 1, humores no mínimo − 10000. Sem contrato, sem clube ou sem jogador, a caixa "Error"',
          'cuts the board confidence and moods after a release, by the reaction (judge_player_release), the player\'s age and injury (over 31 and more than 90 on the injury) and his happiness, in the bands of reactions 1, 2 and 0x400; then − 500 favourites mood if the club favours him and − 500 if the chairman does; confidence at least 1, moods at least − 10000. No contract, no club or no player, the "Error" box'),
         ('post_board_release_reaction_news', 'MANAGER_MANAGER::post_board_release_reaction_news', '0x6a0990', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'posta a notícia 0x1778 da reação da diretoria à liberação: sem jogador, sem clube, clube sem técnico humano ou antes da data das notícias, nada; campos o jogador e os nomes, o clube e as marcas; ao técnico do clube',
          "posts the 0x1778 news of the board's reaction to the release: no player, no club, a club without a human manager or before the news date, nothing; slots the player and his names, the club and the flags; to the club's manager"),
         ('post_board_interest_memo_news', 'MANAGER_MANAGER::post_board_interest_memo_news', '0x69c7e0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'posta a notícia 0x1390 de que a diretoria sabe da candidatura do técnico em outro clube: a pessoa, os dois clubes, os ids dos nomes e, para técnico humano, os nomes por extenso; entregue a ele',
          "posts the 0x1390 news that the board knows of the manager's application to another club: the person, the two clubs, the name ids and, for a human manager, the names in full; delivered to him"),
         ('post_board_request_reply_news', 'MANAGER_MANAGER::post_board_request_reply_news', '0x6a9350', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'posta a notícia 0x1786 da resposta da diretoria a um pedido: sem clube ou sem finanças, nada; o clube, o pedido, a resposta, a confiança da diretoria e a marca de pouco dinheiro (faixa de riqueza −2 ou −1, ou saldo abaixo de 1 milhão); ao técnico do clube',
          "posts the 0x1786 news of the board's reply to a request: no club or no finances, nothing; the club, the request, the answer, the board confidence and the low funds flag (wealth band −2 or −1, or a balance below 1 million); to the club's manager"),
         ('post_ban_appeal_result_news', 'MANAGER_MANAGER::post_ban_appeal_result_news', '0x6ac060', 'manager_manager.cpp', 'thiscall, ret 0x14',
          'posta a notícia 0x178a do veredito da apelação de suspensão: sem jogador, sem clube ou clube sem nação, nada; o jogador e os nomes, o veredito, a nação do clube, os jogos, a suspensão e os outros jogos; ao técnico do clube',
          "posts the 0x178a news of the ban appeal verdict: no player, no club or a club without a nation, nothing; the player and his names, the verdict, the club's nation, the games, the ban and the other games; to the club's manager"),
         ('post_job_offer_to_human_news', 'MANAGER_MANAGER::post_job_offer_to_human_news', '0x6a4f80', 'manager_manager.cpp', 'thiscall, ret 8',
          'posta a oferta 0x1774 do cargo de técnico de um clube ao técnico humano: clube que já tem técnico, nada é enviado; seleção, verba 0 e expectativas −1; clube, o investimento no elenco zerado, a verba de transferências e as expectativas da temporada calculadas com ele posto como técnico (e a marca de recém-chegado), depois o técnico e a marca antigos de volta, e um sorteio de 2; entregue a ele',
          "posts the 0x1774 offer of a club's manager job to the human manager: a club that already has a manager, nothing is sent; a national team, budget 0 and expectations −1; a club, the squad investment cleared, the transfer budget and the season expectations worked out with him put in as manager (and the newcomer flag), then the old manager and flag back, and a random(2); delivered to him")])
