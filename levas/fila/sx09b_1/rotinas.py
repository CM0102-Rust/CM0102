# A noticia de uma suspensao nova (Discipline::post_ban_news 0x557280, discipline.cpp, 1.120 bytes; leitor sx09b,
# rodada 5, 23/11/2026). A rotina vai de 0x557280 ao ret 0xc de 0x5576d3 (0x5576d6 e enchimento, a 0x5576e0 e a
# write_ban_news_text, ja Rust). Chamada pelo dia das regras de disciplina (os pendentes que entram, 0x7e7e98) e pelos
# lances de cada regra (0x7db285...0x7e4630, 44 chamadas) e pela agressao ao arbitro (0x55e4c0): 343 vezes na carreira
# ENG ate 2002-06-01 (docs/cobertura_competicoes.txt).
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(Discipline, pessoa, suspensao de 20 bytes, jogo ou 0), ret 0xc; o ecx nao e lido (o ecx e
#   trocado pelo [0xde1f54] na primeira instrucao util);
# - 0x8d1da0 TRANSFER_MANAGER::collect_player_news_recipients thiscall([0xde1f54], pessoa, 0xda, vetor, 0x1e) ret 0x10
#   (al com sinal); 0x77bbe0 NewsItem::NewsItem thiscall(0x7d2, 0) ret 8; 0x785790 set_slot thiscall(k, valor) ret 8
#   (o k e o byte da contagem local, que tambem vai por ponteiro para as duas de baixo);
# - 0x7e4920 fill_staff_name_ref cdecl(pessoa, &registro de 0x18 bytes, a data em +0x10 iniciada pelo Date::Date
#   0x54a110); 0x7e5160 write_staff_name_ref_to_news cdecl(&registro, &noticia, &k);
# - 0x53b900 database_is_national_team cdecl(clube da casa do jogo); 0x7e4750 fill_match_record_from_fixture
#   cdecl(jogo, clube, &registro de 0x1f bytes, a data em +0 iniciada pelo Date::Date); 0x7e4de0
#   write_match_record_to_news cdecl(&registro, &noticia, &k);
# - 0x786780 News::send_to_all_managers thiscall([0xb7431c], noticia) ret 4; 0x7861e0 send_to_club_manager
#   thiscall([0xb7431c], noticia, clube) ret 8; 0x7875e0 news_club_has_human_manager (ret 4; o ecx = [0xb7431c] nao e
#   lido).
# - a comparacao do inicio: o ano sem sinal (jb/ja), o dia com sinal (setl); dias > 50 com sinal; reputacao mundial
#   > 8000 com sinal.
# Defeito do original (registrado, a copia faz o mesmo): com jogo, se o clube da pessoa (selecao, quando a casa e
# selecao; senao o clube) nao e nenhum dos dois do jogo, a noticia montada nao vai a ninguem.
# Comparador: ensaio (a copia sem mandar, guardando cada envio), depois a original do mesmo estado do gerador; compara
# o gerador e as noticias 0x7d2 que chegaram as caixas dos tecnicos humanos (so humano recebe) com os envios da copia.
# Os auxiliares de caixa (human_inbox_marks, delivered_news_items, usados tambem pelo sx09b_6) fazem o mesmo que os do
# sx07_1 (fila, ainda nao publicado: human_inbox_positions, offer_response_delivered): na publicacao dos dois, juntar.
SAVE = False
ROTINAS = [('post_ban_news', 0x557280, [0x6a, 0xff, 0x68, 0x01, 0x7d, 0x95, 0x00])]
DOCS = ('a noticia de uma suspensao nova',
        'the news item of a new ban',
        [('post_ban_news', 'Discipline::post_ban_news', '0x557280', 'discipline.cpp', 'thiscall, ret 0xc',
          'monta a noticia 2002 (0x7d2) de que uma pessoa foi suspensa: o recurso feito (0), o arbitro do jogo (-1 sem jogo), os jogos e os dias da suspensao, o inicio quando ainda vai chegar (senao 0 e 0), as quatro regras, o motivo e a quantidade do motivo, os bits de "vale em todas as competicoes", "estendida por um pendente" e "somada a uma existente", a data de hoje, a referencia da pessoa e, com jogo, o registro do jogo pelo clube dela (a selecao quando a casa e selecao; se esse clube nao esta no jogo, nada e mandado). Antes, o gerente de transferencias escolhe ate 30 clubes para as noticias do jogador. Suspensao de mais de 50 dias: a todos os tecnicos humanos quando a reputacao mundial do jogador passa de 8000, senao aos clubes escolhidos; suspensao menor (ou por jogos): so ao clube da pessoa, se o tecnico dele e humano',
          'builds news item 2002 (0x7d2) that a person was banned: the appeal made (0), the match referee (-1 without a match), the ban\'s matches and days, the start when it is still to come (otherwise 0 and 0), the four rules, the reason and its amount, the bits "valid in every competition", "extended by a pending ban" and "added to an existing one", today\'s date, the person\'s reference and, with a match, the match record by his club (the national side when the home side is a national team; if that club is not in the match, nothing is sent). First, the transfer manager picks up to 30 clubs for the player\'s news. A ban of more than 50 days: to every human manager when the player\'s world reputation is above 8000, otherwise to the picked clubs; a shorter ban (or one by matches): only to the person\'s club, if its manager is human')])
