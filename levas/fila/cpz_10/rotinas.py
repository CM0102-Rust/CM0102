# Os veiculos da imprensa (printouts.cpp, a lista de comentaristas do pundit.cfg que a imprensa usa, e media.cpp; a
# familia media.cpp/news.cpp esta esgotada de rotinas com efeito na simulacao, entao este item pega o sorteio do
# comentarista, que entra na contagem dos sorteios a cada historia da imprensa; nenhum outro agente usa printouts.cpp
# nesta rodada): PunditList::pick_random_pundit 0x7dab90 (pelas historias da imprensa 0x72c1f0 e 0x733a60 e pelos
# textos das noticias: os comentaristas do tipo, locais pelo clube ou pela nacao, e sorteio(n)), PunditList::
# get_pundit_name 0x7dad20 (pelos textos das noticias: o nome do comentarista ou o nome padrao do veiculo) e Media::
# queue_manager_fa_complaint_story 0x758de0 (pela disciplina 0x55e300 e pelo MANAGER_MANAGER 0x6a9e60/0x6ab800: a
# noticia 0x178e da queixa do tecnico a federacao). Tamanhos reais (candidatos.py): 395, 471 e 500 bytes. A 0x7dab90
# tem quadro de 0x1094 bytes (0x945610, sem SEH; os registros de 0xd4 bytes copiados para a pilha: a copia guarda so os
# ids); a 0x758de0 tem SEH so da noticia local (sem destrutor). Iguais no cm0102-1080.exe.
# Convencoes conferidas nos pontos de chamada (mov ecx antes = thiscall; add esp depois = cdecl; os push contados):
#   0x54a3f0 dia da semana thiscall em 0xae2c90 0 arg; 0x68d1b0 get_next thiscall 0 arg; 0x90cfc0 sorteio cdecl 1 arg;
#   0x944c9f sprintf cdecl 3 args (add esp,0xc); 0x77bbe0 NewsItem thiscall 2 args; 0x785790 set_slot thiscall 2 args;
#   0x7863f0 send_to_nation_managers thiscall em [0xb7431c] 2 args; 0x7861e0 send_to_club_manager thiscall em
#   [0xb7431c] 2 args. Os chamadores da 0x7dab90 (0x72cdba, 0x72cdd4, 0x733e9a) poem [0xb74430] em ecx e 3 push.
# Comparadores: o sorteio do comentarista e o nome lado a lado (cabecalhos das listas, sorteio, nome e marca; a
# devolucao); a queixa so a original (a noticia aloca; sem noticia as duas nao fazem nada).
SAVE = False
ROTINAS = [('sortear_comentarista', 0x7dab90, [0xb8, 0x94, 0x10, 0x00, 0x00]),
           ('nome_do_veiculo', 0x7dad20, [0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('noticia_queixa_a_federacao', 0x758de0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os veículos da imprensa: o comentarista sorteado, o nome do veículo e a notícia da queixa do técnico à federação',
        'the media outlets: the random pundit, the outlet name and the manager\'s FA complaint news',
        [('sortear_comentarista', 'PunditList::pick_random_pundit', '0x7dab90', 'printouts.cpp', 'thiscall, ret 0xc',
          'tipo 1 no domingo vira 7; tipos locais (2, 4): os comentaristas do clube; os outros: do tipo e da nação (−1 qualquer); até 20, sorteio(n) e o id, ou −1',
          'type 1 on Sunday becomes 7; local types (2, 4): the club\'s pundits; the others: of the type and nation (−1 any); up to 20, draw(n) and the id, or −1'),
         ('nome_do_veiculo', 'PunditList::get_pundit_name', '0x7dad20', 'printouts.cpp', 'thiscall, ret 0x10',
          'o nome (+2) e a marca (+0xd3) do comentarista do id, ou o nome padrão do tipo de veículo e marca 0',
          'the name (+2) and flag (+0xd3) of the pundit with the id, or the default outlet type name and flag 0'),
         ('noticia_queixa_a_federacao', 'Media::queue_manager_fa_complaint_story', '0x758de0', 'media.cpp', 'thiscall, ret 0x18',
          'clube com nação e técnico: a notícia 0x178e (clube, técnico, tipo, pessoa, a, b, nação, c) aos técnicos da nação se a reputação passar de 4750, senão ao clube',
          'club with a nation and manager: news 0x178e (club, manager, type, person, a, b, nation, c) to the nation\'s managers if the reputation is above 4750, else to the club')])
CAMPOS = [('PunditList +0 lista nacional, +4 lista local (LLIST de registros de 0xd4 bytes: +0 id, +2 nome, +0xca tipo, +0xcb clube, +0xcf nação, +0xd3 marca); id e valor; lê (sortear_comentarista, nome_do_veiculo)',
           'PunditList +0 national list, +4 local list (LLIST of 0xd4-byte records: +0 id, +2 name, +0xca type, +0xcb club, +0xcf nation, +0xd3 flag); id and value; reads (sortear_comentarista, nome_do_veiculo)'),
          ('clube +0x69 estádio (+0x38 o clube do estádio), +0x53 nação, +0x80 reputação, +0xcf técnico; ponteiro e valor; lê (sortear_comentarista, noticia_queixa_a_federacao)',
           'club +0x69 stadium (+0x38 the stadium\'s club), +0x53 nation, +0x80 reputation, +0xcf manager; pointer and value; reads (sortear_comentarista, noticia_queixa_a_federacao)'),
          ('[0xae2c90] hoje (o domingo); [0xb74430] a lista de comentaristas; valor e ponteiro; lê (sortear_comentarista)',
           '[0xae2c90] today (the Sunday); [0xb74430] the pundit list; value and pointer; reads (sortear_comentarista)')]
INTERESSA = []
