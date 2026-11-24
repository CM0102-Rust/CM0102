# As noticias de acesso e queda forcados e o progresso de um clube nas estatisticas (comp.cpp; leitor cx01, rodada 5,
# 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x48ece0 Comp::send_promotion_relegation_override_news thiscall(outra, clube, mae, tipo) ret 0x10: chamada no fim da
#   temporada pela cadeia de acesso e queda (EngPrm 0x574ff4/0x57545f: o reserva rebaixado pela queda do clube-mae ou o
#   acesso negado pelo estadio; GerFirst, PorPrm, SpaFirst). Dentro: NewsItem::NewsItem thiscall(0x23, 0) ret 8;
#   NewsItem::set_slot thiscall(k, valor) ret 8; News::send_to_division_managers (mov ecx, [0xb7431c]; push comp; push
#   noticia) ret 8; a caixa (linha 0x897) sem clube.
# - 0x48ee40 Comp::write_promotion_relegation_override_news_text thiscall(noticia, corpo, saida) ret 0xc (o
#   NewsItem::write_text, tipo 0x23; so a comparacao com CM_TEXTOS=1 o ve).
# - 0x48e380 Comp::set_club_progress thiscall(clube, _, atual, melhor, pior) ret 0x14 (slot 0x68; ScoFirst o chama
#   direto): a etapa repassa ao slot 0x68 da mae (5 push; mov ecx), senao Comp_Stats::find_club_entry thiscall(clube)
#   ret 4 e os bytes +0x73..+0x75. Prologo cmp byte [ecx+0x44], 0xff; je rel8: RELATIVAS (0x48e380, 4).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o envio so a original (entrega a noticia); o texto por compare_text_writer; o progresso lado a lado nos
# 3 bytes da entrada do clube nas estatisticas de quem as guarda.
# Defeito do original (0x48ee40): no corpo do tipo 1 o nome longo da competicao vai com o genero do nome curto (+0x52).
# Defeito do original (0x48e380): a mae da etapa nao e testada (nula derruba o jogo).
SAVE = False
ROTINAS = [('comp_send_promotion_relegation_override_news', 0x48ece0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_write_promotion_relegation_override_news_text', 0x48ee40, [0x83, 0xec, 0x0c, 0x55, 0x56]),
           ('comp_set_club_progress', 0x48e380, [0x80, 0x79, 0x44, 0xff, 0x74, 0x2f])]
DOCS = ('as notícias de acesso e queda forçados e o progresso de um clube nas estatísticas',
        "the promotion and relegation override news and a club's progress in the statistics",
        [('comp_send_promotion_relegation_override_news', 'Comp::send_promotion_relegation_override_news', '0x48ece0', 'comp.cpp', 'thiscall, ret 0x10',
          'manda aos técnicos da divisão desta competição (e da outra, se houver) a notícia do clube reserva rebaixado pela queda do clube-mãe (tipo 1) ou do acesso negado (tipo 2), com a competição, o clube, o clube-mãe e a outra competição; sem clube, a caixa de erro',
          "sends the managers of this competition's division (and of the other one, if any) the news of the reserve club relegated because of its parent club (kind 1) or of the promotion refused (kind 2), with the competition, the club, the parent club and the other competition; without a club, the error box"),
         ('comp_write_promotion_relegation_override_news_text', 'Comp::write_promotion_relegation_override_news_text', '0x48ee40', 'comp.cpp', 'thiscall, ret 0xc',
          'escreve essa notícia: tipo 1, no corpo o clube, a competição, o clube-mãe e a outra competição, na manchete o clube e a competição; tipo 2, no corpo o clube e a competição, na manchete só o clube; outro tipo não escreve nada',
          'writes that news: kind 1, in the body the club, the competition, the parent club and the other competition, in the headline the club and the competition; kind 2, in the body the club and the competition, in the headline only the club; another kind writes nothing'),
         ('comp_set_club_progress', 'Comp::set_club_progress', '0x48e380', 'comp.cpp', 'thiscall, ret 0x14',
          'guarda as posições atual, melhor e pior do clube nas estatísticas da competição; uma etapa repassa à competição-mãe',
          "stores the club's current, best and worst positions in the competition statistics; a stage passes it on to the parent competition")])
CAMPOS = []
INTERESSA = []
