# O texto da noticia de suspensao a mais (Discipline::write_further_ban_news_text 0x55b8b0, discipline.cpp; escrito pelo
# leitor sxB, rodada 5, 24/11/2026, sobre as notas do FORA.md). A rotina inteira (0x55b8b0..0x55c9fd, ret 0xc em
# 0x55c9fa; o resto ate 0x55ca00 e enchimento), em pedacos com nome: FurtherBan (as vagas), further_ban_sentence (o
# modelo) e a rotina (manchete, corpo, vinculos). Chamada so pelo despacho NewsItem::write_text (0x7816ae,
# NEWS_WRITERS (3, 0x7d3)); na carreira automatica pelo modo CM_TEXTO_NOTICIAS, ligado na cadeia em todo jogo
# comparando (levas/reorganizacao/proximas/textos_noticias.md). Irma: write_ban_news_text (motor/discipline.rs),
# de quem reusa BanMatch, BanArgs, BAN_DEFAULT_DATE, news_text_format e FN_DISCIPLINE_GET_SCOPE_TEXT.
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - a rotina: thiscall(disciplina, noticia, corpo, Text), ret 0xc (o ecx vai para a get_scope_text e a
#   write_ban_news_text);
# - 0x785830 get_slot thiscall(noticia, k) ret 4 (o indice num byte que anda: 0..3, 6..13, 17..; a vaga depois da
#   pessoa, a marca da partida, e pulada sem ler); 0x54a110 Date::Date (duas datas: a da pessoa e a da partida);
#   0x7e52c0 cdecl(noticia, &k, pessoa); 0x7e4fa0 cdecl(noticia, &k, partida) (sempre); 0x5576e0
#   write_ban_news_text thiscall(disciplina, noticia, corpo, Text) ret 0xc quando a vaga 12 nao e 0;
#   0x53e990 cdecl(pessoa, n1, n2, n3, &data); 0x729500 thiscall([0xb6b7c0], ficha, clube, adversario, dia, ano) ret
#   0x14; 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, modo, id); 0x7e5780 cdecl(dias (movsx word), buf, 0x1e, 0);
#   0x55ce40 thiscall(disciplina, regras, buf, 0x64, 0, forma 5 ou 6) ret 0x14 -> al (movsx); 0x66f4e0 langlib_sprintf
#   cdecl; 0x944c9f sprintf cdecl(0xde1f64, "%s\n\n%s" ou "%s", ...); 0x4ae660 set_string ret 4; 0x4ae8a0 add_link
#   ret 0xc.
# Defeitos do original: a suspensao a mais em dias formata o modelo com o buffer do escopo nunca escrito (lixo da
# pilha) e o byte da vaga 0 no lugar do genero do escopo; um tipo acima de 4 entrega o buffer do modelo nunca escrito.
# A copia escreve escopo vazio (ou frase vazia) e o comparador deixa o texto de fora nesses casos.
SAVE = False
ROTINAS = [('write_further_ban_news_text', 0x55b8b0, [0x6a, 0xff, 0x68, 0x56, 0x7d, 0x95, 0x00])]
DOCS = ('o texto da noticia de suspensao a mais', 'the text of the further ban news',
        [('write_further_ban_news_text', 'Discipline::write_further_ban_news_text', '0x55b8b0', 'discipline.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0x7d3, a suspensao a mais de um jogador: com recurso pendente (vaga 12) quem escreve e a write_ban_news_text; senao a manchete "suspenso por um jogo", "por N jogos" ou pelo tempo em dias, e o corpo com o placar (maior-menor), o escopo da punicao (so nas suspensoes em jogos) e o modelo pelo tipo (pontos, cartoes amarelos, um amarelo na partida, expulsao, ma conduta, amarelos em jogos seguidos; os de partida pela vitoria, derrota ou empate do clube), com "voce recorreu" quando houve recurso. Vinculos: o jogador quando os nomes conferem, os dois clubes (sempre: a partida e sempre lida) e o placar quando a ficha confere. Defeitos do original: em dias o escopo e um buffer nunca escrito (com o byte do recurso como genero); tipo acima de 4 entrega o buffer do modelo nunca escrito',
          'writes the headline or body of news item 0x7d3, a player\'s further ban: with an appeal pending (slot 12) the main ban writer writes it; otherwise the headline "banned for one match", "for N matches" or for the length in days, and the body with the score (more-fewer), the scope of the ban (only for bans in matches) and the template by kind (points, yellow cards, a yellow card in the match, a sending-off, misconduct, yellow cards in successive matches; the match ones by the club\'s win, defeat or draw), with "you have appealed" when the appeal was made. Links: the player when the names match, both clubs (always: the match is always read) and the score when the fixture matches. Defects of the original: in days the scope is a buffer never written (with the appeal byte as its gender); a kind above 4 hands over the template buffer never written')])
