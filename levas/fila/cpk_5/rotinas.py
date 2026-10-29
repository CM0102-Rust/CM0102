# Notícias para os técnicos humanos: para os de um continente (News::send_to_continent_managers 0x786640, irmã das
# 0x786520/0x7863f0/0x786780 da 245ª e 246ª; a 0x785d40 entrega), para a região de uma competição
# (News::send_to_comp_region 0x786860, pelas copas 0x518790 e notícias 0x4a2450/0x786090: nação, continente ou todos) e
# se a notícia é importante para a pessoa (NewsItem::is_important_for 0x784eb0, chamada pela entrega 0x785d40 para o
# byte +4; a categoria 0x785670 já em Rust). Simulação: as mutações pela carreira e a confirmação no jogo real
# comparando. Sem SEH, sem sorteio; caixa só com notícia, continente, competição ou pessoa nulos e tipo fora da faixa
# (a original). Comparadores: as entregas só pela original (mexem nas caixas dos técnicos); a importância pelo valor.
SAVE = False
ROTINAS = [('noticia_para_os_tecnicos_do_continente', 0x786640, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('noticia_para_a_regiao_da_competicao', 0x786860, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_importante_para_a_pessoa', 0x784eb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as notícias para os técnicos humanos de um continente e da região da competição, e se a notícia é importante para a pessoa',
        'the news for the human managers of a continent and of the competition region, and whether the news is important for the person',
        [('noticia_para_os_tecnicos_do_continente', 'News::send_to_continent_managers', '0x786640', 'news.cpp', 'thiscall, ret 8',
          'alcance do continente (+0xde = 3) e a entrega a cada técnico humano ativo cujo clube ou nação é do continente',
          'continent scope (+0xde = 3) and delivery to every active human manager whose club or nation is in the continent'),
         ('noticia_para_a_regiao_da_competicao', 'News::send_to_comp_region', '0x786860', 'news.cpp', 'thiscall, ret 8',
          'para os técnicos da nação da competição (0x786520), senão do continente (0x786640), senão para todos (0x786780)',
          "to the managers of the competition's nation (0x786520), else of its continent (0x786640), else to all (0x786780)"),
         ('noticia_importante_para_a_pessoa', 'NewsItem::is_important_for', '0x784eb0', 'news.cpp', 'thiscall, ret 4',
          'se a notícia é importante para a pessoa, pela categoria e o tipo (ofertas pelo clube dela, os humanos...)',
          "whether the news is important for the person, by category and type (bids on the person's club, the humans...)")])
CAMPOS = [('notícia +0 tipo, +5 dado 0, +0x15/+0x19 dados 4 e 5 (ids de clube), +0xde alcance; técnico humano (0x6e) +0x1a nação, +0x39 clube, +0x5f ativo; clube +0x53 nação, +0xcf técnico; nação +0x71 continente; competição +0x59 continente, +0x5d nação; lê (noticia_importante_para_a_pessoa, noticia_para_a_regiao_da_competicao); grava +0xde = 3 (noticia_para_os_tecnicos_do_continente)',
           'news +0 type, +5 slot 0, +0x15/+0x19 slots 4 and 5 (club ids), +0xde scope; human manager (0x6e) +0x1a nation, +0x39 club, +0x5f active; club +0x53 nation, +0xcf manager; nation +0x71 continent; competition +0x59 continent, +0x5d nation; reads (noticia_importante_para_a_pessoa, noticia_para_a_regiao_da_competicao); writes +0xde = 3 (noticia_para_os_tecnicos_do_continente)'),
          ('[0xae2364] quantidade de pessoas (os 16 últimos são os humanos); valor; lê (noticia_importante_para_a_pessoa)',
           '[0xae2364] people count (the last 16 are the humans); value; reads (noticia_importante_para_a_pessoa)')]
INTERESSA = ['CMRTE']
