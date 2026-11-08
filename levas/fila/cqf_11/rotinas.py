# As notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários (national_teams.cpp):
# - 0x7706c0 (NationalTeams::send_club_international_report_news, 1568 bytes, quadro SEH da notícia local): pelo
#   resultado entre seleções (0x76b600, Rust), de cada lado; 808 chamadas até 2002-06-01 (simular.py --cobertura,
#   02/11/2026). Cada notícia sorteia (sorteio(2)): é a rotina das mutações deste item.
# - 0x76e0b0 (national_teams_post_call_up_news, 768 bytes): a notícia da convocação aos clubes de técnico humano, pela
#   convocação (0x76c120, Rust); 1022 chamadas. Só notícia (passageira).
# - 0x76d560 (national_teams_collect_fixture_opponents, 542 bytes): a lista dos adversários, pela 0x76e0b0 e pelo
#   pedido ao técnico (0x76f1a0); 5 chamadas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias: 0x7706c0 thiscall, ret 0x18 (6 pushes; mov ecx, this, como o Rust 0x76b600 chama); 0x76e0b0 thiscall,
#   ret 0x1c (7 pushes; mov ecx, this, como o Rust 0x76c120 chama); 0x76d560 thiscall, ret 8 (push jogo; push seleção;
#   mov ecx, [esp+0x28] = this em 0x76e186).
#   Na 0x7706c0: 0x945333 cdecl (4 pushes; add esp, 0x10), 0x54a110 thiscall (lea ecx), 0x77bbe0 thiscall (push 0;
#   push 0x24; lea ecx), 0x7e4750 cdecl (3 pushes; add esp, 0xc), 0x7e4de0 cdecl (3 pushes; add esp, 0xc), 0x785790
#   thiscall (push v; push k; lea ecx; ret 8), 0x90cfc0 cdecl (push 2; add esp, 4), 0x7861e0 thiscall (push clube;
#   push &notícia; mov ecx, [0xb7431c]; ret 8).
#   Na 0x76e0b0: 0x7875e0 thiscall (push clube; mov ecx, [0xb7431c]; ret 4), 0x76d560 como acima, 0x77bbe0 (push 0;
#   push 0x1d), 0x785790, 0x68d1b0 thiscall (mov ecx, lista), 0x7861e0, 0x68d040 thiscall (mov ecx, lista), 0x944c94
#   cdecl (push; add esp, 4).
#   Na 0x76d560: 0x68d020 thiscall (lea ecx, local / mov ecx, bloco novo), 0x944cf1 cdecl (push 0xe; add esp, 4),
#   0x5409b0 e 0x53b4f0 cdecl (push; add esp, 4), 0x68d0d0 thiscall (push 4; push &valor; mov ecx, lista; ret 8),
#   0x549ef0 thiscall (push +0x28; push +0x2a; lea ecx; ret 8), 0x54c5b0 thiscall (push n; push &saída; mov ecx, data;
#   ret 8), 0x5acbe0 thiscall (9 pushes; mov ecx, [0xae2a58]; ret 0x24), 0x68d1b0 / 0x68d040 thiscall (lea ecx).
# Comparadores: 0x7706c0 só a original (a entrega aloca nas caixas) com os sorteios rastreados e o elenco guardado;
# depois o elenco volta, a cópia refaz em ensaio (sorteios da fita, sem entregar) e o elenco tem de sair igual, com os
# sorteios do ensaio iguais à fita. 0x76d560 só a original (aloca a lista) com o plano da cópia em ensaio (os valores
# da lista, na ordem). 0x76e0b0 só a original (só monta e entrega notícias; a lista dela é conferida pela 0x76d560).
# Determinismo: na 0x76e0b0 a quantidade de adversários fica sem valor quando não há lista nem montagem (a original
# grava sobra da pilha na notícia); a cópia usa 0. O 9º argumento da 0x5acbe0 na 0x76d560 é o dword do argumento
# `jogo` (ou do 0x53b4f0 gravado em cima) com o byte baixo do +0x40, como na original. Nada lê endereço como valor.
# Provas (02/11/2026, mesma cópia: cqf_10, cqf_11, cqf_12, cqg_10):
# - duas carreiras Rust até 2002-06-01 iguais entre si e ao detector ('detector de lixo: igual'), 825918943f68aa97;
# - jogo comparando (Inglaterra, sem o motor) até 2002-059: 0 divergências; com ENG ESP ITA FRA até 2001-304: 1
#   DIVERGENCIA, de aviso_ao_rival_de_posicao (não é deste item);
# - mutações (carreira Rust até 2002-06-01 com o motor): as três mudam a impressão (sem sorteio 009be8d94d295f13, um
#   grupo só 0d4f1195a436d541, sem tirar do elenco 6a5a462035525e15); 'cqf_11 club news' 2048+ até 2002-06-01.
SAVE = False
ROTINAS = [('cqf11_relatorio_da_selecao', 0x7706c0, [0x6a, 0xff, 0x68, 0xa6, 0xde, 0x95, 0x00]),
           ('cqf11_noticia_da_convocacao', 0x76e0b0, [0x6a, 0xff, 0x68, 0xfb, 0xdd, 0x95, 0x00]),
           ('cqf11_adversarios_da_selecao', 0x76d560, [0x6a, 0xff, 0x68, 0xbb, 0xdd, 0x95, 0x00])]
DOCS = ('as notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários',
        'the national team news to clubs: the match report, the call-up and the opponents',
        [('cqf11_relatorio_da_selecao', 'NationalTeams::send_club_international_report_news', '0x7706c0', 'national_teams.cpp', 'thiscall, ret 0x18',
          'o elenco do jogo é ordenado pelo clube; cada grupo do mesmo clube vira a notícia 0x24 ao técnico dele: os dados do jogo, o destaque, quantos, até 4 jogadores (o destaque primeiro, os outros pelo código de como foram), sorteio(2) e os códigos; os jogadores do grupo saem do vetor',
          'the match squad is sorted by club; each group of the same club becomes news 0x24 to its manager: the match data, the star player, how many, up to 4 players (the star first, the others by how they did), draw(2) and the codes; the group players leave the array'),
         ('cqf11_noticia_da_convocacao', 'national_teams_post_call_up_news', '0x76e0b0', 'national_teams.cpp', 'thiscall, ret 0x1c',
          'os convocados (marca 4, ou 8 na segunda seleção) de clubes de técnico humano, agrupados pelo clube, viram a notícia 0x1d ao técnico: seleção, clube, tipo, quantos, os adversários (lista dada ou montada pela 0x76d560) e até 3 jogadores',
          'the called-up players (flag 4, or 8 for the second team) of human-managed clubs, grouped by club, become news 0x1d to the manager: national team, club, type, how many, the opponents (given list or built by 0x76d560) and up to 3 players'),
         ('cqf11_adversarios_da_selecao', 'national_teams_collect_fixture_opponents', '0x76d560', 'national_teams.cpp', 'thiscall, ret 8',
          'uma lista nova com o adversário do jogo (a seleção principal da nação dele, para a segunda seleção) e os adversários dos jogos de amanhã até 13 dias depois que não são de fase final',
          'a new list with the match opponent (its nation\'s first team, for the second team) and the opponents of the fixtures from tomorrow to 13 days later that are not finals')])
CAMPOS = [('jogo +0x1c mandante, +0x20 visitante, +0x14 competição, +0x28 ano, +0x2a dia, +0x40 byte do tipo [inferência]; ponteiro/valor; lê (as três)',
           'fixture +0x1c home club, +0x20 away club, +0x14 competition, +0x28 year, +0x2a day, +0x40 type byte [inference]; pointer/value; reads (all three)'),
          ('resultado da partida +0x3c id da ficha do melhor da partida, +0x190 os 20 registros de 0x49 bytes de cada lado (+0x625): +0 id, +0x21 tipo, +0x23/+0x24 [inferência], +0x34 nota; valor; lê (cqf11_relatorio_da_selecao)',
           'match result +0x3c player record id of the man of the match, +0x190 the 20 records of 0x49 bytes per side (+0x625): +0 id, +0x21 type, +0x23/+0x24 [inference], +0x34 rating; value; reads (cqf11_relatorio_da_selecao)'),
          ('seleção (clube) +0xd7 as 50 vagas do elenco (ponteiros), +0x53 nação; pessoa +0x5e marca da convocação (4 principal, 8 segunda), +0x39 clube; ponteiro/valor; lê (cqf11_noticia_da_convocacao)',
           'national team (club) +0xd7 the 50 squad slots (pointers), +0x53 nation; person +0x5e call-up flag (4 first team, 8 second), +0x39 club; pointer/value; reads (cqf11_noticia_da_convocacao)'),
          ('pessoa +0 id, +4/+8/+0xc chaves (+0x33), +0x39 clube, +0x61 ficha (+0 id); id/ponteiro; lê (cqf11_relatorio_da_selecao)',
           'person +0 id, +4/+8/+0xc keys (+0x33), +0x39 club, +0x61 player record (+0 id); id/pointer; reads (cqf11_relatorio_da_selecao)')]
INTERESSA = []
