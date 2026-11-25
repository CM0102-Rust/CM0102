# O desempate da Serie B e o resultado dele (ita_ser_b.cpp; leitor rx12, rodada 6, 25/11/2026). Usa o
# create_italian_tiebreak_playoff e o compare_italian_tiebreak_playoff do rx12_14 e o swap_table_rows do cx13_12
# (ver INTEGRAR_ANTES.txt); serie_b_zones (ita_ser_a.rs) ja publicado.
# O play-off so roda no fim de uma temporada da Serie B com um empate numa fronteira (sorte): ATE 2003-08-15. O
# update_staff_records_for_status e a virtual +0x44 da Serie B: o ramo da liga (fase 0xff) roda a cada jogo; o do
# desempate so com o play-off.
# Convencoes conferidas na desmontagem:
# - 0x646bb0 thiscall(this) ret 0: as faixas locais da Serie B (1; 2, 2, 2; zeros; 3 nas quatro ultimas de 20, como o
#   serie_b_zones); o resto igual ao da Serie A (rx12_14), com a caixa na linha 0x28e e, sem empate, sem a virtual
#   +0x94;
# - 0x646e70 thiscall(clube, situacao, fase, a4, jogo, a6) ret 0x18: fase 0xff: League::update_staff_records_for_status
#   0x686940 (clube, situacao, -1, a4, jogo, a6); situacao != 1: -1; senao a busca da linha do clube (word com sinal),
#   a troca com a de cima (copia de 0x3b bytes por um temporario) ou a marca com a de baixo (lida mesmo depois da
#   ultima), +4 = posicao, +0x37 = 0xff; sem linha com situacao 1: +0xe6 = 0, +0xdd = 0, virtual +0x94, virtual +0x10
#   (0, 0), +0xdd = 1; devolve -1 (or eax, -1).
# Comparadores: o play-off so a original e a lista da fase conferida contra o plano; os recordes so a original e o
# -1 do desempate.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('ita_ser_b_create_tiebreak_playoff_stage', 0x646bb0, [0x6a, 0xff, 0x68, 0x4e, 0xba, 0x95, 0x00]),
           ('ita_ser_b_update_staff_records_for_status', 0x646e70, [0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x3c])]
DOCS = ('o desempate da Série B e o resultado dele',
        'the Serie B tie-break play-off and its result',
        [('ita_ser_b_create_tiebreak_playoff_stage', 'ItaSerB::create_tiebreak_playoff_stage', '0x646bb0', 'comp/leagues/ita_ser_b.cpp', 'thiscall',
          'o desempate da Série B: dois clubes vizinhos empatados em pontos em faixas diferentes (título, as vagas de acesso, as quatro de rebaixamento) jogam um play-off pela vaga',
          'the Serie B tie-break: two neighbouring clubs level on points in different bands (title, promotion places, the four relegation places) play off for the place'),
         ('ita_ser_b_update_staff_records_for_status', 'ItaSerB::update_staff_records_for_status', '0x646e70', 'comp/leagues/ita_ser_b.cpp', 'thiscall, ret 0x18',
          'na liga, os registros comuns; no desempate, o vencedor fica acima do adversário na tabela (as linhas trocam de lugar se ele estava abaixo) e, sem outro desempate pendente, a temporada fecha',
          'in the league, the common records; in the tie-break, the winner finishes above his opponent in the table (the rows swap if he was below) and, with no other tie-break pending, the season closes')])
CAMPOS = []
INTERESSA = []
