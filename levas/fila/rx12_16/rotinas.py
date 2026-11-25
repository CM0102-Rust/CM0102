# A Serie C2 e a Serie D na virada italiana: a troca com a Serie D e o reagrupamento da C2 (ita_ser_a.cpp; leitor
# rx12, rodada 6, 25/11/2026). Usa comp_by_global (bra_nat_first.rs) e comp_record_of_key (fra_first.rs), ja
# publicados; o molde e o exchange_serie_c2_clubs_unplayed e o regroup_serie_c1_clubs do ita_ser_a.rs.
# So existem com a Serie C2 carregada (base NORMAL: o setup apaga o bit 4 na MINIMUM): chamadas pelo
# ita_ser_a_start_new_season (Rust) na virada da Serie A (verao de 2002), quando o objeto da C2 A existe. BASE NORMAL,
# LIGAS 'ENG ITA' e ATE 2002-09-30, como os itens da C2 (cx11, cx12).
# Convencoes conferidas na desmontagem:
# - 0x6421c0 thiscall(this) ret 0: os objetos de C2 A, B e C lidos antes; malloc(4 x G_CLUB_COUNT), nulo: caixa linha
#   0x46b; os clubes com divisao (+0x57) e nacao (+0x53), [divisao] = [KEY_CLUB_COMP_ITALIAN_SERIE_D] e [nacao] =
#   [KEY_NATION_ITALY]; contagem em word; < 9 (com sinal): nada; senao 0x4afbd0 (lista, n, 9) cdecl, os 9 primeiros
#   por 0x6830b0 (clube, registro da C2 A, 1) ret 0xc; depois, i < club_count da C2 A (lida uma vez): em C2 A, B e C,
#   0x6826d0(i) com +0x37 == 3 leva o clube (0x6826d0 de novo) por 0x6831a0 (clube, registro da Serie D, 1); free
#   (0x9452ca) sempre;
# - 0x642790 (ecx = a Serie A, sem argumentos) ret 0: os registros de C2 A, B, C (G_CLUB_COMP_TABLE + chave x 0x6b);
#   ate 54 clubes (contagem em byte com sinal, conferida antes de cada clube); qsort 0x945333 (lista, n, 4, 0x53fd40);
#   k < 0x12: C2 C; k < 0x24: C2 B; senao C2 A (+0x57).
# Comparadores: so a original; a troca conferida pelos rebaixados da C2 (situacao 3) na Serie D; o reagrupamento
# contra o plano da mesma ordenacao.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_a_exchange_serie_d_clubs', 0x6421c0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('ita_ser_a_regroup_serie_c2_clubs', 0x642790, [0xa1, 0x80, 0xf5, 0x9c, 0x00])]
DOCS = ('a Série C2 e a Série D na virada italiana',
        'Serie C2 and Serie D at the Italian turnover',
        [('ita_ser_a_exchange_serie_d_clubs', 'ItaSerA::exchange_serie_d_clubs', '0x6421c0', 'comp/leagues/ita_ser_a.cpp', 'thiscall',
          'com a Série C2 carregada: os nove melhores clubes italianos da Série D (por reputação, embaralhados de nove em nove) sobem à Série C2 A, e os rebaixados de cada grupo da C2 descem à Série D; com menos de nove, nada muda',
          'with Serie C2 loaded: the nine best Italian Serie D clubs (by reputation, shuffled in nines) go up to Serie C2 A, and the clubs relegated from each C2 group go down to Serie D; with fewer than nine, nothing changes'),
         ('ita_ser_a_regroup_serie_c2_clubs', 'ita_ser_a_regroup_serie_c2_clubs', '0x642790', 'comp/leagues/ita_ser_a.cpp', 'thiscall',
          'reagrupa a Série C2: até 54 clubes dos três grupos, do norte para o sul; os 18 primeiros vão para a C2 C, os 18 seguintes para a C2 B e o resto para a C2 A',
          'regroups Serie C2: up to 54 clubs of the three groups, from north to south; the first 18 go to C2 C, the next 18 to C2 B and the rest to C2 A')])
CAMPOS = []
INTERESSA = []
