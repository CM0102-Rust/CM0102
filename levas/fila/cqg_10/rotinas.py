# O amistoso cancelado, o jogo da seleção que sai junto e o torneio cancelado (friendly.cpp):
# - 0x5c6570 (Friendly::on_friendly_fixture_cancelled, 413 bytes, quadro SEH da notícia local): pelo calendário
#   (0x5aa680, Rust) e pela tela do clube; 3 chamadas até 2002-06-01 (simular.py --cobertura, 02/11/2026). É a rotina
#   das mutações deste item (o jogo que sai ou fica no calendário e a vaga do amistoso).
# - 0x5cb9e0 (o amistoso de seleção cancelado, 278 bytes, quadro SEH da lista local): o jogo da véspera da seleção sai
#   junto (mutação da véspera).
# - 0x5c8260 (Friendly::cancel_tournament, 874 bytes, quadro SEH): pelo amistoso de outra competição e pelo dia dos
#   amistosos (0x5c2460, Rust); a contagem da cobertura não a pega (o prólogo dela não aceita o contador), passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias: 0x5c6570 thiscall, ret 8 (push clube; push jogo; mov ecx, [0xae2c74], como o Rust 0x5aa680 chama);
#   0x5cb9e0 thiscall, ret 4 (push jogo; mov ecx, edi = this em 0x5c66e0); 0x5c8260 thiscall, ret 8 (push 0; push
#   torneio; mov ecx, edi em 0x5c65e3).
#   Na 0x5c6570: 0x77bbe0 thiscall (push 0; push 8; lea ecx), 0x785790 thiscall (push v; push k; lea ecx; ret 8),
#   0x7861e0 thiscall (push clube; push &notícia; mov ecx, [0xb7431c]; ret 8), 0x5c6740 thiscall (4 pushes; mov ecx,
#   edi; ret 0x10: as palavras altas de dx/ax são do +0x14 e do +0x1c, como a cópia passa), 0x53b900 cdecl (push; add
#   esp, 4), 0x5abc30 thiscall (push jogo; mov ecx, [0xae2a58]; ret 4).
#   Na 0x5cb9e0: 0x68d020 / 0x68d040 / 0x68d1b0 thiscall (lea ecx), 0x53b4f0 cdecl (push; add esp, 4), 0x549ef0
#   thiscall (push +0x28; push +0x2a; lea ecx; ret 8), 0x54c8e0 thiscall (push 1; push &saída; lea ecx; ret 8),
#   0x417c10 thiscall (push origem; push &saída; lea ecx; ret 8), 0x5acbe0 thiscall (9 pushes; mov ecx, [0xae2a58];
#   ret 0x24), 0x5abc30 como acima.
#   Na 0x5c8260: 0x54c8e0 / 0x54c5b0 thiscall (push n; push &saída; mov ecx, data; ret 8), 0x77bbe0 (push 0; push 0xb),
#   0x785790, 0x7861e0, 0x68d1b0 / 0x68d1f0 thiscall (mov ecx, [this+0x34d0]; o 0x68d1f0 com push item, ret 4),
#   0x68d020 / 0x68d040 thiscall (lea ecx), 0x5acbe0, 0x549ef0, 0x5ac7a0 / 0x5abc30 / 0x5ab230 thiscall (push jogo; mov
#   ecx, [0xae2a58]; ret 4), o destrutor virtual da competição (vtable +0: push 1; mov ecx, competição; ret 4).
# Comparadores: as três só a original (tiram jogos do calendário, entregam notícias, destroem a competição: não podem
# rodar duas vezes). 0x5c6570 confere o plano: a vaga do amistoso (achada antes pela 0x5c6740) fica livre. 0x5c8260
# confere o plano: o dono e a competição zerados, a vaga da tabela de competições zerada e o torneio fora da lista dos
# pendentes. 0x5cb9e0 sem plano (a busca no calendário já mudado não se refaz).
# Determinismo: nada lê pilha sem valor nem usa endereço como valor (as datas locais são construídas antes do uso).
SAVE = False
ROTINAS = [('cqg10_amistoso_cancelado', 0x5c6570, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqg10_jogo_da_selecao_cancelado', 0x5cb9e0, [0x6a, 0xff, 0x68, 0x80, 0xa3, 0x95, 0x00]),
           ('cqg10_torneio_cancelado', 0x5c8260, [0x6a, 0xff, 0x68, 0x77, 0xa2, 0x95, 0x00])]
DOCS = ('o amistoso cancelado, o jogo da seleção que sai junto e o torneio cancelado',
        'the cancelled friendly, the national team fixture removed with it and the cancelled tournament',
        [('cqg10_amistoso_cancelado', 'Friendly::on_friendly_fixture_cancelled', '0x5c6570', 'friendly.cpp', 'thiscall, ret 8',
          'amistoso desta competição: a notícia 8 aos técnicos dos dois clubes que não cancelaram e a vaga do amistoso livre; de outra competição, o torneio dela é cancelado; jogo sem rodada sai do calendário (o de seleção pela 0x5cb9e0)',
          'a friendly of this competition: news 8 to the managers of both clubs that did not cancel and the friendly slot freed; of another competition, its tournament is cancelled; a fixture without a round leaves the calendar (the national team one through 0x5cb9e0)'),
         ('cqg10_jogo_da_selecao_cancelado', 'friendly_cancel_national_fixture', '0x5cb9e0', 'friendly.cpp', 'thiscall, ret 4',
          'pela marca do jogo, a seleção mandante (ou a principal da nação dele): o jogo dela na véspera sai do calendário; depois o próprio jogo sai',
          'by the fixture flag, the home national team (or its nation\'s first team): its fixture on the previous day leaves the calendar; then the fixture itself leaves'),
         ('cqg10_torneio_cancelado', 'Friendly::cancel_tournament', '0x5c8260', 'friendly.cpp', 'thiscall, ret 8',
          'a notícia 0xb aos convidados (torneio que começa depois de uma semana atrás); o torneio sai dos pendentes; os jogos da competição criada voltam a ser amistosos ou saem; a competição é destruída e a vaga dela e o dono zeram',
          'news 0xb to the invited clubs (a tournament starting after a week ago); the tournament leaves the pending list; the created competition fixtures become friendlies again or leave; the competition is destroyed and its slot and owner are cleared')])
CAMPOS = [('Friendly +4 a competição dos amistosos, +0xaf índice da primeira competição de torneio, +0xb1 os torneios (0x69 bytes), +0x34cc vagas dos amistosos, +0x34d0 lista dos torneios pendentes; ponteiro/valor; lê e grava (as três)',
           'Friendly +4 the friendly competition, +0xaf index of the first tournament competition, +0xb1 the tournaments (0x69 bytes), +0x34cc friendly slots, +0x34d0 pending tournament list; pointer/value; reads and writes (all three)'),
          ('torneio +0 dono, +4 começo (dia, ano), +0x15 os 4 convidados, +0x25 respostas (1 recusou), +0x64 índice da competição, +0x65 a competição criada; ponteiro/valor; lê e grava (cqg10_torneio_cancelado)',
           'tournament +0 owner, +4 start (day, year), +0x15 the 4 invited clubs, +0x25 replies (1 declined), +0x64 competition index, +0x65 the created competition; pointer/value; reads and writes (cqg10_torneio_cancelado)'),
          ('jogo +0 chave da competição, +0x14 competição, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x42/+0x43 rodada, +0x4d marca (0x100/0x200 seleção); valor/ponteiro; lê e grava (as três)',
           'fixture +0 competition key, +0x14 competition, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x42/+0x43 round, +0x4d flag (0x100/0x200 national team); value/pointer; reads and writes (all three)'),
          ('[0xadadfc] tabela das competições; ponteiro; grava 0 (cqg10_torneio_cancelado)', '[0xadadfc] competition table; pointer; writes 0 (cqg10_torneio_cancelado)')]
INTERESSA = []
