# O período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha
# (england_rules.cpp, scotland_rules.cpp, italy_rules.cpp, ireland_rules.cpp, spain_rules.cpp), pedido do Lucas para a
# família dos empréstimos (referência para o CM0102-Rust-Tweaks, docs/EMPRESTIMOS.md):
# - 0x579c70 (ENGLAND_RULES::get_loan_period, 229 bytes, quadro SEH sem destrutor): o período de um empréstimo para um
#   clube inglês, pelo TRANSFER_MANAGER::get_loan_period (0x8d0590, 203886 chamadas até 2002-06-01): o período da regra
#   base depois de 19 de junho para quem tem menos de 24 anos (16384+ até 2001-09-01), senão do mês seguinte a três
#   meses depois. É a rotina das mutações deste item.
# - 0x7f6650 (SCOTLAND_RULES::can_loan_player, 214 bytes), 0x7f6530 (SCOTLAND_RULES::get_loan_period, 155 bytes),
#   0x661770 (ITALY_RULES::can_loan_player, 384 bytes), 0x63c3d0 (IRELAND_RULES::can_loan_player, 264 bytes), 0x859630
#   (SPAIN_RULES::can_loan_player, 326 bytes): só rodam para clubes dessas nações pegando emprestado; na carreira
#   automática (só a Inglaterra) não rodam: passageiras (conferidas pelo comparador numa carreira com essas ligas).
# Ficaram para outra vez as regras da Finlândia (0x5a9a80, 0x5a9af0), da Noruega (0x794760) e da Austrália
# (0x412dd0): mesma forma da janela, também não rodam na carreira inglesa.
# Iguais no cm0102-1080.exe (as seis inteiras, conferido byte a byte).
# Convenções conferidas (métodos virtuais das regras; tabelas achadas pelos endereços: Inglaterra 0x969f4c, Escócia
# 0x96f838, Itália 0x96c96c, Irlanda 0x96c190, Espanha 0x96fdec):
# - quem chama os can_loan_player: o virtual +0x20 pela 0x8d0040 (push 0 ou motivo; push mensagem; push jogador; push
#   clube; mov ecx, regra) -> thiscall, ret 0x10 (as quatro terminam em ret 0x10).
# - quem chama os get_loan_period: o virtual +0x28 pela 0x8d0590 (sete push; mov ecx, regra) -> thiscall, ret 0x1c.
# - dentro: 0x549f70 push -1; push 0x7ce; push 5; push 0x13; lea ecx, local -> thiscall, ret 0x10; 0x8e8bd0 os sete
#   argumentos de novo; mov ecx, esi -> thiscall, ret 0x1c; 0x54cfb0 mov ecx, 0xae2c90 -> thiscall, ret 0; 0x4d45f0
#   push a1; mov ecx, [0xae1050] -> thiscall, ret 4; 0x4cf5a0 / 0x4cf6d0 push pessoa; mov ecx, [0xae1050] -> thiscall,
#   ret 4; 0x8e8c40 push janela; push 0xae2c90; ecx = this -> thiscall, ret 8; 0x8e8e50 push 0; push 0xae2c90; mov ecx,
#   this -> thiscall, ret 8; 0x7c1e30 push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall, ret 0x10
#   (st0); 0x66f4e0 push ...; push formato; push destino; add esp, 0x14 -> cdecl variádica; 0x6181d0 push 2; push 0x64;
#   push buffer; push nação; add esp, 0x10 -> cdecl.
# Comparadores: todas lado a lado. Os períodos nas quatro saídas e em [0xae1080]; os can_loan_player na devolução, em
# [0xae1080] e no motivo, e a mensagem pelo texto (só escrita: a cópia começa com o primeiro byte zerado e no fim volta
# o texto da original).
# Determinismo: nada lê pilha ou heap como valor; as datas e o buffer do adjetivo são locais preenchidos pelas originais.
# Provas (01/11/2026, cópia fora do projeto: motor.rs da principal com a 266ª + os três itens cqe_4..6, 11 rotinas):
# - duas carreiras Rust até 2002-06-01 com o motor de partidas (como a cadeia) iguais entre si: impressão
#   6dec43875616a596, 19404 partidas, e uma terceira igual noutra pasta; até 2001-09-01 sem o motor também iguais;
# - jogo comparando (Inglaterra, sem o motor) até 2002-095: 0 divergências em 10 bilhões de comparações;
#   com Escócia, Itália, Espanha e Irlanda até 2002-014: 0 divergências destes itens (há 15 de duas rotinas que
#   não são deles: copa_criar_jogos_do_confronto e preencher_vaga_da_comissao, só nessas ligas);
# - as 9 mutações dos três itens, cada uma numa carreira Rust até 2002-06-01 com o motor: todas mudam a impressão.
# - contadores até 2002-06-01: período da base 65536+, três meses 65536+; com as outras ligas, Escócia (até o fim
#   da temporada 131072+, mesma divisão 512+), Espanha (fora da janela 32768+, mesma divisão 16+), Irlanda (mesma
#   divisão 8+), Itália (fora da janela 65536+).
SAVE = False
ROTINAS = [('cqe6_periodo_inglaterra', 0x579c70, [0x6a, 0xff, 0x68, 0x08, 0x87, 0x95, 0x00]),
           ('cqe6_pode_emprestar_escocia', 0x7f6650, [0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('cqe6_periodo_escocia', 0x7f6530, [0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('cqe6_pode_emprestar_italia', 0x661770, [0x83, 0xec, 0x64, 0x55, 0x56]),
           ('cqe6_pode_emprestar_irlanda', 0x63c3d0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('cqe6_pode_emprestar_espanha', 0x859630, [0x83, 0xec, 0x64, 0x56, 0x57])]
DOCS = ('o período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha',
        'the English loan period and the Scottish, Italian, Irish and Spanish loan rules',
        [('cqe6_periodo_inglaterra', 'ENGLAND_RULES::get_loan_period', '0x579c70', 'england_rules.cpp', 'thiscall, ret 0x1c',
          'depois de 19 de junho e com o jogador abaixo de 24 anos, o período da regra base; senão do mês seguinte a três meses depois',
          'after 19 June and with the player under 24, the base rule period; otherwise from next month to three months later'),
         ('cqe6_pode_emprestar_escocia', 'SCOTLAND_RULES::can_loan_player', '0x7f6650', 'scotland_rules.cpp', 'thiscall, ret 0x10',
          'recusa (0x1d) entre dois clubes da mesma divisão escocesa',
          'refuses (0x1d) between two clubs of the same Scottish division'),
         ('cqe6_periodo_escocia', 'SCOTLAND_RULES::get_loan_period', '0x7f6530', 'scotland_rules.cpp', 'thiscall, ret 0x1c',
          'começa no mês seguinte; abaixo de 22 anos acaba no mês do fim da temporada (neste ano ou no seguinte), senão três meses depois',
          'starts next month; under 22 it ends in the season end month (this year or next), otherwise three months later'),
         ('cqe6_pode_emprestar_italia', 'ITALY_RULES::can_loan_player', '0x661770', 'italy_rules.cpp', 'thiscall, ret 0x10',
          'recusa fora das duas primeiras janelas (9) e, nas cinco divisões listadas, com quatro jogadores emprestados de outros clubes (0x25)',
          'refuses outside the first two windows (9) and, in the five listed divisions, with four players on loan from other clubs (0x25)'),
         ('cqe6_pode_emprestar_irlanda', 'IRELAND_RULES::can_loan_player', '0x63c3d0', 'ireland_rules.cpp', 'thiscall, ret 0x10',
          'recusa com cinco empréstimos na temporada (0x1e) e entre clubes da mesma divisão (0x1d)',
          'refuses with five loans this season (0x1e) and between clubs of the same division (0x1d)'),
         ('cqe6_pode_emprestar_espanha', 'SPAIN_RULES::can_loan_player', '0x859630', 'spain_rules.cpp', 'thiscall, ret 0x10',
          'recusa fora da janela (9) e entre clubes da mesma divisão depois de cinco jogos do jogador na liga (0x1f)',
          'refuses outside the window (9) and between clubs of the same division after five league games of the player (0x1f)')])
CAMPOS = [('jogador +0x18 idade, +0x61 ficha; valor e ponteiro; lê (cqe6_periodo_inglaterra, cqe6_periodo_escocia, cqe6_pode_emprestar_espanha)',
           'player +0x18 age, +0x61 player record; value and pointer; reads (cqe6_periodo_inglaterra, cqe6_periodo_escocia, cqe6_pode_emprestar_espanha)'),
          ('clube +0x38 nome, +0x52, +0x57 divisão, +0xd7 as 50 vagas do elenco; ponteiro e valor; lê (os can_loan_player)',
           'club +0x38 name, +0x52, +0x57 division, +0xd7 the 50 squad slots; pointer and value; reads (the can_loan_player)'),
          ('[[0xde1f54] + 0x91c] + id × 16, byte +0: a conta da temporada; valor; lê (cqe6_pode_emprestar_irlanda); regras +4 as janelas, +0xa a nação; ponteiro; lê; [0x9cf5dc] a divisão escocesa, [0x9cf578]..[0x9cf588] as divisões italianas; id; lê; [0xae1080] o último contrato; ponteiro; grava',
           '[[0xde1f54] + 0x91c] + id × 16, byte +0: the season count; value; reads (cqe6_pode_emprestar_irlanda); rules +4 the windows, +0xa the nation; pointer; reads; [0x9cf5dc] the Scottish division, [0x9cf578]..[0x9cf588] the Italian divisions; id; reads; [0xae1080] the last contract; pointer; writes')]
INTERESSA = ['CMRTE']
