# A aprovacao final e o fechamento da oferta (transfer_manager.cpp): a aprovacao de cada clube (TRANSFER_MANAGER::
# confirm_offer_by_club 0x8c3310, pela oferta passada aos clubes 0x8c2f50: com os dois de acordo, agenda a conclusao
# ou faz a transferencia futura), o fechamento da oferta agendada (try_complete_offer 0x8c34e0, pela oferta agendada
# 0x8c63a0: reconfirma comprador e vendedor e conclui, desfaz ou adia) e o acerto das clausulas das transferencias
# anteriores (settle_previous_transfer_clauses 0x8c91e0, pela conclusao 0x8c9390). Chamadas originais pelo endereco:
# 0x8c41c0, 0x8cfc90, 0x8c9390, 0x8dbef0, 0x8e05d0, 0x8cb0f0, 0x68d020/0x68d040 (a lista local); as outras ja em Rust
# (0x8cee70, 0x8c6220, 0x8cb9a0, 0x8c47d0, 0x4cf560, 0x8c36b0, 0x8c44e0, 0x8c6ae0, 0x8cb340, 0x8cb4f0, 0x8cb590,
# 0x8cb630). Iguais no cm0102-1080.exe. Comparadores: a aprovacao lado a lado sobre a oferta quando o acordo nao fecha
# (so o estado muda), so a original quando fecha; o acerto lado a lado sem clausula marcada, so a original com alguma
# (paga e da noticia); o fechamento so a original (conclui transferencias).
SAVE = False
ROTINAS = [('confirmar_oferta_pelo_clube', 0x8c3310, [0x6a, 0xff, 0x68, 0x10, 0x3c, 0x96, 0x00]),
           ('tentar_concluir_oferta', 0x8c34e0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('acertar_clausulas_anteriores', 0x8c91e0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14])]
DOCS = ('a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas anteriores',
        'the clubs\' final approval of an offer, the completion of an approved offer and the settling of earlier clauses',
        [('confirmar_oferta_pelo_clube', 'confirm_offer_by_club', '0x8c3310', 'transfer_manager.cpp', 'thiscall',
          'registra a aprovação de um clube (estados 0xb a 0xe); com os dois de acordo, agenda a conclusão para hoje ou, com a data do prazo ainda por vir, faz a transferência futura',
          'records one club\'s approval (states 0xb to 0xe); once both agree, schedules completion for today or, with the deadline still ahead, books a future transfer'),
         ('tentar_concluir_oferta', 'try_complete_offer', '0x8c34e0', 'transfer_manager.cpp', 'thiscall',
          'reconfirma comprador e vendedor; uma recusa desfaz ou recusa a oferta; com os dois de acordo, conclui a transferência ou a adia para o fim do contrato',
          'rechecks buyer and seller; a refusal collapses or rejects the offer; with both agreeing, completes the transfer or defers it to the end of the contract'),
         ('acertar_clausulas_anteriores', 'settle_previous_transfer_clauses', '0x8c91e0', 'transfer_manager.cpp', 'thiscall',
          'paga e tira as cláusulas antigas (percentual da próxima venda, prêmios por jogo no time e na seleção) do jogador e das pessoas da troca',
          'pays and clears the old clauses (next sale percentage, club and international appearance money) of the player and the exchange players')])
CAMPOS = [('oferta +4: lista dos jogadores da troca; ponteiro; lê (as três)', 'offer +4: list of exchange players; pointer; reads (all three)'),
          ('oferta +8: clube comprador; +0xc: jogador; id; lê (as três)', 'offer +8: buying club; +0xc: player; id; reads (all three)'),
          ('oferta +0x2c: estado (0xb a 0xe na aprovação); valor; lê e grava (confirmar_oferta_pelo_clube)',
           'offer +0x2c: state (0xb to 0xe on approval); value; reads and writes (confirmar_oferta_pelo_clube)'),
          ('oferta +0x2f: confirmação; +0x30 & 0xf: tipo de prazo (3 = fim do contrato); valor; lê (confirmar_oferta_pelo_clube, tentar_concluir_oferta)',
           'offer +0x2f: confirmation; +0x30 & 0xf: deadline type (3 = end of contract); value; reads (confirmar_oferta_pelo_clube, tentar_concluir_oferta)'),
          ('contrato +4: clube; id; +0x35 & 0x3f: tipo (5 sem clube); +0x2d: fim; lê (tentar_concluir_oferta)',
           'contract +4: club; id; +0x35 & 0x3f: type (5 without club); +0x2d: end; reads (tentar_concluir_oferta)'),
          ('[this+0x86e] marcas de 8 bytes por pessoa (bits 2, 4, 8: cláusulas); valor; lê (acertar_clausulas_anteriores)',
           '[this+0x86e] 8-byte flags per person (bits 2, 4, 8: clauses); value; reads (acertar_clausulas_anteriores)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (tentar_concluir_oferta)',
           '[0xae1080] the last contract asked for; pointer; writes (tentar_concluir_oferta)')]
INTERESSA = ['CMRTE']
