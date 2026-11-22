# As telas do clube (3): a aba Elenco (Squad) da tela do clube. Leitor tel01 (rodada das telas), 21/11/2026.
# A rotina vai de 0x456900 ate o ret em 0x45cb54 (~25 KB, 7.660 instrucoes; o mapa do grupo dava 25.184 bytes);
# portada em pedacos com nome (menus, selecao, cabecalho, linhas, celulas), desviada inteira.
# Convencao conferida na desmontagem: cdecl(top, bottom), ret 0; os dois argumentos sao alterados no lugar (o topo
# desce e o pe sobe), aqui locais.
# Alterada no cm0102-1080.exe (0x459a22..0x459a28: os pesos das colunas da visao de posicoes em 0x601eeb em vez da
# pilha; 0x459a3b: 12 colunas em vez de 8; 0x45c018: tres jogadores por linha em vez de dois): a copia le esses bytes
# do exe (`0x459a22 as *const`, `0x459a3b`, `0x45c018`) e monta o que cada exe monta.
# Usa os auxiliares do tel01_1 e do tel01_2 (screens_current_display, DisplayAddAreaFn, club_screens_llist,
# club_screens_squad_view_settings, club_screens_format_cash, club_screens_text_length): entra junto com eles.
# Defeitos do original: o estado 4 (convocado) nao escreve texto e mostra o que 0xde1f64 tinha (o numero/posicao da
# linha ou o texto da linha anterior); o nome com estrela e formatado num buffer da pilha de 0x12c bytes com o tamanho
# 0x7d0; os itens "Available Only", "Unavailable", "European Squad" e "Include Reserves" marcados mandam 0x800, 0x800,
# 0x4000 e 0x200 em vez do proprio bit; a visao de atributos ordena no lugar as tabelas estaticas do exe (0x98de9c..);
# a disponibilidade da linha de selecao recebe o endereco da data atual como competicao; varias palavras de cor e
# bytes empurrados levam no alto lixo de registrador ou da pilha (aqui zero; o Display guarda palavras).
# Comparador: pela lista de exibicao (compare_by_display_list_with) mais as tabelas de atributos (0x98de84..0x98ded8),
# o bloco da visao do elenco do humano e a ficha da ultima partida (0xad9db8); o que roda fora da lista e idempotente
# (o tatico ativo, os numeros podados, o treino posto em dia na semana, a ficha limpa de quem saiu).
TELAS = True
ROTINAS = [('club_screen_draw_squad_tab', 0x456900, [0x6a, 0xff, 0x68, 0x31, 0x59, 0x95, 0x00])]
DOCS = ('as telas do clube (3): a aba Elenco da tela do clube',
        'the club screens (3): the club screen\'s squad tab',
        [('club_screen_draw_squad_tab', 'club_screen_draw_squad_tab', '0x456900', 'club_screens.cpp', 'cdecl',
          'a aba Elenco da tela do clube: os menus (visão, ordem, competições, atributos, filtro, limpar a seleção), o título, os números ou a ficha da seleção, os cabeçalhos da visão e uma linha por jogador (número ou vaga, situação, nome com estrela para os três de maior reputação de quem não é humano, e as colunas da visão: posições em duas colunas, contrato, seleção, estatísticas, atributos ou outros, e o valor)',
          'the club screen\'s squad tab: the menus (view, sort, competitions, attributes, filter, clear the selection), the title, the squad numbers or the team sheet, the view\'s headings and one row per player (number or slot, status, name starred for the three most reputed of a non-human club, and the view\'s columns: positions in two columns, contract, selection, statistics, attributes or other, and the value)')])
