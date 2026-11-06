# As copas sul-americanas de clubes: o calendário da Libertadores e da Mercosul e os convidados da Mercosul (o slot
# +0x3c create_schedule e o select_invited_clubs; conmebol_liber.cpp, conmebol_merc.cpp, em eurocomp\). Frente das
# competições (Agente D, 05/11/2026), carreira de 8 ligas (os clubes brasileiros jogáveis).
# - 0x4c31c0 (ConmebolLiber::create_schedule, 1203 bytes; 20 chamadas até 2002-06-01 com as 8 ligas): as 6 rodadas de
#   cada grupo e as 4 do mata-mata da Libertadores. Uma mutação.
# - 0x4c4930 (ConmebolMerc::create_schedule, 1036 bytes; 8 chamadas): as 6 rodadas de cada grupo e as 3 do mata-mata
#   da Mercosul. Uma mutação.
# - 0x4c5850 (ConmebolMerc::select_invited_clubs, 270 bytes; 1 chamada, no construtor): os 20 convidados, embaralhados.
#   Uma mutação.
# Reaproveita do motor.rs os ajudantes da 276a (cql10_rodada, cql10_gravados, Cql10R) para o mata-mata; os grupos
# (rodadas de 0x41 bytes pela 0x68a160 e 0x68a1c0) são o cqo13_tabela, no molde da 0x92c290 (Copa do Mundo, cqf_6). As
# tabelas foram extraídas da desmontagem; a final da Mercosul grava também a palavra +0x11 = 3 (fora do Cql10R; vai
# em CQO13_MERCOSUL_A_MAIS e entra na comparação).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as duas tabelas são a virtual +0x3c (push saida; push &extra; push &rodadas;
# push fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte com sinal, cmp al, 7 / 4 com jge); a 0x4c5850
# é thiscall, ret 0 (sem argumentos). Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x68a160 (9 pushes ->
# cdecl; o ano em registrador com lixo nos 16 bits altos, a 0x68a160 usa só a palavra: a cópia passa limpo), 0x68a1c0
# (7 pushes -> cdecl), 0x521e60/0x521eb0 pela cql10_rodada, 0x4c6890 (push clube; mov ecx, [0xae1000] -> thiscall, ret
# 4; já em Rust, cqj_1), 0x4afb40 (push n; push lista; add esp, 8 -> cdecl; a palavra +0xf2 num registrador com lixo
# nos 16 bits altos: a 0x4afb40 lê a palavra, a cópia passa limpo) e a caixa (aviso_em).
# Comparadores: lado a lado. As tabelas: a original com as saídas de quem chama, a cópia com saídas próprias que começam
# iguais; o [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a da cópia é liberada. Acima
# do mata-mata só a original (nenhuma grava nada). Os convidados: o +0xf2, o [0xb67a34] e o estado do sorteador (a
# cópia sorteia sem contar); a lista da cópia comparada nos convidados gravados e liberada.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (os grupos da
#   Libertadores de 2001 em 2001-049, os da Mercosul em 2001-201, os convidados da Mercosul em 2001-166).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqo13_libertadores_tabela', 0x4c31c0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo13_mercosul_tabela', 0x4c4930, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo13_mercosul_convidados', 0x4c5850, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as copas sul-americanas de clubes: o calendário da Libertadores e da Mercosul e os convidados da Mercosul',
        'the South American club cups: the Copa Libertadores and Copa Mercosur calendars and the Mercosur invitees',
        [('cqo13_libertadores_tabela', 'ConmebolLiber::create_schedule', '0x4c31c0', 'conmebol_liber.cpp', 'thiscall, ret 0x10',
          'regra: a Libertadores começa por grupos de seis rodadas às terças [inferência: dia da semana 2], de 25/2 a 1º/4 (cada grupo um estágio de liga, id 0x3e9 + fase, até a fase 6: 8 grupos [inferência]); depois o mata-mata (fase 7) em ida e volta: oitavas sorteadas em 2/4 e jogadas em 15/4 (16 times, volta 14 dias depois), quartas 30/4 e 20/5, semifinal 28/5 e 3/6, final sorteio 11/6 e jogo em 17/6 (volta 7 dias depois); desempate de ida e volta 1 e sorteio com o bit 4; categoria 7 nas oitavas e 8 depois; prêmio da final 2 milhões ao campeão e 800 mil ao vice',
          'rule: the Copa Libertadores starts with six-round groups on Tuesdays [inference: weekday 2], from 25/2 to 1/4 (each group a league stage, id 0x3e9 + stage, up to stage 6: 8 groups [inference]); then the knockout (stage 7) over two legs: round of 16 drawn on 2/4 and played on 15/4 (16 teams, return 14 days later), quarter-finals 30/4 and 20/5, semi-final 28/5 and 3/6, final draw 11/6 and match on 17/6 (return 7 days later); two-leg tie-break 1 and draw with bit 4; category 7 in the round of 16 and 8 after; final prize 2 million to the winner and 800 thousand to the runner-up'),
         ('cqo13_mercosul_tabela', 'ConmebolMerc::create_schedule', '0x4c4930', 'conmebol_merc.cpp', 'thiscall, ret 0x10',
          'regra: a Mercosul começa por grupos de seis rodadas às terças, de 29/7 a 14/10 (id 0x3e9 + fase, até a fase 3: 5 grupos [inferência]); o mata-mata (fase 4) em ida e volta: quartas sorteadas em 15/10 e jogadas em 28/10 (8 times, volta 3 dias depois), semifinal 5/11 e 11/11 (volta 3 dias depois), final sorteio 19/11 e jogo em 2/12 (volta 7 dias depois, desempate 0x2000 e a palavra +0x11 = 3 [inferência: pênaltis]); categoria 8; prêmio de 2 milhões ao campeão',
          'rule: the Copa Mercosur starts with six-round groups on Tuesdays, from 29/7 to 14/10 (id 0x3e9 + stage, up to stage 3: 5 groups [inference]); the knockout (stage 4) over two legs: quarter-finals drawn on 15/10 and played on 28/10 (8 teams, return 3 days later), semi-final 5/11 and 11/11 (return 3 days later), final draw 19/11 and match on 2/12 (return 7 days later, tie-break 0x2000 and the word +0x11 = 3 [inference: penalties]); category 8; prize of 2 million to the winner'),
         ('cqo13_mercosul_convidados', 'ConmebolMerc::select_invited_clubs', '0x4c5850', 'conmebol_merc.cpp', 'thiscall, ret 0',
          'regra: a Mercosul é só de convidados: os 20 clubes da lista fixa das regras sul-americanas (0x4c6890), na ordem da tabela de clubes, embaralhados para o sorteio dos grupos',
          'rule: the Copa Mercosur is invitation-only: the 20 clubs of the South American rules\' fixed list (0x4c6890), in club table order, shuffled for the group draw')])
CAMPOS = [('copa +0x40 ano da copa; valor (palavra com sinal); lê (tabelas)',
           'cup +0x40 cup year; value (signed word); reads (tables)'),
          ('rodada de copa (0x68 bytes) +0..+0x22 e +0x5c..+0x67, na final da Mercosul também +0x11; valor; grava (mata-mata)',
           'cup round (0x68 bytes) +0..+0x22 and +0x5c..+0x67, in the Mercosur final also +0x11; value; writes (knockout)'),
          ('rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5..+0xb jogo 0 vazio, +0x3d valor; valor; grava (grupos)',
           'group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5..+0xb empty match 0, +0x3d value; value; writes (groups)'),
          ('copa +0xee lista dos convidados (ponteiros de clube), +0xf2 máximo (dword, 20); ponteiro/valor; grava (convidados)',
           'cup +0xee invitee list (club pointers), +0xf2 maximum (dword, 20); pointer/value; writes (invitees)'),
          ('[0xae1000] as regras sul-americanas (ConmebolSeeding); ponteiro; lê (convidados)',
           '[0xae1000] the South American rules (ConmebolSeeding); pointer; reads (invitees)')]
INTERESSA = ['CMRTE']
