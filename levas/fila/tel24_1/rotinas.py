# The contract screens (1): offering a contract to a person, the set role screen, the openers of the offer contract
# screen, the disabled popup item, leaving the offer screen and the staff offer dialog (contract_screens.cpp; reader
# tel24, the screens round, 21/11/2026). Equal in cm0102-1080.exe (no byte of 0x4dd0f0..0x4e4e94 differs).
# Conventions checked in the disassembly and at the call sites:
# - 0x4dd0f0 offer_contract_to_staff cdecl(staff, transfer slot, force, ask) ret 0 (also called from 0x4dc9b0 and
#   0x4e5770, outside the screens).
# - 0x4dd6c0 build_set_role_screen cdecl() ret 0; 0x4ddf20 handle_set_role_screen cdecl(item) -> int.
# - 0x4de1f0 open_offer_contract_screen cdecl(staff, char role, char job, short mode) ret 0, SEH frame (Cash).
# - 0x4de5d0 open_offer_contract_screen_for_transfer cdecl(slot, staff, char role, char job, short mode) ret 0, SEH
#   frame; the role dword goes whole to Finance::get_max_wage.
# - 0x4e0180 add_disabled_menu_item thiscall(screen manager, area, text) ret 8 (18 dwords to Display::add_item).
# - 0x4e06a0 close_offer_contract_screen cdecl() ret 0.
# - 0x4e4730 open_staff_offer_contract_screen cdecl(slot, char, char, withdraw, window, char window state) ret 0.
# - 0x4e4820 build_staff_offer_contract_screen cdecl() ret 0, SEH frame (Text, Date); gui_add_text_area is variadic
#   (14 or 17 arguments).
# - Called by address (all hooked or exe): staff_can_take_job 0x4cb990 stdcall(job, staff) ret 8 (ecx = the contract
#   manager, not read); shortlist_manager_squad_status_allowed 0x84b4d0 stdcall(staff, status) ret 8 (ecx = the
#   shortlist manager); get_contract_satisfaction 0x866620 thiscall(contract, staff); get_offer_job 0x4d44e0,
#   find_talks_record 0x4ca2a0, add_talks_record 0x4c9f60 (ret 0x10), lowest_wage_demand 0x4ce450 thiscall on the
#   contract manager; squad_status_text 0x84b330, max_fee_for_player_status 0x84b1f0 thiscall on the shortlist
#   manager ([transfer manager + 0x856]); Cash::step_down 0x43f580 thiscall ret 4 (stores and returns the amount);
#   HUMAN_MANAGER::get_contract_offer 0x5fd930 thiscall ret 4.
# Comparers: the set role screen and the staff offer dialog side by side over the display list (+
# G_CONTRACT_LOOKUP_RESULT for the dialog); the set role answers that only set a param (role 3, job 0x2f, Back 1) side
# by side; add_disabled_menu_item side by side like the screen manager forwarders (tel01_1); only the original for
# offer_contract_to_staff and the openers (they open screens), Set (it records the talks and opens the offer screen),
# leaving the offer screen (it frees the edited offer) and the dialog without an edited offer (the assert box).
# Depends on tel26_1 (tel26_*), tel01_1 (screens_current_display, compare_screens_forwarder), tel29_1 (STR_EMPTY) and
# tel17_2 (ts_*). The other tel24 items use CS_CPP, cs_append_text and CS_FN_HUMAN_CONTRACT_OFFER of this one.
# Defects of the original: the staff offer dialog links the person's club name (0x7d5) to the human's own club; a
# negative compensation fee would add the money text without formatting it (an uninitialised buffer; the copy starts
# it empty) [inference: the fee is not seen negative].
TELAS = True
ROTINAS = [('contract_screens_offer_contract_to_staff', 0x4dd0f0, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('contract_screens_build_set_role_screen', 0x4dd6c0, [0x81, 0xec, 0x04, 0x03, 0x00, 0x00]),
           ('contract_screens_handle_set_role_screen', 0x4ddf20, [0x83, 0xec, 0x08, 0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('contract_screens_open_offer_contract_screen', 0x4de1f0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('contract_screens_open_offer_contract_screen_for_transfer', 0x4de5d0, [0x6a, 0xff, 0x68, 0xf8, 0x70, 0x95, 0x00]),
           ('contract_screens_add_disabled_menu_item', 0x4e0180, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('contract_screens_close_offer_contract_screen', 0x4e06a0, [0x56, 0x6a, 0x02, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('contract_screens_open_staff_offer_contract_screen', 0x4e4730, [0x56, 0x6a, 0x00, 0x6a, 0x00]),
           ('contract_screens_build_staff_offer_contract_screen', 0x4e4820, [0x6a, 0xff, 0x68, 0xf1, 0x72, 0x95, 0x00])]
_F = 'contract_screens.cpp'
DOCS = ('as telas de contrato (1): oferecer contrato a uma pessoa, a tela do papel no clube, os abridores da tela da oferta, o item desativado, sair da tela da oferta e a janela da oferta ao técnico',
        'the contract screens (1): offering a contract to a person, the set role screen, the openers of the offer screen, the disabled item, leaving the offer screen and the staff offer dialog',
        [('contract_screens_offer_contract_to_staff', 'contract_screens_offer_contract_to_staff', '0x4dd0f0', _F, 'cdecl',
          'escolhe a tela de oferecer contrato: direto para a oferta quando o clube já sabe o papel (conversa de hoje com papel, um só cargo possível, ou um contrato com situação 1 a 6 que não seja dos tipos 5 e 6), senão a tela do papel no clube com o papel pré-escolhido: o do contrato quando ele está insatisfeito e é permitido, ou o papel 1 a 6 de maior interesse (acima de 3,15 e até o limiar (lealdade + ambição + 42 + profissionalismo) × 0,1 + 3,5), com as exigências de reputação dos papéis 1 a 3; sem o gerente da vaga ou sem clube, a caixa de erro (linhas 0x193 e 0x19b)',
          'picks the offer contract screen: straight to the offer when the club already knows the role (talks today with a role, a single job open, or a contract with status 1 to 6 that is not of types 5 and 6), otherwise the set role screen with the role preselected: the contract\'s when he is unhappy there and it is allowed, or the role 1 to 6 of highest interest (above 3.15 and at most the threshold (loyalty + ambition + 42 + professionalism) × 0.1 + 3.5), with the reputation requirements of roles 1 to 3; without the slot\'s manager or without a club, the error box (lines 0x193 and 0x19b)'),
         ('contract_screens_build_set_role_screen', 'contract_screens_build_set_role_screen', '0x4dd6c0', _F, 'cdecl',
          'monta a tela do papel no clube: o nome, os papéis 1 a 6 do jogador (botão quando permitido e dentro da taxa máxima do papel; "+" em cinza quando não; "*" o esperado pelo jogador; amarelo o escolhido), os cargos que a pessoa pode ocupar, as notas (oferecido em, esperado, recusado pela diretoria por taxa ou idade, jogador acima de 30 que não quer parar) e os botões Voltar e Definir',
          "builds the set role screen: the name, the player's roles 1 to 6 (a button when allowed and within the role's maximum fee; \"+\" in grey when not; \"*\" the one the player expects; yellow the chosen one), the jobs the person can take, the notes (offered on, expected, refused by the board for the fee or the age, a player over 30 who will not retire) and the Back and Set buttons"),
         ('contract_screens_handle_set_role_screen', 'contract_screens_handle_set_role_screen', '0x4ddf20', _F, 'cdecl',
          'responde à tela do papel: o papel e o cargo vão para os parâmetros 1 e 4; Voltar sai; Definir registra a conversa e abre a tela da oferta (ou a de proposta mudada, quando a proposta de transferência mudou, foi recusada ou retirada)',
          'answers the set role screen: the role and the job go to params 1 and 4; Back leaves; Set records the talks and opens the offer screen (or the offer changed screen, when the transfer offer changed, was rejected or withdrawn)'),
         ('contract_screens_open_offer_contract_screen', 'contract_screens_open_offer_contract_screen', '0x4de1f0', _F, 'cdecl',
          'abre a tela da oferta de contrato com os parâmetros: a pessoa, o papel, o cargo, o orçamento de transferências (0 a 5 milhões), o valor (70% do valor do jogador, arredondado para baixo), o teto salarial (o do clube para os cargos de jogador; para os outros o do cargo, ou o salário atual se ele já está no clube) e as queixas do contrato',
          "opens the offer contract screen with its params: the person, the role, the job, the transfer budget (0 to 5 million), the value (70% of the player's value, stepped down), the wage limit (the club's for the player jobs; for the others the job's, or his current wage when he is already at the club) and the contract's grievances"),
         ('contract_screens_open_offer_contract_screen_for_transfer', 'contract_screens_open_offer_contract_screen_for_transfer', '0x4de5d0', _F, 'cdecl',
          'abre a mesma tela para o jogador de uma proposta de transferência: os dados da proposta (avisado, índice, número de série, se é de outra pessoa), o orçamento menos a taxa da proposta, a menor exigência salarial e, no período protegido da FIFA ou com proposta, a marca 3 no parâmetro 0x1c',
          "opens the same screen for the player of a transfer offer: the offer's data (aware, index, serial, whether it is for someone else), the budget less the offer's fee, the lowest wage demand and, in the FIFA protected period or with an offer, the mark 3 in param 0x1c"),
         ('contract_screens_add_disabled_menu_item', 'contract_screens_add_disabled_menu_item', '0x4e0180', _F, 'thiscall',
          'acrescenta à lista suspensa um item desativado (só o texto, nas cores da lista, sem id)',
          'adds a disabled item to a popup list (only the text, in the popup colours, no id)'),
         ('contract_screens_close_offer_contract_screen', 'contract_screens_close_offer_contract_screen', '0x4e06a0', _F, 'cdecl',
          'ao sair da tela da oferta, libera a oferta editada guardada no parâmetro 2',
          'on leaving the offer screen, frees the edited offer kept in param 2'),
         ('contract_screens_open_staff_offer_contract_screen', 'contract_screens_open_staff_offer_contract_screen', '0x4e4730', _F, 'cdecl',
          'abre a janela Sim/Não que oferece (ou retira) a oferta de contrato, com a proposta de transferência, as marcas, o estado da janela de transferências e o índice e número de série da proposta (-1 sem ela)',
          'opens the Yes/No dialog that offers (or withdraws) the contract offer, with the transfer offer, the flags, the transfer window state and the offer\'s index and serial (-1 without one)'),
         ('contract_screens_build_staff_offer_contract_screen', 'contract_screens_build_staff_offer_contract_screen', '0x4e4820', _F, 'cdecl',
          'monta a janela: o nome da pessoa e o clube, "Oferecer contrato a ... ?" ou "Retirar a oferta ... ?", a indenização ao clube atual quando houver, quando começa a próxima janela internacional de transferências ou que ele sai no fim do contrato, e os botões Não e Sim; sem oferta editada, a caixa de erro (linha 0x11a0)',
          'builds the dialog: the person\'s name and club, "Offer contract to ... ?" or "Withdraw the contract offer ... ?", the compensation to his current club when due, when the next international transfer window starts or that he moves at the end of his contract, and the No and Yes buttons; without an edited offer, the error box (line 0x11a0)')])
