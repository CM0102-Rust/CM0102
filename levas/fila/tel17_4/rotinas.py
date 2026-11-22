# The offer changed screen, the remove exchange player dialog and the confirm transfer news button
# (transfer_screens.cpp, part 1; reader tel17, the screens round, 21/11/2026). Equal in cm0102-1080.exe. Needs tel17_2
# (ts_* helpers) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f2470 open_offer_changed_screen cdecl(offer, serial) ret 0; 0x8f24c0 build_offer_changed_screen cdecl() ret 0,
#   SEH frame (Text); 0x8f2800 handle_offer_changed_action cdecl(item) -> int.
# - 0x8f2870 open_remove_exchange_player_dialog cdecl(slot, player id) ret 0; 0x8f28c0 build cdecl() ret 0; 0x8f2b90
#   handle cdecl(item) -> int.
# - 0x8f2d20 news_confirm_transfer_action cdecl(news id a, news id b) ret 0, SEH frame (NewsItem).
# gui_add_text_area 0x5ed070 is variadic here: after the text, (link text, id, object) triples ended by 0.
# Comparers: the builds side by side over the display list (the offer changed one also over the contract lookup
# global); the openers and the news button only the original; Done and No side by side; Yes of the exchange dialog
# only the original (it changes the bid).
TELAS = True
ROTINAS = [('transfer_screens_open_offer_changed_screen', 0x8f2470, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_offer_changed_screen', 0x8f24c0, [0x6a, 0xff, 0x68, 0x5b, 0x4d, 0x96, 0x00]),
           ('transfer_screens_handle_offer_changed_action', 0x8f2800, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('transfer_screens_open_remove_exchange_player_dialog', 0x8f2870, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_remove_exchange_player_dialog', 0x8f28c0, [0x6a, 0xff, 0x68, 0x7b, 0x4d, 0x96, 0x00]),
           ('transfer_screens_handle_remove_exchange_player_action', 0x8f2b90, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_news_confirm_transfer_action', 0x8f2d20, [0x6a, 0xff, 0x68, 0x9b, 0x4d, 0x96, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('a tela de oferta mudada, a janela de remover jogador de troca e o botão de confirmar transferência',
        'the offer changed screen, the remove exchange player dialog and the confirm transfer button',
        [('transfer_screens_open_offer_changed_screen', 'transfer_screens_open_offer_changed_screen', '0x8f2470', _F, 'cdecl',
          'abre a tela que avisa que o clube mudou, recusou, retirou ou aceitou a proposta (parâmetros: a oferta e o número de série conhecido)',
          'opens the screen that tells that the club changed, rejected, withdrew or accepted the bid (params: the offer and the known serial)'),
         ('transfer_screens_build_offer_changed_screen', 'transfer_screens_build_offer_changed_screen', '0x8f24c0', _F, 'cdecl',
          'monta a tela de oferta mudada: "Proposta de transferência por <jogador>" e o texto pelo número de série (mudada) ou pelo status (recusada, aceita, retirada), com os links dos clubes e do jogador, e o botão Concluído',
          "builds the offer changed screen: \"Transfer Bid for <player>\" and the text by the serial (changed) or the status (rejected, accepted, withdrawn), with the clubs' and the player's links, and the Done button"),
         ('transfer_screens_handle_offer_changed_action', 'transfer_screens_handle_offer_changed_action', '0x8f2800', _F, 'cdecl',
          'responde à tela de oferta mudada: Concluído -10; o resto vai para a barra de menus',
          'answers the offer changed screen: Done -10; the rest goes to the menu bar'),
         ('transfer_screens_open_remove_exchange_player_dialog', 'transfer_screens_open_remove_exchange_player_dialog', '0x8f2870', _F, 'cdecl',
          'abre a janela que pergunta se tira um jogador de troca da proposta (parâmetros: a vaga da proposta e o jogador)',
          'opens the dialog that asks whether to remove an exchange player from the bid (params: the bid slot and the player)'),
         ('transfer_screens_build_remove_exchange_player_dialog', 'transfer_screens_build_remove_exchange_player_dialog', '0x8f28c0', _F, 'cdecl',
          'monta a janela de remover jogador de troca: o texto com os dois jogadores e a nova parte em dinheiro (a taxa mais o valor do jogador), Não e Sim',
          "builds the remove exchange player dialog: the text with both players and the new cash part (the fee plus the player's value), No and Yes"),
         ('transfer_screens_handle_remove_exchange_player_action', 'transfer_screens_handle_remove_exchange_player_action', '0x8f2b90', _F, 'cdecl',
          'responde à janela: Sim soma o valor do jogador à taxa e, se ele está na lista de troca, tira-o e, com comprador do computador, o clube revê a proposta (-10); Não -9',
          "answers the dialog: Yes adds the player's value to the fee and, when he is in the exchange list, removes him and, with a computer buyer, the club reviews the bid (-10); No -9"),
         ('transfer_screens_news_confirm_transfer_action', 'transfer_screens_news_confirm_transfer_action', '0x8f2d20', _F, 'cdecl',
          'Confirmar uma transferência acertada: com a oferta igual (ou ausente) e não retirada, abre a tela final de confirmação; senão a tela de oferta mudada',
          'Confirm an agreed transfer: with the offer unchanged (or missing) and not withdrawn, opens the final confirmation screen; otherwise the offer changed screen')])
