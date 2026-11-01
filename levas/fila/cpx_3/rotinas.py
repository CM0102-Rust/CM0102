# A transferencia efetivada, com a noticia da transferencia concluida e a do acordo entre os clubes
# (transfer_manager.cpp): a transferencia (TRANSFER_MANAGER::finalise_transfer 0x8c9390, 750 chamadas ate 2002-06-01 na
# cobertura de 17/10, pelo fechamento da oferta 0x8c34e0: a confianca das diretorias, as regras da nacao, as clausulas,
# o pagamento, os registros, a parte dos clubes anteriores, o jogador e os da troca de clube, os numeros de camisa,
# estado 0x14 e a volta em 10 dias), a noticia 0xfa8 da transferencia concluida (post_transfer_completed_news 0x8dbef0,
# 750 chamadas, com um sorteio(3) e os pontos fortes sorteados 0x546ab0) e a 0xfb3 do acordo (post_transfer_agreed_news
# 0x8e05d0, 149 chamadas). Chamadas originais pelo endereco: 0x6978f0, os metodos das regras da nacao, 0x86cd70,
# 0x448760, 0x8e4f20, 0x8c9720 (mudar de clube), 0x600a50, 0x85add0, 0x85a470, 0x8c6db0, 0x8d1130, 0x8e7a70, 0x8e7ef0,
# 0x546ab0, 0x5b7d20, 0x786780, 0x77bbe0; as outras ja em Rust (0x4cf5a0, 0x68d1b0, 0x8c91e0 na fila cpw_2, 0x8ca8a0,
# 0x8c6220, 0x84aeb0, 0x7875e0, 0x785790, 0x7861e0, 0x90cfc0). Iguais no cm0102-1080.exe. Comparadores: a
# transferencia so a original com o plano do estado final (0x14); as noticias so a original (sorteiam e alocam).
SAVE = False
ROTINAS = [('efetivar_transferencia', 0x8c9390, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('noticia_transferencia_concluida', 0x8dbef0, [0x6a, 0xff, 0x68, 0x5b, 0x45, 0x96, 0x00]),
           ('noticia_transferencia_acertada', 0x8e05d0, [0x6a, 0xff, 0x68, 0x9b, 0x46, 0x96, 0x00])]
DOCS = ('a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes',
        'a transfer carried out, with the news of the completed transfer and of the clubs\' agreement',
        [('efetivar_transferencia', 'finalise_transfer', '0x8c9390', 'transfer_manager.cpp', 'thiscall',
          'efetiva a transferência: confiança das diretorias, regras da nação, cláusulas, pagamento, registros, parte dos clubes anteriores, o jogador e os da troca de clube, números de camisa, estado 0x14 e a volta em 10 dias',
          'carries out the transfer: board confidence, nation rules, clauses, payment, records, former clubs\' cut, the player and the exchange players moved, squad numbers, state 0x14 and back in 10 days'),
         ('noticia_transferencia_concluida', 'post_transfer_completed_news', '0x8dbef0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfa8 da transferência concluída, com o valor, os da troca, um sorteio e os pontos fortes do jogador',
          'the 0xfa8 news of a completed transfer, with the fee, the exchange players, a draw and the player\'s strong points'),
         ('noticia_transferencia_acertada', 'post_transfer_agreed_news', '0x8e05d0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfb3 do acordo entre os clubes, com o valor, os da troca e a data da conclusão',
          'the 0xfb3 news of the clubs\' agreement, with the fee, the exchange players and the completion date')])
CAMPOS = [('oferta +4: lista dos jogadores da troca; ponteiro; lê (as três)', 'offer +4: exchange players list; pointer; reads (all three)'),
          ('oferta +8 comprador, +0xc jogador; id; lê (as três)', 'offer +8 buyer, +0xc player; id; reads (all three)'),
          ('oferta +0x1b: valor da transferência; valor; lê (as três)', 'offer +0x1b: transfer fee; value; reads (all three)'),
          ('oferta +0x1f, +0x11, +0x2f, +0x30; valor; lê', 'offer +0x1f, +0x11, +0x2f, +0x30; value; reads'),
          ('oferta +0x2c: estado; valor; grava 0x14 (efetivar_transferencia)', 'offer +0x2c: state; value; writes 0x14 (efetivar_transferencia)'),
          ('gerência de transferências +0x8ac: as 28 regras de nação (+0xa: nação); ponteiro; lê (efetivar_transferencia)',
           'transfer manager +0x8ac: the 28 nation rules (+0xa: nation); pointer; reads (efetivar_transferencia)'),
          ('gerência de transferências +0x86e: registro de 8 bytes por pessoa, bit 0 = oferta em andamento; valor; lê e apaga (efetivar_transferencia)',
           'transfer manager +0x86e: 8-byte record per person, bit 0 = offer in progress; value; reads and clears (efetivar_transferencia)'),
          ('clube +0xd7: elenco (50 vagas); ponteiro; grava 0 na vaga do primeiro da troca (efetivar_transferencia)',
           'club +0xd7: squad (50 slots); pointer; writes 0 to the first exchange player\'s slot (efetivar_transferencia)'),
          ('pessoa +0x39 clube, +0x69, +0x61 ficha; ponteiro; lê', 'person +0x39 club, +0x69, +0x61 player record; pointer; reads'),
          ('contrato +0x2d/+0x2f: fim do contrato; valor; lê (noticia_transferencia_concluida)', 'contract +0x2d/+0x2f: contract end; value; reads (noticia_transferencia_concluida)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (as três)', '[0xae1080] last contract looked up; pointer; writes (all three)')]
INTERESSA = ['CMRTE']
