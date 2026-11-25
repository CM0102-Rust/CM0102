# O texto da noticia "busca do olheiro iniciada" (player_search.cpp; leitor rx06, rodada 6, 25/11/2026):
# player_search_write_search_started_news_text (0x7b7840). Rotina inteira (0x7b7840..0x7b81e7 ret; o resto e enchimento).
# Quem chama: o escritor de textos das noticias (newsitem_write_text, pela tabela de news.rs: noticia 0x1389, Plain),
# ao abrir a noticia na tela. A carreira automatica nao abre noticias e nao tem olheiros do humano: contador 0.
# TELAS: a prova e o Lucas jogando comparando: dar uma tarefa a um olheiro (nacao, regiao, competicao, base) e abrir a
# noticia "... search started" (titulo e corpo).
# Convencoes conferidas na desmontagem:
# - 0x7b7840 cdecl(noticia, corpo, texto) -> 1 ou 0 (moldura SEH so para a data local);
# - 0x785830 NewsItem::get_slot thiscall(noticia, k); 0x7e52c0 read_staff_name_ref_from_news cdecl(noticia, &n, &ref);
#   0x4afa40 Text::clear thiscall(texto); 0x53e140 database_format_name_from_ids cdecl(id1, id2, id3, buf, 0xc8, 1,
#   indice); 0x6181d0 nation_get_nationality_adjective cdecl(nacao, buf, 0xc8, 1); 0x616f20 get_region_name cdecl(regiao
#   como dword: so o byte e iniciado, os outros 3 sao resto de pilha; a funcao so testa o byte), buf, 0xc8, 1); o nome
#   da competicao copiado de +4 (strcpy inline); 0x66f4e0 langlib_sprintf em 0xde1f64; 0x4ae660 Text::set_string;
#   0x4ae8a0 Text::add_link thiscall(texto, nome, 0x7d7 pessoa ou 0x7e4 busca, alvo) ret 0xc; 0x53e990
#   database_staff_matches_names; 0x7f6cd0 SCOUT_MANAGER::get_search thiscall([0xb74448], vaga, 0) ret 8 e o get_scout
#   inline; a caixa player_search.cpp 0x12a5 / 0x139f / 0x133e (show_error_at).
# Defeito do original: a regiao vai para o get_region_name como dword com 3 bytes de resto de pilha (so o byte conta).
TELAS = True
ROTINAS = [('player_search_write_search_started_news_text', 0x7b7840, [0x6a, 0xff, 0x68, 0xfb, 0xf5, 0x95, 0x00])]
DOCS = ('o texto da noticia "busca do olheiro iniciada"',
        'the text of the "scout search started" news',
        [('player_search_write_search_started_news_text', 'player_search_write_search_started_news_text', '0x7b7840', 'player_search.cpp', 'cdecl',
          'escreve o titulo ou o corpo da noticia 0x1389 (vaga da busca, referencia do olheiro, nacao, regiao, competicao, base). Sem noticia ou texto: a caixa e 0. O titulo: "busca de <nacionalidade>/<regiao>/<competicao> iniciada", com o vinculo da busca quando o olheiro ainda e o dela, ou "busca de jovens iniciada" sem vinculo. O corpo nomeia o olheiro (vinculado se ainda e a mesma pessoa) e o alvo do mesmo jeito ("busca de jovens jogadores" na base, tambem vinculada). Sem alvo: a caixa e 0; senao 1',
          'writes the headline or the body of news 0x1389 (search slot, scout reference, nation, region, competition, youth). Without a news item or text: the box and 0. The headline: "<nationality>/<region>/<competition> search started", linked to the search while the scout is still its own, or "Youth search started" without a link. The body names the scout (linked while he is the same person) and the subject the same way ("search for youth players" on a youth search, also linked). No subject: the box and 0; otherwise 1')])
