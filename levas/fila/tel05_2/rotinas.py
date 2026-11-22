# As telas do clube (5b): a tela da troca de jogadores numa proposta ("Add Player To ... Transfer Bid"). Leitor tel05
# (rodada das telas), 21/11/2026.
# A rotina vai de 0x47c180 ate o ret em 0x4814d0 (~21 KB); o mapa do grupo a cortava em 0x480020 (o comeco do tel06,
# um pedaco dela, "fragment" no mapa da comunidade). Portada em pedacos com nome (cabecalho e menus, candidatos, area
# da lista, linhas, celulas, rodape), desviada inteira.
# Convencao conferida na desmontagem: cdecl, ret 0 (quadro alinhado a 8). Igual no cm0102-1080.exe (a rotina inteira).
# Usa, do tel26_1, tel26_add_item, tel26_add_area, tel26_add_button, tel26_area_button, tel26_area_text e
# tel26_slot_manager; do tel01_1, screens_current_display; do tel01_2, club_screens_club e club_screens_staff: entra
# depois deles.
# Defeitos do original: o orcamento da proposta (valor x 1,2 menos os oferecidos) e calculado e nunca lido; na visao
# de atributos sem grupo escolhido a palavra das visoes vira o endereco da tabela (o qsort leria la); o nome do clube
# comprador no aviso "nao interessa" liga para o clube vendedor; o item de convocacao sem clube mostra o texto que
# 0xde1f64 tinha; varias palavras de cor e o numero de linhas levam no alto lixo de registrador ou da pilha (aqui zero,
# ou o mesmo registrador do original quando se sabe).
# Comparador: pela lista de exibicao (compare_by_display_list_with) mais [0xae1080] (o contrato do jogador, gravado),
# as tabelas de atributos (0x98de84..0x98ded8, ordenadas no lugar) e a lista dos oferecidos da proposta.
TELAS = True
ROTINAS = [('part_exchange_screen_build', 0x47c180, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('as telas do clube (5b): a tela da troca de jogadores numa proposta',
        'the club screens (5b): the part exchange screen of a bid',
        [('part_exchange_screen_build', 'part_exchange_screen_build', '0x47c180', 'club_screens.cpp', 'cdecl',
          'monta a tela de acrescentar um jogador em troca numa proposta: o título nas cores do clube, o texto da proposta, os menus (visão, ordem, competições, atributos, filtro) e o botão Exchange; lista os jogadores do clube que o vendedor aceitaria (do banco, com ficha, sem empréstimo, sem outra proposta ou só com esta, que o clube possa oferecer e cujo salário pedido não passe de 1,5 vez o máximo do vendedor; no clube do humano, só os que um vendedor do computador quer), dois por linha na visão tradicional ou um por linha com as colunas da visão e o valor; sem ninguém, diz se o vendedor não se interessa ou se os salários são altos demais; Back e Next',
          'builds the screen to add a player in exchange to a bid: the title in the club\'s colours, the bid\'s text, the menus (view, sort, competitions, attributes, filter) and the Exchange button; lists the club\'s players the seller would take (database players with a record, not on loan, with no other bid or only this one, that the club may offer and whose wage demand is at most 1.5 times the seller\'s maximum; for the human\'s club, only those a computer seller wants), two per row in the traditional view or one per row with the view\'s columns and the value; with nobody, says whether the seller is not interested or the wages are too high; Back and Next')])
