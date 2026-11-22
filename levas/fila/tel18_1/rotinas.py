# The final transfer confirmation, the shortlist and comparison prompts and the withdraw bid dialog
# (transfer_screens.cpp, part 2; reader tel18, the screens round, 21/11/2026). Equal in cm0102-1080.exe (no byte of
# 0x8f2e10..0x8fc3ac differs). Needs tel17_1 (ts_copy_offer, ts_copy_offer_list, TS_FN_NEW_TRANSFER_OFFER,
# TS_FN_SELLING_CLUB_MUST_ACCEPT), tel17_2 (the ts_* helpers, ts_contract_club_not_type_5), tel17_4
# (TsTextAreaLinks), tel26_1 and tel29_1 (copy_c_string). Its helpers (ts_news_slot, ts_word_over,
# ts_open_must_accept, TS_CANCEL, TS_DELAY, TS_FN_CONFIRM_OFFER_SLOT) are used by tel18_2..tel18_4.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f2e10 open_confirm_transfer_screen cdecl(slot, news id a, news id b) ret 0.
# - 0x8f2ec0 build_confirm_transfer_screen cdecl() ret 0, SEH frame (operator new of the confirm copy); assert box
#   line 0x15e1 for a sixth exchange player.
# - 0x8f3c70 handle_confirm_transfer_action cdecl(item) -> int; the restriction reason byte is pushed as a word over
#   the high half of whatever register held it (param 6, the offer pointer, News::set_item_slot's eax): the copy
#   pushes the same dwords (ts_word_over). The Delay path passes the item argument's own slot as the must accept
#   reason (it starts as the item argument).
# - 0x8f4160 open_shortlist_dialog cdecl(&player id, add) ret 0; 0x8f41b0 open_comparison_prompt cdecl(&player id,
#   unused, char which) ret 0; 0x8f4c70 open_withdraw_bid_dialog cdecl(slot) ret 0.
# - 0x8f4210 / 0x8f4650 / 0x8f4cb0 builds cdecl() ret 0; 0x8f44d0 / 0x8f48b0 (SEH frame, a Date) / 0x8f4ed0 handlers
#   cdecl(item) -> int.
# Callees: HUMAN_MANAGER::get_confirm_offer_slot 0x5fdb30 (thiscall, 1), TRANSFER_MANAGER::check_offer_restrictions
# 0x8cfc90 (thiscall, 4: offer, club, &short reason, 1; returns the state in al), SCOUT_MANAGER::set_compare_player
# 0x7f7780 (thiscall, 3), remove_from_shortlist 0x7f7a20 (thiscall, 2), add_to_shortlist 0x7f75f0 (2),
# shortlist_manager_is_compensation_deal 0x84b6f0 (thiscall on [TM+0x856], 4: deal record, contract, club, &out).
# Comparers: the builds side by side over the display list (the confirm screen also over the contract lookup global;
# only the original while its confirm copy is still to be created); the openers only the original; the answers that
# only return a code (No) side by side, the rest only the original (they change the offer, the shortlist, the scout
# manager or open screens). The automatic career opens no screen: the proof is Lucas's game.
# Defects of the original: none found (the shortlist's press check passes a deal record whose other bytes are the
# frame's leftovers; the copy zeroes them).
TELAS = True
ROTINAS = [('transfer_screens_open_confirm_transfer_screen', 0x8f2e10, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_confirm_transfer_screen', 0x8f2ec0, [0x6a, 0xff, 0x68, 0xe2, 0x4d, 0x96, 0x00]),
           ('transfer_screens_handle_confirm_transfer_action', 0x8f3c70, [0x83, 0xec, 0x10, 0x8b, 0x0d, 0x98, 0x3c, 0xb6, 0x00]),
           ('transfer_screens_open_shortlist_dialog', 0x8f4160, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_open_comparison_prompt', 0x8f41b0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_comparison_prompt', 0x8f4210, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_comparison_prompt_action', 0x8f44d0, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_build_shortlist_dialog', 0x8f4650, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_shortlist_dialog', 0x8f48b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('transfer_screens_open_withdraw_bid_dialog', 0x8f4c70, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_withdraw_bid_dialog', 0x8f4cb0, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_withdraw_bid_dialog', 0x8f4ed0, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('a confirmação final da transferência, as janelas da lista de observação e da comparação e a de retirar a proposta',
        'the final transfer confirmation, the shortlist and comparison prompts and the withdraw bid dialog',
        [('transfer_screens_open_confirm_transfer_screen', 'transfer_screens_open_confirm_transfer_screen', '0x8f2e10', _F, 'cdecl',
          'abre a tela da confirmação final da transferência: parâmetros 0 a oferta viva, 1 e 2 os números da notícia, 4 o número de série da oferta e 5 o jogador',
          "opens the final transfer confirmation screen: params 0 the live offer, 1 and 2 the news item's ids, 4 the offer's serial and 5 the player"),
         ('transfer_screens_build_confirm_transfer_screen', 'transfer_screens_build_confirm_transfer_screen', '0x8f2ec0', _F, 'cdecl',
          'monta a confirmação final: cria a cópia de confirmação da oferta quando falta; as restrições da transferência (estado e motivo, parâmetros 6 e 3) dão o subtítulo (confirmar, adiar, cancelada); no estado 0, comprar ou vender o jogador pelo valor (a taxa, ou ela mais as cláusulas e a porcentagem da próxima venda quando isso passa a taxa) e os jogadores de troca (até 5; o sexto mostra a caixa de erro, linha 0x15e1); nos outros estados o texto da restrição; a barra Voltar / Cancelar / Adiar ou Voltar / Não / Adiar / Sim',
          'builds the final confirmation: creates the confirm copy of the offer when missing; the transfer restrictions (state and reason, params 6 and 3) give the subtitle (confirm, delay, cancelled); in state 0, buy or sell the player for the money (the fee, or it plus the clauses and the sell-on percentage when that is above the fee) and the exchange players (up to 5; a sixth shows the error box, line 0x15e1); in the other states the restriction text; the bar Back / Cancel / Delay or Back / No / Delay / Yes'),
         ('transfer_screens_handle_confirm_transfer_action', 'transfer_screens_handle_confirm_transfer_action', '0x8f3c70', _F, 'cdecl',
          'responde à confirmação final: Voltar desfaz a oferta no estado 2; Não, Adiar e Sim primeiro conferem se a oferta mudou (série, recusada, retirada: tela de oferta mudada, -11); Não retira a proposta do próprio clube ou recusa (ou abre a aceitação obrigatória, -4); Adiar marca a notícia (0x12 ou 0x11) e o clube recusa (ou a aceitação obrigatória); Sim confirma pelo clube; todos apagam a cópia de confirmação (-9)',
          "answers the final confirmation: Back collapses the offer in state 2; No, Delay and Yes first check whether the offer changed (serial, rejected, withdrawn: the offer changed screen, -11); No withdraws the club's own bid or rejects (or opens the must accept dialog, -4); Delay marks the news item (0x12 or 0x11) and the club rejects (or the must accept dialog); Yes confirms by the club; all delete the confirm copy (-9)"),
         ('transfer_screens_open_shortlist_dialog', 'transfer_screens_open_shortlist_dialog', '0x8f4160', _F, 'cdecl',
          'abre a janela de pôr ou tirar o jogador da lista de observação (parâmetros: o jogador e se é pôr)',
          'opens the dialog that adds the player to or removes him from the shortlist (params: the player and whether to add)'),
         ('transfer_screens_open_comparison_prompt', 'transfer_screens_open_comparison_prompt', '0x8f41b0', _F, 'cdecl',
          'abre a janela de pôr o jogador como o 1º ou o 2º da comparação (parâmetros: o jogador e se é o 1º)',
          'opens the dialog that sets the player as the 1st or 2nd in the comparison (params: the player and whether it is the 1st)'),
         ('transfer_screens_build_comparison_prompt', 'transfer_screens_build_comparison_prompt', '0x8f4210', _F, 'cdecl',
          'monta a janela da comparação: o título com o nome do gerente nas cores do clube dele (ou da seleção), "Pôr <jogador> como 1º/2º jogador da comparação?" com o link do jogador e os botões Não e Sim (2 e 1 para o 1º, 0xb e 0xa para o 2º)',
          "builds the comparison prompt: the title with the manager's name in his club's (or national team's) colours, \"Set <player> as 1st/2nd player in comparison?\" with the player's link and the No and Yes buttons (2 and 1 for the 1st, 0xb and 0xa for the 2nd)"),
         ('transfer_screens_handle_comparison_prompt_action', 'transfer_screens_handle_comparison_prompt_action', '0x8f44d0', _F, 'cdecl',
          'responde à janela da comparação: Não -9; Sim põe o jogador como o 1º ou o 2º da comparação do olheiro (-9); o resto vai para a barra de menus',
          "answers the comparison prompt: No -9; Yes sets the player as the scout manager's 1st or 2nd compared player (-9); the rest goes to the menu bar"),
         ('transfer_screens_build_shortlist_dialog', 'transfer_screens_build_shortlist_dialog', '0x8f4650', _F, 'cdecl',
          'monta a janela da lista de observação: "Pôr <jogador> na lista de observação?" ou "Tirar <jogador> da lista?", com o link do jogador e os botões Não (2) e Sim (1)',
          'builds the shortlist dialog: "Add <player> to shortlist?" or "Remove <player> from shortlist?", with the player\'s link and the No (2) and Yes (1) buttons'),
         ('transfer_screens_handle_shortlist_dialog', 'transfer_screens_handle_shortlist_dialog', '0x8f48b0', _F, 'cdecl',
          'responde à janela da lista de observação: Não -9; Sim tira o jogador ou põe e, por sorteio (1 em 37 menos a reputação/300; reputação acima de 5000 mais um sorteio de 2000, ou jogador de clube humano; interesse dele de pelo menos 2,75), a imprensa noticia o interesse quando o orçamento ou o saldo do clube cobre 75% do preço pedido; -9',
          "answers the shortlist dialog: No -9; Yes removes the player or adds him and, by a draw (1 in 37 minus reputation/300; reputation above 5000 plus a draw of 2000, or a human club's player; his interest at least 2.75), the press reports the interest when the club's budget or balance covers 75% of the asking price; -9"),
         ('transfer_screens_open_withdraw_bid_dialog', 'transfer_screens_open_withdraw_bid_dialog', '0x8f4c70', _F, 'cdecl',
          'abre a janela de retirar a proposta (parâmetro: a oferta)',
          'opens the withdraw bid dialog (param: the offer)'),
         ('transfer_screens_build_withdraw_bid_dialog', 'transfer_screens_build_withdraw_bid_dialog', '0x8f4cb0', _F, 'cdecl',
          'monta a janela de retirar a proposta: o título nas cores do clube comprador, "Retirar a proposta por <jogador>?", a pergunta com o link do jogador e os botões Não (1) e Sim (2)',
          "builds the withdraw bid dialog: the title in the buyer's colours, \"Withdraw <player> transfer bid?\", the question with the player's link and the No (1) and Yes (2) buttons"),
         ('transfer_screens_handle_withdraw_bid_dialog', 'transfer_screens_handle_withdraw_bid_dialog', '0x8f4ed0', _F, 'cdecl',
          'responde à janela de retirar: Não -10; Sim retira a proposta (-10); o resto vai para a barra de menus',
          'answers the withdraw dialog: No -10; Yes withdraws the bid (-10); the rest goes to the menu bar')])
