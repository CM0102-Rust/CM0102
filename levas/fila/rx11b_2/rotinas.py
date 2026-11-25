# A noticia do resultado de um jogo reserva: quem a recebe e o texto dela (record_utils.cpp; leitor rx11b, rodada 6,
# 25/11/2026). Simulacao, carreira ENG (sem LIGAS): o resultado de cada jogo reserva passa pelo slot 0x4 (add_result)
# da vtable 0x96ef54 da ReserveComp; o texto, pelo despachante das noticias (0x77bc80, tipo 0x25, ja Rust) quando a
# noticia e escrita (CM_TEXTOS=1 na comparacao).
# 0x7e6f70 nao esta no mapa de fronteiras (fronteiras.py: "nao e inicio de funcao conhecido"); e entrada de verdade: o
# slot 1 da vtable 0x96ef54, prologo push -1; push 0x960821 depois do ret 4 de 0x7e6f60 e do enchimento.
# Convencoes conferidas na desmontagem:
# - 0x7e6f70 ReserveComp::add_result thiscall(jogo, nao lido) ret 8 -> 1: 0x54a110 Date (o registro de 0x1f bytes);
#   0x53b900 cdecl(clube); 0x540a50 club_get_linked_b_team cdecl(clube, &saida ou 0, 1); 0x7875e0 thiscall([0xb7431c],
#   clube) ret 4; 0x5fd480 thiscall([0xb63c98], tecnico) ret 4; 0x77bbe0 NewsItem(0x25, 0) ret 8; 0x7e4750
#   cdecl(jogo, clube, &registro); 0x7e4de0 cdecl(&registro, noticia, &k); 0x785790 thiscall(noticia, k, valor) ret 8 (k
#   num dword com lixo em cima: le o byte); 0x7861e0 thiscall([0xb7431c], noticia, [tecnico+0x39]) ret 8. O NewsItem
#   local nao tem destrutor chamado (o SEH so marca o estado).
# - 0x7e7280 reserve_result_news_text thiscall([0xb74434], noticia, completo, Text) ret 0xc: 0x7e4fa0 cdecl(noticia,
#   &k, &registro); 0x785830 thiscall(noticia, k) ret 4; 0x542100 cdecl(clube, vaga, buf, 0x12c, 1, 1 na manchete / 0 no
#   corpo); 0x66f4e0 langlib_sprintf (cdecl, variadica); 0x4ae660 thiscall(Text, s) ret 4; 0x729500 thiscall([0xb6b7c0],
#   vaga, clube, adversario, data+0, data+2) ret 0x14; 0x4ae8a0 thiscall(Text, s, tipo, alvo) ret 0xc.
# Comparadores: add_result so a original (as noticias vao para as caixas), com o plano: as noticias que a copia
# mandaria, conferidas na caixa do tecnico humano (tipo e vagas escritas); o texto lado a lado (compare_text_writer).
SAVE = False
ROTINAS = [('reserve_comp_add_result', 0x7e6f70, [0x6a, 0xff, 0x68, 0x21, 0x08, 0x96, 0x00]),
           ('reserve_result_news_text', 0x7e7280, [0x6a, 0xff, 0x68, 0x3b, 0x08, 0x96, 0x00])]
DOCS = ('a notícia do resultado de um jogo reserva: quem a recebe e o texto dela',
        'the reserve match result news: who gets it and its text',
        [('reserve_comp_add_result', 'ReserveComp::add_result', '0x7e6f70', 'record_utils.cpp', 'thiscall, ret 8',
          'regra: o resultado de um jogo reserva vai como notícia ao técnico humano de cada clube, ou do time ligado a ele; o lado marcado precisa de um time ligado sem saída e não vai ao técnico que controla o time reserva; seleções não recebem; a notícia leva o registro do jogo, as marcas dos dois lados e o estádio',
          "rule: a reserve match result goes as news to the human manager of each club, or of its linked team; a marked side needs a linked team with no output and does not go to the manager who controls the reserve team; national teams get nothing; the news carries the match record, both sides' marks and the stadium"),
         ('reserve_result_news_text', 'reserve_result_news_text', '0x7e7280', 'record_utils.cpp', 'thiscall, ret 0xc',
          'regra: a manchete diz se o clube venceu, empatou ou perdeu; o corpo diz o placar (o maior número de gols primeiro), o adversário e, com estádio, onde; o clube e o adversário ganham vínculo, e o placar também quando as estatísticas do jogo ainda batem',
          'rule: the headline says whether the club won, drew or lost; the body gives the score (the larger goal count first), the opponent and, with a stadium, where; the club and the opponent get links, and so does the score when the match statistics still match')])
CAMPOS = []
INTERESSA = []
