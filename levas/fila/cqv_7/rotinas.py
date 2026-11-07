# A Copa da Inglaterra (FA Cup): os 152 inscritos (eng_fa_cup.cpp, em cups\). Frente das competições (Agente D,
# 06/11/2026), carreira de 8 ligas.
# - 0x56fab0 (EngFaCup::collect_entrants, 2.260 bytes até o ret em 0x570383; o mapa diz 668, mas a rotina vai até lá):
#   chamada pelo dia da copa (0x570550, já em Rust) todo 3 de setembro; n = 1 até 2002-06-01 (a de 2001/02).
# Igual no cm0102-1080.exe (inteira).
# Convenções conferidas no ponto de chamada (lição da 258a): thiscall, ret 0; o método +0x58 da competição achada (push 0;
# push &lista; mov ecx, comp -> thiscall, ret 8; devolve quantos em ax); free 0x9452ca e malloc 0x944e46 (push; add esp,
# 4 -> cdecl); comp_util_get_latest_placed_club 0x4afd80 (push lugar; push registro, duas vezes, add esp, 0x10 -> cdecl);
# utils_random 0x90cfc0 (push n; o add esp vem junto com o da 0x4afbd0 ou depois das duas: cdecl; o resultado é lido como
# byte com sinal); comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (push vagas; push n; push lista; add esp, 0x14 com os
# dois sorteios -> cdecl; a palavra das vagas vai num edi com lixo em cima, que a rotina não lê); a caixa (aviso_em).
# Comparador lado a lado a partir do mesmo estado (o sorteador, +0x3e, +0xb3/+0xb4, [0xb67a34]): a tabela da cópia (sem
# contar os sorteios) comparada nas +0x3e vagas (o clube e os dois bytes zerados) e liberada; a devolução.
# Determinismo: a original não zera as duas listas da pilha (Conference e de fora da liga); só leria a sobra se houvesse
# menos de 38 ingleses de fora da liga (a carreira tem mais); a cópia usa 0, que a original também pula. Nada lê heap ou
# endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqv7_inscritos_da_copa_da_inglaterra', 0x56fab0, [0x81, 0xec, 0x34, 0x07, 0x00, 0x00])]
DOCS = ('a Copa da Inglaterra (FA Cup): os 152 inscritos',
        'the FA Cup: the 152 entrants',
        [('cqv7_inscritos_da_copa_da_inglaterra', 'EngFaCup::collect_entrants', '0x56fab0', 'eng_fa_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Inglaterra tem 152 inscritos, na ordem em que entram: 38 clubes ingleses de fora da liga (os de melhor reputação, com os de cima embaralhados), os 22 da Conference (dois sorteados vão para o fim), o campeão e o vice do FA Trophy da última temporada [inferência: a competição [0x9cf6a0]] (com a regra da nação ou em 2001; eles saem da lista deles e o número de vagas dela cai um para cada), as três divisões da Football League (a terceira, a segunda e a primeira, esta com 60 trocas sorteadas entre as suas 24 vagas), a Premier e, por fim, os ingleses de uma competição [0x9cf7bc] do mesmo ano (o Mundial de Clubes [inferência]); em 1999, com um só clube nela, ele sai da copa (+0xb4) e a copa fica com 151',
          'rule: the FA Cup has 152 entrants, in the order they enter: 38 English non-league clubs (the best by reputation, the top ones shuffled), the 22 Conference clubs (two drawn go to the end), the FA Trophy winner and runner-up of the last season [inference: competition [0x9cf6a0]] (with the nation rule or in 2001; they leave their own list and its number of places drops by one each), the three Football League divisions (the third, the second and the first, this one with 60 drawn swaps among its 24 places), the Premier League and, last, the English clubs of a competition [0x9cf7bc] of the same year (the Club World Championship [inference]); in 1999, with a single club in it, that club leaves the cup (+0xb4) and the cup has 151')])
CAMPOS = [('copa +0x3e quantos times (152, menos um com +0xb4 = 1), +0xa7 tabela dos inscritos (6 bytes: o clube, +4 e +5 zerados), +0xb3 quantos da competição [0x9cf7bc] entram no fim, +0xb4 a dispensa de 1999, +0xb2 a escolha do humano em 1999 [inferência]; ponteiro/valor; grava e lê',
           'cup +0x3e team count (152, one less with +0xb4 = 1), +0xa7 entrant table (6 bytes: the club, +4 and +5 zeroed), +0xb3 how many of competition [0x9cf7bc] enter at the end, +0xb4 the 1999 withdrawal, +0xb2 the human choice in 1999 [inference]; pointer/value; writes and reads'),
          ('clube +0x53 nação, +0x57 divisão; ponteiro; lê', 'club +0x53 nation, +0x57 division; pointer; reads'),
          ('[0x9cf5bc] Premier, [0x9cf5c0]/[0x9cf5c4]/[0x9cf5c8] as três divisões da Football League, [0x9cf69c] Conference, [0x9cf764] divisão genérica, [0x9cf2e4] Inglaterra, [0x9cf6a0] o FA Trophy, [0x9cf7bc] o Mundial de Clubes [inferência nos dois últimos]; id; lê',
           '[0x9cf5bc] Premier, [0x9cf5c0]/[0x9cf5c4]/[0x9cf5c8] the three Football League divisions, [0x9cf69c] Conference, [0x9cf764] generic division, [0x9cf2e4] England, [0x9cf6a0] the FA Trophy, [0x9cf7bc] the Club World Championship [inference on the last two]; id; reads'),
          ('[0xb63dfc]/[0xb63e00] as competições; [0xae23d0] registros das competições (0x6b bytes); [0xae23a8] nações (0x122 bytes); valor/ponteiro; lê',
           '[0xb63dfc]/[0xb63e00] the competitions; [0xae23d0] competition records (0x6b bytes); [0xae23a8] nations (0x122 bytes); value/pointer; reads')]
INTERESSA = []
