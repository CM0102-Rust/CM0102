# Os textos das notícias do jogador posto na lista de transferências e do pedido de transferência (contract_manager.cpp;
# leitor rxs1, rodada 6, 25/11/2026). As duas inteiras (0x4d7770..0x4d860f, 3,7 KB; 0x4d8990..0x4d9440, 2,7 KB), iguais
# no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até 2002-08-01 (os textos só são escritos quando alguém
# abre a notícia: 0xbea e 0xbbf, despachadas pelo NewsItem::write_text).
# Convenções conferidas na desmontagem: as duas thiscall(this, notícia, corpo, Text) ret 0xc (0x4d7770 com quadro SEH
# só para a quantia); 0x5461b0 write_position_name(posição, buffer, 0x78, 1), o retorno é o gênero; os nomes da
# 0x4d7770 em 120 bytes, os da 0x4d8990 em 300; 0x944c9f o sprintf da CRT com "%s" (0x991944) para um motivo só.
# Os modelos e a ordem dos argumentos saíram da desmontagem, conferidos com o pseudocódigo do Ghidra.
# Depende do rxs1_1 (news_text_name_n, news_text_link_name_n) e do rx07_9 (news_text_link_club_field). LeaveReasons
# é pub(crate).
# Defeitos do original (a cópia mantém): 0x4d7770, os clubes interessados vão a um vetor de três seguido do valor
# (um quarto clube toma o lugar do valor; quem posta cita no máximo três); 0x4d8990, "não gosta do auxiliar técnico"
# testa de novo o bit 0x800 do "não gosta de um colega".
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_transfer_listed_news_text', 0x4d7770, [0x6a, 0xff, 0x68, 0x2b, 0x6f, 0x95, 0x00]),
           ('write_transfer_request_news_text', 0x4d8990, [0x81, 0xec, 0x6c, 0x0b, 0x00, 0x00])]
DOCS = ('os textos das notícias do jogador posto na lista de transferências e do pedido de transferência',
        'the news texts of the player put on the transfer list and of the transfer request',
        [('write_transfer_listed_news_text', 'CONTRACT_MANAGER::write_transfer_listed_news_text', '0x4d7770', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'o título (um de quatro pelo resto de ((motivos / 5 + sorteio) × jogador) por 4: à venda, de saída, o técnico o põe na lista ou atende o pedido, insatisfeito na lista) ou o corpo: posto à venda pelo técnico ou a pedido dele, com os motivos (até cinco, das queixas do contrato) e, pelo sorteio, o valor e os clubes interessados ("a notícia já chamou a atenção de...", "um valor de cerca de ... não deve afastar...") ou "<clube> está pronto a gastar..."; os vínculos ao clube, ao jogador, ao técnico e aos clubes interessados',
          'the headline (one of four by ((grievances / 5 + draw) × player) % 4: up for sale, on the move, the manager lists him or grants his request, unhappy and listed) or the body: put up for sale by his manager or at his own request, with the reasons (up to five, from the contract grievances) and, by the draw, the fee and the interested clubs ("the news has already alerted...", "a projected fee of around ... is unlikely to deter...") or "<club> are ready to spend..."; the links to the club, the player, the manager and the interested clubs'),
         ('write_transfer_request_news_text', 'CONTRACT_MANAGER::write_transfer_request_news_text', '0x4d8990', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> pede para sair" ou o corpo: insatisfeito e pediu transferência, "pediu para sair" ou "exigiu furioso" (quando os motivos passam de jogador % 3 + 2), pelo jogador ou pelo empresário, com até cinco motivos das queixas (clube maior, contrato, banco, desafio, outro técnico, titularidade, ambição, adaptação, saudade, finanças...); mais "o pedido foi aceito" ou "recusado" quando respondido; os vínculos ao clube e à pessoa',
          'the headline "<name> requests move" or the body: unhappy and requested a transfer, "has requested to leave" or "a furious ... demanded" (when the reasons exceed player % 3 + 2), by the player or his agent, with up to five reasons from the grievances (bigger club, contract, left out, challenge, another manager, first team, ambition, adapting, homesick, finances...); plus "the request has been granted" or "turned down" when answered; the links to the club and the person')])
