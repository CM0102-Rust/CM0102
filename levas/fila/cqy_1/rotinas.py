# O calendário da Série C (o slot create_schedule, a virtual +0x3c, de bra_nat_third.cpp; leitor cqy, 12/11/2026).
# Roda 20 vezes na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): os grupos da primeira fase na
# criação da carreira e quando a temporada vira, a fase 7, os grupos da terceira fase (8 e 9) e a final (10) quando
# chegam. No mesmo molde dos calendários da cqw_4 (Série A e B): conforme a fase, grava as saídas (é_liga se o ponteiro
# não é nulo, as rodadas e o extra), aloca a tabela (sem memória, a caixa e 0) e grava as datas e os jogos de cada
# rodada; fase acima de 10: 0, sem gravar nada. Diferente das irmãs, o extra das fases de grupos é a fase + 0x420
# (primeira fase) ou + 0x476 (terceira), a palavra do byte com sinal (movsx dx, al; add edx, imm): a fase de cada
# grupo vai para a tabela. As operações foram extraídas seguindo o fluxo da original com cada fase de −128 a 127
# (o fases.py do agente C, com o add de registro), e as fases −128..6, 8..9 dão as mesmas operações.
# - 0x42d620 BraNatThird::create_schedule (bra_nat_third.cpp), igual no cm0102-1080.exe (inteira).
# Convenções conferidas:
# - a rotina, a virtual +0x3c (de League::create_fixtures 0x6835c0, dos grupos e da final 0x42e980: push &é_liga;
#   push &extra; push &rodadas; push fase; mov ecx, liga; call [vtable + 0x3c]) -> thiscall, ret 0x10;
# - 0x944e46 (malloc): push tamanho; call; add esp, 4 -> cdecl;
# - 0x68a160 (a data da rodada) e 0x68a1c0 (o jogo da rodada), 0x521e60 (o sorteio), 0x521eb0 (o jogo da copa): 9, 7, 7 e
#   9 pushes; call; add esp -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290; fonte 0x98aba4, linhas 0xbb, 0x10d, 0x136, 0x15d): show_error_at,
#   pelo create_round_table.
# Comparador: lado a lado (asia_club_champ_compare_round_table, o mesmo da cqw_4): a original com as saídas de quem
# chama, a cópia com saídas próprias que começam com os mesmos valores, a partir do mesmo [0xb67a34]; as saídas e os
# bytes gravados das duas tabelas iguais; a tabela da cópia é liberada. Na fase sem tabela, só a original (devolve 0).
# Determinismo: só constantes, a fase e o ano da liga.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_nat_third_create_schedule', 0x42d620, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Série C',
        'the Série C schedule',
        [('bra_nat_third_create_schedule', 'BraNatThird::create_schedule', '0x42d620', 'bra_nat_third.cpp', 'thiscall, ret 0x10',
          'O calendário da Série C: os grupos da primeira fase (fases até 6, é liga): 14 rodadas, de 5/jul a 7/set, extra = fase + 0x420; a fase 7 (mata-mata): 1 rodada, sorteio em 8/set e jogo em 14/set; os grupos da terceira fase (fases 8 e 9, é liga): 6 rodadas, de 28/set a 2/nov, extra = fase + 0x476; a fase 10 (a final): 1 rodada, sorteio em 3/nov e jogo em 9/nov; acima de 10, nada (0); cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação',
          'The Série C schedule: the first-round groups (stages up to 6, league): 14 rounds, from 5 Jul to 7 Sep, extra = stage + 0x420; stage 7 (knockout): 1 round, draw on 8 Sep and match on 14 Sep; the third-round groups (stages 8 and 9, league): 6 rounds, from 28 Sep to 2 Nov, extra = stage + 0x476; stage 10 (the final): 1 round, draw on 3 Nov and match on 9 Nov; above 10, nothing (0); each round has the date (day, month, years after the season one, weekday and code) and the matches, and the rest of the table is left as allocated')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada de liga, 0x68 por rodada de mata-mata): +0 o dia do ano, +2 os anos depois do da liga, +4 o código, +5 os jogos; grava', 'round table (0x41 bytes per league round, 0x68 per knockout round): +0 day of year, +2 years after the league one, +4 code, +5 the matches; writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads')]
INTERESSA = ['CMRTE']
