# O texto da noticia de aposentadoria (player_regen_write_retirement_news_text 0x7a9650, player_regen.cpp; leitor sxB,
# rodada 5, 24/11/2026). A rotina inteira (0x7a9650..0x7aa334, ret 0xc em 0x7a9777 e 0x7aa331; o resto ate 0x7aa340 e
# enchimento), em pedacos com nome pelos ramos do corpo. Chamada so pelo despacho NewsItem::write_text (0x781d53,
# NEWS_WRITERS (4, 0xbba)); na carreira automatica pelo modo CM_TEXTO_NOTICIAS, ligado na cadeia em todo jogo
# comparando (levas/reorganizacao/proximas/textos_noticias.md). Molde: a irma 0x7aa4c0 (o anuncio da aposentadoria).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - a rotina: stdcall(noticia, corpo, Text), ret 0xc (o despacho a chama como thiscall; o ecx nao e lido);
# - 0x785830 NewsItem::get_slot thiscall(noticia, k) ret 4 (vagas 0..9 e 11..14; a 10 nao e lida);
#   0x53e140 cdecl(n1, n2, n3, buf, 0x190, estilo, id); 0x66f4e0 langlib_sprintf cdecl(0xde1f64, modelo, ...) (generos
#   movsx, como dword); 0x53ad00 database_get_job_name cdecl(cargo (so o byte e lido), 0, 0, &genero);
#   0x53ac40 database_get_coaching_job_name cdecl(cargo (byte), &genero) (so escreve o genero no idioma 4: fora dele
#   o modelo leva o byte da vaga 5, que ocupava a mesma casa); 0x90cfc0 utils_random cdecl(2); 0x4ae660
#   Text::set_string thiscall(Text, buf) ret 4; 0x53e990 cdecl(pessoa, n1, n2, n3, 0); 0x4ae8a0 Text::add_link
#   thiscall(Text, texto, 0x7d7, pessoa) ret 0xc.
# Defeitos do original: o internacional lesionado com nacao no clube de id 1 recebe o texto simples (compara com 1, nao
# com -1); o ex-internacional lesionado le genero e nome do registro de nacao antes da tabela (id -1); os textos com
# clube leem o registro do clube -1 quando a vaga 6 e -1.
SAVE = False
ROTINAS = [('player_regen_write_retirement_news_text', 0x7a9650, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xb0, 0x01, 0x00, 0x00])]
DOCS = ('o texto da noticia de aposentadoria', 'the text of the retirement news',
        [('player_regen_write_retirement_news_text', 'player_regen_write_retirement_news_text', '0x7a9650', 'player_regen.cpp', 'stdcall, ret 0xc',
          'escreve a manchete ou o corpo da noticia (4, 0xbba), a aposentadoria de uma pessoa: a manchete pelo motivo (8 lesao "se aposenta por lesao"; anunciada "se aposenta"; senao "aposentadoria surpresa de"); o corpo pelo motivo e pelas vagas: lesao recorrente (internacional com ou sem clube e cargo, ex-internacional, jogador de clube); aposentadoria anunciada (o membro da comissao que deixa o cargo, com clube, selecao ou o cargo antigo; quem fica no futebol, internacional ou nao, com o cargo de treinador ou a procura de um, sorteando entre treinador e tecnico; o internacional com clube ou sem); aposentadoria imediata (nao se sustenta ou a familia, pela idade abaixo de 27; o clube que precisa de um novo ocupante do cargo; o internacional que continua como tecnico, auxiliar ou treinador; o ex-internacional; o texto simples). O nome e vinculado a pessoa quando os nomes conferem. Defeitos do original: compara o clube com 1 em vez de -1 no ramo da lesao, le a nacao -1 do ex-internacional lesionado e o clube -1 sem clube',
          "writes the headline or body of news item (4, 0xbba), a person's retirement: the headline by the reason (8 injury \"retires due to injury\"; announced \"retires\"; otherwise \"Shock retirement of\"); the body by the reason and the slots: a recurring injury (an international with or without a club and job, a former international, a club player); an announced retirement (the staff member leaving his job, with a club, a national team or the old job; one who stays in football, international or not, with a coaching job or looking for one, drawing between a coaching role and a management position; the international with or without a club); an immediate retirement (cannot support himself or his family, by an age under 27; the club that needs a new job holder; the international who continues as manager, assistant or coach; the former international; the plain text). The name links to the person when the names match. Defects of the original: it compares the club with 1 instead of -1 in the injury branch, reads nation -1 for the injured former international and club -1 without a club")])
