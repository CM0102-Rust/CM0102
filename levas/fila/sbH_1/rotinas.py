# A copa de acesso e as repescagens de acesso da Segunda Divisao argentina (arg_second.cpp; leitor sbH, rodada 5,
# 24/11/2026; as sobras do sbD, ver levas/fila/sbD_10/FORA.md). Simulacao com a Argentina ('ENG KOR ARG ITA FRA ESP AUS',
# ate 2002-08-01, como os irmaos cx08 e sbD_7..9): as duas rodam uma vez no fim da temporada argentina (cobertura do
# cx01_8: create_promotion_cup_stage e create_promotion_playoffs 1 vez cada ate 2002-08-01), chamadas direto pelo
# ArgSecond::advance_stage (0x408187 e 0x408178).
# - 0x408630 ArgSecond::create_promotion_cup_stage thiscall() ret 0 (quadro SEH): virtual 0x34 (update_relegation_
#   averages, 0); de baixo para cima nas linhas dos promedios (+0x18, contagem +0x34; k < +0x3e), os dois primeiros
#   clubes cuja linha da tabela (League::find_club_row 0x68ab70 (clube, 0)) nao tem status 3: status 3, virtual 0x68
#   (clube, -2, 0x1e, 0x1e, 0x1e), +0xee = clube, virtual 0x40 (clube, 3, -1, 0, 0); a lista de 8 (local zerada): os
#   lideres das fases 0, 1 e 2 nas vagas 0, 2 e 4; [0xadae04] = 2 e qsort 0x945333 (linhas, n, 0x13, 0x48f450); as
#   vagas livres com os clubes das linhas nessa ordem, sem o lider da propria tabela e sem repetir, ate encher; qsort
#   de volta com 0x48f410; virtual 0x3c (3, &rodadas, &extra, 0); operator new 0xb2 e Cup_Stage 0x522e00 (this, 8,
#   lista, rodadas, competicao, tabela, ano, 3, 1, extra, 0x14, 1, 0, 0, 0) na vaga 3; sem fase a caixa da linha 0x3e3
#   (sem liberar a tabela); com ela Cup::set_up_round 0x51c800 (0), free da tabela e +0x30 = 3. Prologo mov eax, fs:[0]
#   (6 bytes).
# - 0x408910 ArgSecond::create_promotion_playoffs thiscall() ret 0 (quadro SEH): malloc 0x944e46 (8) (caixa 0x407);
#   a Primeira ([[0xadadfc] + 4 x [0x9cf654]]) com a virtual 0x34 (0); par[0] = clube da linha 17 dos promedios dela,
#   par[1] = o primeiro time da fase 3 (+0xa7, 6 bytes, contagem +0x3e) com +5 = 2 (sem achar fica o lixo do malloc:
#   defeito); fase 4 (2 clubes) na vaga 4 (caixa 0x42b, sem liberar par e tabela); set_up_round e free da tabela; par[0]
#   = linha 16; [0xadae04] = 2, qsort por temporada, par[1] = o primeiro clube das linhas que esta na fase 3 com +5 fora
#   de 1 e 2 (sem achar fica o anterior); qsort de volta; fase 5 na vaga 5 (caixa 0x46a); set_up_round, free da tabela e
#   do par, +0x30 = 5. Os locais conferidos pela pilha: o 0x408939 grava a fase 3 em [esp+0x18] com um push pendente
#   (o quadro +0x14), lido de volta em 0x408b61 e 0x408b76. Prologo push -1; push 0x95369c (7 bytes).
# Auxiliar comum (pub(crate)): league_knockout_stage (o Cup_Stage de uma fase de mata-mata de uma liga: tabela de
# rodadas pela virtual 0x3c, new 0xb2 e os 15 argumentos), que os irmaos (bel_third, fin_first, aus_nsl) podem usar.
# Comparadores: so a original (rebaixa, anuncia e monta fases), conferida depois: a fase 3 com os 8 clubes da copia
# (pelas linhas ordenadas por temporada numa copia local); as fases 4 e 5 com os pares da copia; a fase atual.
# Defeitos do original: a tabela de rodadas nao e liberada quando a fase nao nasce; na 0x408910, sem time de status 2 na
# copa o par[1] fica com o lixo do malloc.
# Depende do cx08_4 (ARG_AVERAGES, ARG_AVERAGE_COUNT, ARG_AVERAGE_ROW_SIZE, arg_average_row_club, this_average_count,
# update_relegation_averages_virtual). Iguais no cm0102-1080.exe (inteiras).
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_create_promotion_cup_stage', 0x408630, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('arg_second_create_promotion_playoffs', 0x408910, [0x6a, 0xff, 0x68, 0x9c, 0x36, 0x95, 0x00])]
DOCS = ('a copa de acesso e as repescagens de acesso da Segunda Divisão argentina',
        'the Argentine Second Division promotion cup and promotion play-offs',
        [('arg_second_create_promotion_cup_stage', 'ArgSecond::create_promotion_cup_stage', '0x408630', 'arg_second.cpp', 'thiscall, ret 0',
          'no fim da Segunda Divisão argentina os dois clubes de pior promédio caem; a copa de acesso junta os três campeões das zonas e os cinco melhores em pontos na temporada (sem o líder da tabela, que sobe direto), em mata-mata (fase 3)',
          "at the end of the Argentine Second Division the two clubs with the lowest averages go down; the promotion cup brings together the three zone winners and the five best clubs of the season's points (without the table leader, promoted directly), as a knockout (stage 3)"),
         ('arg_second_create_promotion_playoffs', 'ArgSecond::create_promotion_playoffs', '0x408910', 'arg_second.cpp', 'thiscall, ret 0',
          'as duas repescagens contra a Primeira: o 18º nos promédios da Primeira contra o clube da copa de acesso com o resultado 2 (fase 4); o 17º contra o melhor em pontos na temporada entre os que jogaram a copa sem os resultados 1 e 2 (fase 5)',
          "the two play-offs against the Premier Division: the Premier Division's 18th in the averages against the promotion cup's club with outcome 2 (stage 4); the 17th against the best of the season's points among those who played the cup without outcomes 1 and 2 (stage 5)")])
CAMPOS = []
INTERESSA = []
