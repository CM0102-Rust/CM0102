# As notícias de seleção do técnico humano (national_teams.cpp; leitor sx15, rodada 5, 23/11/2026). Item de TELAS:
# nenhuma das cinco roda na carreira automática (cobertura ENG até 2002-06-01: 0 chamadas). O postador 0x76f1a0 só é
# chamado pela select_squad (0x76c744) no ramo em que o elenco é do humano (técnico humano da seleção); os escritores de
# texto só rodam quando a notícia chega a um humano e o texto é pedido (NewsItem::write_text 0x77bc80, tabela dos
# escritores: tipos 0x1e, 0x20, 0x21, 0x22): 0x20/0x21 vão ao técnico humano do clube do jogador (os postadores
# 0x76f7d0/0x76fd10 rodam na carreira, mas o clube humano da carreira de férias não teve jogador convocado: o escritor
# ficou em 0), 0x1e e 0x22 ao técnico humano da seleção.
# Convenções conferidas na desmontagem:
# - 0x76f1a0: thiscall(this, time, variante, jogo), ret 0xc (SEH sem destrutor); 0x77bbe0/0x785790/0x68d1b0: ecx ->
#   thiscall; 0x76d560: push jogo; push time; mov ecx, this -> ret 8; 0x53b3d0: push nação; add esp, 4 -> cdecl;
#   0x785d40: push 0 (antes do ramo); push técnico; push &notícia; mov ecx, [0xb7431c] -> thiscall ret 0xc;
# - os escritores: thiscall(this, notícia, corpo, Text), ret 0xc (o this não é lido); 0x785830 (get_slot) ecx ->
#   thiscall ret 4; 0x542100 cdecl(clube, variante, buf, 0x64 ou 0xc8, 2, 1); 0x53e140 cdecl(1º, 2º, comum, buf,
#   0x12c, modo, id); 0x53e990 cdecl(pessoa, 1º, 2º, comum, 0); 0x66f4e0 cdecl, os argumentos push a push (as
#   leituras [esp+k] recontadas depois de cada push); 0x4ae660 e 0x4ae8a0 thiscall (Text).
# Comparadores: compare_text_writer nos quatro escritores (o corpo do pedido sem adversário fora das finais lê buffer
# não escrito: só a original); o postador só a original (entrega na caixa do humano e aloca a lista).
TELAS = True
ROTINAS = [('national_teams_post_squad_request_news', 0x76f1a0, [0x6a, 0xff, 0x68, 0x1b, 0xde, 0x95, 0x00]),
           ('write_squad_request_news_text', 0x76f370, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xac, 0x01, 0x00, 0x00]),
           ('write_player_withdrawn_news_text', 0x76fa40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('write_player_called_up_news_text', 0x76ff40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('write_replacement_needed_news_text', 0x7703f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as notícias de seleção do técnico humano: o pedido de convocação e os textos de cortado, convocado e substituto',
        "the human manager's national squad news: the squad request and the texts of a player removed, called up or to be replaced",
        [('national_teams_post_squad_request_news', 'national_teams_post_squad_request_news', '0x76f1a0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'monta a notícia 0x1e (o pedido para o técnico humano escolher o elenco da seleção) e a entrega ao técnico do time ou, sem ele, ao da seleção principal da nação: 0 o time, 1 a variante (segundo time), 2 = 0, 3 = 1 se a competição do jogo é uma das nove finais, 4 quantos adversários, 5 a competição, 6 o dia e 7 o ano do jogo e, com menos de 4 adversários, 8 em diante os adversários (o adversário do jogo e os dos próximos 13 dias; nas finais não há lista). Defeitos: a lista de adversários nunca é liberada; a competição é lida sem conferir nulo',
          'builds news item 0x1e (the request that the human manager pick the national squad) and delivers it to the team\'s manager or, without one, to the manager of the nation\'s first team: 0 the team, 1 the variant (second team), 2 = 0, 3 = 1 if the game\'s competition is one of the nine finals, 4 the number of opponents, 5 the competition, 6 the day and 7 the year of the game and, with fewer than 4 opponents, 8 onwards the opponents (the game\'s opponent and those of the next 13 days; no list in the finals). Defects: the opponents list is never freed; the competition is read without a null check'),
         ('write_squad_request_news_text', 'NationalTeams::write_squad_request_news_text', '0x76f370', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ("elenco da <seleção> a decidir") ou o corpo do pedido de convocação: nas finais, para as finais da competição; com um adversário, para o jogo contra ele; com dois ou três, para os jogos contra a lista ("a e b", "a, b e c"); com quatro ou mais, para os próximos jogos; depois os vínculos dos adversários, do time (0x7d6 no segundo time, senão 0x7d5) e, nas finais, da competição. Defeitos: com três adversários, o segundo e o terceiro iguais e o primeiro outro, a lista sai "b e b" sem o primeiro; sem adversário fora das finais o texto junta um buffer nunca escrito (a cópia usa vazio); nas finais o gênero do nome do time é o do nome longo (+0x37)',
          'the headline ("<team> squad to be decided") or the body of the squad request: in the finals, for the competition\'s finals; with one opponent, for the match against him; with two or three, for the matches against the list ("a and b", "a, b and c"); with four or more, for the forthcoming matches; then the links of the opponents, of the team (0x7d6 for a second team, else 0x7d5) and, in the finals, of the competition. Defects: with three opponents, the second and third the same and the first another, the list reads "b and b" without the first; with no opponent outside the finals the text joins a never-written buffer (the copy uses an empty one); in the finals the team name\'s gender is the long name\'s (+0x37)'),
         ('write_player_withdrawn_news_text', 'NationalTeams::write_player_withdrawn_news_text', '0x76fa40', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ("<jogador> cortado do elenco da <seleção>") ou o corpo da notícia 0x20: o jogador cortado do elenco por doença (gravidade 0) ou por lesão; vínculos do jogador (se os nomes ainda batem) e do time',
          'the headline ("<player> removed from <team> squad") or the body of news item 0x20: the player removed from the squad because of illness (severity 0) or injury; links of the player (when the names still match) and of the team'),
         ('write_player_called_up_news_text', 'NationalTeams::write_player_called_up_news_text', '0x76ff40', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ("<jogador> chamado para o elenco da <seleção>") ou o corpo da notícia 0x21: o jogador convocado; vínculos do jogador (se os nomes ainda batem) e do time',
          'the headline ("<player> brought in to <team> squad") or the body of news item 0x21: the player called up; links of the player (when the names still match) and of the team'),
         ('write_replacement_needed_news_text', 'NationalTeams::write_replacement_needed_news_text', '0x7703f0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ("<jogador> cortado do elenco da <seleção>") ou o corpo da notícia 0x22: o jogador cortado por doença (gravidade 0) ou lesão e um substituto precisa ser convocado; vínculos do jogador e do time',
          'the headline ("<player> removed from <team> squad") or the body of news item 0x22: the player removed because of illness (severity 0) or injury and a replacement must be called up; links of the player and of the team')])
