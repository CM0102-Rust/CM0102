# Os recordes de pessoal e o resultado do play-off do titulo da Primeira Liga russa (rus_prm.cpp; leitor cx13, rodada
# 5, 24/11/2026). Simulacao com a Russia (as ligas e a data dos itens mwd): 18 chamadas na cobertura do mundo ate
# 2002-08-01, todas na fase da liga (novembro de 2001); o ramo do play-off so roda com os dois primeiros empatados.
# - 0x7ebc40 RusPrm::update_staff_records_for_status thiscall(clube, status, fase, a4, jogo, a6) ret 0x18: fase 0xff,
#   League::update_staff_records_for_status 0x686940 (clube, status, -1, a4, jogo, a6; ret 0x18) e o retorno dele. No
#   play-off, o vencedor (status 1): descendo a tabela (contador de 16 bits) ate a linha k do clube, se a linha k-1
#   (k > 0) tem um clube do jogo (+0x1c, +0x20) as linhas k e k-1 sao trocadas (59 bytes, por uma copia na pilha) com
#   as posicoes k e k-1; senao, se a linha k+1 (lida mesmo depois da ultima: defeito do original) tem um clube do jogo,
#   as posicoes k e k+1; as duas com status 0xff e fim. Depois, sem linha com status 1: +0xe6 = 0, +0xdd = 0, a
#   virtual +0x94 e +0xdd = 1. O play-off devolve -1. Prologo mov al, [esp+0xc]; sub esp, 0x3c (7 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: so a original (recordes e a tabela); no play-off o retorno -1.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rus_prm_update_staff_records_for_status', 0x7ebc40, [0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x3c])]
DOCS = ('os recordes de pessoal e o resultado do play-off do título da Primeira Liga russa',
        'the Russian Premier League\'s staff records and title play-off result',
        [('rus_prm_update_staff_records_for_status', 'RusPrm::update_staff_records_for_status', '0x7ebc40', 'rus_prm.cpp', 'thiscall, ret 0x18',
          'na liga, os recordes comuns; no play-off do título, o vencedor fica acima do perdedor na tabela e, decidido o empate, as posições e os status são refeitos',
          'in the league, the common records; in the title play-off, the winner is put above the loser in the table and, once the tie is settled, the positions and statuses are worked out again')])
CAMPOS = []
INTERESSA = []
