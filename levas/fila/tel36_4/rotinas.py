# Opening the bid screen for a player (transfer_screens.cpp; reader tel36, the screens round, 21/11/2026). Only the
# opener: the builder 0x8e9760 and the handler 0x8ec420 are other items.
# Conventions checked in the disassembly: cdecl(Staff*, offer slot*) (open_screen thiscall(SM, 0x8e9760, 0x8ec420, 1,
# 0x93a710, 0) ret 0x14; set_screen_param thiscall(SM, k, v, 0) ret 0xc; CONTRACT_MANAGER::get_contract 0x4cf5a0
# thiscall(cm, person) ret 4; TRANSFER_MANAGER::can_sign_player 0x8ced40 thiscall(tm, club, &today, person, seller,
# 1, 0) ret 0x18).
# Equal in cm0102-1080.exe.
# Comparer: only the original (it opens a screen).
TELAS = True
ROTINAS = [('transfer_screens_open_bid_screen_for_player', 0x8e9350, [0x55, 0x8b, 0x6c, 0x24, 0x0c])]
DOCS = ('abrir a tela de proposta por um jogador',
        'opening the bid screen for a player',
        [('transfer_screens_open_bid_screen_for_player', 'transfer_screens_open_bid_screen_for_player', '0x8e9350', 'transfer_screens.cpp', 'cdecl',
          'abre a tela de proposta: o jogador, a casa da proposta e o número de série dela (ou -1); o contrato do jogador dá o clube vendedor (fora o tipo 5); o parâmetro 9 diz se o clube do humano pode contratá-lo agora, o 7 se a proposta já tem situação 5 ou mais (fora 0x10); 0xf a 0x16 = -1',
          "opens the bid screen: the player, the offer slot and its serial number (or -1); the player's contract gives the selling club (except kind 5); param 9 says whether the human's club may sign him now, 7 whether the offer already has status 5 or more (other than 0x10); 0xf to 0x16 = -1")])
