# As funcoes de ordem do qsort/bsearch do gerente de transferencias (transfer_manager.cpp; leitor sx02b, rodada 5,
# 23/11/2026). Nao aparecem na cobertura (o prologo delas nao e o das listas), mas rodam na carreira ENG:
# - 0x8e54d0: qsort e bsearch dos percentuais de revenda (add_next_sale_percentage, find_next_sale_percentage,
#   pay_next_sale_percentage, ja em Rust e chamando pelo endereco);
# - 0x8e5590: o qsort da tabela das janelas na setup_transfer_rules (1 vez, no create_game);
# - 0x8e55c0: qsort e bsearch das permissoes de trabalho (init_work_permits, update_work_permits, add_work_permit,
#   player_needs_work_permit...), milhares de vezes.
# Convencoes conferidas: cdecl(a, b), devolvem eax; prologos de 8 bytes (dois mov reg, [esp+n]), sem enderecos relativos.
# Comparadores: lado a lado (so leem); o valor devolvido tem de ser o mesmo.
SAVE = False
ROTINAS = [('compare_next_sale_entries', 0x8e54d0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_window_dates', 0x8e5590, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('compare_work_permit_entries', 0x8e55c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('as ordens dos percentuais de revenda, das datas das janelas e das permissões de trabalho',
        'the orderings of the sell-on percentages, the window dates and the work permits',
        [('compare_next_sale_entries', 'transfer_manager_compare_next_sale_entries', '0x8e54d0', 'transfer_manager.cpp', 'cdecl',
          'regra: dois percentuais de revenda pela pessoa (+9), depois pelo clube (+1), a diferença dos inteiros',
          'rule: two sell-on percentages by the person (+9), then by the club (+1), the difference of the ints'),
         ('compare_window_dates', 'transfer_manager_compare_window_dates', '0x8e5590', 'transfer_manager.cpp', 'cdecl',
          'regra: duas datas de janela de transferência (por ponteiro) pelo mês, depois pelo dia, a diferença dos bytes com sinal',
          'rule: two transfer window dates (through pointers) by month, then day, the difference of the signed bytes'),
         ('compare_work_permit_entries', 'transfer_manager_compare_work_permit_entries', '0x8e55c0', 'transfer_manager.cpp', 'cdecl',
          'regra: duas permissões de trabalho pela pessoa (+0), com a chave vazia (−1) por último',
          'rule: two work permits by the person (+0), the empty key (−1) last')])
CAMPOS = []
INTERESSA = []
