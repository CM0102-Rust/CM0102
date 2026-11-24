# A resposta automática a uma notícia da caixa do humano (NewsItem::auto_respond 0x784100, news.cpp, 0x784100..0x784eaa,
# 3,5 KB; leitor sxA, rodada 5, 24/11/2026). Chamada pela News::auto_respond_all (0x7874b0, ao sair de férias, ser
# demitido ou contratado) e pela deliver_to_manager com o humano de férias; roda na carreira (463 entregas até
# 2002-06-01). Simulação, sem LIGAS.
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(notícia, técnico, clube), ret 8, sem retorno; quadro SEH com uma LLIST local (0x68d020 /
#   0x68d040) e datas locais (0x54a110, 0x549ef0 com as palavras, 0x417c10);
# - 0x782b70 thiscall(notícia, técnico, 0xde1f64, 0x7d0, &local) -> al; 0x785670 thiscall -> al (lida até três vezes
#   pela original, pura); 0x786d30 thiscall em [0xb7431c](técnico, id +0xd6, campo, valor, cópia) ret 0x14;
# - 0x84af00 é stdcall (o ecx carregado não é lido): (vendedor, jogador, qword 0, comprador, qword -1, 1);
# - 0x76d780 stdcall(clube, campo 1); 0x76e010/0x76c120/0x76c0f0 recebem o byte do campo (a original empilha o dword
#   com os 3 bytes de cima sem valor; as chamadas só leem o byte);
# - 0x5acee0 thiscall ret 0x48 (18 argumentos, a ordem do comp_screens): o dia e o ano saem de registradores com a
#   palavra de cima do índice do clube (mantida);
# - 0x8c0660 (accept_loan_offer) thiscall(vaga, 0) ret 8, chamada pelo endereço (FN declarado no item).
# Prólogo: mov eax, fs:[0] (6 bytes, sem relativo). Corpo igual no cm0102.exe e no cm0102-1080.exe.
# Comparador: só a original (as respostas mudam amistosos, emprego e transferências); confere que o tipo e o id da
# notícia não mudam. A cópia é provada pelas mutações na carreira (modo Rust).
# Defeitos do original: no convite de torneio (tipo 9), clube fora dos convidados deixa o byte do índice sem valor e a
# original lê a marca do convite por ele (a cópia roda a original nesse caso, sem inventar o byte); a resposta 0xfab
# passa cópia 0 à set_item_slot.
ROTINAS = [('auto_respond', 0x784100, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a resposta automática a uma notícia da caixa do humano',
        "the automatic reply to a news item of the human's inbox",
        [('auto_respond', 'NewsItem::auto_respond', '0x784100', 'news.cpp', 'thiscall, ret 8',
          'a resposta automática a uma notícia (férias, demissão, contratação): sem técnico, a caixa de erro; só a notícia cujo botão é o de responder (2); com o clube sendo seleção, só as respostas de seleção. Categoria 6, tipo 0x1774 (proposta de emprego): de férias e (sem clube ou com a opção 1 das férias), recusa a do clube de reputação menor que a do seu (e para) ou aceita e assume; senão recusa. Categoria 2: aceita o amistoso (6) e o convite de torneio (9), responde 0x16, numera o elenco da liga (0x18), inscreve no europeu (0x19), convocação (0x1e: o próximo jogo da seleção, os elencos 0 e da vaga), substituto na convocação (0x22) e 0x28. Categoria 4: pedido de lista (0xbbf: valor × 1,5 acima do preço pedido aceita, lista e notícia; senão moral do contrato e, por sorteio, a marca 0x10000), situação no elenco (0xbc1), 0xbc2 e proposta de contrato (0xbb8). Categoria 5: empréstimos pela previsão do vendedor (0xfa0, 0xfb5), lances revistos (0xfa2), o contrato da oferta (0xfac), rodada 3 (0xfa4), restrições da oferta (0xfa5), número no elenco (0xfa8, 0xfb6), lances retirados ou recusados (0xfab, 0xfb4) e o contrato do empréstimo (0xfc0)',
          "the automatic reply to a news item (holiday, sacking, hiring): without a manager, the error box; only an item whose action button is the reply button (2); with the club being a national team, only the national answers. Category 6, type 0x1774 (job offer): on holiday and (without a club or with holiday option 1), declines a club whose reputation is below his own (and stops) or accepts and takes over; otherwise declines. Category 2: accepts the friendly (6) and the tournament invitation (9), answers 0x16, numbers the league squad (0x18), registers for Europe (0x19), the call-up (0x1e: the national side's next match, squads 0 and the slot's), the call-up replacement (0x22) and 0x28. Category 4: the transfer request (0xbbf: value × 1.5 above the asking price accepts, lists and posts the news; otherwise the contract's morale and, by a draw, flag 0x10000), the squad status (0xbc1), 0xbc2 and the contract offer (0xbb8). Category 5: loan bids by the seller's prediction (0xfa0, 0xfb5), bids reviewed (0xfa2), the offer's contract (0xfac), response round 3 (0xfa4), offer restrictions (0xfa5), a squad number (0xfa8, 0xfb6), bids withdrawn or rejected (0xfab, 0xfb4) and the loan contract (0xfc0)")])
