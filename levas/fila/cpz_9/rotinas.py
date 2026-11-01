# A forma ruim do jogador e as noticias das multas (awol.cpp e Fine.cpp; a familia media.cpp/news.cpp esta esgotada
# de rotinas com efeito na simulacao, entao este item junta as noticias das multas, que a imprensa repercute, ao
# incidente de forma que leva a multa da IA; nenhum outro agente usa awol.cpp nem Fine.cpp nesta rodada):
# AWOL::report_poor_fitness 0x41add0 (pelo diario das pessoas 0x5ce2d0, na queda grande de condicao do dia: o registro
# na lista dos incidentes, a noticia ao clube humano e, no clube da IA, os sorteios da multa 0x5a5980),
# post_fine_reaction_news 0x5a74b0 (pela multa 0x5a5980, cinco chamadas: a noticia 0xbc6 ao clube humano, com
# sorteio(2)) e post_coach_fine_feedback_news 0x5a7f20 (pelo diario das multas 0x5a5820: a noticia 0x1394 com
# sorteio(5)). Tamanhos reais (candidatos.py): 594, 556 e 364 bytes. As tres com SEH so da data e da noticia local (sem
# destrutor). Iguais no cm0102-1080.exe.
# Convencoes conferidas nos pontos de chamada (mov ecx antes = thiscall; add esp depois = cdecl; os push contados):
#   0x54a110 Date() thiscall 0 arg; 0x417c10 atribuir thiscall 2 args; 0x68d0d0 add_tail thiscall 2 args (registro,
#   0x11); 0x7875e0 news_club_has_human_manager stdcall 1 arg (ret 4; a original deixa o News em ecx, que ela nao le);
#   0x90cfc0 sorteio cdecl 1 arg; 0x77bbe0 NewsItem thiscall 2 args; 0x785790 set_slot thiscall 2 args; 0x7861e0
#   send_to_club_manager thiscall em [0xb7431c] 2 args; 0x600a50 clube humano cdecl 5 args (add esp,0x14); 0x544410
#   nota cdecl 2 args (add esp junto com os sorteios); 0x5a5980 Fine::fine_player thiscall em [0xae2a3c] 3 args;
#   0x4cf5a0 contrato thiscall em [0xae1050] 1 arg; 0x4cf5f0 contrato no clube thiscall em [0xae1050] 2 args;
#   0x8cca10 transferencia futura thiscall em [0xde1f54] 2 args. As duas noticias de Fine.cpp sao stdcall (ret 0x14 e
#   0xc, sem add esp nos chamadores; a 0x5a5820 deixa o Fine em ecx).
# Comparadores: o incidente so a original (lista, noticia e multa: alocam e sorteiam) conferindo o plano (o registro
# com jogador, clube e hoje logo depois da cauda de antes); a reacao a multa em ensaio (a copia para antes da noticia:
# sem noticia, [0xae1080] e o sorteio iguais; com noticia, so [0xae1080]); o retorno do treinador so a original (sem
# noticia as duas nao fazem nada).
SAVE = False
ROTINAS = [('incidente_de_forma', 0x41add0, [0x6a, 0xff, 0x68, 0x36, 0x3f, 0x95, 0x00]),
           ('noticia_reacao_a_multa', 0x5a74b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_retorno_do_treinador', 0x5a7f20, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a forma ruim do jogador e as notícias das multas: o incidente de forma com a multa da IA, a reação à multa e o retorno do treinador',
        'the player\'s poor fitness and the fine news: the fitness incident with the AI fine, the fine reaction and the coach feedback',
        [('incidente_de_forma', 'AWOL::report_poor_fitness', '0x41add0', 'awol.cpp', 'thiscall, ret 8',
          'o registro (jogador, clube, hoje) na lista dos incidentes; clube humano: um treinador sorteado e a notícia 0x138f; clube da IA com técnico: sorteio(+0x14) acima de sorteio(10) leva à multa, prazo pela nota do técnico',
          'the record (player, club, today) in the incident list; human club: a random coach and news 0x138f; AI club with a manager: draw(+0x14) above draw(10) leads to the fine, period by the manager\'s rating'),
         ('noticia_reacao_a_multa', 'post_fine_reaction_news', '0x5a74b0', 'Fine.cpp', 'stdcall, ret 0x14',
          'jogador com contrato num clube humano: a notícia 0xbc6 com motivo, reação, sorteio(2), personalidade, prazo, tipo de contrato, transferência futura e efeitos',
          'player under contract at a human club: news 0xbc6 with reason, reaction, draw(2), personality, period, contract type, future transfer and effects'),
         ('noticia_retorno_do_treinador', 'post_coach_fine_feedback_news', '0x5a7f20', 'Fine.cpp', 'stdcall, ret 0xc',
          'jogador e treinador com ficha e nação, jogador com clube: a notícia 0x1394 com o desfecho e sorteio(5)',
          'player and coach with a record and nation, player with a club: news 0x1394 with the outcome and draw(5)')])
CAMPOS = [('AWOL +8: lista dos incidentes de forma (registro de 0x11 bytes: +0 jogador, +4 clube, +8 data, +0x10 byte); id e valor; grava (incidente_de_forma)',
           'AWOL +8: fitness incident list (0x11-byte record: +0 player, +4 club, +8 date, +0x10 byte); id and value; writes (incidente_de_forma)'),
          ('clube +0x19f: as 5 vagas de treinador; +0xcf técnico; técnico +0x69 ficha (+0x14, +0x19); ponteiro e valor; lê (incidente_de_forma)',
           'club +0x19f: the 5 coach slots; +0xcf manager; manager +0x69 record (+0x14, +0x19); pointer and value; reads (incidente_de_forma)'),
          ('pessoa +0x18 idade, +0x1a nação, +0x39 clube, +0x59, +0x5b, +0x5d personalidade, +0x61 ficha (+0xb reputação); valor; lê (as notícias das multas)',
           'person +0x18 age, +0x1a nation, +0x39 club, +0x59, +0x5b, +0x5d personality, +0x61 record (+0xb reputation); value; reads (the fine news)'),
          ('contrato +0x4f tipo (nibble alto); [0xae1080] último contrato; valor e ponteiro; lê e grava [0xae1080] (noticia_reacao_a_multa)',
           'contract +0x4f type (high nibble); [0xae1080] last contract; value and pointer; reads and writes [0xae1080] (noticia_reacao_a_multa)'),
          ('[0xae2a3c] o objeto das multas (Fine); ponteiro; lê (incidente_de_forma)',
           '[0xae2a3c] the fines object (Fine); pointer; reads (incidente_de_forma)')]
INTERESSA = ['CMRTE']
