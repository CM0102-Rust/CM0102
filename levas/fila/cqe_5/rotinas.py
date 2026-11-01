# As regras de empréstimo da Inglaterra (england_rules.cpp), pedido do Lucas para a família dos empréstimos
# (referência para o CM0102-Rust-Tweaks, docs/EMPRESTIMOS.md):
# - 0x579d60 (ENGLAND_RULES::can_loan_player, 622 bytes): pode emprestar? Mesma divisão inglesa, cinco na temporada,
#   dois ao mesmo tempo e a janela. Roda pela TRANSFER_MANAGER::can_loan_player (0x8d0040, 309108 chamadas até
#   2002-06-01) para os clubes ingleses; na carreira até 2001-09-01: "dois ao mesmo tempo" 512+, "mesma divisão" 64+.
# - 0x579fd0 (ENGLAND_RULES::record_loan, 68 bytes): a conta da temporada do clube inglês que pega emprestado, quando o
#   empréstimo se conclui (0x8c9390): 16+ até 2001-09-01.
# - 0x57a020 (ENGLAND_RULES::daily_update, 111 bytes): em 20 de junho a conta volta a zero (o mesmo método serve às
#   regras da Irlanda): roda todo dia, o efeito só depois de 2002-06-01: passageira.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas (as três são métodos virtuais das regras: tabela em 0x969f4c, achada pelos endereços):
# - quem chama a 0x579d60: o virtual +0x20 das regras da nação do clube, pela 0x8d0040 (push 0 ou motivo; push
#   mensagem; push jogador; push clube; mov ecx, regra) -> thiscall, ret 0x10 (ela termina em ret 0x10).
# - quem chama a 0x579fd0: o virtual +8 (empréstimo; +4 transferência) pela 0x8c9390 (push oferta; mov ecx, regras) ->
#   thiscall, ret 4.
# - quem chama a 0x57a020: o virtual +0 pelo dia das regras (push &data; mov ecx, regra) -> thiscall, ret 4.
# - dentro da 0x579d60: 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4 (e grava [0xae1080]); 0x4cf6d0
#   push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x66f4e0 push ...; push formato; push destino; add esp, 0x14 ->
#   cdecl variádica; 0x8e8e50 push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8; 0x6181d0 push 2; push
#   0x64; push buffer; push nação; add esp, 0x10 -> cdecl.
# - dentro da 0x57a020: 0x549f70 push -1; push 0x7ce; push 5; push 0x14; lea ecx, local -> thiscall, ret 0x10.
# Comparadores: as três lado a lado. A 0x579d60 na devolução, em [0xae1080] e no motivo, e a mensagem pelo texto (só
# escrita: a cópia começa com o primeiro byte zerado e no fim volta o texto da original). A 0x579fd0 no byte da conta.
# A 0x57a020 na tabela inteira das contas no dia em que ela muda (nos outros só a original roda: nada é gravado).
# Determinismo: nada lê pilha ou heap como valor; o buffer do adjetivo é local e preenchido pela 0x6181d0.
# Provas (01/11/2026, cópia fora do projeto: motor.rs da principal com a 266ª + os três itens cqe_4..6, 11 rotinas):
# - duas carreiras Rust até 2002-06-01 com o motor de partidas (como a cadeia) iguais entre si: impressão
#   6dec43875616a596, 19404 partidas, e uma terceira igual noutra pasta; até 2001-09-01 sem o motor também iguais;
# - jogo comparando (Inglaterra, sem o motor) até 2002-095: 0 divergências em 10 bilhões de comparações;
#   com Escócia, Itália, Espanha e Irlanda até 2002-014: 0 divergências destes itens (há 15 de duas rotinas que
#   não são deles: copa_criar_jogos_do_confronto e preencher_vaga_da_comissao, só nessas ligas);
# - as 9 mutações dos três itens, cada uma numa carreira Rust até 2002-06-01 com o motor: todas mudam a impressão.
# - contadores até 2002-06-01: dois ao mesmo tempo 32768+, cinco na temporada 8192+, fora da janela 8192+, mesma
#   divisão 1024+, empréstimo contado 128+.
SAVE = False
ROTINAS = [('cqe5_pode_emprestar', 0x579d60, [0x83, 0xec, 0x6c, 0x55, 0x56]),
           ('cqe5_registrar_emprestimo', 0x579fd0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00]),
           ('cqe5_dia_das_regras', 0x57a020, [0x83, 0xec, 0x08, 0x56, 0x6a, 0xff])]
DOCS = ('as regras de empréstimo da Inglaterra: pode emprestar (mesma divisão, cinco na temporada, dois ao mesmo tempo, a janela), a conta da temporada e a volta a zero em 20 de junho',
        'the English loan rules: can loan (same division, five a season, two at a time, the window), the season count and its reset on 20 June',
        [('cqe5_pode_emprestar', 'ENGLAND_RULES::can_loan_player', '0x579d60', 'england_rules.cpp', 'thiscall, ret 0x10',
          'recusa (0, com a mensagem e o motivo) entre clubes da mesma divisão inglesa (0x1d), com cinco empréstimos na temporada (0x1e), com mais de um jogador emprestado de outro clube no elenco (0x26) e fora da janela (9); senão 1',
          'refuses (0, with the message and the reason) between clubs of the same English division (0x1d), with five loans this season (0x1e), with more than one player on loan from another club in the squad (0x26) and outside the window (9); otherwise 1'),
         ('cqe5_registrar_emprestimo', 'ENGLAND_RULES::record_loan', '0x579fd0', 'england_rules.cpp', 'thiscall, ret 4',
          'soma 1 na conta da temporada do clube que pega emprestado, quando ele é inglês',
          'adds 1 to the season loan count of the borrowing club when it is English'),
         ('cqe5_dia_das_regras', 'ENGLAND_RULES::daily_update', '0x57a020', 'england_rules.cpp', 'thiscall, ret 4',
          'em 20 de junho, zera a conta da temporada de cada clube da nação destas regras',
          'on 20 June, resets the season loan count of every club of this rule set nation')])
CAMPOS = [('[[0xde1f54] + 0x91c] + id × 16, byte +0: a conta dos empréstimos da temporada do clube; valor; lê (cqe5_pode_emprestar), soma 1 (cqe5_registrar_emprestimo), zera (cqe5_dia_das_regras)',
           '[[0xde1f54] + 0x91c] + id × 16, byte +0: the club season loan count; value; reads (cqe5_pode_emprestar), adds 1 (cqe5_registrar_emprestimo), resets (cqe5_dia_das_regras)'),
          ('clube +0x38 nome, +0x52, +0x53 nação, +0x57 divisão, +0xd7 as 50 vagas do elenco; ponteiro e valor; lê (cqe5_pode_emprestar, cqe5_registrar_emprestimo, cqe5_dia_das_regras)',
           'club +0x38 name, +0x52, +0x53 nation, +0x57 division, +0xd7 the 50 squad slots; pointer and value; reads (cqe5_pode_emprestar, cqe5_registrar_emprestimo, cqe5_dia_das_regras)'),
          ('oferta +8 o clube que pega (índice em [0xae23b4]); id; lê (cqe5_registrar_emprestimo); regras +0xa a nação; ponteiro; lê; [0x9cf5bc] a divisão inglesa, [0x9cf2e4] a Inglaterra, [0xae2364] o limite dos ids; id; lê; [0xae1080] o último contrato; ponteiro; grava (cqe5_pode_emprestar)',
           'offer +8 the borrowing club (index into [0xae23b4]); id; reads (cqe5_registrar_emprestimo); rules +0xa the nation; pointer; reads; [0x9cf5bc] the English division, [0x9cf2e4] England, [0xae2364] the id limit; id; reads; [0xae1080] the last contract; pointer; writes (cqe5_pode_emprestar)')]
INTERESSA = ['CMRTE']
