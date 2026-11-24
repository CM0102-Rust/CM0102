# O que um olheiro, treinador ou fisioterapeuta percebe num jogador (player_search.cpp; leitor sx17, rodada 5,
# 24/11/2026): player_search_assess_player_traits (0x7bd9d0), player_search_judge_notices_trait (0x7bdd70) e
# player_search_write_traits_report_text (0x7bdee0). Rotinas inteiras (0x7bd9d0..0x7bdd6a ret; 0x7bdd70..0x7bdedb ret;
# 0x7bdee0..0x7be63e ret, com o _chkstk de 0x11fc bytes; o resto e enchimento).
# Quem chama: o relatorio do olheiro sobre um jogador (report_on_player 0x7b738d, o texto 0x7bc405), o dia de trabalho
# do olheiro (0x7b4747, 0x7b4c8b, ja em Rust) e as telas do jogador com o relatorio do treinador (0x487d2e, 0x488260;
# 0x7a1b3b). A carreira automatica (sem olheiros do humano) nao passa: contadores 0.
# TELAS: a prova e o Lucas jogando comparando: o relatorio do treinador na tela de um jogador e os relatorios de olheiro
# (noticias de olheiro sobre jogadores observados).
# Convencoes conferidas na desmontagem:
# - 0x7bd9d0 cdecl(jogador, juiz, &fortes, &fracos) -> al; 0x7bdd70 cdecl(juiz, jogador, valor: byte) -> al; 0x7bdee0
#   cdecl(fortes, fracos, reputacao do clube (i16), idade (byte), saida, nome do olheiro, nome do jogador), sem retorno;
# - 0x66f4d0 langlib_get_language cdecl(); 0x543f30 staff_get_non_player_attribute cdecl(pessoa, 5) -> al; 0x5407e0
#   database_get_staff_reputation_for_viewer cdecl(jogador, juiz, 0) -> ax; 0x54cdb0 Date::days_since
#   thiscall(0xae2c90, +0x3e, +0x42) ret 8; 0x945640 _ftol (x87: 1,5 de 0x968940); 0x544410
#   non_player_attribute_rating cdecl(nao-jogador, byte +0x19) -> al; 0x66f4e0 langlib_sprintf cdecl.
# Defeitos do original: como pontos fortes, o ritmo de trabalho e o profissionalismo sao julgados pelo valor da
# determinacao do jogador; so o regen tem pontos fracos; na frase "bons e maus pontos sem bolas paradas" vai a contagem
# de especialidades (sempre 0 ali) onde os outros formatos passam um genero.
TELAS = True
ROTINAS = [('player_search_assess_player_traits', 0x7bd9d0, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14]),
           ('player_search_judge_notices_trait', 0x7bdd70, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('player_search_write_traits_report_text', 0x7bdee0, [0xb8, 0xfc, 0x11, 0x00, 0x00])]
DOCS = ('o que um olheiro, treinador ou fisioterapeuta percebe num jogador',
        'what a scout, coach or physio notices in a player',
        [('player_search_assess_player_traits', 'player_search_assess_player_traits', '0x7bd9d0', 'player_search.cpp', 'cdecl',
          'marca os pontos fortes e fracos que o juiz (com clube) percebe: zera as duas palavras; sem clube 0; nas linguas 4 e 5 nada (1); jogador sem ficha 0; com o desvio do juiz (pela capacidade de julgar e o id do jogador), cada atributo acima de 14 + desvio (consistencia 15) e forte se o juiz o percebe (lideranca, consistencia, lealdade, pressao, determinacao, ritmo, profissionalismo, temperamento, coragem e, fora o goleiro, escanteios, faltas e penaltis); so o regen tem fracos, abaixo de 6 + desvio (lealdade so com o clube do juiz abaixo de 4750 de reputacao); devolve 1',
          'sets the strengths and weaknesses the judge (with a club) notices: zeroes both words; no club 0; in languages 4 and 5 nothing (1); a player without a sheet 0; with the judge offset (from his judging ability and the player id), each attribute above 14 + offset (consistency 15) is a strength when the judge notices it (leadership, consistency, loyalty, pressure, determination, work rate, professionalism, temperament, bravery and, except goalkeepers, corners, free kicks and penalties); only a regen has weaknesses, below 6 + offset (loyalty only with the judge club below reputation 4750); returns 1'),
         ('player_search_judge_notices_trait', 'player_search_judge_notices_trait', '0x7bdd70', 'player_search.cpp', 'cdecl',
          'diz se o juiz percebe um atributo: a barra 35 − valor, mais 2 por ano abaixo de 22; a reputacao vista do jogador, se os dois estao no mesmo clube ha mais de 150 dias, vira o maior entre 1,5 × ela e (dias do jogador no clube + 1300) × 5; acima de 5000 a barra cai (10000 − reputacao) / 1000; o olho do juiz: (soma dos ids mod 17) + 4 × |nota de julgar − 10| − 8 tem de passar a barra',
          'tells whether the judge notices an attribute: the bar 35 − value, plus 2 a year under 22; the player seen reputation, when both have been at the same club over 150 days, becomes the larger of 1.5 × it and (player days at the club + 1300) × 5; above 5000 the bar drops by (10000 − reputation) / 1000; the judge eye: (ids sum mod 17) + 4 × |judging rating − 10| − 8 must pass the bar'),
         ('player_search_write_traits_report_text', 'player_search_write_traits_report_text', '0x7bdee0', 'player_search.cpp', 'cdecl',
          'escreve as frases do relatorio sobre bolas paradas e pontos bons e maus: as especialidades (escanteios, faltas, penaltis), os pontos bons (ate dois), os maus (a ambicao pela idade e pela reputacao do clube, preguica, inconstancia, falta de profissionalismo, nervos, pressao, determinacao) e a frase que a combinacao pede, com o nome do olheiro e do jogador; sem nada a dizer, a saida vazia',
          'writes the report sentences about set pieces and good and bad points: the specialities (corners, free kicks, penalties), the good points (two at most), the bad ones (ambition by age and club reputation, laziness, inconsistency, unprofessionalism, nerves, pressure, determination) and the sentence the combination calls for, with the scout and player names; with nothing to say, the output emptied')])
