# Dois escritores de texto de noticia do manager_manager.cpp (leitor sxB, rodada 5, 24/11/2026): a resposta da
# diretoria a um pedido do tecnico (0x6a9470, noticia (6, 0x1786)) e a resposta a um ultimato (0x6adb70, (6, 0x178f)).
# Cada um inteiro, de um desvio. Chamados so pelo despacho NewsItem::write_text (0x7817ef e 0x781ae7, NEWS_WRITERS);
# na carreira automatica pelo modo CM_TEXTO_NOTICIAS, ligado na cadeia em todo jogo comparando
# (levas/reorganizacao/proximas/textos_noticias.md).
# Fronteiras conferidas na desmontagem: 0x6a9470..0x6a9d65 (ret 0xc em 0x6a9c5b e 0x6a9d62; depois as tabelas de
# salto 0x6a9d68 (o tipo do pedido) e as das respostas, ate 0x6a9e20); 0x6adb70..0x6ae179 (ret 0xc em 0x6ae0a3 e
# 0x6ae176).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - 0x6a9470: thiscall(manager_manager, noticia, corpo, Text), ret 0xc (o this da o registro da diretoria do clube,
#   [this] + 0x49 x clube); 0x6adb70: stdcall(noticia, corpo, Text), ret 0xc (o ecx nao e lido);
# - 0x785830 get_slot thiscall(noticia, k) ret 4 (0x6a9470: o tipo do pedido e um byte com sinal, a resposta um byte
#   sem sinal nos tipos 1..4 e com sinal no 5, o valor da vaga 3 uma palavra com sinal); 0x66f4e0 langlib_sprintf
#   cdecl(0xde1f64, modelo, genero, genero, nome); 0x944c9f sprintf cdecl (os dois textos provisorios do tipo 6 e dos
#   outros); 0x53dba0 database_format_staff_name cdecl(pessoa, buf, 0x1f4, 1); 0x53e990 cdecl(pessoa, n1, n2, n3, 0);
#   0x4ae660 set_string ret 4; 0x4ae8a0 add_link ret 0xc.
# Defeito do original: 0x6adb70 com uma resposta da diretoria fora de 1..4 no corpo poe no Text o que o buffer
# compartilhado tinha (a copia faz o mesmo: o buffer e o mesmo).
SAVE = False
ROTINAS = [('news_board_request_reply_text', 0x6a9470, [0x51, 0x8b, 0x44, 0x24, 0x0c]),
           ('news_ultimatum_response_text', 0x6adb70, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('as respostas da diretoria a um pedido e a um ultimato', "the board's replies to a request and to an ultimatum",
        [('news_board_request_reply_text', 'MANAGER_MANAGER::news_board_request_reply_text', '0x6a9470', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da resposta da diretoria a um pedido do tecnico: a manchete "reacao da diretoria ao pedido" pelo tipo (estadio, base, verba de transferencias, folha salarial, tempo para reconstruir, bonus; os outros "laranjas maiores no intervalo"); o corpo pela resposta (0xc e 10 recusas gerais) e pelo tipo, com o nome longo do clube; na verba, a resposta 2 olha o valor e a bandeira e o investimento no elenco (acima de 7500000 nao investe mais nesta temporada); o tipo 6 e os outros tem os textos provisorios dos programadores. O nome longo do clube e vinculado',
          "writes the headline or body of the board's reply to a request by the manager: the headline \"Board reaction to request\" by the kind (stadium, youth, transfer funds, wage budget, time to rebuild, bonus payments; others \"larger half time oranges\"); the body by the reply (0xc and 10 general refusals) and the kind, with the club's long name; for transfer funds reply 2 looks at the value, the flag and the squad investment (above 7500000 no more money this season); kind 6 and the others have the developers' placeholder texts. The club's long name is linked"),
         ('news_ultimatum_response_text', 'news_ultimatum_response_text', '0x6adb70', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'escreve a manchete ou o corpo da resposta a um ultimato do tecnico: o boato na imprensa (a torcida responde; o tecnico sai se recusarem, ou o que a torcida pensa do estadio, do centro de treinamento, da verba ou dos salarios) ou a diretoria (inexperiente demais, passa desta vez, limpe a mesa, ou um fax seco concordando), pela exigencia (estadio, treinamento, verba, salarios); a manchete "a diretoria recusa" ou "concorda" ou "a torcida responde". Vinculos do corpo: o tecnico quando os nomes conferem e o clube. Defeito do original: resposta desconhecida no corpo poe o que o buffer compartilhado tinha',
          "writes the headline or body of the reply to a manager's ultimatum: the press rumour (the fans respond; the manager leaves if refused, or what the fans think of the stadium, training facilities, funds or wages) or the board (too inexperienced, overlooked this time, clear your desk, or a terse fax agreeing), by the demand (stadium, training, funds, wages); the headline \"Board refuse\" or \"agree\" or \"fans respond\". Body links: the manager when the names match and the club. Defect of the original: an unknown answer in the body sets whatever the shared buffer held")])
