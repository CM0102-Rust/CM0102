# O olheiro na partida observada e o histórico do clube observado (player_search.cpp):
# - 0x7b5be0 (PLAYER_SEARCH::scout_match, 1720 bytes, quadro SEH da lista local e das datas, sem destrutor fora do
#   caminho normal): pela partida observada (0x7f8c00, Rust); 50770 chamadas até 2002-06-01 (simular.py --cobertura,
#   02/11/2026). É a rotina das mutações deste item: o relatório (0x7b62a0) sorteia.
# - 0x7bd310 (player_search_record_scouted_club_match, 1000 bytes): o histórico de 10 partidas das vagas do clube
#   observado; 0 chamadas na carreira automática (passageira: só com um clube acompanhado, +0x190).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias: 0x7b5be0 thiscall, ret 0xc (push saida; push resultado; push jogo; mov ecx, lista em 0x7f8c00);
#   0x7bd310 thiscall, ret 8 (push resultado; push jogo; mov ecx, [lista+0x170] em 0x7b5daa).
#   Na 0x7b5be0: 0x53e990 cdecl (5 pushes; add esp, 0x14), 0x7b7490 thiscall (push nação; mov ecx, lista; ret 4),
#   0x7f9690 thiscall (push &lista; push clube; mov ecx, [0xb74448]; ret 8), 0x68d1b0/0x68d020/0x68d040 thiscall (lea
#   ecx, lista), 0x7f8d70 thiscall (push 1|0; push clube; push jogador; mov ecx, [0xb74448]; ret 0xc), 0x54c5b0
#   thiscall (push 1; push &tmp; mov ecx, 0xae2c90; ret 8), 0x417c10 thiscall (push data; push &copia; lea ecx,
#   [lista+0x1a5]; ret 8), 0x4cf5a0 thiscall (push jogador; mov ecx, [0xae1050]; ret 4), 0x5afde0 thiscall (push dono;
#   mov ecx, [0xae2a60]; ret 4), 0x5b09d0 thiscall (push olheiro; push jogador; mov ecx, eax; ret 8), 0x5407e0 cdecl
#   (3 pushes; add esp, 0xc), 0x7b62a0 thiscall (7 pushes; mov ecx, lista; ret 0x1c), 0x68d0d0 thiscall (push 0x67;
#   push &registro; mov ecx, saida; ret 8), 0x54a110 / 0x450d40 thiscall (lea ecx, parte do registro).
#   Na 0x7bd310: 0x600a50 cdecl (5 pushes; add esp, 0x14), 0x5afde0 e 0x5b09d0 como acima (0x5b09d0 com push 0),
#   0x7bd700 thiscall (push 1; push jogador; mov ecx, this; ret 8), 0x4afa80 cdecl (push jogo; push clube; add esp, 8).
# Comparadores: 0x7b5be0 só a original (o relatório sorteia e põe registros na saída, que aloca), com o plano feito
# antes pela cópia em ensaio (sem o histórico e sem o relatório, com o estado do sorteador guardado e devolvido): a
# devolução e se o começo da lista (+0x1a5) vai a amanhã. 0x7bd310 lado a lado nas 50 vagas (só a original quando as
# máscaras de atributos estão ligadas e o clube observado tem técnico: a 0x5b09d0 sorteia).
# Determinismo: o registro local de 0x67 bytes começa zerado na cópia (na original, sobra da pilha que a 0x7b62a0
# sobrescreve), com as partes que a entrada da original constrói (a data em +0 e os recordistas em +0x1f e +0x37); é
# reaproveitado entre os jogadores como na original. Nada lê endereço como valor.
SAVE = False
ROTINAS = [('cqf10_olheiro_no_jogo', 0x7b5be0, [0x6a, 0xff, 0x68, 0x68, 0xf5, 0x95, 0x00]),
           ('cqf10_historico_do_clube_observado', 0x7bd310, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('o olheiro na partida observada e o histórico do clube observado',
        'the scout at the watched match and the watched club history',
        [('cqf10_olheiro_no_jogo', 'PLAYER_SEARCH::scout_match', '0x7b5be0', 'player_search.cpp', 'thiscall, ret 0xc',
          'a lista do olheiro (tipo 4) que confere vê a partida jogada: o histórico do clube acompanhado; dentro das datas, com o jogador acompanhado no jogo ou com pedido do clube por algum jogador dos dois clubes, o começo da lista vai a amanhã e cada jogador de outro clube com nota de ao menos 15 (e pedido, acompanhado ou com reputação vista abaixo de 4000) vira um relatório na saída',
          'the matching scout list (type 4) watches the played match: the history of the followed club; within the dates, with the followed player in the match or with the club asking for a player of either club, the list start moves to tomorrow and every player of another club rated at least 15 (and asked for, followed or with a seen reputation below 4000) becomes a report in the output'),
         ('cqf10_historico_do_clube_observado', 'player_search_record_scouted_club_match', '0x7bd310', 'player_search.cpp', 'thiscall, ret 8',
          'as vagas de quem saiu do clube observado são limpas; cada jogador do clube que jogou ganha uma linha no histórico de 10 partidas: gols, nota, data, adversário, placar, lado, estatísticas e o tipo (titular, reserva, nota pelo +0x30)',
          'the slots of players who left the watched club are cleared; every club player who played gets a line in the 10-match history: goals, rating, date, opponent, score, side, statistics and the type (starter, substitute, rating from +0x30)')])
CAMPOS = [('lista de busca +0xa tipo (4 = observar partidas), +0x170 registro do clube observado, +0x174..+0x184 nomes do olheiro, +0x180 id do olheiro, +0x190 clube acompanhado, +0x195 jogador acompanhado; valor/ponteiro/id; lê (cqf10_olheiro_no_jogo)',
           'search list +0xa type (4 = watch matches), +0x170 watched club record, +0x174..+0x184 scout names, +0x180 scout id, +0x190 followed club, +0x195 followed player; value/pointer/id; reads (cqf10_olheiro_no_jogo)'),
          ('lista de busca +0x1a5 data de começo (dia, ano), +0x1ad data de fim; valor; lê e grava o começo = amanhã (cqf10_olheiro_no_jogo)',
           'search list +0x1a5 start date (day, year), +0x1ad end date; value; reads and writes start = tomorrow (cqf10_olheiro_no_jogo)'),
          ('resultado da partida +0x4c sem notas, +0x4d nota padrão, +0x190/+0x1c0 os 20 registros de 0x49 bytes de cada lado (+0x625): +0 id, +4 entrada, +0x21 tipo, +0x30/+0x34 notas, +0x32 minuto da entrada [inferência], +0x35 nota do jogo; valor; lê (as duas)',
           'match result +0x4c no ratings, +0x4d default rating, +0x190/+0x1c0 the 20 records of 0x49 bytes per side (+0x625): +0 id, +4 entry, +0x21 type, +0x30/+0x34 ratings, +0x32 entry minute [inference], +0x35 match rating; value; reads (both)'),
          ('registro do clube observado +0 id do clube, +4 50 vagas de 0x1a2 bytes (+0 id, +4 jogos, +0x1a0 começo e +0x1a1 tamanho do anel de 10 linhas de 0x29 bytes a partir de +6); id/valor; lê e grava (cqf10_historico_do_clube_observado)',
           'watched club record +0 club id, +4 50 slots of 0x1a2 bytes (+0 id, +4 matches, +0x1a0 start and +0x1a1 size of the 10-line ring of 0x29 bytes from +6); id/value; reads and writes (cqf10_historico_do_clube_observado)'),
          ('[0x9cc270] máscaras de atributos ligadas; valor; lê (as duas)', '[0x9cc270] attribute masking enabled; value; reads (both)'),
          ('pessoa +0x39 clube, +0x61 ficha de jogador; ponteiro; lê (as duas)', 'person +0x39 club, +0x61 player record; pointer; reads (both)')]
INTERESSA = ['CMScout']
