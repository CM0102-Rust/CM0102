# A entrega das notícias (news.cpp): pelo alcance (News::send_by_scope 0x786090, 25417 chamadas na carreira até
# 2002-06-01: copas, lesões, ligas, técnicos, regens), ao técnico do clube (News::send_to_club_manager 0x7861e0, o caso
# 0 da anterior) e na caixa do técnico humano (News::deliver_to_manager 0x785d40: o anel de 100 vagas por humano, os ids
# e, com o técnico de férias, a resposta automática 0x784100: convites de amistoso, propostas de emprego e de
# transferência, convocações). Sem SEH, sem sorteio, sem leitura de lixo. A 0x785d40 realoca o vetor das notícias
# (0x945667) na primeira notícia de cada humano. As caixas "Error" (sem notícia, sem clube, sem técnico, alcance fora,
# sem memória) pela aviso_em. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira (o técnico de testes
# fica de férias e a resposta automática muda a carreira) e a confirmação no jogo real comparando. Comparadores: as
# três rodam só a original (a entrega não pode rodar duas vezes) e conferem o plano (o anel, os ids, o alcance).
SAVE = False
ROTINAS = [('noticia_pelo_alcance', 0x786090, [0x0f, 0xbe, 0x44, 0x24, 0x18]),
           ('noticia_ao_tecnico_do_clube', 0x7861e0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('entregar_noticia_ao_tecnico', 0x785d40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a entrega das notícias: pelo alcance, ao técnico do clube e na caixa do técnico humano',
        'news delivery: by scope, to the club manager and into the human manager\'s inbox',
        [('noticia_pelo_alcance', 'News::send_by_scope', '0x786090', 'news.cpp', 'thiscall, ret 0x18',
          'entrega a notícia pelo alcance: 0 o técnico do clube, 1 os da divisão, 2 os do país, 3 os do continente, 4 todos, 5 os da região da competição; fora disso a caixa "Error"',
          'delivers the news item by scope: 0 the club manager, 1 the division\'s managers, 2 the country\'s, 3 the continent\'s, 4 all, 5 the competition region\'s; anything else shows the "Error" box'),
         ('noticia_ao_tecnico_do_clube', 'News::send_to_club_manager', '0x7861e0', 'news.cpp', 'thiscall, ret 8',
          'marca o alcance como clube (+0xde = 0) e entrega a notícia ao técnico do clube (+0xcf) se ele for humano (entre as 16 últimas pessoas)',
          'marks the scope as club (+0xde = 0) and delivers the item to the club manager (+0xcf) when he is human (among the last 16 people)'),
         ('entregar_noticia_ao_tecnico', 'News::deliver_to_manager', '0x785d40', 'news.cpp', 'thiscall, ret 0xc',
          'copia a notícia no anel de 100 vagas do humano (criado na primeira, realocando o vetor), dá o id, não repete a última e, com o técnico de férias, dá a resposta automática pelo clube e pela seleção',
          'copies the item into the human\'s 100-slot ring (created on the first, reallocating the array), gives it an id, skips a repeat of the last one and, with the manager on holiday, applies the automatic reply for the club and the national team')])
CAMPOS = [('News +0/+4: o vetor das notícias guardadas (0xdf bytes cada) e a conta de vagas; ponteiro e valor; lê e grava (entregar_noticia_ao_tecnico)',
           'News +0/+4: the stored news array (0xdf bytes each) and the slot count; pointer and value; reads and writes (entregar_noticia_ao_tecnico)'),
          ('News +8: as entradas dos 16 humanos (12 bytes: início do anel, próxima vaga, conta; −1 sem anel), pelo id − [0xae2364] + 16; ponteiro e valor; lê e grava (entregar_noticia_ao_tecnico)',
           'News +8: the 16 humans\' entries (12 bytes: ring start, next slot, count; −1 without a ring), by id − [0xae2364] + 16; pointer and value; reads and writes (entregar_noticia_ao_tecnico)'),
          ('News +0x10: o último id de notícia; valor; lê e grava (entregar_noticia_ao_tecnico)',
           'News +0x10: the last news id; value; reads and writes (entregar_noticia_ao_tecnico)'),
          ('notícia +4, +0xd6, +0xda, +0xde: importante, id, livre/lida, alcance (0 clube); valor; grava (entregar_noticia_ao_tecnico, noticia_ao_tecnico_do_clube)',
           'news item +4, +0xd6, +0xda, +0xde: important, id, free/read, scope (0 club); value; writes (entregar_noticia_ao_tecnico, noticia_ao_tecnico_do_clube)'),
          ('clube +0xcf: o técnico; ponteiro; lê (noticia_ao_tecnico_do_clube)',
           'club +0xcf: the manager; pointer; reads (noticia_ao_tecnico_do_clube)'),
          ('pessoa +0, +0x24, +0x39: id, a seleção e o clube que ele treina; id e ponteiro; lê (entregar_noticia_ao_tecnico)',
           'person +0, +0x24, +0x39: id, the national team and the club he manages; id and pointer; reads (entregar_noticia_ao_tecnico)'),
          ('[0xae2364] o número de pessoas (os humanos são as 16 últimas); [0xdd6bec] todos de férias; valor; lê',
           '[0xae2364] the number of people (humans are the last 16); [0xdd6bec] everyone on holiday; value; reads')]
INTERESSA = ['CMRTE']
