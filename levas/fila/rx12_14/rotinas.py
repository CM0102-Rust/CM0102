# O desempate (spareggio) da Serie A e da Serie B: a fase seguinte das duas e o play-off da Serie A (ita_ser_a.cpp,
# ita_ser_b.cpp; leitor rx12, rodada 6, 25/11/2026). Usa state_league_advance_stage (bra_reg_bahia.rs),
# ita_ser_c1a_compare_advance_stage (ita_ser_c1a.rs), serie_a_zones (ita_ser_a.rs) e o molde do desempate croata
# (create_croatian_tiebreak_playoff, cro_a1.rs), ja publicados. O create_italian_tiebreak_playoff e o
# compare_italian_tiebreak_playoff ficam pub(crate) para o rx12_15 (Serie B).
# So rodam no fim de uma temporada italiana com dois clubes empatados em pontos numa fronteira de faixa (o
# update_for_date chama a virtual ADVANCE_STAGE so entao; nao houve empate ate 2002-08-01): ATE 2003-08-15 (o fim da
# temporada 2002/03), e mesmo assim so com o empate (sorte).
# Convencoes conferidas na desmontagem:
# - 0x6419e0 e 0x646b20 thiscall(this) ret 0: +0x30 >= +0x2c - 1 (com sinal): caixa linha 0x328 (ita_ser_a.cpp) /
#   0x23d (ita_ser_b.cpp); senao +0x30 + 1 e, em 0, chamada direta a 0x641a70 / 0x646bb0 com o ecx da liga;
# - 0x641a70 thiscall(this) ret 0: as faixas como o serie_a_zones (UefaSeeding::get_nation_places 0x905c60 (nacao,
#   &cl, &uefa), 18 bytes locais); a varredura igual a do desempate croata, com a condicao das faixas no lugar dos
#   status 3; +0xee = contagem / 2; sem empate: +0x2c = 0, +0x30 = -1, +0xe6 = 0 e a virtual +0x94; com empate: a
#   virtual +0x3c (0, &rodadas, &extra, 0), operator new(0xb2), Cup_Stage 0x522e00 (15 argumentos, ret 0x3c), [+0xc] =
#   a fase; nula: caixa linha 0x38e; senao free (0x9452ca) da rodada, Cup::set_up_round 0x51c800 (0), +0x30 = 0,
#   +0xc5 = 6.
# Comparadores: advance_stage lado a lado no +0x30 quando nao cria a fase; o play-off so a original e a lista da fase
# conferida contra o plano.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('ita_ser_a_advance_stage', 0x6419e0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('ita_ser_a_create_tiebreak_playoffs', 0x641a70, [0x6a, 0xff, 0x68, 0x9e, 0xb9, 0x95, 0x00]),
           ('ita_ser_b_advance_stage', 0x646b20, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('o desempate da Série A e da Série B',
        'the Serie A and Serie B tie-break play-off',
        [('ita_ser_a_advance_stage', 'ItaSerA::advance_stage', '0x6419e0', 'comp/leagues/ita_ser_a.cpp', 'thiscall',
          'passa à fase seguinte da Série A: depois da última, a caixa de erro; ao chegar à fase 0 (a liga fechou com um empate numa fronteira), monta o desempate',
          'moves Serie A to its next stage: past the last one, the error box; on reaching stage 0 (the league closed with a tie on a boundary), builds the tie-break play-off'),
         ('ita_ser_a_create_tiebreak_playoffs', 'ItaSerA::create_tiebreak_playoffs', '0x641a70', 'comp/leagues/ita_ser_a.cpp', 'thiscall',
          'o desempate da Série A: dois clubes vizinhos na tabela, empatados em pontos e em faixas diferentes (título, vagas da Liga dos Campeões, da Copa da UEFA, as duas seguintes, rebaixamento), jogam um play-off pela vaga; os empatados ficam com a situação 1 e as posições possíveis ajustadas; sem empate, a liga fecha e as situações são recalculadas',
          'the Serie A tie-break: two neighbouring clubs level on points in different bands (title, Champions League places, UEFA Cup places, the next two, relegation) play off for the place; the tied clubs get status 1 and their possible positions set; with no tie, the league closes and the statuses are recomputed'),
         ('ita_ser_b_advance_stage', 'ItaSerB::advance_stage', '0x646b20', 'comp/leagues/ita_ser_b.cpp', 'thiscall',
          'passa à fase seguinte da Série B, como a da Série A, com o desempate da Série B',
          "moves Serie B to its next stage, as Serie A's, with the Serie B tie-break play-off")])
CAMPOS = []
INTERESSA = []
