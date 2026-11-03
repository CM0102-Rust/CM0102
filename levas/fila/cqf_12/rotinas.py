# Os olheiros do dia, os relatórios deles e a notícia do relatório (scout_manager.cpp):
# - 0x7f8750 (SCOUT_MANAGER::daily_update, 1198 bytes, quadro SEH da lista local e das datas): pelo dia (0x5cccd0); 966
#   chamadas até 2002-06-01 (simular.py --cobertura, 02/11/2026). Poda os pedidos de observação e faz os relatórios dos
#   pedidos vencidos (0x7b62a0, que sorteia): é a rotina das mutações deste item.
# - 0x7f9890 (SCOUT_MANAGER::send_scout_reports, 460 bytes): 5520 chamadas (também pela busca 0x7f8c00).
# - 0x7f9a60 (SCOUT_MANAGER::send_scout_report_news, 686 bytes): a notícia 0x138e de um grupo de relatórios.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias: 0x7f8750 thiscall, ret 0 (mov ecx, this; sem argumentos); 0x7f9890 thiscall, ret 4 (push &lista; mov
#   ecx, esi em 0x7f8b6e); 0x7f9a60 thiscall, ret 8 (push quantos; push registro; mov ecx, [esp+0x20] = this em 0x7f9a33).
#   Na 0x7f8750: 0x54c5b0 thiscall (push n; push &saída; mov ecx, 0xae2c90 / lea ecx, [pedido+8]; ret 8), 0x54a110 /
#   0x68d020 / 0x68d040 thiscall (lea ecx), 0x600a50 cdecl (5 pushes; add esp, 0x14), 0x4cf5a0 thiscall (push jogador;
#   mov ecx, [0xae1050]; ret 4), 0x62c400 thiscall (push jogador; mov ecx, [0xb63d28]; ret 4), 0x4cf5f0 thiscall (push
#   clube; push jogador; mov ecx, [0xae1050]; ret 8), 0x5acbe0 thiscall (9 pushes; mov ecx, [0xae2a58]; ret 0x24),
#   0x7f6cd0 thiscall (push técnico; push k; mov ecx, this; ret 8), 0x7b7490 thiscall (push nação; mov ecx, lista;
#   ret 4), 0x7b62a0 thiscall (7 pushes; mov ecx, lista; ret 0x1c), 0x68d0d0 thiscall (push 0x67; push &registro; lea
#   ecx, local; ret 8), 0x945333 cdecl (4 pushes; add esp, 0x10), 0x7b3e20 thiscall (mov ecx, lista; ret 0).
#   Na 0x7f9890: 0x944e46 e 0x9452ca cdecl (push; add esp, 4), 0x68d1b0 thiscall (mov ecx, lista), 0x945333 cdecl.
#   Na 0x7f9a60: 0x77bbe0 thiscall (push 0; push 0x138e; lea ecx), 0x785790 thiscall (push v; push k; lea ecx; ret 8),
#   0x7e5160 e 0x7e4de0 cdecl (3 pushes; add esp, 0xc), 0x546ab0 cdecl (push clube; push jogador; add esp, 8),
#   0x7861e0 thiscall (push clube; push &notícia; mov ecx, [0xb7431c]; ret 8).
# Comparadores: 0x7f8750 só a original (os relatórios sorteiam e alocam; as buscas mexem nas listas dos humanos), com o
# plano da cópia em ensaio: os pedidos que ficam, podados numa cópia do vetor e ordenados (0x945333 com 0x7fc270), têm
# de ser os da original (conta e os 0x11 bytes de cada). 0x7f9890 só a original (a notícia aloca na caixa), com o plano
# da cópia em ensaio: os grupos (jogador, clube, quantos), na ordem, têm de ser os que a original passa à 0x7f9a60
# (anotados no comparador dela). 0x7f9a60 só a original (só monta e entrega a notícia).
# Determinismo: o 2º dword da data passada à 0x5acbe0 tem a palavra alta sem valor na original (sobra da pilha); a
# cópia usa 0 (a 0x5acbe0 só lê a palavra baixa). O registro do relatório começa zerado na cópia, com as datas que a
# entrada da original constrói (+0, +0x2f, +0x47). Nada lê endereço como valor.
# Provas (02/11/2026, mesma cópia: cqf_10, cqf_11, cqf_12, cqg_10):
# - duas carreiras Rust até 2002-06-01 iguais entre si e ao detector ('detector de lixo: igual'), 825918943f68aa97;
# - jogo comparando (Inglaterra, sem o motor) até 2002-059: 0 divergências; com ENG ESP ITA FRA até 2001-304: 1
#   DIVERGENCIA, de aviso_ao_rival_de_posicao (não é deste item);
# - mutações: escapam (nenhum pedido processado na carreira automática; tirar as buscas dos humanos também não mudou
#   a impressão; ver mutacoes.py).
SAVE = False
ROTINAS = [('cqf12_olheiros_do_dia', 0x7f8750, [0x6a, 0xff, 0x68, 0xbe, 0x0f, 0x96, 0x00]),
           ('cqf12_relatorios_dos_olheiros', 0x7f9890, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('cqf12_noticia_do_relatorio', 0x7f9a60, [0x6a, 0xff, 0x68, 0x3b, 0x10, 0x96, 0x00])]
DOCS = ('os olheiros do dia, os relatórios deles e a notícia do relatório',
        'the scouts\' day, their reports and the report news',
        [('cqf12_olheiros_do_dia', 'SCOUT_MANAGER::daily_update', '0x7f8750', 'scout_manager.cpp', 'thiscall, ret 0',
          'na fatia 2 do dia: poda os pedidos de observação (clube que não é humano, jogador já do clube, mais de um ano, 3 relatórios, machucado); o pedido com mais de 30 dias vira até 3 relatórios dos olheiros do técnico que cobrem a nação do jogador e sai; depois os relatórios, a ordem dos pedidos e uma busca em cada lista dos técnicos humanos',
          'on day slice 2: prunes the scouting requests (non-human club, player already at the club, older than a year, 3 reports, injured); a request older than 30 days becomes up to 3 reports from the manager\'s scouts covering the player\'s nation and leaves; then the reports, the request order and one search tick on every human manager list'),
         ('cqf12_relatorios_dos_olheiros', 'SCOUT_MANAGER::send_scout_reports', '0x7f9890', 'scout_manager.cpp', 'thiscall, ret 4',
          'copia os registros da lista num vetor, ordena e manda uma notícia por grupo seguido do mesmo jogador e clube',
          'copies the list records into an array, sorts it and sends one news item per run of the same player and club'),
         ('cqf12_noticia_do_relatorio', 'SCOUT_MANAGER::send_scout_report_news', '0x7f9a60', 'scout_manager.cpp', 'thiscall, ret 8',
          'a notícia 0x138e com até 3 registros, o olheiro, o jogo e as forças do jogador para o clube, ao técnico do clube',
          'the 0x138e news item with up to 3 records, the scout, the match and the player\'s strengths for the club, to the club manager')])
CAMPOS = [('SCOUT_MANAGER +0x5c vetor dos pedidos de observação (0x11 bytes: +0 id do clube, +4 id do jogador, +8 data, +0x10 relatórios feitos), +0x64 conta; id/valor; lê e grava (cqf12_olheiros_do_dia)',
           'SCOUT_MANAGER +0x5c scouting request array (0x11 bytes: +0 club id, +4 player id, +8 date, +0x10 reports made), +0x64 count; id/value; reads and writes (cqf12_olheiros_do_dia)'),
          ('[0xae2c88] fatia do dia; valor; lê (cqf12_olheiros_do_dia)', '[0xae2c88] day slice; value; reads (cqf12_olheiros_do_dia)'),
          ('contrato +0x4f estado (& 0xf0: 0, 0x60, 0x70); valor; lê (cqf12_olheiros_do_dia)', 'contract +0x4f status (& 0xf0: 0, 0x60, 0x70); value; reads (cqf12_olheiros_do_dia)'),
          ('pessoa +0x1a nação, +0x39 clube, +0x5f humano (1); ponteiro/valor; lê (cqf12_olheiros_do_dia)', 'person +0x1a nation, +0x39 club, +0x5f human (1); pointer/value; reads (cqf12_olheiros_do_dia)'),
          ('registro do relatório (0x67 bytes) +0x2b id do jogador, +0x50 id do clube, +0x37 os registros seguidos (0x67), +0x54..+0x66 dados; id/valor; lê (cqf12_relatorios_dos_olheiros, cqf12_noticia_do_relatorio)',
           'report record (0x67 bytes) +0x2b player id, +0x50 club id, +0x37 the following records (0x67), +0x54..+0x66 data; id/value; reads (cqf12_relatorios_dos_olheiros, cqf12_noticia_do_relatorio)')]
INTERESSA = ['CMScout']
