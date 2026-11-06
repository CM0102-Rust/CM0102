# O calendário da CFA e da Série C1 italiana (as duas chaves) (Agente B da frente das competições, 05/11/2026). O mesmo
# slot (16, +0x3c da vtable, `create_schedule`) em três ligas: a tabela das rodadas da temporada (0x41 bytes cada) e,
# na C1, as fases de copa dos play-offs (rodadas de 0x68 bytes). Rodam na carreira de 8 ligas
# (docs/cobertura_competicoes.txt: 0x5b9510 1 vez, 0x64ad80 e 0x64ef30 3 vezes cada: a fase principal na criação e
# [inferência] os play-offs e os play-outs no fim da temporada, em maio de 2002):
# - 0x5b9510 FraCfa::create_schedule (2.703 bytes): só a fase principal;
# - 0x64ad80 ItaSerC1a::create_schedule e 0x64ef30 ItaSerC1b::create_schedule (2.279 bytes cada; o mesmo código, só
#   mudam a fonte e as linhas das caixas).
# Iguais no cm0102-1080.exe (inteiras). Sem quadro SEH. As chamadas da fase principal (constantes) extraídas da
# desmontagem com um leitor capstone e conferidas (uma data e as faixas por rodada, em sequência); as fases de copa da
# C1 pela mesma `cql10_rodada` da 276ª (as datas por 0x521e60/0x521eb0 e os campos), cada campo conferido.
# Convenções conferidas:
# - quem chama (a virtual +0x3c): na montagem, push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, esi -> thiscall,
#   ret 0x10 (o `ret 0x10` das três conferido em todos os caminhos);
# - 0x944e46 (malloc): push n; add esp, 4 -> cdecl;
# - 0x68a160: 9 pushes (tabela, i, dia, mês, anos, semana, código 1 ou 2, ano, 0); add esp -> cdecl;
# - 0x68a1c0: 7 pushes (tabela, i, j, a, b, c, 0); add esp -> cdecl;
# - 0x521e60: 7 pushes (tabela, i, dia, mês, anos, semana, ano com sinal); 0x521eb0: 9 pushes (tabela, i, dia, mês,
#   anos, semana, turno, ano, mando); add esp -> cdecl;
# - a caixa: aviso_em (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# Na fase principal o ano é o eax com só o ax de +0x40 (os bytes altos não mudam nada: a data só lê a palavra); nas
# fases de copa, movsx.
# Comparador: aloca: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias, o mesmo
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas iguais (a fase principal: as datas, +0x3d e as faixas até
# a que fecha a lista; as fases de copa: +0..+0x10, +0x13..+0x22, +0x5c..+0x67); a tabela da cópia é solta.
# Determinismo: só constantes do exe e o ano da liga.
# Provas (05-06/11/2026, cópia fora do projeto: o motor.rs da principal com a 277ª e a 278ª + os 9 itens cqo_4..6,
# cqp_4..6, cqq_4..6, 17 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151; inclui os play-offs e o
#   desempate de maio): 0 linhas DIVERGENCIA, 0 EXCECAO (motor 10^10, data 10^9, aleatorio 10^9 comparações, 0
#   divergências); todos os ramos marcados rodaram;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base): todas ACUSADAS:
#   cqo5_cfa_primeira_rodada_em_outro_dia: divergiu da base em 2001-209;
#   cqo5_c1_primeira_rodada_em_outro_dia: divergiu da base em 2001-230;
#   cqo5_semifinais_uma_semana_antes: divergiu da base em 2002-132;
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_da_cfa', 0x5b9510, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_c1_a', 0x64ad80, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('calendario_da_c1_b', 0x64ef30, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da CFA e da Série C1 italiana: as rodadas da temporada e os play-offs de acesso',
 'the schedule of the CFA and of the Italian Serie C1: the season rounds and the promotion play-offs',
 [('calendario_da_cfa', 'FraCfa::create_schedule', '0x5b9510', 'fra_cfa.cpp', 'thiscall, ret 0x10',
   'o calendário da CFA (a quarta divisão francesa, de fundo, 4 grupos de 18): 34 rodadas, de 31 de julho a 20 de maio '
   'do ano seguinte; em 25 rodadas dois jogos vão para faixas de TV (a maioria no dia 6 ou 4, turnos 1 e 2); só a fase '
   'principal',
   'the CFA schedule (the French fourth level, background, 4 groups of 18): 34 rounds, from 31 July to 20 May of the '
   'next year; in 25 rounds two matches go to TV slots (mostly day 6 or 4, slots 1 and 2); main stage only'),
  ('calendario_da_c1_a', 'ItaSerC1a::create_schedule', '0x64ad80', 'ita_ser_c1a.cpp', 'thiscall, ret 0x10',
   'o calendário da Série C1 (cada chave de 18): 34 rodadas de 5 de setembro a 7 de maio, quase todas no mesmo dia da '
   'semana (6) e duas no meio da semana; sem faixas de TV. Os play-offs de acesso (fase 0): semifinais de ida e volta '
   'em 21 e 28 de maio (4 clubes, sem sorteio) e a final em 4 e 11 de junho; os play-outs (fase 1) [inferência: o '
   'rebaixamento]: 2 confrontos de ida e volta nas datas das semifinais, sem sorteio',
   'the Serie C1 schedule (each group of 18): 34 rounds from 5 September to 7 May, nearly all on the same weekday (6) '
   'and two midweek; no TV slots. The promotion play-offs (stage 0): two-legged semi-finals on 21 and 28 May (4 clubs, '
   'no draw) and the final on 4 and 11 June; the play-outs (stage 1) [inference: relegation]: 2 two-legged ties on the '
   'semi-final dates, no draw'),
  ('calendario_da_c1_b', 'ItaSerC1b::create_schedule', '0x64ef30', 'ita_ser_c1b.cpp', 'thiscall, ret 0x10',
   'o mesmo para a outra chave da C1 (a mesma tabela)', 'the same for the other Serie C1 group (the same table)')])
CAMPOS = [('tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] '
           'o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava',
           'league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time '
           'of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes'),
          ('rodada de copa (0x68 bytes, docs/.md §2.4): as datas, +7 nome, +9 índice, '
           '+0xb..+0xf bits e desempates, +0x17 categoria, +0x18..+0x1e times, +0x20..+0x22 confronto, +0x5c..+0x64 '
           'prêmios; grava (as da C1)',
           'cup round (0x68 bytes): dates, +7 name, +9 index, +0xb..+0xf bits and tie-breaks, +0x17 category, '
           '+0x18..+0x1e teams, +0x20..+0x22 tie, +0x5c..+0x64 prizes; writes'),
          ('liga +0x40 ano da temporada; valor; lê', 'league +0x40 season year; value; reads')]
INTERESSA = ['CMRTE']
