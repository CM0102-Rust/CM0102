# Apagar as queixas e as marcas de evento de um contrato (staff_contracts.cpp; leitor rx13, rodada 6, 25/11/2026).
# Simulação: a carreira ENG até 2002-06-01 as roda (contract_manager, manager_manager, media, o depois do jogo: ~80 e
# ~25 pontos de chamada).
# - 0x8674e0 STAFF_CONTRACT::clear_grievances thiscall(bits) ret 4: +0x45 &= ~bits; check_transfer_request_withdrawal
#   0x867240 thiscall(contrato, queixas antigas) ret 4; a pessoa [0xae23bc] + [+0] x 0x6e; staff_remove_opinion_link
#   0x549d60 cdecl(pessoa, bits) (add esp, 8). Prólogo push esi; push edi; mov edi, [esp+0xc] (6 bytes).
# - 0x8676d0 STAFF_CONTRACT::clear_flags thiscall(bits) ret 4: +0x40 &= ~bits. Prólogo mov eax, [esp+4];
#   mov edx, [ecx+0x40] (7 bytes).
# As duas estão nas portas do depois do jogo (POS_PROLOGUES, match_day.rs): no modo de comparação a porta é instalada
# antes e fica com o endereço, como a 0x733a60 e a 0x867700; o desvio do motor só vale no modo Rust. Prova pela
# carreira Rust e pelas mutações.
# Iguais no cm0102-1080.exe. Comparadores: clear_grievances só a original; clear_flags lado a lado no dword +0x40.
SAVE = False
ROTINAS = [('staff_contract_clear_grievances', 0x8674e0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('staff_contract_clear_flags', 0x8676d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x40])]
DOCS = ('apagar as queixas e as marcas de evento de um contrato',
        "clearing a contract's grievances and event flags",
        [('staff_contract_clear_grievances', 'STAFF_CONTRACT::clear_grievances', '0x8674e0', 'staff_contracts.cpp', 'thiscall, ret 4',
          'regra: as queixas dadas da pessoa são apagadas; depois o pedido de transferência é retirado se nenhuma queixa ainda o justifica, e os vínculos de opinião ligados a essas queixas são removidos',
          "rule: a person's grievances (the given bits) are cleared; then the transfer request is withdrawn if no grievance still justifies it, and the opinion links tied to those grievances are removed"),
         ('staff_contract_clear_flags', 'STAFF_CONTRACT::clear_flags', '0x8676d0', 'staff_contracts.cpp', 'thiscall, ret 4',
          'regra: as marcas de evento dadas do contrato são apagadas',
          "rule: the given event flag bits of a contract are cleared")])
CAMPOS = []
INTERESSA = []
