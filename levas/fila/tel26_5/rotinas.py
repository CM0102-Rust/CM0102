# tel26_5: the Manager Stats screen build (manager_screens.cpp), one big routine (~9 KB) in named pieces, one hook.
# Conventions checked at the call sites: manager_stats_screen_build cdecl, no arguments, ret 0 (SEH frame, 0x570 of
# locals); Display::add_item thiscall 18 args ret 0x48; Display::add_area thiscall 11 args ret 0x2c; the
# ScreenManager add_text_item (11) / add_button_item (12) / add_area_button (11) / add_area_text (9) thiscall;
# gui_add_page_buttons 0x5ede20 cdecl 8 args; gui_add_text_area cdecl 11 args; key_nation_sorted_order 0x66a0c0 cdecl;
# NATION_MENU_SPLIT 0x66a280 (ret 0xc) / part_count 0x66a350 (ret 0, al) / format_header 0x66a360 (ret 0xc) /
# in_second_part 0x66a430 (ret 4) thiscall; Date::operator= 0x417c10 (thiscall, hidden result, ret 8) and
# run_record_assign 0x448170 in the record copy amount to a plain 210-byte copy; Cash::get_display_amount thiscall
# ret 4 (i64), cash_format_money cdecl (flags 5 pushed before the amount call).
# The range 0x6ae2a0..0x6b05e0 is identical in cm0102.exe and cm0102-1080.exe.
# Defects of the original: (1) with exactly 100 managers there is one page but only the rows that fit are shown
# (one-page test "<= 100", row loop "< 100"); (2) a failed crt_realloc loses the old list and leaves the screen
# without parameters; (3) the name is formatted into a 200-byte buffer with size 2000; (4) the purchases and sales
# totals (64-bit) are shown by their low dword only.
MOTOR = True
TELAS = True
ROTINAS = [('manager_stats_screen_build', 0x6ae2a0, [0x6a, 0xff, 0x68, 0x4c, 0xd1, 0x95, 0x00])]
DOCS = ('a tela de estatísticas dos técnicos (manager_screens.cpp)',
        'the manager stats screen (manager_screens.cpp)',
        [('manager_stats_screen_build', 'manager_stats_screen_build', '0x6ae2a0', 'manager_screens.cpp', 'cdecl',
          'monta a tela Manager Stats: título, barra de menus, o botão da estatística com o menu das 16 (reputação, '
          'pontos, tempo no clube atual, empregos, maior e menor tempo num clube, copas, ligas, prêmios, campanha, '
          'comprados, vendidos, maior compra e venda, totais de compras e vendas; separador depois da 2, 6 e 10) e o '
          'filtro de nação ("All Nations" e as nações-chave selecionadas, em duas partes com mais de 16); sem lista '
          '(parâmetro 1), junta o registro de técnico de quem treina clube ou seleção (cargo 5 ou 12) e passa no '
          'filtro, de 50 em 50, ordena pela estatística e grava os parâmetros 1 e 2 (vazia: "There are no manager '
          'stats available."); até 100 técnicos uma página, acima disso páginas de 0x15d / altura da fonte 3 linhas '
          '(com o parâmetro 7, a página do primeiro humano, um dos 16 últimos); cada linha: posição, nome (amarelo '
          'para humano), clube e/ou seleção (ou o clube do maior/menor tempo) e o valor; rola até o humano (meia '
          'página acima) ou até o parâmetro 5 e grava a área no 6; botões voltar e avançar',
          'builds the Manager Stats screen: title, menu bar, the statistic button with its 16-entry menu (reputation, '
          'points, period at the current club, jobs, longest and shortest period at a club, cups, leagues, awards, '
          'match record, bought, sold, highest fee paid and received, total fees paid and received; a separator after '
          '2, 6 and 10) and the nation filter ("All Nations" and the selected key nations, in two parts above 16); '
          'without a list (parameter 1), gathers the manager record of everyone managing a club or national team (job '
          '5 or 12) who passes the filter, 50 at a time, sorts by the statistic and writes parameters 1 and 2 (empty: '
          '"There are no manager stats available."); up to 100 managers one page, above that pages of 0x15d / font 3 '
          'height rows (with parameter 7, the page of the first human, one of the last 16); each row: place, name '
          '(yellow for a human), club and/or national team (or the club of the longest/shortest job) and the value; '
          'scrolls to the human (half a page above) or to parameter 5 and writes the area to 6; back and forward '
          'buttons')])
