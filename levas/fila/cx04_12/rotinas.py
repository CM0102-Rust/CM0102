# Os textos das historias do olheiro, do sistema de transferencias e do seguro (european_cup.cpp pelo mapa; leitor
# cx04, rodada 5, 24/11/2026). Chamados so pelo despacho das noticias (NewsItem::write_text 0x77bc80, NEWS_WRITERS
# Plain: (5, 0xfbd), (5, 0xfc1), (5, 0xfc2)) quando o texto e escrito: na carreira automatica pelo modo CM_TEXTO_NOTICIAS
# (levas/reorganizacao/proximas/textos_noticias.md); sem ele, abrindo a noticia jogando comparando.
# Convencoes conferidas: as tres cdecl(noticia, corpo, Text) (ret simples); NewsItem::get_slot 0x785830 (push k; mov
# ecx, noticia -> thiscall, ret 4), database_format_name_from_ids 0x53e140 (cdecl, 7), langlib_sprintf 0x66f4e0 (cdecl,
# variadico), Text::set_string 0x4ae660 (thiscall, ret 4), database_staff_matches_names 0x53e990 (cdecl, 5), Text::add_link
# 0x4ae8a0 (thiscall, ret 0xc), Cash::get_display_amount 0x43f460 (thiscall(&valor, 6) -> edx:eax; o primeiro push 6 e
# do cash_format_money 0x441c10 cdecl(buf, 0x64, i64, 6)). Iguais no cm0102-1080.exe.
# - 0x587370 news_write_scout_watching_text: corpo pela redacao (vaga 6: 0x9c4b04, 0x9c4b9c, 0x9c4c50) com o nome curto
#   do clube (vaga 4) e o nome do jogador, no buffer 0xde1f64; manchete 0x9c4ad4. Prologo mov eax, [esp+8]; sub esp,
#   0x198 (10 bytes).
# - 0x58a900 news_transfer_system_change_text: pelo tipo (vaga 0) e a parte, quatro modelos num buffer de 0x1388 do
#   quadro. Prologo mov eax, 0x1388 (5 bytes; depois o call __chkstk).
# - 0x58aa60 news_insurance_payout_text: corpo 0x9c7334 com o valor formatado, o clube e o jogador, vinculos do jogador
#   e do nome longo do clube; manchete 0x9c72cc. Prologo push -1; mov eax, fs:[0] (8 bytes).
# Defeito do original: na 0x58aa60 o nome e formatado com espaco declarado de 0x190 bytes num buffer do quadro que o
# texto do valor segue a 100 bytes (inofensivo para nomes reais; a copia usa buffers separados).
# Comparadores: compare_text_writer (o texto, os vinculos e o gerador).
SAVE = False
ROTINAS = [('news_write_scout_watching_text', 0x587370, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('news_transfer_system_change_text', 0x58a900, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('news_insurance_payout_text', 0x58aa60, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos das histórias do olheiro, do sistema de transferências e do seguro',
        'the texts of the scout watching, transfer system and insurance payout stories',
        [('news_write_scout_watching_text', 'news_write_scout_watching_text', '0x587370', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> observado") ou o corpo da história do olheiro, com a redação sorteada, o nome curto do clube que observa e o nome do jogador; vínculos do jogador (quando os nomes conferem) e do clube',
          'writes the headline ("<player> watched") or the body of the scout watching story, with the drawn wording, the watching club\'s short name and the player\'s name; links to the player (when the names match) and to the club'),
         ('news_transfer_system_change_text', 'news_transfer_system_change_text', '0x58a900', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ou o corpo da história do sistema de transferências: a FIFA anuncia a mudança, ou, antes dela, os clubes correm para renovar contratos',
          'writes the headline or body of the transfer system story: FIFA announces the change, or, before it, the clubs rush to renew contracts'),
         ('news_insurance_payout_text', 'news_insurance_payout_text', '0x58aa60', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ou o corpo da história do seguro pela aposentadoria precoce de um jogador, com o clube, o jogador e o valor; vínculos do jogador e do clube',
          'writes the headline or body of the story of the insurance payout for a player\'s early retirement, with the club, the player and the amount; links to the player and to the club')])
CAMPOS = []
INTERESSA = []
