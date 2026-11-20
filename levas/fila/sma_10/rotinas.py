# O texto da notícia da saída de um técnico (news_manager_departure_text 0x6a1d60, manager_manager.cpp; leitor sma,
# 20/11/2026). Chamado pelas notícias (0x77bc80, o texto de cada tipo). Convenções conferidas:
# - a rotina: stdcall(notícia, corpo, Text), ret 0xc (o ecx não é usado);
# - 0x785830 (newsitem_get_slot): ecx = notícia -> thiscall; manchete lê os campos 0..5 e 7..10, corpo 0..5 e 7..11;
# - o nome: humano (id >= [0xae2364] - 16) pelos textos dos campos 13 e [campo 12] (0x785a20, Rust: thiscall(notícia,
#   k, buf, 0x33), ret 0xc) e 0x53e2e0 (cdecl, estilo 1, -1); senão 0x53e140 (cdecl, 0x190, estilo 5 na manchete e 1
#   no corpo, id); o segundo nome pelos campos 8..10 com id -1 (estilo 5 na manchete, onde não é usado, e 1 no corpo);
# - 0x944c9f (sprintf da CRT) com "%s" (0x991944) para o nome do clube (curto na manchete) e para "Sedgefordian";
#   0x6181d0 (adjetivo da nação, Rust): cdecl(nação, buf, 0x190, 9); 0x53b900 e 0x5409b0: cdecl(clube), testam eax;
# - os argumentos de cada formato conferidos por um script que segue os push e os lea [esp+k] de cada bloco até a
#   0x66f4e0 (as duas tabelas de salto 0x6a2d78 e 0x6a2db8, 16 entradas cada, padrão 0x6a290b e 0x6a2b87; a entrada 13
#   da segunda não escreve nada; a entrada 10 das duas usa o segundo nome);
# - 0x945a60 (strstr da CRT): cdecl(0xde1f64, nome); 0x4ae660 e 0x4ae8a0 (o Text): thiscall; o vínculo do adjetivo
#   aponta para o clube (como no original).
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('news_manager_departure_text', 0x6a1d60, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xc0, 0x06, 0x00, 0x00])]
DOCS = ('o texto da notícia da saída de um técnico',
        'the manager departure news text',
        [('news_manager_departure_text', 'news_manager_departure_text', '0x6a1d60', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'o título ou o corpo da notícia de que o técnico de um clube ou de uma seleção foi demitido, pediu demissão ou se aposentou: a manchete pelo modo da saída e pelo cargo (técnico, jogador-técnico, ou com motivo), com o nome curto do clube; o corpo, numa seleção, pela primeira ou segunda seleção, pelo modo e pelo motivo, com o adjetivo da nação; num clube, demitido como técnico ou jogador-técnico com ou sem um segundo nome, ou com um dos 16 motivos de duas tabelas; demissão pedida pelo cargo; os outros modos com um texto só; os vínculos da pessoa (quando os ids batem), do clube e do adjetivo (quando aparecem no texto)',
          "the headline or body of the news that a club or national team manager was sacked, resigned or retired: the headline by how he left and by the job (manager, player-manager, or with a reason), with the club's short name; the body, for a national team, by the first or second national team, how he left and the reason, with the nation's adjective; for a club, sacked as manager or player-manager with or without a second name, or with one of 16 reasons from two tables; resigned by the job; the other ways with a single text; the links of the person (when the ids match), of the club and of the adjective (when they appear in the text)")])
