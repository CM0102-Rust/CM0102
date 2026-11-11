# O calendário das primeiras divisões da Itália e da França e da Série B italiana (Agente B da frente das competições,
# 05/11/2026). O mesmo slot (16, +0x3c da vtable, `create_schedule`) em três ligas: a tabela das rodadas da temporada
# (0x41 bytes cada) e, nas italianas, a fase 0, o jogo-desempate (uma rodada de copa de 0x68 bytes). Rodam uma vez cada
# na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt: 1 vez cada):
# - 0x6409b0 ItaSerA::create_schedule (2.901 bytes) e 0x645af0 ItaSerB::create_schedule (3.215 bytes): o mesmo corpo,
#   mudam a tabela, as datas do desempate, a fonte e as linhas;
# - 0x5bb690 FraFirst::create_schedule (3.090 bytes): 34 rodadas em 2001 (18 clubes), 38 depois (20 clubes); as 4
#   rodadas a mais entram no meio (jan-mar) e o índice segue em sequência; no fim confere a conta (a caixa, sem parar).
# Iguais no cm0102-1080.exe (inteiras). Sem quadro SEH. As chamadas (constantes) extraídas da desmontagem com um leitor
# capstone e conferidas (uma data e as faixas por rodada, em sequência; na francesa, os blocos condicionais `cmp bp,
# 0x26; jne` marcados um a um); o desempate pela mesma `cql10_rodada` da 276ª, cada campo conferido.
# Convenções conferidas:
# - quem chama (a virtual +0x3c): na montagem, push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, esi -> thiscall,
#   ret 0x10 (o `ret 0x10` das três conferido em todos os caminhos);
# - 0x944e46 (malloc): push n; add esp, 4 -> cdecl;
# - 0x68a160: 9 pushes (tabela, i, dia, mês, anos, semana, código 1 ou 2, ano, 0); add esp -> cdecl;
# - 0x68a1c0: 7 pushes (tabela, i, j, a, b, c, 0); add esp -> cdecl;
# - 0x521e60: 7 pushes; 0x521eb0: 9 pushes; add esp -> cdecl (o ano com sinal, movsx);
# - a caixa: aviso_em (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# O ano das datas da fase principal é o eax com só o ax de +0x40 (os bytes altos não mudam nada).
# Comparador: aloca: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias, o mesmo
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas iguais (a fase principal: as datas, +0x3d e as faixas até
# a que fecha a lista; o desempate: +0..+0x10, +0x13..+0x22, +0x5c..+0x67); a tabela da cópia é solta.
# Determinismo: só constantes do exe, o ano e o +0xee da liga.
# Provas (05-06/11/2026, cópia fora do projeto: o motor.rs da principal com a 277ª e a 278ª + os 9 itens cqo_4..6,
# cqp_4..6, cqq_4..6, 17 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151; inclui os play-offs e o
#   desempate de maio): 0 linhas DIVERGENCIA, 0 EXCECAO (motor 10^10, data 10^9, aleatorio 10^9 comparações, 0
#   divergências); todos os ramos marcados rodaram;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base): todas ACUSADAS:
#   cqo6_serie_a_primeira_rodada_em_outro_dia: divergiu da base em 2001-210;
#   cqo6_serie_b_primeira_rodada_em_outro_dia: divergiu da base em 2001-210;
#   cqo6_francesa_primeira_rodada_em_outro_dia: divergiu da base em 2001-181;
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_da_serie_a', 0x6409b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_serie_b', 0x645af0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_primeira_francesa', 0x5bb690, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Série A, da Série B e da primeira divisão francesa (as rodadas e o jogo-desempate italiano)',
 'the schedule of the Serie A, the Serie B and the French first division (the rounds and the Italian play-off match)',
 [('calendario_da_serie_a', 'ItaSerA::create_schedule', '0x6409b0', 'ita_ser_a.cpp', 'thiscall, ret 0x10',
   'o calendário da Série A: 34 rodadas (18 clubes), de 28 de agosto a 12 de maio do ano seguinte, todas no mesmo dia '
   'da semana (6) menos duas (6 de janeiro, 30 de março); em quase todas, dois jogos vão para cada uma de duas faixas '
   'de TV (dias 6 e 5, turno 2) [inferência: os jogos televisionados de sábado e domingo à noite]. A fase 0, o '
   'jogo-desempate (spareggio) [inferência: clubes empatados em pontos numa vaga]: ida e volta com 3 dias entre os '
   'jogos, sorteio em 13 e jogo em 19 de maio, sem sorteio dos confrontos, tantos confrontos quanto o byte +0xee',
   'the Serie A schedule: 34 rounds (18 clubs), from 28 August to 12 May of the next year, all on the same weekday (6) '
   'except two (6 January, 30 March); in nearly all, two matches go to each of two TV slots (days 6 and 5, slot 2) '
   '[inference: the televised weekend evening matches]. Stage 0, the play-off match (spareggio) [inference: clubs '
   'level on points for a place]: two legs 3 days apart, draw on 13 and match on 19 May, ties in fixed order, as many '
   'ties as the byte +0xee'),
  ('calendario_da_serie_b', 'ItaSerB::create_schedule', '0x645af0', 'ita_ser_b.cpp', 'thiscall, ret 0x10',
   'o calendário da Série B: 38 rodadas (20 clubes), de 29 de agosto a 14 de maio, no mesmo dia da semana (6) menos '
   'uma (6 de janeiro); em quase todas, dois jogos em cada uma de duas faixas de TV (dias 4 e 0, turno 2). O '
   'jogo-desempate como na Série A, em 15 e 21 de maio',
   'the Serie B schedule: 38 rounds (20 clubs), from 29 August to 14 May, on the same weekday (6) except one (6 '
   'January); in nearly all, two matches in each of two TV slots (days 4 and 0, slot 2). The play-off match as in the '
   'Serie A, on 15 and 21 May'),
  ('calendario_da_primeira_francesa', 'FraFirst::create_schedule', '0x5bb690', 'fra_first.cpp',
   'thiscall, ret 0x10',
   'o calendário da primeira divisão francesa: em 2001/02, 34 rodadas (18 clubes) de 31 de julho a 20 de maio; a '
   'partir de 2002/03, 38 rodadas (20 clubes), com quatro datas a mais em janeiro e março (8 e 22 de janeiro, 4 e 18 '
   'de março); quase todas com dois jogos em faixas de TV (dias 6, 4 ou 1, turnos 1 e 2); só a fase principal',
   'the French first division schedule: in 2001/02, 34 rounds (18 clubs) from 31 July to 20 May; from 2002/03, 38 '
   'rounds (20 clubs), with four extra dates in January and March (8 and 22 January, 4 and 18 March); nearly all with '
   'two matches in TV slots (days 6, 4 or 1, slots 1 and 2); main stage only')])
CAMPOS = [('tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] '
           'o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava',
           'league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time '
           'of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes'),
          ('rodada de copa (0x68 bytes); grava (o desempate)',
           'cup round (0x68 bytes); writes (the play-off match)'),
          ('liga +0x40 ano da temporada, +0xee [inferência] os confrontos de desempate (byte com sinal); valor; lê',
           'league +0x40 season year, +0xee [inference] the play-off ties (signed byte); value; reads')]
INTERESSA = ['CMRTE']
