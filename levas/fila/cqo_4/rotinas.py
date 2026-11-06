# O calendário das ligas francesas de baixo: o National (a terceira divisão), a segunda divisão e a divisão de baixo
# (de fundo) (Agente B da frente das competições, 05/11/2026). O mesmo slot (16, +0x3c da vtable, `create_schedule`) em
# três ligas: a tabela das 38 rodadas da temporada (0x41 bytes cada: a data e as faixas de TV). Rodam uma vez cada na
# criação da carreira de 8 ligas, chamadas pelas regras de cada liga (docs/cobertura_competicoes.txt: 1 vez cada):
# - 0x5bfe30 FraThird::create_schedule (1.948 bytes);
# - 0x5be620 FraSecond::create_schedule (2.275 bytes);
# - 0x5bd920 FraLower::create_schedule (2.311 bytes).
# Iguais no cm0102-1080.exe (inteiras). Sem quadro SEH; o corpo das três é o mesmo, só mudam a fonte, a linha e as
# chamadas (constantes, extraídas da desmontagem com um leitor capstone e conferidas: em cada rodada, uma data e as
# faixas em sequência, todas com o mesmo ponteiro da tabela, código 2 e valor 0).
# Convenções conferidas:
# - quem chama (a virtual +0x3c, nas regras: 0x5ba0a0 e as das outras ligas): push 0; push &+0x3a; push &+0xa9;
#   push −1; mov ecx, esi -> thiscall, ret 0x10 (o `ret 0x10` das três conferido);
# - 0x944e46 (malloc): push n; add esp, 4 -> cdecl;
# - 0x68a160 (league_round_set_date): 9 pushes (tabela, i, dia, mês, anos, semana, 2, ano +0x40, 0); add esp -> cdecl;
# - 0x68a1c0 (league_round_set_match_slot): 7 pushes (tabela, i, j, a, b, c, 0); add esp -> cdecl;
# - a caixa: aviso_em (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# O ano empurrado é o eax inteiro com só o ax carregado de +0x40 (bytes altos de sobra); a 0x68a160 soma os anos e passa
# o ano à Date::Date, que só lê a palavra, e grava ano − base como palavra: os bytes altos não mudam nada; a cópia passa
# o ano limpo.
# Comparador: aloca: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias, o mesmo
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas (as datas, +0x3d e as faixas até a que fecha a lista)
# iguais; a tabela da cópia é solta.
# Determinismo: só constantes do exe e o ano da liga.
# Provas (05-06/11/2026, cópia fora do projeto: o motor.rs da principal com a 277ª e a 278ª + os 9 itens cqo_4..6,
# cqp_4..6, cqq_4..6, 17 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151; inclui os play-offs e o
#   desempate de maio): 0 linhas DIVERGENCIA, 0 EXCECAO (motor 10^10, data 10^9, aleatorio 10^9 comparações, 0
#   divergências); todos os ramos marcados rodaram;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base): todas ACUSADAS:
#   cqo4_terceira_primeira_rodada_em_outro_dia: divergiu da base em 2001-213;
#   cqo4_segunda_primeira_rodada_em_outro_dia: divergiu da base em 2001-181;
#   cqo4_baixo_primeira_rodada_em_outro_dia: divergiu da base em 2001-208;
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_da_terceira_francesa', 0x5bfe30, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_segunda_francesa', 0x5be620, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_divisao_de_baixo_francesa', 0x5bd920, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das ligas francesas de baixo: o National, a segunda divisão e a divisão de baixo (as 38 rodadas)',
 'the schedule of the lower French leagues: the National, the second division and the lower division (the 38 rounds)',
 [('calendario_da_terceira_francesa', 'FraThird::create_schedule', '0x5bfe30', 'fra_third.cpp', 'thiscall, ret 0x10',
   'o calendário do National: 38 rodadas (turno e returno de 20 clubes), de 7 de agosto a 20 de maio do ano seguinte, '
   'quase todas no fim de semana e algumas no meio da semana; a 21ª rodada já em 7 de janeiro; nenhuma faixa de TV (nenhum jogo muda de dia); só a fase principal (as outras fases não têm calendário)',
   'the National schedule: 38 rounds (home and away with 20 clubs), from 7 August to 20 May of the next year, '
   'mostly at weekends and some midweek; round 21 already on 7 January; no TV slots (no match '
   'moves day); main stage only'),
  ('calendario_da_segunda_francesa', 'FraSecond::create_schedule', '0x5be620', 'fra_second.cpp', 'thiscall, ret 0x10',
   'o calendário da segunda divisão: 38 rodadas, de 31 de julho a 20 de maio; em 18 rodadas um jogo vai para uma faixa '
   'de TV (dia 5, turno 1) [inferência: o jogo televisionado]',
   'the second division schedule: 38 rounds, from 31 July to 20 May; in 18 rounds one match goes to a TV slot (day 5, '
   'slot 1) [inference: the televised match]'),
  ('calendario_da_divisao_de_baixo_francesa', 'FraLower::create_schedule', '0x5bd920', 'fra_lower.cpp',
   'thiscall, ret 0x10',
   'o calendário da divisão de baixo (de fundo): as mesmas 38 datas da segunda divisão, com a faixa de TV também nas '
   'duas últimas rodadas',
   'the lower division (background) schedule: the same 38 dates as the second division, with the TV slot also in the '
   'last two rounds')])
CAMPOS = [('tabela de rodadas (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código 2, +5 as 8 faixas de '
           '7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava (as três)',
           'round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code 2, +5 the 8 slots of 7 bytes '
           '(value, day, part, matches; −1 ends), +0x3d value; writes'),
          ('liga +0x40 ano da temporada; valor; lê (as três)', 'league +0x40 season year; value; reads')]
INTERESSA = ['CMRTE']
