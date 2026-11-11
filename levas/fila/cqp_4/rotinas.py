# O calendário da segunda e da terceira divisões inglesas (Division Two e Division Three) (Agente B da frente das competições, 06/11/2026): o slot 16 (+0x3c da vtable, `create_schedule`), a
# tabela das rodadas da temporada (0x41 bytes cada). Rodam na carreira de 8 ligas:
# - 0x5757a0 EngSecond::create_schedule e 0x5770e0 EngThird::create_schedule (3.692 bytes cada; o mesmo código, só
#   mudam a fonte, as linhas e a data da final dos play-offs); 1 vez cada na criação e [inferência] mais uma nos
#   play-offs de maio de 2002 (docs/cobertura_competicoes.txt: 2 vezes cada).
# Iguais no cm0102-1080.exe (inteiras). Sem quadro SEH. As chamadas (constantes) e os caminhos (a fase, a paridade do
# ano e os 7 casos do `switch` do Natal mais o padrão) seguidos por um emulador simbólico da desmontagem (capstone),
# que acompanha a pilha empurrada (os casos do Natal saltam para o meio dos `push` uns dos outros); cada caminho
# refeito pelas tabelas (o começo comum, o meio de cada caso e o fim comum) foi conferido contra o emulador.
# Convenções conferidas:
# - quem chama (a virtual +0x3c): na montagem, push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, esi -> thiscall,
#   ret 0x10 (o `ret 0x10` conferido em todos os caminhos);
# - 0x944e46 (malloc): push n; add esp, 4 -> cdecl;
# - 0x68a160: 9 pushes (tabela, i, dia, mês, anos, semana, código, ano, 0); 0x68a1c0: 7 pushes (tabela, i, j, a, b, c,
#   0); add esp -> cdecl;
# - 0x549f70 (Date::Date): push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10 (a data de 1º de
#   dezembro; a palavra +0 é o dia do ano, 0 = 1º de janeiro);
# - a caixa: aviso_em (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# O ano das datas é o eax/ecx com só o ax/cx de +0x40 (os bytes altos não mudam nada: a data só lê a palavra).
# O Natal: depois das rodadas do começo, k = (dia da rodada de referência − dia de 1º/dez + 1, em 16 bits) − ajuste
# (o dia de dezembro em que a rodada de referência caiu depois de levada ao dia da semana, menos o ajuste); de 0 a 6
# escolhe as rodadas do Natal; fora disso a caixa e o caso padrão (não acontece: a data levada ao dia da semana fica a
# ±3 dias). Dia da semana: 0 = segunda (Date.cpp, a contagem desde 1600 com o dia 0 = 1º de janeiro).
# Comparador: aloca: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias, o mesmo
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas iguais (as rodadas de liga: as datas, +0x3d e as faixas
# até a que fecha a lista; as de copa dos play-offs: +0..+0x10, +0x13..+0x22, +0x5c..+0x67); a tabela da cópia é
# solta.
# Determinismo: só constantes do exe, o ano e as datas calculadas pela Date::Date.
# Provas (05-06/11/2026, cópia fora do projeto: o motor.rs da principal com a 277ª e a 278ª + os 9 itens cqo_4..6,
# cqp_4..6, cqq_4..6, 17 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151; inclui os play-offs e o
#   desempate de maio): 0 linhas DIVERGENCIA, 0 EXCECAO (motor 10^10, data 10^9, aleatorio 10^9 comparações, 0
#   divergências); todos os ramos marcados rodaram;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base): todas ACUSADAS:
#   cqp4_segunda_primeira_rodada_em_outro_dia: divergiu da base em 2001-213;
#   cqp4_terceira_natal_deslocado: divergiu da base em 2001-354;
#   cqp4_semifinais_da_segunda_uma_semana_antes: divergiu da base em 2002-128;
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_da_segunda_inglesa', 0x5757a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('calendario_da_terceira_inglesa', 0x5770e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da segunda e da terceira divisões inglesas (46 rodadas, o Natal e os play-offs)',
 'the schedule of the English second and third divisions (46 rounds, Christmas and the play-offs)',
 [('calendario_da_segunda_inglesa', 'EngSecond::create_schedule', '0x5757a0', 'eng_second.cpp', 'thiscall, ret 0x10',
   'o calendário da Division Two: 46 rodadas (24 clubes), de 12 de agosto a 6 de maio, quase todas no mesmo dia da semana (5) e algumas no meio da semana; no Natal, quatro rodadas de datas fixas (de 20 de dezembro a 5 de janeiro) escolhidas pelo dia em que cai a rodada de 16 de dezembro. Os play-offs de acesso (fase 0): semifinais de ida e volta (4 clubes, sorteio em 7 de maio, ida em 12, 4 dias entre os jogos) e a final em jogo único em 27 de maio [inferência: o mando 4 é o estádio neutro]',
   'the Division Two schedule: 46 rounds (24 clubs), from 12 August to 6 May, nearly all on the same weekday (5) and some midweek; at Christmas, four fixed-date rounds (20 December to 5 January) chosen by the day the 16 December round falls on. The promotion play-offs (stage 0): two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 27 May [inference: venue 4 is the neutral ground]'),
  ('calendario_da_terceira_inglesa', 'EngThird::create_schedule', '0x5770e0', 'eng_third.cpp', 'thiscall, ret 0x10',
   'o calendário da Division Three: as mesmas 46 rodadas da Division Two. Os play-offs de acesso (fase 0): semifinais de ida e volta (4 clubes, sorteio em 7 de maio, ida em 12, 4 dias entre os jogos) e a final em jogo único em 26 de maio [inferência: o mando 4 é o estádio neutro]',
   'the Division Three schedule: the same 46 rounds as Division Two. The promotion play-offs (stage 0): two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 26 May [inference: venue 4 is the neutral ground]')])
CAMPOS = [('tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] '
           'o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da '
           'rodada de referência do Natal)',
           'league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time '
           'of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads'),
          ('rodada de copa (0x68 bytes); grava (os play-offs)',
           'cup round (0x68 bytes); writes (the play-offs)'),
          ('liga +0x40 ano da temporada; valor; lê', 'league +0x40 season year; value; reads')]
INTERESSA = ['CMRTE']
