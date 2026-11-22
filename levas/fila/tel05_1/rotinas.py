# As telas do clube (5a): a tela dos numeros do elenco e do elenco europeu (os eventos e a saida) e abrir a tela da
# troca de jogadores numa proposta. Leitor tel05 (rodada das telas), 21/11/2026.
# A lista do grupo trazia 3 pedacos de tamanho fixo (0x4771a5, 0x47a6c6, 0x47c0b0); as rotinas que comecam neles:
# 0x47a9e0..0x47c038 (+ a tabela de saltos ate 0x47c068), 0x47c070, 0x47c0b0 e 0x47c180 (esta no tel05_2).
# A 0x475520 (squad_numbers_screen_build, ~21 KB, de 0x475520 a 0x47a9d0) comeca no trecho do tel04: nao portada aqui.
# Convencoes conferidas na desmontagem: as tres cdecl, ret 0; o tratador devolve o codigo da tela (-0xb redesenha, -9,
# -4, 0); News::find_item_by_id thiscall ret 0xc; News::set_item_slot thiscall ret 0x14; confirm_push cdecl (7 args);
# transfer_screens_open_transfer_status_screen cdecl (4 args); SQUAD_MANAGER::set_squad_number le so o byte do numero.
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Usa get_screen_param/set_screen_param, item_id_or_default, current_human_display e show_error_at (HEAD) e, do
# tel26_1, tel26_item_value, display_item_at e display_item::DROP_TARGET: entra depois do tel26_1.
# Defeitos do original: o NewsItem local nunca e destruido; ao arrastar um numero para um jogador a varredura pergunta
# o numero de uma pessoa nula (vaga do clube com a copia vazia); as cores do clube das caixas sao locais nao
# iniciados (os tons vem do database_get_club_kit_colours).
# Comparadores: o tratador so roda a original (grava numeros e inscricoes, abre caixas e telas); a saida so a original
# e confere o param 0x11 vazio; a abertura so a original e confere os params 1..9 contra PART_EXCHANGE_OPEN_PARAMS.
TELAS = True
ROTINAS = [('squad_numbers_screen_handle_event', 0x47a9e0, [0x6a, 0xff, 0x68, 0x6b, 0x5e, 0x95, 0x00]),
           ('squad_numbers_screen_leave', 0x47c070, [0x6a, 0x11, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('open_add_part_exchange_player_screen', 0x47c0b0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01])]
DOCS = ('as telas do clube (5a): os eventos e a saída da tela dos números do elenco e do elenco europeu, e abrir a tela da troca de jogadores',
        'the club screens (5a): the squad numbers and European squad screen\'s events and leaving it, and opening the part exchange screen',
        [('squad_numbers_screen_handle_event', 'squad_numbers_screen_handle_event', '0x47a9e0', 'club_screens.cpp', 'cdecl',
          'responde aos botões da tela dos números do elenco (e do elenco europeu): visões, ordens e colunas gravadas nos parâmetros; números dados, trocados (arrastar um número para outro ou para um jogador), liberados, o primeiro livre ou os automáticos; inscrições europeias alternadas, refeitas ou desfeitas; a submissão pede confirmação ou diz o que falta (todos os escalados com número; ao menos o número pedido de inscritos, no máximo 20); as respostas das caixas confirmam os números (o provisório vira definitivo) ou o elenco europeu e marcam a notícia',
          'answers the squad numbers (and European squad) screen\'s buttons: views, sorts and columns written to the parameters; numbers given, swapped (dragging a number onto another or onto a player), freed, the first free one or the automatic ones; European registrations toggled, redone or undone; the submission asks for confirmation or says what is missing (every selected player numbered; at least the asked number registered, at most 20); the boxes\' answers confirm the numbers (the provisional becomes final) or the European squad and mark the news item'),
         ('squad_numbers_screen_leave', 'squad_numbers_screen_leave', '0x47c070', 'club_screens.cpp', 'cdecl',
          'ao sair da tela dos números do elenco, libera a cópia do elenco (parâmetro 0x11) e limpa o parâmetro',
          'leaving the squad numbers screen frees its squad copy (parameter 0x11) and clears the parameter'),
         ('open_add_part_exchange_player_screen', 'open_add_part_exchange_player_screen', '0x47c0b0', 'club_screens.cpp', 'cdecl',
          'abre a tela de acrescentar um jogador em troca numa proposta, com a proposta e a visão, a ordem e o filtro padrão',
          'opens the screen to add a player in exchange to a bid, with the bid and the default view, sort and filter')])
