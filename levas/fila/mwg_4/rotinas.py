# As linhas dos técnicos humanos no fim da tabela de staff (Setup.cpp; leitor mwg, rodada 2 do mundo inteiro,
# 12/11/2026): a primeira linha livre, quantas estão em uso e se o técnico da vaga de tela atual está sendo criado.
# Chamadas pelo laço do jogo e pelas telas (cobertura_mundo.txt até 2002-08-01: 299, 1.802 e 2.494 chamadas); rodam em
# qualquer carreira (a de férias também tem o humano), daí LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x838c60..0x838d08, 0x838d10..0x838dda, 0x838de0..0x838e05): as três cdecl;
# find_free_human_manager_row() devolve a linha (ponteiro) ou 0; count_human_managers(só_rede) devolve a contagem;
# is_current_manager_being_created() devolve 0/1. 0x7ffe40 ScreenManager::is_manager_done: thiscall(ecx = 0xb74450,
# o objeto, não um ponteiro; push linha), ret 4. Sem a tabela de staff, a caixa de erro (linhas 0x39b3 e 0x39d8 de
# Setup.cpp, 0xa801dc) e 0, como show_error_at.
# Comparadores: pelo valor (as três são leituras).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('find_free_human_manager_row', 0x838c60, [0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00]),
           ('count_human_managers', 0x838d10, [0xa1, 0xbc, 0x23, 0xae, 0x00]),
           ('is_current_manager_being_created', 0x838de0, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as linhas dos técnicos humanos na tabela de staff',
        'the human manager rows of the staff table',
        [('find_free_human_manager_row', 'find_free_human_manager_row', '0x838c60', 'Setup.cpp', 'cdecl',
          'a primeira das 16 linhas de técnico humano (as 16 últimas da tabela de staff, da última para trás) livre (classificação 0), ou 0; sem a tabela de staff, a caixa de erro (linha 0x39b3) e 0',
          'the first of the 16 human manager rows (the last 16 of the staff table, from the last one back) that is free (classification 0), or 0; without the staff table, the error box (line 0x39b3) and 0'),
         ('count_human_managers', 'count_human_managers', '0x838d10', 'Setup.cpp', 'cdecl',
          'quantas das 16 linhas de técnico humano estão em uso (classificação 1); com só_rede, só as cujo técnico está pronto na tela (ScreenManager::is_manager_done); sem a tabela de staff, a caixa de erro (linha 0x39d8) e 0',
          'how many of the 16 human manager rows are in use (classification 1); with network_only, only those whose manager is done on the screen manager (ScreenManager::is_manager_done); without the staff table, the error box (line 0x39d8) and 0'),
         ('is_current_manager_being_created', 'is_current_manager_being_created', '0x838de0', 'Setup.cpp', 'cdecl',
          '1 quando a vaga de tela atual tem um técnico cuja linha ainda está sendo criada (classificação 5), senão 0',
          '1 when the current screen slot has a manager whose row is still being made (classification 5), else 0')])
