# As inscricoes do Brasil nas copas: a Copa do Brasil na virada e a Copa dos Campeoes no fim dos estaduais
# (bra_nat_first.cpp pelo vizinho; leitor sbD, rodada 5, 24/11/2026; as sobras do cx09). Simulacao com as 8 ligas
# (como os irmaos cx09_16 e cx09_17).
# - 0x42b760 bra_nat_first_fill_brazilian_cup_entrants thiscall() ret 0 (chamada pela virada da Serie A,
#   bra_nat_first_start_new_season, ja em Rust, com mov ecx, esi; nao le o ecx): os 71 lugares (70 a 0) do livro de
#   inscritos (+0x14, entradas de 6 bytes) da Copa do Brasil ([0x9cf9e8]). Fora: o campeao e o vice da Serie A, o
#   campeao da Copa do Brasil e o da Copa dos Campeoes (comp_util_get_latest_placed_club 0x4afd80, cdecl(registro, k);
#   a Copa dos Campeoes pelo registro [0xae23d0] + 0x6b x [0x9cf9b8]). League::get_table_row 0x6826d0 (push r; mov ecx,
#   liga -> thiscall, ret 4). Cotas (bytes do quadro): 5, e 7 no Nordeste, Centro-Oeste e Norte. Prologo sub esp, 0x64;
#   mov eax, [0xadadfc] (8 bytes).
# - 0x42bba0 bra_nat_first_fill_champions_cup_entrants cdecl() (o salto final da 0x432970 e o exe:: dos estaduais em
#   Rust): com os doze estaduais completos (virtual +0x10 check_complete(1, 0), al) e a Copa dos Campeoes ([0x9cf9b8])
#   inativa (+0x45 = 0), o campeao de cada estadual (Comp_Stats::find_club_entry 0x4a4850 sobre +8, thiscall(clube),
#   ret 4; a menor posicao +0x73 abaixo de 100) e os pontos por jogo (fild/fidiv, guardado em float); os nove lugares e
#   o set-up da copa 0x427320 (thiscall, ret 0). Prologo sub esp, 0xa0 (6 bytes).
# Defeitos do original: na 0x42bba0, um estadual sem posicao abaixo de 100 deixa o campeao e a media com o lixo da pilha
# (a copia usa 0); na 0x42b760, o passo 3 nao confere o lugar antes de gravar (gravaria antes do livro se os passos 1 e
# 2 esgotassem os lugares; inalcancavel com as ligas reais).
# Comparadores: a Copa do Brasil lado a lado nos 71 lugares; a Copa dos Campeoes so a original (o set-up), os nove
# lugares conferidos contra o plano da copia (champions_cup_entrants) feito antes.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('bra_nat_first_fill_brazilian_cup_entrants', 0x42b760, [0x83, 0xec, 0x64, 0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('bra_nat_first_fill_champions_cup_entrants', 0x42bba0, [0x81, 0xec, 0xa0, 0x00, 0x00, 0x00])]
DOCS = ('as inscrições do Brasil na Copa do Brasil e na Copa dos Campeões', 'the Brazilian Cup and Champions Cup entrants',
        [('bra_nat_first_fill_brazilian_cup_entrants', 'bra_nat_first_fill_brazilian_cup_entrants', '0x42b760', 'bra_nat_first.cpp', 'thiscall, ret 0',
          'os 71 inscritos da Copa do Brasil na virada, do lugar 70 ao 0, sem o campeão e o vice da Série A nem os campeões da Copa do Brasil e da Copa dos Campeões: primeiro o campeão de cada um dos doze estaduais; depois os clubes da Série A do terceiro em diante; depois, linha a linha da segunda à décima, um clube de cada estadual até a cota dele (5; 7 no Nordeste, no Centro-Oeste e no Norte), sem repetir',
          "the Brazilian Cup's 71 entrants at the turnover, from slot 70 down to 0, leaving out the Series A champion and runner-up and the Brazilian Cup and Champions Cup winners: first each of the twelve state champions; then the Series A clubs from the third on; then, row by row from the second to the tenth, one club from each state league up to its quota (5; 7 for the Northeast, Centre-West and North), with no repeats"),
         ('bra_nat_first_fill_champions_cup_entrants', 'bra_nat_first_fill_champions_cup_entrants', '0x42bba0', 'bra_nat_first.cpp', 'cdecl',
          'quando os doze estaduais terminaram e a Copa dos Campeões ainda não começou, põe nela os campeões: Rio, São Paulo, Minas, Rio Grande do Sul e Paraná diretos; o melhor em pontos por jogo entre Norte/Nordeste e Centro-Oeste, entre Goiás e Santa Catarina e entre Bahia e Pernambuco (este par entra inteiro); depois monta a copa',
          'once the twelve state leagues are over and the Champions Cup has not started, puts the champions into it: Rio, São Paulo, Minas, Rio Grande do Sul and Paraná directly; the better on points per game between North/Northeast and Centre-West, between Goiás and Santa Catarina and between Bahia and Pernambuco (this pair goes in whole); then sets the cup up')])
CAMPOS = []
INTERESSA = []
