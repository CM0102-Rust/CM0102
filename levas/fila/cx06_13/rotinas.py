# A nova edicao, a parada na virada da temporada e o registro da fase final das eliminatorias da Eurocopa
# (euro_champ_qual.cpp; leitor cx06, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01 (o mesmo do
# cx06_11/cx06_12 e da Copa da Asia do cx05): as eliminatorias atravessam a virada de 2003 (a parada) e terminam em
# nov/2003; a nova edicao vem na virada seguinte; na cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas
# [inferencia: as datas exatas da parada, da nova edicao e de quem pede o registro da fase final nao foram vistas].
# - 0x57eb20 EuroChampQual::start_new_season thiscall() ret 0 (virtual 8): +0x4c 0, League::free_table 0x687970 (push 0),
#   free(+0xba) e zera, as fases ate a atual (virtual 0 com 1; o vetor fica), +0x30 -1, LLIST::clear 0x68d080 da lista
#   +0x10, free(+0xee) e zera +0xee/+0xf2, o ano + 4 ate nao ser antes do ano atual [0xae2c92], +0x45, +0xb5 e +0xfb 0,
#   Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1) e set_up_rules (virtual 0x8c; 0: caixa 0x378). Devolve o
#   eax do set_up_rules. Prologo sub esp, 0x200 (6 bytes).
# - 0x57eb10 EuroChampQual::skip_season_reset thiscall() ret 0 (virtual 0x2c): +0xb5 = 1 e 1. Prologo mov eax, 1 (5
#   bytes).
# - 0x57d060 EuroChampQual::get_finals_comp thiscall() ret 0 (virtual 0x78): [0xae23d0] + 0x6b x [0x9cf7a4] (o ecx nao e
#   lido). Prologo mov ecx, [0x9cf7a4] (6 bytes).
# A nova edicao fica sem mutacao propria: o que ela monta (a data do sorteio de 2005) so e usado depois da data da
# cadeia; o molde e o das outras eliminatorias ja provadas (wc_europe_league_start_new_season).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: a nova edicao so a original (libera as fases), com o plano (o ano no passo de 4, +0x30 -1, +0x45 e
# +0xfb 0); a parada lado a lado em +0xb5 e no retorno; o registro lado a lado (so le).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('euro_champ_qual_start_new_season', 0x57eb20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('euro_champ_qual_skip_season_reset', 0x57eb10, [0xb8, 0x01, 0x00, 0x00, 0x00]),
           ('euro_champ_qual_get_finals_comp', 0x57d060, [0x8b, 0x0d, 0xa4, 0xf7, 0x9c, 0x00])]
DOCS = ('a nova edição, a parada na virada e o registro da fase final das eliminatórias da Eurocopa',
        "the European Championship qualifiers' new edition, season stop and finals record",
        [('euro_champ_qual_start_new_season', 'EuroChampQual::start_new_season', '0x57eb20', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'começa a edição seguinte das eliminatórias: limpa a tabela, as datas, as fases, os resultados guardados e os inscritos; o ano da edição avança de 4 em 4 até não ser anterior ao ano atual; arquiva as estatísticas e refaz as regras',
          'starts the next qualifiers edition: clears the table, the dates, the stages, the stored results and the entrants; the edition year moves on 4 at a time until it is not before the current year; archives the statistics and sets up the rules again'),
         ('euro_champ_qual_skip_season_reset', 'EuroChampQual::skip_season_reset', '0x57eb10', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'na virada da temporada, marca as eliminatórias como paradas e devolve 1, para que não recomecem no meio (elas atravessam duas temporadas)',
          'at the season turnover, marks the qualifiers as stopped and returns 1, so that they do not restart midway (they span two seasons)'),
         ('euro_champ_qual_get_finals_comp', 'EuroChampQual::get_finals_comp', '0x57d060', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'devolve o registro da fase final da Eurocopa, a competição que as eliminatórias alimentam',
          'returns the record of the European Championship finals, the competition the qualifiers feed')])
CAMPOS = []
INTERESSA = []
