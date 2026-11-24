# Os textos das noticias do awol.cpp (leitor sx05, rodada 5, 23/11/2026): o jogador ausente sem licenca (tipo 0xbc5,
# write_awol_news_text 0x419c60), o treino perdido (0xbc9, write_missed_training_news_text 0x41a8e0) e a forma fisica
# depois do atraso no treino (0x138f, write_fitness_worry_news_text 0x41b030). Chamados pelo despacho das noticias
# (NewsItem::write_text, tabela NEWS_WRITERS em news.rs, Global(G_AWOL, ..)); as noticias sao postadas pelas rotinas
# do AWOL ja em Rust (awol.rs: miss_training, report_poor_fitness, daily_update).
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate a chamada):
# - as tres: thiscall(AWOL, noticia, corpo, Text), ret 0xc; o ecx nao e lido; 0x785830 NewsItem::get_slot
#   thiscall(noticia, k) ret 4; 0x53e140 cdecl(n1, n2, n3, buf, 0x190, estilo, id); 0x53e990 cdecl(pessoa, n1, n2,
#   n3, 0); 0x66f4e0 langlib_sprintf cdecl (os generos movsx, como dword); 0x4ae660 Text::set_string e 0x4ae8a0
#   Text::add_link thiscall;
# - as vagas: AWOL corpo 0..8, manchete 0..6 e de novo a 6 (a 7 nao); treino corpo 0..10, manchete 0..3, 5 e 6 (a 4
#   nao); forma fisica 0..9 nas duas;
# - o AWOL e o treino escrevem no buffer compartilhado 0xde1f64; a forma fisica num buffer local (0x1378 bytes);
# - os nomes: AWOL e treino (1, 2, 3) com o id na 0, o tecnico do treino (8, 9, 10) com o id na 7; forma fisica
#   (3, 4, 5) com o id na 0, o tecnico (7, 8, 9) com o id na 6.
# Defeito do original: o AWOL de tipo 3 (e acima de 4) nao formata nada e o texto e o que o 0xde1f64 tinha antes (a
# copia faz o mesmo: o comparador roda a original primeiro e o buffer nao muda).
# Comparadores: compare_text_writer (o texto, os vinculos e o gerador).
SAVE = False
ROTINAS = [('write_awol_news_text', 0x419c60, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xa8, 0x01, 0x00, 0x00]),
           ('write_missed_training_news_text', 0x41a8e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x3c, 0x03, 0x00, 0x00]),
           ('write_fitness_worry_news_text', 0x41b030, [0xb8, 0x74, 0x1b, 0x00, 0x00])]
DOCS = ('os textos das notícias do AWOL',
        'the AWOL news texts',
        [('write_awol_news_text', 'write_awol_news_text', '0x419c60', 'awol.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia do jogador ausente: a manchete pelo tipo (0 volta ao clube, 1 "perde o treino" quando o id do jogador é múltiplo de 3, senão "some", 2 pelo id módulo 3 "em briga por transferência", "perde o treino" ou "some", 4 "não volta da licença") com o vínculo do clube; o corpo muda quando o clube é o do técnico humano da tela (0 voltou e o elenco não gostou, 1 não quer mais jogar e quer sair, 2 faltou depois de ter a transferência negada para o outro clube, 4 não voltou da licença: para a nação, com "perdeu o voo" abaixo de 7 dias, ou o recado do empresário com a nação, ou sem ela); os outros técnicos leem o mesmo com o nome do clube na frente; depois os vínculos do jogador e dos dois clubes. Defeito do original: o tipo 3 (e acima de 4) não formata nada e o texto é o que o buffer compartilhado tinha',
          'the headline or body of the AWOL news: the headline by kind (0 returns to the club, 1 "misses training" when the player\'s id is a multiple of 3, else "goes AWOL", 2 by the id modulo 3 "in transfer row", "misses training" or "goes AWOL", 4 "fails to return from leave") with the club\'s link; the body differs when the club is the screen\'s human manager\'s (0 he is back and the squad is unhappy, 1 he will not play and wants a move, 2 he missed training after a move to the other club was denied, 4 he did not return from leave: to the nation, with "missed his flight" under 7 days, or the agent\'s message with the nation, or without one); the other managers read the same with the club\'s name first; then the player\'s and the two clubs\' links. Defect of the original: kind 3 (and above 4) formats nothing and the text is whatever the shared buffer held'),
         ('write_missed_training_news_text', 'write_missed_training_news_text', '0x41a8e0', 'awol.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia do treino perdido: a manchete "ausente do treino" com a marca, senão "teria perdido o treino" (tipo 0) ou "punido pelo clube"; o corpo, no clube do técnico humano, o treinador principal (ou o técnico com nome) informa que o jogador não apareceu (0), faltou (1) ou não se apresentou; nos outros, a diretoria perde a paciência (sem técnico), ou o técnico perde a paciência (0) ou o multa em duas (1) ou uma semana de salário; os vínculos do jogador, do técnico e do clube',
          'the headline or body of the missed training news: the headline "absent from training" with the flag, else "rumoured to have missed training" (kind 0) or "disciplined by club"; the body, at the human manager\'s club, the chief coach (or the named coach) reports the player did not turn up (0), was absent (1) or failed to report; elsewhere the board is losing patience (no coach), or the manager is losing patience (0) or fines him two (1) or one week\'s wages; the links of the player, the coach and the club'),
         ('write_fitness_worry_news_text', 'write_fitness_worry_news_text', '0x41b030', 'awol.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia da forma física depois do atraso no treino: a manchete "preocupação com a forma" com a marca, senão "falta de forma"; o corpo com o técnico (preocupado com a forma geral, com a marca, ou "chegou atrasado e sem forma") ou sem ele (membros da comissão preocupados, ou a falta de forma decepcionou os veteranos do elenco); os vínculos do jogador, do clube (nome curto) e do técnico; num buffer local',
          'the headline or body of the fitness news after a late arrival at training: the headline "fitness worry" with the flag, else "lacking fitness"; the body with the coach (concerned about his general fitness, with the flag, or "turned up late and lacking fitness") or without one (backroom staff concerned, or the lack of fitness disappointed the senior players); the links of the player, the club (short name) and the coach; in a local buffer')])
CAMPOS = []
INTERESSA = []
