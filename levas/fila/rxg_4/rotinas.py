# A negotiated loan contract submitted (transfer_manager.cpp; reader rxg, round 6, 25/11/2026). Item of TELAS: called
# by the answer to a news item (0x784e8d) and by 0x8c684d, when the human closes a loan contract negotiation; 0 calls
# in the automatic career. Path in the game: bid to take a computer club's player on loan, negotiate the contract (the
# loan's wage share) and confirm it from the news item.
# - 0x8c0f40 TRANSFER_MANAGER::submit_loan_contract_offer thiscall(this, &offer) ret 4 (the argument is the address of
#   the offer pointer). Prologue push -1; push 0x963b96 (7 bytes; SEH frame for the local TRANSFER_OFFER, destroyed by
#   0x8e5710). Last ret at 0x8c127f. Same bytes in cm0102-1080.exe.
# - Calls conferred at the sites: LLIST::get_next 0x68d1b0 (cursor +0xa = head +2 first); CONTRACT_MANAGER::get_contract
#   0x4cf5a0 thiscall([0xae1050], person); FinanceTable::get_club_finance 0x5a0590 thiscall([0xae2a38], club);
#   Finance::get_max_wage 0x5968d0 thiscall(finance, person, -1) (pushed before the share is computed); Date::days_since
#   0x54cdb0 thiscall(date, today lo, today hi); Date::add_weeks 0x54c770 thiscall(0xae2c90, &out, 2); Date::operator=
#   0x417c10 thiscall(date, &scratch, src); TRANSFER_OFFER 0x8e5680 / init_loan_offer 0x8e7810(person, club);
#   STAFF_CONTRACT 0x85e410 / init_from_current_contract 0x85e590(person, club, &date, share, [+0x1c], [+0x1d]) (the
#   share pushed as the dword at esp+0x14 whose high bytes the routine does not set); add_transfer_offer 0x8be300(&offer,
#   &contract); CONTRACT_MANAGER::add_contract_offer 0x4cc490(&contract, offer +0x20, offer +0x24, 0xb, 0, 1, 0);
#   schedule_offer 0x8c6220(offer, 1, 0) on the failures (the "Error" box, lines 0xb11 and 0xb1a).
# The original's local offer and contract are not filled whole by their constructors (stack residue): the copy zeroes
# them, as make_loan_offer (0x8bed60) does. Uses TRANSFER_MANAGER_CPP from rxs1_11 (INTEGRAR_ANTES).
# Comparer: only the original (it adds the offer and the contract).
SAVE = False
TELAS = True
ROTINAS = [('transfer_manager_submit_loan_contract_offer', 0x8c0f40, [0x6a, 0xff, 0x68, 0x96, 0x3b, 0x96, 0x00])]
_F = 'transfer_manager.cpp'
DOCS = ('o contrato de empréstimo negociado entregue',
        'a negotiated loan contract submitted',
        [('transfer_manager_submit_loan_contract_offer', 'TRANSFER_MANAGER::submit_loan_contract_offer', '0x8c0f40', _F, 'thiscall, ret 4',
          'entrega o contrato de empréstimo negociado pelo humano: acha na lista das negociações de empréstimo o item da pessoa e do clube; a parte do salário que o clube paga = o valor combinado × 100 / o salário do contrato, cortada para dezenas e baixada de 10 em 10 enquanto passar do teto salarial do clube (abaixo de 0, 0); a data do item vira daqui a duas semanas quando já passou ou falta menos de uma; monta a oferta de empréstimo e o contrato e entrega os dois. Sem item ou sem contrato: a caixa de erro, a oferta com estado 0x10 e reagendada',
          "submits the loan contract the human negotiated: finds the person's and the club's item in the loan negotiations list; the share of the wage the club pays = the agreed amount × 100 / the contract's wage, cut down to tens and lowered by 10 while above the club's wage limit (below 0, 0); the item's date becomes two weeks from today when it has passed or is less than one week away; builds the loan offer and the contract and submits both. No item or no contract: the error box, the offer with status 0x10 and scheduled again")])
CAMPOS = []
INTERESSA = []
