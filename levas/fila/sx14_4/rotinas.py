# Os textos das noticias dos memorandos da diretoria e das respostas da federacao (manager_manager.cpp; leitor sx14,
# rodada 5, 23/11/2026). Todos so escritos quando a noticia e aberta na caixa do tecnico humano (0 chamadas na carreira
# de teste): item de TELAS. Chamados pelas noticias (0x77bc80, Global(0xb67a44, ..)): thiscall(MANAGER_MANAGER,
# noticia, corpo, Text), ret 0xc; o this nao e lido.
# Convencoes conferidas na desmontagem: 0x785830 thiscall ret 4; 0x66f4e0 (langlib) e 0x944c9f (sprintf) cdecl, os
# argumentos de cada formato conferidos push a push; 0x6181d0 cdecl(nacao, buf, tamanho, 9); 0x53e140 cdecl(n1, n2,
# n3, buf, 0x190, 1, id); 0x53e990 cdecl(pessoa, n1, n2, n3, 0); 0x53dba0 cdecl(pessoa, buf, 0x1f4, 1); 0x53b3d0
# cdecl(nacao); Text 0x4ae660/0x4ae8a0 thiscall. A 0x69c6b0 poe o vinculo antes do texto (ordem do original).
# Prologos (fronteira de instrucao, sem relativo): 0x69c6b0 `mov eax, 0x1388` (5; o `call _chkstk` fica fora);
# 0x69c910 `push ebx/esi/edi; mov edi, [esp+0x10]` (7); 0x6a5b40 e 0x6ac190 `sub esp, imm32` (6); 0x6a5e30
# `mov eax, [esp+8]; sub esp, 0x2ac` (10); 0x6aa910, 0x6aab70, 0x6aadb0 `mov eax, [esp+8]; sub esp, imm8` (7).
# Comparador: compare_text_writer em todas.
# Caminho no jogo (para o Lucas provar comparando): a caixa de noticias do tecnico humano, abrindo cada noticia:
# - 0x1773 resposta a candidatura: candidatar-se a um cargo vago (perfil do clube > "Apply for job");
# - 0x1390 memorando da diretoria: depois de se candidatar, a propria diretoria manda o memorando;
# - 0x1781 avaliacao da federacao: tecnico de selecao, ao fim de uma competicao da selecao;
# - 0x1782 protesto da torcida: confianca da torcida abaixo de 3000 (sequencia de derrotas);
# - 0x29 partida remarcada e 0x1789 resposta da federacao: menu do tecnico > pedidos a federacao ("Request to FA":
#   adiar partida ou reclamar do arbitro);
# - 0x2b reacao a reclamacao do arbitro: a reclamacao aceita (a federacao troca o arbitro);
# - 0x178a resultado da apelacao: apelar de uma suspensao (perfil do jogador suspenso > "Appeal ban").
TELAS = True
ROTINAS = [('news_job_application_reply_text', 0x69c6b0, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('news_board_interest_memo_text', 0x69c910, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('news_fa_performance_assessment_text', 0x6a5b40, [0x81, 0xec, 0x58, 0x02, 0x00, 0x00]),
           ('news_supporter_protest_text', 0x6a5e30, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xac, 0x02, 0x00, 0x00]),
           ('news_match_rescheduled_text', 0x6aa910, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x78]),
           ('news_fa_request_reply_text', 0x6aab70, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x7c]),
           ('news_fa_referee_complaint_text', 0x6aadb0, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x78]),
           ('news_ban_appeal_result_text', 0x6ac190, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00])]
DOCS = ('os textos das notícias dos memorandos da diretoria e das respostas da federação',
        "the news texts of the board memos and the FA replies",
        [('news_job_application_reply_text', 'news_job_application_reply_text', '0x69c6b0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da resposta de um clube à candidatura do técnico humano: recusada ou recebida, com o nome do clube e o vínculo dele',
          "the headline or body of a club's reply to the human manager's job application: dismissed or acknowledged, with the club's name and its link"),
         ('news_board_interest_memo_text', 'news_board_interest_memo_text', '0x69c910', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete "memorando da diretoria" ou o corpo, um de três modelos pela soma dos campos 0 e 2 módulo 3, sobre o interesse do técnico pelo cargo no outro clube, com o vínculo dele',
          'the "memo from the board" headline or the body, one of three templates by the sum of slots 0 and 2 modulo 3, on the manager\'s interest in the other club\'s job, with its link'),
         ('news_fa_performance_assessment_text', 'news_fa_performance_assessment_text', '0x6a5b40', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da avaliação da federação sobre o técnico da seleção numa competição, pela nota (3, 2, 0, −1, −2 ou outra), com a nacionalidade ("Sedgefordian" sem nação) e a competição, e os vínculos da seleção e da competição',
          "the headline or body of the FA's assessment of the national team manager in a competition, by the grade (3, 2, 0, −1, −2 or other), with the nationality (\"Sedgefordian\" without a nation) and the competition, and the links to the national team and the competition"),
         ('news_supporter_protest_text', 'news_supporter_protest_text', '0x6a5e30', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete "protesto da torcida" ou o corpo com o clube e o nome do técnico, com os vínculos do clube e do técnico (se o nome ainda é o dele)',
          'the "supporter protest" headline or the body with the club and the manager\'s name, with the links to the club and to the manager (when the name is still his)'),
         ('news_match_rescheduled_text', 'news_match_rescheduled_text', '0x6aa910', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia de que a federação remarcou a partida entre dois clubes, com a nacionalidade e os vínculos dos clubes e da seleção da nação',
          "the headline or body of the news that the FA rescheduled the match between two clubs, with the nationality and the links to the clubs and to the nation's national team"),
         ('news_fa_request_reply_text', 'news_fa_request_reply_text', '0x6aab70', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da resposta da federação à reclamação contra o árbitro ou ao pedido de adiamento, pelo resultado e, na reclamação, pela nota do árbitro; pedido de outro tipo, a frase "não vão lhe dar nada"; com o vínculo do clube',
          "the headline or body of the FA's reply to a referee complaint or a postponement request, by the outcome and, for the complaint, by the referee's standing; another request, the \"gladly give you sod all\" sentence; with the club's link"),
         ('news_fa_referee_complaint_text', 'news_fa_referee_complaint_text', '0x6aadb0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da reação da federação à reclamação contra o árbitro, com a nacionalidade e o sobrenome antigo e o novo do árbitro, num modelo para o técnico da tela atual e noutro para os demais; sem vínculos',
          "the headline or body of the FA's reaction to the referee complaint, with the nationality and the referee's old and new second names, in one template for the manager of the current screen and another for the others; no links"),
         ('news_ban_appeal_result_text', 'news_ban_appeal_result_text', '0x6ac190', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete (revogada, reduzida ou recusada) ou o corpo da decisão da federação sobre a apelação de suspensão, pelos jogos tirados das duas suspensões, com o vínculo do jogador; defeito: a segunda suspensão reduzida cita −1 em vez dos jogos dela',
          "the headline (revoked, reduced or dismissed) or body of the FA's decision on the ban appeal, by the games taken off the two bans, with the player's link; defect: the reduced second ban quotes −1 instead of its games")])
