# The contract screens (2): the offer contract screen (its build with the clauses tab inline, its answers) and the
# bonuses tab (contract_screens.cpp; reader tel24, the screens round, 21/11/2026). Equal in cm0102-1080.exe.
# Conventions checked in the disassembly and at the call sites:
# - 0x4deac0 build_offer_contract_screen cdecl() ret 0, SEH frame (3 MenuItems of 0xd4 bytes built by the vector
#   constructor 0x9463af with MenuItem::MenuItem 0x4568f0, which writes the word 0xffff at +0: text at +4, font +0xcc,
#   id +0xd0; two Texts). Calls the tab builders cdecl(terms, edited, staff, top, bottom).
# - 0x4e01e0 handle_offer_contract_screen cdecl(item) -> int.
# - 0x4e30f0 build_bonuses_tab cdecl(terms, edited terms, staff, top, bottom) ret 0, SEH frame (Text, Cash).
# - By address: will_negotiate_offer 0x4d4660 and prefill_offer_from_pending 0x4d4830 thiscall on the contract manager
#   ret 4; is_clause_allowed 0x867ad0 cdecl(staff, club, clause); get_release_clause_name 0x867a00 cdecl(clause,
#   buffer); is_bonus_type_allowed 0x867aa0 cdecl(staff, kind); get_bonus_type_name 0x8679a0 cdecl(kind, buffer);
#   Cash::format_display_amount 0x450aa0 thiscall(cash, buffer, size, mode) ret 0xc; Cash::set_from_amount_table
#   0x43f720 thiscall(cash, index, table, count) ret 0xc; ScreenManager::add_menu_item 0x415640 thiscall ret 0x10;
#   date_get_month 0x54cfb0 thiscall, no argument (the 0 pushed before it is set_screen_param's third argument).
# Comparers: the build side by side over the display list, the human's terms and the edited terms (0x50 bytes each),
# G_CONTRACT_LOOKUP_RESULT and the person's club squad (the popup sorts it in place); only the original on the first
# build (no edited terms: it allocates and fills them) and without the person (assert box). The bonuses tab side by
# side over the display list and both terms (it caps the human's bonuses). The answers: the tab (0xc) and squad player
# (3) side by side over the screen params; the rest only in the original (screens, the terms, the offer).
# Depends on tel24_1 (CS_CPP, cs_human_club, cs_contract_type, CS_FN_HUMAN_CONTRACT_OFFER, CS_FN_OPEN_OFFER_CHANGED),
# tel26_1, tel29_1 (c_string_len, STR_EMPTY) and tel17_2.
# Defects of the original: the squad player answer drops the edited terms (param 2 = 0) without freeing them; Cancel on
# a changed transfer offer deletes it and then opens the offer changed screen with the deleted offer.
TELAS = True
ROTINAS = [('contract_screens_build_offer_contract_screen', 0x4deac0, [0x6a, 0xff, 0x68, 0x84, 0x71, 0x95, 0x00]),
           ('contract_screens_handle_offer_contract_screen', 0x4e01e0, [0x83, 0xec, 0x24, 0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('contract_screens_build_bonuses_tab', 0x4e30f0, [0x6a, 0xff, 0x68, 0x78, 0x72, 0x95, 0x00])]
_F = 'contract_screens.cpp'
DOCS = ('as telas de contrato (2): a tela da oferta de contrato (a montagem com a aba das cláusulas, as respostas) e a aba dos bônus',
        'the contract screens (2): the offer contract screen (its build with the clauses tab, its answers) and the bonuses tab',
        [('contract_screens_build_offer_contract_screen', 'contract_screens_build_offer_contract_screen', '0x4deac0', _F, 'cdecl',
          'monta a tela da oferta: na primeira vez cria os termos editados (o contrato desejado pela pessoa) e preenche os termos do humano (da oferta atual do clube ou dos termos novos, o valor cortado ao orçamento) e as datas; o título com o nome e o clube, o elenco do clube num menu quando ele joga no clube do humano (o elenco do clube ordenado no próprio registro), o botão das regras da FIFA, as abas Básico, Bônus e Cláusulas; na aba das cláusulas, as cláusulas de rescisão permitidas, a multa (com o menu de 0 a 15 milhões, abaixo do mínimo desativado) e a cláusula 4; e os botões Voltar/Cancelar/Oferecer',
          "builds the offer screen: the first time it makes the edited terms (the person's desired contract) and fills the human's terms (from the club's current offer or the new terms, the amount cut to the budget) and the dates; the title with the name and club, the club's squad in a menu when he plays for the human's club (the squad sorted in the club record itself), the FIFA rules button, the Basic, Bonuses and Clauses tabs; on the clauses tab, the allowed release clauses, the release fee (with the menu from 0 to 15 million, below the minimum disabled) and clause 4; and the Back/Cancel/Offer buttons"),
         ('contract_screens_handle_offer_contract_screen', 'contract_screens_handle_offer_contract_screen', '0x4e01e0', _F, 'cdecl',
          'responde à tela da oferta: a aba, outro jogador do elenco (descarta os termos editados sem liberar), as regras, Cancelar (proposta mudada: apaga a proposta e abre a tela de proposta mudada; senão a janela de jogador de troca, a confirmação da proposta ou a de retirar), Voltar (apaga os termos do humano), Oferecer (o fim do contrato no dia do início, até 28, do mês e ano escolhidos, e abre a janela da oferta) e as respostas das abas',
          'answers the offer screen: the tab, another squad player (drops the edited terms without freeing them), the rules, Cancel (changed offer: deletes the offer and opens the offer changed screen; else the exchange player dialog, the bid confirm or the withdraw dialog), Back (clears the human\'s terms), Offer (the contract ends on the start day, at most the 28th, of the chosen month and year, and opens the offer dialog) and the tab answers'),
         ('contract_screens_build_bonuses_tab', 'contract_screens_build_bonuses_tab', '0x4e30f0', _F, 'cdecl',
          'monta a aba dos bônus: para cada bônus permitido (gol, assistência, jogo sem sofrer gol) corta o bônus do humano a um décimo do salário máximo do clube, mostra o pedido da pessoa e os botões -, valor (com menu de cinco passos de um quinto do máximo), + e o último',
          "builds the bonuses tab: for each allowed bonus (goal, assist, clean sheet) cuts the human's bonus to a tenth of the club's maximum wage, shows the person's request and the -, amount (with a menu of five steps of a fifth of the maximum), + and last buttons")])
