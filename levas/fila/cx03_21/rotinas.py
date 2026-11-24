# A reserva de clubes, os inscritos e as tabelas de rodadas da Copa da Suecia (swe_cup.cpp; leitor cx03, rodada 5,
# 24/11/2026). Competicoes da Suecia: LIGAS do grupo ENG SWE FIN NOR POR, ate 2002-08-01 (no mundo ate essa data: a
# reserva 4 vezes, os inscritos 2, as tabelas 17).
# - 0x88da20 SweCup::build_club_pool thiscall() ret 0 -> 0/1: +0xb6 = 0xf4; malloc 0x3d0 em +0xb2 (a caixa da linha
#   0x29e e 0); quatro passadas pela tabela de clubes: a Segunda (chave 0x9cf7c8, sem limite), os clubes suecos
#   (+0x53 = [0x9cf4a8]) das divisoes inferiores (0x9cf6ac) enquanto k < +0xb6 - 30, a Primeira (0x9cf5f0) enquanto k <
#   +0xb6, a mistura dos 16 ultimos (0x4afb40, cdecl), a Allsvenskan (0x9cf5ec); qsort dos 232 primeiros (0x945333,
#   comparador 0x53fd40); as secoes (malloc 4 x +0xb6; a caixa 0x2d9 e 0): 8 secoes de 30 (2 cabecas) e 31 (1 cabeca)
#   alternadas, as vagas sem cabeca da frente da reserva, as cabecas no fim da secao sorteadas entre as 12 ultimas
#   (0x90cfc0(12) + 0xe8, de novo enquanto a vaga estiver vazia); free da reserva e as secoes em +0xb2; 1.
# - 0x88d5a0 SweCup::collect_entrants thiscall() ret 0 -> -1/1: +0x3e = 30; malloc 0xb4 em +0xa7 (a caixa 0x1a4 e -1);
#   os 30 primeiros da reserva (classe e estado 0), zerados nela.
# - 0x88cf40 SweCup::create_round_table thiscall(fase, &rodadas, &extra, &saida) ret 0x10: fase < 7: 2 rodadas, extra
#   0x464 + fase, malloc 0xd0 (a caixa 0xcb), a primeira de 30 (fase par) ou 28 (impar, o resto de -1 tambem); fase 7:
#   6 rodadas, extra 0, malloc 0x270 (a caixa 0x119); acima: 0. As datas pelo 0x521e60 e 0x521eb0 (cdecl, 7 e 9 pushes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: lado a lado: a reserva (com o gerador nas regioes; as secoes da copia comparadas e liberadas), os
# inscritos (eng_cc_cup_compare_entrant_list, com as 30 vagas da reserva) e as tabelas (de toda fase ate 7).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_cup_build_club_pool', 0x88da20, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('swe_cup_collect_entrants', 0x88d5a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_cup_create_round_table', 0x88cf40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a reserva de clubes, os inscritos e as tabelas de rodadas da Copa da Suécia',
        "the Swedish Cup's club pool, entrants and round tables",
        [('swe_cup_build_club_pool', 'SweCup::build_club_pool', '0x88da20', 'swe_cup.cpp', 'thiscall, ret 0',
          'regra: os 244 clubes da Copa da Suécia são todos os da Segunda Divisão, os suecos das divisões inferiores (até 214 no total), os da Primeira (os 16 últimos misturados) e os da Allsvenskan; os 232 primeiros são ordenados e distribuídos em ordem por oito seções, alternadamente de 30 (com 2 cabeças de chave) e 31 (com 1): as vagas comuns da frente da reserva, as cabeças no fim da seção, sorteadas entre os 12 últimos da reserva',
          "rule: the Swedish Cup's 244 clubs are all the Second Division clubs, the Swedish lower division clubs (up to 214 in all), the First Division clubs (the last 16 shuffled) and the Allsvenskan clubs; the first 232 are sorted and dealt in order into eight sections, alternately 30 (with 2 seeds) and 31 (with 1) clubs: the ordinary slots from the front of the pool, the seeds at the back of the section, drawn among the last 12 of the pool"),
         ('swe_cup_collect_entrants', 'SweCup::collect_entrants', '0x88d5a0', 'swe_cup.cpp', 'thiscall, ret 0',
          'regra: os inscritos da própria Copa da Suécia são os 30 primeiros clubes da reserva (a primeira seção), tirados dela',
          "rule: the Swedish Cup's own entrants are the first 30 clubs of the pool (the first section), taken out of it"),
         ('swe_cup_create_round_table', 'SweCup::create_round_table', '0x88cf40', 'swe_cup.cpp', 'thiscall, ret 0x10',
          'regra: cada uma das sete seções preliminares da Copa da Suécia (fases abaixo de 7) joga duas rodadas de jogo único: 30 clubes (fases pares) ou 28 (ímpares) em 27/7, e os 16 restantes em 27/8 com um ou dois clubes entrando ali; a chave principal (fase 7) tem seis rodadas: 64 clubes em 25/9, depois 29/3, 9/4, as quartas em 30/4, as semifinais em 7/5 do ano seguinte e a final em ida e volta a partir de 21/5; os prêmios das rodadas são uma parte do fundo (−1); categoria 4 até as oitavas, 6 depois',
          "rule: each of the Swedish Cup's seven preliminary sections (stages below 7) plays two single-match rounds: 30 clubs (even stages) or 28 (odd stages) on 27/7, and the 16 left on 27/8 with one or two clubs entering there; the main draw (stage 7) has six rounds: 64 clubs on 25/9, then 29/3, 9/4, the quarter-finals on 30/4, the semi-finals on 7/5 of the next year and the two-legged final from 21/5; the round prizes a share of the fund (−1); category 4 up to the round of 16, 6 after")])
CAMPOS = [('copa da Suécia +0xb2 ponteiro: a reserva de clubes (as seções); +0xb6 palavra: o tamanho (244)', 'Swedish Cup +0xb2 pointer: the club pool (the sections); +0xb6 word: its size (244)')]
INTERESSA = []
