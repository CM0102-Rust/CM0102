# As tabelas de rodadas das eliminatórias da CONCACAF e da Europa para a Copa do Mundo (wc_concacaf_cup.cpp,
# wc_europe_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação, LIGAS = 'ENG', ATE = '2002-08-01' (como o
# mwg_13): a da CONCACAF roda na criação (fase -1, a campanha de 2000) e na virada da seção entre 2002-06-01 e
# 2002-08-01 (fase -1 de novo e os dois grupos da América Central, fases 0 e 1); a da Europa na criação e na virada
# (cobertura_mundo: 4 e 2 chamadas).
# - 0x914de0 WcConcacafCup::create_round_table thiscall(fase, &rodadas, &extra, &é_liga) ret 0x10 (método +0x3c):
#   fase -1 o mata-mata caribenho (3 rodadas de copa, 0x138 bytes, linha 0xd4; os campos de cada rodada dependem do
#   byte +0xbb, isentas caribenhas, e do ano 2000, regra de sorteio 0x200); fases 0, 1 e abaixo de -1 um grupo da
#   América Central (liga, 2 rodadas com o byte +0xbc = 2, ou = 1 na fase 1; senão 6; rótulo 0x473 + (fase != 0); linha
#   0x136); 2 a repescagem (copa, 1 rodada, 0x68, linha 0x162); 3 a 5 os grupos semifinais (6 rodadas, rótulo 0x409 +
#   fase, 0x186, 0x18c); 6 o grupo final (10 rodadas no ano seguinte, rótulo 0x96, 0x28a, 0x1b5); acima, 0 sem escrever.
# - 0x9185c0 WcEuropeLeague::create_schedule thiscall(fase, &rodadas, &extra, &é_liga) ret 0x10: fase (byte com sinal)
#   até o último grupo (+0xff) um grupo de 10 rodadas (rótulo 0x3e9 + fase, 0x28a, linha 0xdb); a fase dos segundos
#   (+0x100) e as que não são a repescagem (+0x101): 0 sem escrever; a repescagem uma rodada de copa (0x68, linha 0x115)
#   com 8 times (4 jogos) se o byte +0xf6 (isentas) for 0 ou 1, senão 6 (3).
# Pela máquina das tabelas de fase da Copa América (build_phase_table; a Europa com as tabelas EURO_QUAL_* do cx06_4,
# as mesmas datas das eliminatórias da Eurocopa; as datas são chamadas da original 0x68a160,
# 0x68a1c0, 0x521e60, 0x521eb0, cdecl) e os campos que dependem do objeto escritos depois
# (build_phase_table_with_fields: cada chamada só toca a sua rodada e a original escreve os campos depois das chamadas
# da rodada). Convenções conferidas na desmontagem e nos pontos de chamada (push 0; push &extra; push &rodadas; push
# fase; mov ecx; call [vt+0x3c]). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: lado a lado (as saídas e os bytes escritos nas duas tabelas; a da cópia é liberada).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('wc_concacaf_cup_create_round_table', 0x914de0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_europe_league_create_schedule', 0x9185c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas de rodadas das eliminatórias da CONCACAF e da Europa',
        'the round tables of the CONCACAF and European qualifying',
        [('wc_concacaf_cup_create_round_table', 'WcConcacafCup::create_round_table', '0x914de0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0x10',
          'regra: as eliminatórias da CONCACAF têm um mata-mata caribenho de três rodadas de ida e volta (janeiro a abril: 24 times menos as caribenhas isentas, depois 12 com as isentas entrando, depois 6), dois grupos da América Central (fevereiro a abril, seis rodadas, ou duas quando a América Central tem isentas), uma repescagem (maio), três grupos semifinais de seis rodadas (junho a outubro) e um grupo final de dez rodadas no ano seguinte (fevereiro a outubro)',
          'rule: the CONCACAF qualifying has a Caribbean knockout of three two-legged rounds (January to April: 24 teams less the exempt Caribbean nations, then 12 with the exempt joining, then 6), two Central American groups (February to April, six rounds, or two when Central America has exempt nations), a play-off (May), three semi-final groups of six rounds (June to October) and a final group of ten rounds the next year (February to October)'),
         ('wc_europe_league_create_schedule', 'WcEuropeLeague::create_schedule', '0x9185c0', 'wc_europe_league.cpp', 'thiscall, ret 0x10',
          'regra: os grupos das eliminatórias europeias jogam dez rodadas de setembro do ano do sorteio a setembro do seguinte; a repescagem é uma rodada de ida e volta em novembro do ano seguinte, com oito seleções quando há no máximo uma isenta, senão seis',
          'rule: the European qualifying groups play ten rounds from September of the draw year to September of the next; the play-off is one two-legged round in November of the next year, with eight nations when at most one is exempt, else six')])
CAMPOS = [('eliminatórias da Europa +0xff último grupo, +0x100 fase dos segundos, +0x101 fase da repescagem, +0xf6 isentas (bytes); eliminatórias da CONCACAF +0xbb isentas caribenhas, +0xbc isentas das outras (bytes); valor; lê',
           'European qualifying +0xff last group, +0x100 runners-up stage, +0x101 play-off stage, +0xf6 exempt (bytes); CONCACAF qualifying +0xbb exempt Caribbean, +0xbc exempt others (bytes); value; reads')]
