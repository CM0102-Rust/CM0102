# Os textos das noticias da competicao: o destino de um clube, o premio em dinheiro e os direitos de TV (comp.cpp;
# leitor cx01, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS). Escritores de texto: rodam quando a noticia e aberta e,
# com CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando ela e enviada; so a comparacao as prova.
# - 0x48c6d0 Comp::build_fate_news_text thiscall(clube, desfecho, _, _, partida, fase_seguinte, _, veredito, corpo,
#   texto) ret 0x28 (o slot 0x48 de quase toda competicao; os build_fate_news_text das copas e ligas o chamam direto, 60
#   lugares). Dentro: 0x4b0b80 cdecl(clube, partida, veredito, desfecho, fase_seguinte, texto) e 0x4b4420 cdecl(clube,
#   partida, desfecho, texto) (add esp); langlib_sprintf cdecl; Text::set_string thiscall(texto) ret 4; Text::add_link
#   thiscall(frase, tipo, dado) ret 0xc; Friendly::is_friendly_competition (mov ecx, [0xae2c74]; push comp) ret 4; crt
#   sprintf cdecl.
# - 0x48e590 Comp::write_prize_money_news_text thiscall(noticia, corpo, saida) ret 0xc (o NewsItem::write_text, tipo
#   0x1a). Dentro: NewsItem::get_slot thiscall(k) ret 4; Cash::get_display_amount thiscall(&valor, 6) ret 4 e
#   cash_format_money cdecl(buf, 0x1e, valor64, 6) (o primeiro push 6 e o quarto argumento do segundo);
#   utils_format_ordinal cdecl(posicao, -1); comp_util_format_comp_stage_round_label cdecl(comp, 0, rodada, 1, buf,
#   0x12c); comp_util_get_label_gender cdecl(comp, 0, rodada, 1).
# - 0x48eb00 Comp::write_tv_rights_news_text thiscall(noticia, corpo, saida) ret 0xc (tipo 0x1f). Dentro:
#   key_nation_format_club_season cdecl(ano, clube, buf, 0x14).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: compare_text_writer (a original no Text de verdade, a copia num local).
# Defeito do original (0x48e590): sem posicao, rodada, tipo 1/2 nem adversario (indice -1), o texto fica vazio.
SAVE = False
ROTINAS = [('comp_build_fate_news_text', 0x48c6d0, [0x8b, 0x44, 0x24, 0x14, 0x53]),
           ('comp_write_prize_money_news_text', 0x48e590, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_write_tv_rights_news_text', 0x48eb00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos das notícias da competição: o destino de um clube, o prêmio em dinheiro e os direitos de TV',
        'the competition news texts: a club\'s fate, the prize money and the TV rights',
        [('comp_build_fate_news_text', 'Comp::build_fate_news_text', '0x48c6d0', 'comp.cpp', 'thiscall, ret 0x28',
          'escreve a notícia do destino de um clube: num confronto de copa, a linha comum da copa (o resultado do confronto no corpo, a vitória ou derrota na manchete); senão, pelo desfecho, o título (5), o acesso (0), a vaga no mata-mata (1), a queda para o mata-mata (2) ou o rebaixamento (3), com o clube e, no corpo, o nome da competição (que não ganha ligação no título de um amistoso); outro desfecho dá a linha de erro',
          'writes the news of a club\'s fate: in a cup tie, the shared cup line (the tie result in the body, the win or defeat in the headline); otherwise, by the outcome, the title (5), promotion (0), the play-off place (1), the drop into the play-offs (2) or relegation (3), with the club and, in the body, the competition\'s name (not linked for a friendly competition\'s title); another outcome gives the error line'),
         ('comp_write_prize_money_news_text', 'Comp::write_prize_money_news_text', '0x48e590', 'comp.cpp', 'thiscall, ret 0xc',
          'escreve a notícia do prêmio: na manchete o clube e o valor; no corpo, pela ordem, a posição final (ordinal e competição), a rodada (o rótulo da fase e rodada), o título ou o vice, ou a partida (o adversário, vitória ou não); sem nada disso o texto fica vazio',
          'writes the prize money news: in the headline the club and the amount; in the body, in order, the final position (ordinal and competition), the round (the stage and round label), the title or runners up, or the match (the opponent, won or not); with none of these the text stays empty'),
         ('comp_write_tv_rights_news_text', 'Comp::write_tv_rights_news_text', '0x48eb00', 'comp.cpp', 'thiscall, ret 0xc',
          'escreve a notícia dos direitos de TV: na manchete o clube e o valor; no corpo também o nome curto da competição e a temporada',
          'writes the TV rights news: in the headline the club and the amount; in the body also the competition\'s short name and the season')])
CAMPOS = []
INTERESSA = []
