# A noticia de uma disputa de contrato (dispute_post_news 0x56a3b0, dispute.cpp, 420 bytes ate o ret 0x14 de 0x56a551;
# leitor sx09b, rodada 5, 23/11/2026). Depende do sx09b_1 (os auxiliares de caixa human_inbox_marks e
# delivered_news_items): integrar junto ou depois.
# Chamada pela process_hearings (0x569ff4, resultado 1 ou 2; o item sx09b_5 a chama pelo endereco), pela file_complaint
# (0x56a38b) e pelo dia do jogo (0x5cf40b).
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(disputa, pessoa, clube, resultado, a, b), ret 0x14; o ecx nao e lido; quadro SEH da noticia
#   local, sem destrutor;
# - 0x77bbe0 NewsItem::NewsItem thiscall(0xbe9, 0) ret 8; 0x785790 set_slot thiscall(k, valor) ret 8 (vagas 0..9, em
#   ordem; os nomes pelo +0x33 de +4, +8, +0xc sem conferir nulo, como a set_news_person_slots);
# - a reputacao mundial (+0xd da ficha) > 7500 com sinal: 0x786780 News::send_to_all_managers thiscall([0xb7431c],
#   noticia) ret 4; senao 0x7863f0 News::send_to_nation_managers thiscall([0xb7431c], noticia, nacao) ret 8, a nacao
#   do clube (+0x53 do clube do jogador, quando ha) e depois a do jogador (+0x1a) quando difere (o clube relido sem
#   conferir nulo; ja conferido na entrada).
# Comparador: ensaio (a copia monta a noticia sem mandar e conta os envios), depois a original; toda noticia 0xbe9 que
# chegou a uma caixa humana tem de ser a da copia, e sem envio nenhuma pode chegar. As disputas provavelmente nao
# acontecem na carreira de ferias (ver sx09b_5): a rotina pode nao rodar ate 2002-06-01.
SAVE = False
ROTINAS = [('dispute_post_news', 0x56a3b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a noticia de uma disputa de contrato',
        'the news item of a contract dispute',
        [('dispute_post_news', 'dispute_post_news', '0x56a3b0', 'dispute.cpp', 'thiscall, ret 0x14',
          'monta a noticia 3049 (0xbe9) da disputa de um jogador com um clube (so com jogador, clube atual, ficha e clube): o jogador e os nomes, o clube da disputa, o clube atual, dois valores do chamador, a nacao do clube atual (-1 sem ela) e o resultado (1 fica no clube, 2 sai livre). Jogador com reputacao mundial acima de 7500: a todos os tecnicos humanos; senao aos tecnicos da nacao do clube e, se a nacao do jogador e outra, tambem aos dela',
          "builds news item 3049 (0xbe9) of a player's dispute with a club (only with a player, a current club, a player sheet and a club): the player and his names, the dispute's club, the current club, two values from the caller, the current club's nation (-1 without one) and the outcome (1 stays at the club, 2 leaves for free). A player with a world reputation above 7500: to every human manager; otherwise to the managers of the club's nation and, if the player's nation is another, also to its managers")])
