# A Copa do Brasil: as rodadas, a tabela dos times e as regras dos jogos (bra_cup.cpp). Frente das competições (Agente
# D, 05/11/2026), carreira de 8 ligas (o Brasil jogável).
# - 0x427700 (BraCup::create_round_table, 1305 bytes; 2 chamadas até 2002-06-01 com as 8 ligas: a montagem e a virada
#   da temporada): as 7 rodadas. Uma mutação.
# - 0x427c20 (BraCup::build_team_table, 227 bytes; 2 chamadas: a montagem e a virada): as 71 vagas a partir dos
#   inscritos (+0x14). Uma mutação.
# - 0x427ec0 (BraCup::set_up_rounds, 50 bytes; 1 chamada): reservas, substituições, fundo de prêmios e a tabela das
#   rodadas. Uma mutação.
# Reaproveita do motor.rs os ajudantes da 276a (cql10_tabela, cql10_comparar, cql10_gravados, Cql10R): a tabela das
# rodadas tem o mesmo corpo das copas inglesas (extraída da desmontagem, rodada a rodada).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x427700 é a virtual +0x3c (push saida; push &extra; push &rodadas; push fase;
# mov ecx, copa -> thiscall, ret 0x10); 0x427c20 é thiscall, ret 0 (mov ecx, esi; call 0x427c20 na virada 0x427d10);
# 0x427ec0 é thiscall, ret 0 (sem argumentos). Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), a virtual +0x3c
# (push 0; push +0x3a; push +0x3c; push −1; mov ecx, esi -> thiscall, 4 argumentos), 0x521e60/0x521eb0 pela
# cql10_rodada e a caixa (aviso_em).
# Comparadores: lado a lado. A tabela das rodadas como a das copas inglesas (cql10_comparar). A tabela dos times: o
# +0x3e e o [0xb67a34]; a lista da cópia comparada nas 71 vagas e liberada. As regras: +0x28, +0x3a..+0x3d, +0x49/+0x4a
# e o [0xb67a34]; as duas tabelas das rodadas (+0xa3, cada uma pelo desvio da 0x427700) comparadas nos bytes gravados e
# a da cópia liberada.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (a 1a rodada
#   em março em 2001-035, as vagas de trás para a frente em 2001-042, uma substituição só em 2002-051: as regras rodam
#   na montagem da copa de 2002).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqo12_copa_do_brasil_tabela', 0x427700, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo12_times_da_copa', 0x427c20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo12_regras_da_copa', 0x427ec0, [0x56, 0x8b, 0xf1, 0x6a, 0x00])]
DOCS = ('a Copa do Brasil: as rodadas, a tabela dos times e as regras dos jogos',
        'the Brazilian Cup: the rounds, the team table and the match rules',
        [('cqo12_copa_do_brasil_tabela', 'BraCup::create_round_table', '0x427700', 'bra_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa do Brasil tem 7 rodadas em ida e volta, todas no ano da copa: 1ª sorteada em 18/1 e jogada em 23/2 (54 times, 27 confrontos), 2ª sorteio 9/3, jogo 22/3 (34 times: os 27 que passaram e 7 que entram agora), 3ª 6/4 e 26/4 (22 times: 17 e 5 novos; volta 21 dias depois), 4ª 18/5 e 7/6 (16 times: 11 e 5 novos; volta 7 dias depois); os que entram depois são os isentos das primeiras rodadas [inferência: os clubes nas copas continentais], quartas 15/6 e 17/6, semifinal 22/6 e 24/6, final sorteio 29/6 e jogo em 1/7 (volta 4 dias depois nas três últimas); desempate de ida e volta 1 (gols fora [inferência]); categoria 4 nas duas primeiras e 6 depois; prêmio da final 1 milhão ao campeão e 800 mil ao vice',
          'rule: the Brazilian Cup has 7 two-legged rounds, all in the cup year: 1st drawn on 18/1 and played on 23/2 (54 teams, 27 ties), 2nd draw 9/3, match 22/3 (34 teams: the 27 through and 7 entering now), 3rd 6/4 and 26/4 (22 teams: 17 and 5 new; return leg 21 days later), 4th 18/5 and 7/6 (16 teams: 11 and 5 new; return 7 days later); the late entrants are those exempt from the early rounds [inference: the clubs in the continental cups], quarter-finals 15/6 and 17/6, semi-final 22/6 and 24/6, final draw 29/6 and match on 1/7 (return 4 days later in the last three); two-leg tie-break 1 (away goals [inference]); category 4 in the first two and 6 after; final prize 1 million to the winner and 800 thousand to the runner-up'),
         ('cqo12_times_da_copa', 'BraCup::build_team_table', '0x427c20', 'bra_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa do Brasil tem 71 vagas, uma por inscrito, na ordem da lista dos inscritos, todas com a classe 0 e a situação 0xff (ainda sem jogo [inferência])',
          'rule: the Brazilian Cup has 71 places, one per entrant, in the entrant list order, all with class 0 and status 0xff (no match yet [inference])'),
         ('cqo12_regras_da_copa', 'BraCup::set_up_rounds', '0x427ec0', 'bra_cup.cpp', 'thiscall, ret 0',
          'regra: os jogos da Copa do Brasil têm 7 reservas no banco e 3 substituições; o fundo de prêmios é de 500 mil; monta a tabela das rodadas',
          'rule: Brazilian Cup matches have 7 substitutes on the bench and 3 substitutions; the prize fund is 500 thousand; builds the round table')])
CAMPOS = [('copa +0x14 lista dos inscritos (6 bytes: clube...); ponteiro; lê (times)',
           'cup +0x14 entrant list (6 bytes: club...); pointer; reads (team table)'),
          ('copa +0x3e times, +0xa7 tabela dos times (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (times)',
           'cup +0x3e teams, +0xa7 team table (6 bytes: club, +4 class, +5 status); value/pointer; writes (team table)'),
          ('copa +0x28 fundo de prêmios, +0x49 reservas, +0x4a substituições, +0x3c rodadas, +0x3a extra, +0xa3 tabela das rodadas; valor/ponteiro; grava (regras)',
           'cup +0x28 prize fund, +0x49 substitutes, +0x4a substitutions, +0x3c rounds, +0x3a extra, +0xa3 round table; value/pointer; writes (rules)'),
          ('copa +0x40 ano da copa; valor; lê (tabela das rodadas)',
           'cup +0x40 cup year; value; reads (round table)')]
INTERESSA = ['CMRTE']
