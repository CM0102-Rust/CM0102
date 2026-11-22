# As telas do clube (5c): a tela dos numeros do elenco e do elenco europeu (montagem). Leitor tel05 (rodada das
# telas), 21/11/2026, a pedido da sessao principal (o tel04 deixou a rotina para o tel05).
# A rotina vai de 0x475520 ate o ret em 0x47a9d0 (~21 KB; os primeiros 7,3 KB estavam na lista do tel04). Portada em
# pedacos com nome (cabecalho e menus, grade dos numeros, area da lista, linhas), desviada inteira; as celulas das
# linhas e os menus sao os mesmos do tel05_2 (squad_list_*), com as diferencas da tela dos numeros (ligacoes de
# selecao nacional, valor e situacao com a ligacao 0xd).
# Convencao conferida na desmontagem: cdecl, ret 0. Igual no cm0102-1080.exe (a rotina inteira).
# Usa os auxiliares do tel05_1 (squad_numbers_holder, squad_numbers_number) e do tel05_2 (squad_list_*,
# part_exchange_*), e os do tel26_1 e tel01_1: entra depois deles.
# Defeitos do original: as cores do titulo sao locais nao iniciados; na visao de atributos sem grupo escolhido o numero
# de linhas vira o endereco da tabela (o qsort leria la); o numero de linhas da visao tradicional leva no alto os bytes
# do ponteiro da copia do elenco (aqui repetidos).
# Comparador: pela lista de exibicao mais [0xae1080], as tabelas de atributos e a copia do elenco; sem a copia (param
# 0x11 vazio) a original a aloca: so a original roda.
TELAS = True
ROTINAS = [('squad_numbers_screen_build', 0x475520, [0x6a, 0xff, 0x68, 0x51, 0x5e, 0x95, 0x00])]
DOCS = ('as telas do clube (5c): a montagem da tela dos números do elenco e do elenco europeu',
        'the club screens (5c): building the squad numbers and European squad screen',
        [('squad_numbers_screen_build', 'squad_numbers_screen_build', '0x475520', 'club_screens.cpp', 'cdecl',
          'monta a tela dos números do elenco (ou do elenco europeu): copia o elenco quando não há cópia (no modo europeu sem os emprestados e os de contrato do tipo 5), o título nas cores do clube, os menus de visão, ordem, competições e atributos, os botões Auto e Clear; no modo dos números, os 50 números em duas fileiras (livre, de posse definitiva ou provisória); no europeu, quantos estão inscritos; a lista dos jogadores (numa partida, só os escalados) com o número ou a inscrição, a situação, o nome e as colunas da visão; Back e Submit',
          'builds the squad numbers (or European squad) screen: copies the squad when there is no copy (in the European mode without the loaned players and the type 5 contracts), the title in the club\'s colours, the view, sort, competitions and attributes menus, the Auto and Clear buttons; in the numbers mode, the 50 numbers on two rows (free, held for good or provisionally); in the European one, how many are registered; the list of players (for a match, only the selected ones) with their number or registration, status, name and the view\'s columns; Back and Submit')])
