# Do index.cpp, os textos que faltavam: o nome da região (0x616f20: a ordem das regiões 0x545dc0, já em Rust, a
# busca de jogadores e telas sem módulo) e o adjetivo do continente (0x6177f0: busca find_screens, Menubar, jogos do
# dia 0x717cc0, já em Rust; o mapa dá 629 bytes, a rotina vai até 0x6181c5). Simulação (SAVE = False), rotinas só
# de tela. Sem SEH, sem sorteio; as locais de 0x200 bytes da 0x616f20 só servem à caixa (região fora de 1..14, texto
# nulo, tamanho < 100), que fica com a original; a 0x6177f0 não tem locais. As tabelas de textos por idioma da
# 0x616f20 ([0x9d0518], [0x9d06d4], [0x9d05f8]) são lidas pelo endereço; os outros textos são imediatos do código. A
# forma do adjetivo (0x545a60, em Rust) é chamada pelo endereço do jogo.
SAVE = False
ROTINAS = [('nome_da_regiao', 0x616f20, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('adjetivo_do_continente', 0x6177f0, [0x55, 0x8b, 0x6c, 0x24, 0x0c])]
DOCS = ('o nome da região e o adjetivo do continente',
        'the region name and the continent adjective',
        [('nome_da_regiao', 'get_region_name', '0x616f20', 'index.cpp', 'cdecl',
          'o nome da região (1..14): pelas tabelas dos idiomas 4, 5 e 9 com a forma do adjetivo, senão o adjetivo ou o substantivo traduzido pela forma',
          'the region name (1..14): from the language 4, 5 and 9 tables with the adjective form, else the translated adjective or noun by form'),
         ('adjetivo_do_continente', 'get_continent_adjective', '0x6177f0', 'index.cpp', 'cdecl',
          'o adjetivo do continente: o do registro (+0x23) ou, nos idiomas 9, 5 e 4, o texto pelo continente e pela forma do adjetivo',
          'the continent adjective: the record one (+0x23) or, in languages 9, 5 and 4, the text by continent and adjective form')])
CAMPOS = [('[0x9d0518], [0x9d06d4], [0x9d05f8]: tabelas dos nomes das regiões por idioma; [0x9eb2dc] idioma; lê (nome_da_regiao)',
           '[0x9d0518], [0x9d06d4], [0x9d05f8]: region name tables by language; [0x9eb2dc] language; reads (nome_da_regiao)'),
          ('continente +0 id, +0x23 adjetivo; [0x9cfa08..0x9cfa18] chaves dos continentes; lê (adjetivo_do_continente)',
           'continent +0 id, +0x23 adjective; [0x9cfa08..0x9cfa18] continent keys; reads (adjetivo_do_continente)')]
INTERESSA = ['SGE', 'CMRTE']
