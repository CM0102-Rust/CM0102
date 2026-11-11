# O calendário da Primera espanhola (Agente B da frente das competições, 06/11/2026): o slot 16 (+0x3c da vtable, `create_schedule`), a
# tabela das rodadas da temporada (0x41 bytes cada). Rodam na carreira de 8 ligas:
# - 0x84e450 SpaFirst::create_schedule (6.107 bytes, só a fase principal; 1 vez).
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
#   cqp6_primera_impar_primeira_rodada_em_outro_dia: divergiu da base em 2001-203;
#   cqp6_primera_impar_natal_deslocado: divergiu da base em 2001-350;
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_da_primera', 0x84e450, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Primera espanhola (38 rodadas, tabela do ano par e do ímpar, o Natal)',
 'the schedule of the Spanish Primera (38 rounds, even and odd year tables, Christmas)',
 [('calendario_da_primera', 'SpaFirst::create_schedule', '0x84e450', 'spa_first.cpp', 'thiscall, ret 0x10',
   'o calendário da Primera: 38 rodadas (20 clubes), quase todas no domingo (dia 6) com uma faixa de TV (um jogo no dia 5); duas tabelas, pela paridade do ano: no ano ímpar (2001/02) de 22 de agosto a 21 de maio, no par de 30 de agosto a 6 de junho; em cada uma, uma rodada de Natal com data fixa (entre 20 e 30 de dezembro, ou 3 de janeiro) escolhida pelo dia em que cai a rodada de meados de dezembro',
   'the Primera schedule: 38 rounds (20 clubs), nearly all on Sunday (day 6) with one TV slot (one match on day 5); two tables, by the parity of the year: in odd years (2001/02) from 22 August to 21 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round (between 20 and 30 December, or 3 January) chosen by the day the mid-December round falls on')])
CAMPOS = [('tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] '
           'o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da '
           'rodada de referência do Natal)',
           'league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time '
           'of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads'),
          ('rodada de copa (0x68 bytes); grava (os play-offs)',
           'cup round (0x68 bytes); writes (the play-offs)'),
          ('liga +0x40 ano da temporada; valor; lê', 'league +0x40 season year; value; reads')]
INTERESSA = ['CMRTE']
