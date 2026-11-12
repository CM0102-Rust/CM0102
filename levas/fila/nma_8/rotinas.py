# O botão de ação de uma notícia (NewsItem::get_action_button 0x782b70, news.cpp, 5520 bytes; lista nma, 12/11/2026).
# Chamada pela resposta automática do humano de férias (NewsItem::auto_respond 0x784100, que só age com 2), pelo
# mark_item_read (0x787140) e pelo has_urgent_items (0x787350), os dois já em Rust, com o rótulo em 0xde1f64 (0x7d0).
# 23705 chamadas na carreira ENG até 2002-06-01. Igual no cm0102-1080.exe (os 5520 bytes conferidos).
# Partida em pedaços pela categoria da notícia (get_category 0x785670): board_button (6000), general_button (1..999),
# player_button (3000), discipline_button (2000), transfer_button (4000) e o "Reply" (5000) na própria rotina; cada um
# com o seu covered! e o comparador diz o pedaço numa divergência. A rotina inteira é um Prologue só.
# Convenções conferidas na desmontagem (0x782b70..0x7840d0):
# - a rotina: thiscall(notícia, gerente, rótulo, tamanho, &tratador), ret 0x10, quadro SEH (sem destrutores), al;
# - get_category 0x785670 (Rust) chamada de novo antes de cada teste de categoria, como o original (a caixa de tipo
#   inválido sai uma vez por teste);
# - cdecl: 0x66f4e0 (langlib_sprintf, variádica), 0x7e52c0 e 0x7e4fa0 (cdecl(item, &slot, saída); escrevem 0x18 e
#   0x1f bytes), 0x53b900, 0x5487b0 (Rust; al);
# - thiscall: 0x549ef0 (Date(dia, ano), ret 8), 0x5c88f0 (mov ecx, [0xae2c74]; push clube; push &data -> ret 8),
#   0x5c89a0 (Friendly::get_tournament: push &data; push índice (só o byte) -> ret 8), 0x5ca090 e 0x5ca290 (Rust, com
#   texto 0: o dia e o ano que o original empurra como dword com a metade de cima do lixo da pilha não são lidos),
#   0x786d30 (set_item_slot, Rust: mov ecx, [0xb7431c]; push cópia, valor, campo, id, gerente -> ret 0x14),
#   0x76d7b0 (read_person_mark, Rust: mov ecx, [0xb74318] -> ret 8), 0x8c61e0 e 0x8c6200 (mov ecx, [0xde1f54] -> ret 4),
#   0x4cc9e0 (mov ecx, [0xae1050]; push clube; push pessoa -> ret 8), 0x4cf5a0 (mov ecx, [0xae1050] -> ret 4; o
#   resultado vai para 0xae1080 pela própria rotina), 0x68d1b0 (ecx = a lista);
# - stdcall com o mov ecx que sobra: 0x76d780 (ret 8, al) e 0x8d0d90 (ret 4);
# - a tabela de saltos do 0xbdf (0x7840d4/0x7840dc): o tipo de história (byte com sinal) − 0x3e em 0..0x20; 0x3e, 0x50,
#   0x51 e 0x5e terminam com 0.
# Comparador: lado a lado (o rótulo até 0x7d0, o tratador, G_CONTRACT_LOOKUP_RESULT, a notícia e o armazém do News que o
# set_item_slot escreve); as caixas (sem gerente, rótulo ou tratador, tipo fora das categorias) rodam só o original.
# Fora deste item: News::News (0x785b20; ver PRONTO.txt).
SAVE = False
ROTINAS = [('get_action_button', 0x782b70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o botão de ação de uma notícia',
        'the action button of a news item',
        [('get_action_button', 'get_action_button', '0x782b70', 'news.cpp', 'thiscall, ret 0x10',
          'escreve o rótulo do botão e o tratador da tela de uma notícia para um gerente e diz se ela pede resposta: 2 pede resposta (a resposta automática do humano de férias, a marcação de lida e as notícias urgentes olham isso), 1 botão sem resposta obrigatória, 0 nenhum; sem gerente, rótulo (ou tamanho abaixo de 0x33) ou tratador, a caixa "Error" e 0. Pela categoria: diretoria (0x1774 "Respond" com o campo 1 zero; 0x1786 "Ultimatum" com o campo 1 diferente de 5, o 2 diferente de 0 e 6 e o 5 zero); geral (convite de amistoso e de torneio ainda de pé, pela Friendly; 0x16, 0x18, 0x19, 0x1e pelo clube do gerente; a seleção 0x22 do dia, pela marca da pessoa na lista da seleção, e a de outro dia fica respondida; o sorteio da copa 0x27 "View Draw" enquanto houver jogo da rodada por criar, depois fica respondido; a Copa da Inglaterra 0x28 com a pergunta da 5ª fase feita; as indicações a prêmio 0x1c); jogador (histórias da imprensa 0xbd0, 0xbd9, 0xbdd, 0xbdf (fora os tipos 0x3e, 0x50, 0x51, 0x5e), 0xbe6, 0xbe8 do clube que ele dirige; 0xbbf, 0xbc1, 0xbc2 "Offer Contract" do seu clube; 0xbb8 "Negotiate" com a proposta de contrato no estado 3); disciplina (0x7d2 e 0x7d3 "Appeal": pessoa válida, jogo com clube e competição válidos, clube que não é seleção, competição que aceita recurso, mesma equipe; o clube fora da Argentina, tipo diferente de 0, 1 e 4, sem recurso feito ou pendente, a audiência hoje ou a suspensão ainda por começar); transferências (0xfa0 e 0xfb5 a oferta por um jogador do clube que ele dirige, aberta e com o número de série; 0xfbc; 0xfa2 e 0xfc0 a negociação em curso; 0xfac "Offer Contract" pela compra do seu clube ou pelo jogador da troca; 0xfa4 "Negotiate" nos passos 3 e 4, senão o campo da oferta vira −1; 0xfa5 "Confirm"; 0xfab; 0xfb4 "Re-Negotiate"; 0xfc1 "Details"); 0x1388 "Reply"',
          'writes the button label and the screen handler of a news item for a manager and says whether it needs a reply: 2 needs a reply (the on-holiday human\'s automatic reply, the read marking and the urgent news look at it), 1 a button without a needed reply, 0 none; without a manager, a label (or a size below 0x33) or a handler, the "Error" box and 0. By category: board (0x1774 "Respond" with field 1 zero; 0x1786 "Ultimatum" with field 1 other than 5, field 2 other than 0 and 6 and field 5 zero); general (friendly and tournament invitations still open, through the Friendly object; 0x16, 0x18, 0x19, 0x1e by the manager\'s club; the national squad 0x22 of today, by the person mark in the squad list, and one of another day is marked answered; the cup draw 0x27 "View Draw" while a fixture of the round is still to be made, then marked answered; the FA Cup 0x28 with the 5th-round question asked; the award nominations 0x1c); player (press stories 0xbd0, 0xbd9, 0xbdd, 0xbdf (except kinds 0x3e, 0x50, 0x51, 0x5e), 0xbe6, 0xbe8 of the club he manages; 0xbbf, 0xbc1, 0xbc2 "Offer Contract" of his club; 0xbb8 "Negotiate" with the contract offer in state 3); discipline (0x7d2 and 0x7d3 "Appeal": a valid person, a match with a valid club and competition, a club that is not a national team, a competition that allows appeals, the same club; the club outside Argentina, a kind other than 0, 1 and 4, no appeal made or pending, the hearing today or the ban still to start); transfers (0xfa0 and 0xfb5 the bid for a player of the club he manages, open and with the serial; 0xfbc; 0xfa2 and 0xfc0 the negotiation under way; 0xfac "Offer Contract" for his club\'s purchase or for the exchange player; 0xfa4 "Negotiate" at steps 3 and 4, otherwise the offer field becomes −1; 0xfa5 "Confirm"; 0xfab; 0xfb4 "Re-Negotiate"; 0xfc1 "Details"); 0x1388 "Reply"')])
CAMPOS = [('notícia 0x27 (sorteio da copa): campo 0 a competição, 1 o rótulo da fase, 4 o índice da fase ou 0xff, 5 a rodada, 6 a resposta; valor; lê e grava (6)',
           'news 0x27 (cup draw): field 0 the competition, 1 the stage label, 4 the stage index or 0xff, 5 the round, 6 the answer; value; reads and writes (6)'),
          ('notícia 0x22 (convocação): campo 4 o clube, 5 a vaga na lista, 7 a resposta; valor; lê e grava (7)',
           'news 0x22 (national squad): field 4 the club, 5 the squad slot, 7 the answer; value; reads and writes (7)'),
          ('notícias 4000 (transferências): campo 6/7/11 o índice da oferta no TRANSFER_MANAGER, o seguinte o número de série (offer::SERIAL) [inferência]; valor; lê (0xfa4 grava −1 no 7)',
           'news 4000 (transfers): field 6/7/11 the offer index in the TRANSFER_MANAGER, the next one its serial (offer::SERIAL) [inference]; value; reads (0xfa4 writes −1 into 7)')]
