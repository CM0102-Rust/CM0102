# The club screen's info tab (club_screens.cpp, part 2; reader tel02, the screens round, 21/11/2026): the View menu,
# General Info, Stats, Finances, Income and Expenditure.
# 0x460590 club_screen_draw_info_tab cdecl(top, bottom) ret 0, SEH frame (two Cash objects whose destructor 0x5a53c0
# is empty). The body runs from 0x460590 to the ret at 0x467190 (27 KB): the map's "fragments" 0x462070, 0x464220,
# 0x464620, 0x464b20, 0x464d20 (tel02's list) and 0x465420 (tel03's list) are pieces of it, not routines; hooked whole
# here, in named pieces (info_tab_general_view, info_tab_stats_view, info_tab_money_views, info_tab_income_rows,
# info_tab_expenditure_rows). The cm0102-1080.exe changes no byte of it.
# Comparer: side by side over the display list (compare_by_display_list_with), with the function-static green and its
# guard (0xadab32..33, 0xadab48).
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - param 0x1b without any view bit (human's club): no menu entry is selected and the title is the uninitialised stack
#   buffer (the copy: empty);
# - General Info, Finances: a wealth band other than 2/1/0/-1/-2 with the +0x165 flag set writes no value (the row
#   shows "  Finances");
# - Stats: the most valuable player's " - " sprintf gets an argument with no %; "No. Of Current ..." untranslated when
#   the database names language is off; the average age is "-" at 0.0 and also NaN;
# - Finances, Income, Expenditure: the players sold/bought (i64) are read as their low dword (the totals add in 32
#   bits); the balance shows only its low dword; the negative "(%s)" text is formatted with size 0x7d0 into a 0x11c-byte
#   buffer.
TELAS = True
ROTINAS = [('club_screen_draw_info_tab', 0x460590, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tela do clube (2): a aba de informações (informações gerais, estatísticas, finanças, receitas e despesas)',
        "the club screen (2): the info tab (general info, stats, finances, income and expenditure)",
        [('club_screen_draw_info_tab', 'club_screen_draw_info_tab', '0x460590', 'club_screens.cpp', 'cdecl',
          'a aba de informações da tela do clube: o menu View (Informações gerais, Estatísticas e, para o clube do humano que não é seleção, Finanças, Receitas e Despesas; a vista marcada vira o título) e a vista escolhida pelo parâmetro 0x1b (sem bit 1 nem 0x10, seleção ou clube alheio caem em Informações gerais). Informações gerais: continente (seleção) ou nação (Swansea, Cardiff e Wrexham leem "Wales"; a nação é link para a seleção), situação profissional, finanças pela faixa de riqueza, estádio (alugado, ou do primeiro outro clube dono dele, com a cidade), instalações (capacidade, lugares sentados, cobertura, aquecimento), centro de treino (reputação/treino até 400, 600, 800, 1000) e a comissão não jogadora ordenada. Estatísticas: jogadores e lesionados, idade média, folha salarial, salário médio, maior e menor (por semana, mês = x4 ou ano = x52, conforme a configuração), o mais velho e o mais novo; na seleção, convocações e acaba; no clube, o mais valioso, internacionais, sub-21, estrangeiros e, na CEE, os de fora da UE. Dinheiro: a tabela de cinco colunas (este mês, mês passado, este ano, ano passado); Finanças com receita, despesa, lucro/(prejuízo), saldo e verba de transferências; Receitas e Despesas linha a linha com o total',
          "the club screen's info tab: the View menu (General Info, Stats and, for a club the human controls that is not a national team, Finances, Income and Expenditure; the selected view becomes the title) and the view chosen by param 0x1b (without bit 1 or 0x10, a national team or someone else's club falls back to General Info). General Info: continent (national team) or nation (Swansea, Cardiff and Wrexham read \"Wales\"; the nation links to its national team), professional status, finances by wealth band, stadium (rented, or owned by the first other club that owns it, with the city), facilities (capacity, seated capacity, roof, heating), training ground (reputation/training up to 400, 600, 800, 1000) and the sorted non-playing staff. Stats: players and injured, average ages, wage bills, average, highest and lowest wage (per week, month = x4 or year = x52, by the Config), oldest and youngest; for a national team the caps and it ends; for a club the highest valued, internationals, under-21s, foreign players and, in the EEC, the non-EU ones. Money: the five-column table (this month, last month, this year, last year); Finances with income, expenditure, profit/(loss), balance and transfer funds; Income and Expenditure row by row with the total")])
