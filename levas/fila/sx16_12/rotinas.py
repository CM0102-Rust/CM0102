# player_regen.cpp (leitor sx16, rodada 5, 24/11/2026): o texto da noticia de juvenil promovido aos reservas ou ao time B.
# - 0x7a8eb0 stdcall(noticia, corpo, Text) ret 0xc (a tabela de escritores do write_text 0x77bc80 a chama como metodo do
#   objeto de regens [0xb7434c]: (4, 0xbce, Global(0xb7434c, 0x7a8eb0)) no news.rs; ecx nao e usado). 0x785830
#   get_slot thiscall(item, k) ret 4 (14 vagas lidas; a 5 e lida e descartada); 0x53e140 cdecl(n1, n2, comum, buf,
#   0x190, estilo, id) (o id empurrado antes do teste do corpo, aproveitado como 7o argumento); 0x66f4e0
#   langlib_sprintf; 0x4ae660 set_string, 0x4ae770 append_string thiscall(t, texto) ret 4; 0x540a50
#   club_get_linked_b_team cdecl(clube, &local, 1); 0x53e990 cdecl(pessoa, n1, n2, comum, 0); 0x4ae8a0 add_link
#   thiscall(t, texto, tipo, dado) ret 0xc.
# Quem posta a noticia: player_regen_post_youth_promotion_news 0x7a8ba0 (ja Rust). Prova com o modo dos textos das
# noticias (CM_TEXTO_NOTICIAS=1, simular.py --textos): a comparacao chama o escritor a cada noticia enviada.
# Defeito do original: o registro do clube (vaga 4 x 581) e lido antes do teste da vaga 4 (sem clube le o registro antes
# da tabela).
SAVE = False
ROTINAS = [('player_regen_write_youth_promotion_news_text', 0x7a8eb0, [0x81, 0xec, 0x48, 0x03, 0x00, 0x00])]
DOCS = ('o texto da notícia de juvenil promovido',
        'the youth promotion news text',
        [('player_regen_write_youth_promotion_news_text', 'player_regen_write_youth_promotion_news_text', '0x7a8eb0', 'player_regen.cpp', 'stdcall, ret 0xc',
          'escreve a manchete ("<nome> promovido da academia" com a vaga 6, senão "promovido ao time reserva") ou o corpo da notícia de juvenil promovido: a frase com o gênero curto do clube, o nome e o nome curto do clube, pela vaga 6 e por o clube ter time B ligado; com um membro da comissão (vaga 8), o nome dele e a frase do parecer (vaga 12: 2, ou 1 com a vaga 13 igual a 2, ou 1, ou outro; cada um em duas versões pela vaga 7) acrescentados; os vínculos da pessoa da comissão, do clube e do jogador',
          'writes the headline ("<name> promoted from youth academy" with slot 6, otherwise "promoted to reserve squad") or the body of the youth promotion news: the sentence with the club\'s short gender, the name and the club\'s short name, by slot 6 and whether the club has a linked B team; with a staff member (slot 8), his name and the verdict sentence (slot 12: 2, or 1 with slot 13 equal to 2, or 1, or other; each in two versions by slot 7) appended; the links of the staff member, the club and the player')])
