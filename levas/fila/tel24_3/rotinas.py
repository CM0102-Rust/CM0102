# The contract screens (3): the answers of the bonuses and clauses tabs of the offer contract screen
# (contract_screens.cpp; reader tel24, the screens round, 21/11/2026). Equal in cm0102-1080.exe.
# Conventions checked in the disassembly: 0x4e39c0 handle_bonuses_action and 0x4e4160 handle_clauses_action
# cdecl(short item) -> int (0 when the item is not theirs), SEH frame (a Cash local). STAFF_CONTRACT::set_bonus 0x85f7d0
# thiscall(terms, kind, amount); remove_bonus 0x85f7f0, add_release_clause 0x85f7a0, remove_release_clause 0x85f7e0
# thiscall(terms, x); Cash::step_down 0x43f580 / step_up 0x43f4e0 thiscall(cash, mode) ret 4; the widget setters
# 0x800580, 0x801350, 0x801fc0, 0x8022d0, 0x801c70 and redraw_screen 0x802d00 thiscall on the screen manager.
# Comparers: the answers that only change the human's terms (popup steps, add/remove a bonus or clause, back to the
# request) side by side over the terms (0x50 bytes) and the answer; the "-"/"+" steps (they change widgets and
# redraw) and the missing terms (fatal box) only in the original.
# Depends on tel24_1 and tel24_2 (cs_human_terms, cs_money_text, CS_TERMS_*, CS_FN_CASH_STEP_DOWN).
# Not in this item: build_basic_terms_tab 0x4e06e0 and handle_basic_terms_action 0x4e2280 (see PRONTO.txt).
# Defects of the original: the bonus popup answer accepts the id 0x2d, which writes the clause bytes (no such item is
# built).
TELAS = True
ROTINAS = [('contract_screens_handle_bonuses_action', 0x4e39c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('contract_screens_handle_clauses_action', 0x4e4160, [0x6a, 0xff, 0x68, 0xc6, 0x72, 0x95, 0x00])]
_F = 'contract_screens.cpp'
DOCS = ('as telas de contrato (3): as respostas das abas dos bônus e das cláusulas',
        'the contract screens (3): the answers of the bonuses and clauses tabs',
        [('contract_screens_handle_bonuses_action', 'contract_screens_handle_bonuses_action', '0x4e39c0', _F, 'cdecl',
          'responde à aba dos bônus: o passo escolhido no menu vira o bônus; - e + mudam o bônus um passo (o - desativa abaixo de 6, o + no teto de um décimo do salário máximo) e redesenham; o último botão volta ao pedido da pessoa (ou um décimo do salário), cortado ao teto; acrescentar ou tirar um bônus; sem os termos do humano, a caixa fatal (linha 0xecb)',
          "answers the bonuses tab: the popup step becomes the bonus; - and + move the bonus one step (- disabled below 6, + at the cap of a tenth of the maximum wage) and redraw; the last button goes back to the person's request (or a tenth of the wage), cut to the cap; adding or removing a bonus; without the human's terms, the fatal box (line 0xecb)"),
         ('contract_screens_handle_clauses_action', 'contract_screens_handle_clauses_action', '0x4e4160', _F, 'cdecl',
          'responde à aba das cláusulas: o passo do menu vira a multa; acrescentar uma cláusula (a de multa mínima sobe a multa ao mínimo) ou tirar; - e + mudam a multa um passo (o - para no mínimo, o + em 50 milhões) e redesenham; o último botão volta à multa dos termos editados; sem os termos do humano, a caixa fatal (linha 0x10a1)',
          'answers the clauses tab: the popup step becomes the release fee; adding a clause (the minimum fee clause raises the fee to the minimum) or removing one; - and + move the fee one step (- stops at the minimum, + at 50 million) and redraw; the last button goes back to the edited terms\' fee; without the human\'s terms, the fatal box (line 0x10a1)')])
