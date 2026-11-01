# Os jogos remarcados (fix_man.cpp; a familia media.cpp/news.cpp esta esgotada: sobram textos de noticia, a carga
# do save e duas noticias sem efeito na simulacao, entao este item pega a noticia do jogo remarcado e as duas
# remarcacoes que a chamam, fix_man.cpp, que nenhum outro agente usa nesta rodada): FIXTURE_MANAGER::
# move_nearby_club_fixtures 0x5ae9d0 (821 chamadas ate 2002-06-01 na cobertura de 17/10, pela juncao 0x5aa680: o jogo
# vizinho de cada clube, dois dias antes ou depois, vai um dia mais para longe), reschedule_club_fixtures_near_date
# 0x5abb10 (7 chamadas, pelas convocacoes 0x777940) e queue_fixture_moved_news 0x5aac70 (201 chamadas: a noticia 1 na
# fila +0x1128, ou so a data nova se o jogo ja esta la). Tamanhos reais (candidatos.py): 1126, 280 e 651 bytes. As tres
# com SEH (listas e noticia locais; os destrutores das listas 0x68d040 sao chamados como na original; a noticia local
# nao tem destrutor). Iguais no cm0102-1080.exe.
# Convencoes conferidas nos pontos de chamada (mov ecx antes = thiscall; add esp depois = cdecl; os push contados):
#   0x54a110 Date() thiscall 0 arg; 0x68d020/0x68d040/0x68d080 LLIST ctor/dtor/clear thiscall 0 arg; 0x68d1b0 get_next
#   thiscall 0 arg; 0x68d0d0 add_tail thiscall 2 args (dados, 0xdf); 0x5c6710 amistosa thiscall em [0xae2c74] 1 arg;
#   0x549ef0 Date(dia, ano) thiscall 2 args; 0x54c8e0/0x54c5b0 menos/mais dias thiscall 2 args (saida, n); 0x417c10
#   atribuir thiscall 2 args (temporaria, origem); 0x5acbe0 find_fixtures thiscall 9 args (ret 0x24); 0x5ad780
#   find_fixture_clashes thiscall 3 args (jogo, &gravidade, 0); 0x944c94 delete cdecl 1 arg (add esp,4); 0x944cf1 new
#   cdecl 1 arg (add esp,4); 0x5abc30 remove_fixture thiscall 1 arg; 0x5ab510 thiscall 1 arg; 0x5aa680 add_fixture
#   thiscall 2 args; 0x5aac70 a noticia thiscall 3 args; 0x77bbe0 NewsItem thiscall 2 args; 0x785790 set_slot
#   thiscall 2 args; 0x785830 get_slot thiscall 1 arg.
# Comparadores: as duas remarcacoes so a original (tiram e juntam jogos no calendario, alocam e liberam listas, dao a
# noticia; as copias levam os ramos); a noticia em ensaio: ja na fila, lado a lado no cabecalho da fila e na noticia
# achada; nova, a copia monta a noticia sem po-la na fila e a original roda (a fila cresce um e a cauda e a noticia da
# copia, 0xde bytes; o 0xdf-esimo e sobra de pilha na original).
SAVE = False
ROTINAS = [('mover_jogos_vizinhos_do_clube', 0x5ae9d0, [0x6a, 0xff, 0x68, 0x46, 0x98, 0x95, 0x00]),
           ('remarcar_jogos_perto_da_data', 0x5abb10, [0x6a, 0xff, 0x68, 0x40, 0x97, 0x95, 0x00]),
           ('noticia_do_jogo_remarcado', 0x5aac70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os jogos remarcados: os jogos vizinhos do clube movidos um dia, os jogos perto da data da seleção e a notícia do jogo remarcado',
        'the rescheduled fixtures: the club\'s neighbouring fixtures moved one day, the fixtures near the national team date and the fixture moved news',
        [('mover_jogos_vizinhos_do_clube', 'FIXTURE_MANAGER::move_nearby_club_fixtures', '0x5ae9d0', 'fix_man.cpp', 'thiscall, ret 4',
          'jogo não amistoso: para cada clube de nação marcada (+0x11c), o jogo dele dois dias antes (com +0x43 = 0xff) vai um dia antes, e o de dois dias depois vai um dia depois, se a data nova estiver livre (0x5ad780); tira, junta e dá a notícia',
          'non-friendly fixture: for each club of a flagged nation (+0x11c), its fixture two days before (with +0x43 = 0xff) moves one day earlier, and the one two days after moves one day later, if the new date is free (0x5ad780); removes, adds and posts the news'),
         ('remarcar_jogos_perto_da_data', 'FIXTURE_MANAGER::reschedule_club_fixtures_near_date', '0x5abb10', 'fix_man.cpp', 'thiscall, ret 8',
          'os jogos do clube de um dia antes a um dia depois da data da seleção: os que saem do calendário ganham data nova, voltam e dão a notícia com a data velha',
          'the club\'s fixtures from one day before to one day after the national team date: those removed from the calendar get a new date, are added back and post the news with the old date'),
         ('noticia_do_jogo_remarcado', 'FIXTURE_MANAGER::queue_fixture_moved_news', '0x5aac70', 'fix_man.cpp', 'thiscall, ret 0xc',
          'a fila de notícias em +0x1128 (criada na primeira vez): troca a data nova da notícia do mesmo jogo, ou põe a notícia 1 com o jogo, a causa ou a data velha',
          'the news queue at +0x1128 (created the first time): updates the new date of the same fixture\'s news, or queues news 1 with the fixture, the cause or the old date')])
CAMPOS = [('jogo (0x4f bytes) +0 id, +0xc, +0x10, +0x14 competição, +0x1c e +0x20 clubes, +0x28 ano e +0x2a dia, +0x30 e +0x32, +0x40 e +0x41, +0x43; id e valor; lê e grava a data na cópia local (mover_jogos_vizinhos_do_clube, noticia_do_jogo_remarcado)',
           'fixture (0x4f bytes) +0 id, +0xc, +0x10, +0x14 competition, +0x1c and +0x20 clubs, +0x28 year and +0x2a day, +0x30 and +0x32, +0x40 and +0x41, +0x43; id and value; reads and writes the date in the local copy (mover_jogos_vizinhos_do_clube, noticia_do_jogo_remarcado)'),
          ('clube +0x53 nação; nação +0x11c marca das remarcações; ponteiro e valor; lê (mover_jogos_vizinhos_do_clube)',
           'club +0x53 nation; nation +0x11c rescheduling flag; pointer and value; reads (mover_jogos_vizinhos_do_clube)'),
          ('FIXTURE_MANAGER +0x1128: fila das notícias de jogo remarcado (LLIST de itens de 0xdf bytes); ponteiro; cria e grava (noticia_do_jogo_remarcado)',
           'FIXTURE_MANAGER +0x1128: queue of fixture moved news (LLIST of 0xdf-byte items); pointer; creates and writes (noticia_do_jogo_remarcado)'),
          ('[0xae2c74] objeto dos amistosos; ponteiro; lê (mover_jogos_vizinhos_do_clube)',
           '[0xae2c74] friendlies object; pointer; reads (mover_jogos_vizinhos_do_clube)')]
INTERESSA = ['CMRTE']
