# A inscrição das três copas europeias (0x85a9b0, squad_manager.cpp: 3 chamadas até 2002-06-01 com 8 ligas, a etapa 3
# em 2002, pela classificação da UEFA 0x9059c5), a tabela inicial do ranking da FIFA (0x58dd50, fifa_rankings.cpp: 1, na
# criação do jogo) e a lista dos estádios para jogos em campo neutro (0x85dfc0, stadium.cpp: 3) (docs/
# cobertura_competicoes.txt). As três inteiras iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x85a9b0: em 0x9059c5, push 3; mov ecx, [0xdd7ef0]; call -> thiscall(this, etapa), ret 4 (só quando o ano não é 2001).
# - 0x58dd50: em 0x58cf06, mov ecx, esi; call (sem push) -> thiscall(this), ret 0.
# - 0x85dfc0: em 0x85b7da (mov ecx, edi) e 0x85b838 (ecx = this de quem chama); sem push -> thiscall(this), ret 0.
# - dentro: o método +0x58 da competição (push etapa; push &lista; mov ecx, comp -> thiscall de 2, devolve ax); 0x7875e0
#   (mov ecx, [0xb7431c]; push clube; ret 4: chamada como thiscall com o mesmo ecx); 0x85b020 (push etapa; push id;
#   push clube; mov ecx, this -> thiscall de 3, original: a notícia da inscrição ao técnico humano); 0x85aae0 (push 0;
#   push etapa; push clube; mov ecx, [0xdd7ef0] -> thiscall de 3); 0x9452ca (cdecl de 1); 0x945501 (push 0x2c; push
#   conta; add esp, 8 -> cdecl de 2); 0x58d610 (push &conta; mov ecx, this -> thiscall de 1, devolve a lista); 0x5fa7d0
#   (push &b; push &a; push id; mov ecx, [0xb5e134] -> thiscall de 3, devolve al); 0x944e46 (cdecl de 1); 0x945667
#   (push tamanho; push lista; add esp, 8 -> cdecl de 2); 0x945333 (qsort, cdecl de 4 com a 0x85e2f0); a caixa (aviso_em).
# Comparadores: 0x85a9b0 lado a lado no ano das inscrições (+0), na marca +0x6d da tabela inteira das pessoas com
# contrato e no [0xae1080] (só a original quando um clube das três copas tem técnico humano: a notícia); 0x58dd50 e
# 0x85dfc0 lado a lado à mão (a original aloca no objeto de quem chama, a cópia num rascunho; iguais as tabelas/listas
# e os campos; as do rascunho são soltas; 0x85dfc0 sem o HostCountry, só a original).
# A 0x58dd50 vai de passageira: o ranking da FIFA só alimenta telas e o histórico (lição da 256ª, cpt_1); as
# mutações estão na inscrição e na lista dos estádios.
# Determinismo: as contas da 0x58dd50 são as da original em f64 (a força × 1/35, × 1000 truncado como o _ftol); nada lê
# pilha, heap ou endereço como valor (a lista dos estádios guarda ponteiros de estádio, como a original, e é ordenada
# pela 0x85e2f0 da original).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('inscricao_europeia', 0x85a9b0, [0x83, 0xec, 0x10, 0x8a, 0x44, 0x24, 0x14]),
           ('ranking_fifa_inicial', 0x58dd50, [0xa1, 0x50, 0x23, 0xae, 0x00]),
           ('lista_de_estadios_neutros', 0x85dfc0, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00])]
DOCS = ('a inscrição das três copas europeias, a tabela inicial do ranking da FIFA e a lista dos estádios neutros',
        'the registration for the three European cups, the initial FIFA ranking table and the list of neutral venues',
        [('inscricao_europeia', 'SQUAD_MANAGER::process_euro_registration', '0x85a9b0', 'squad_manager.cpp', 'thiscall, ret 4',
          'a regra: na primeira etapa as inscrições de todos são zeradas; em cada etapa, cada clube que joga a Liga dos Campeões, a Copa da UEFA ou a terceira copa [inferência: a Recopa] escolhe os inscritos (os clubes de técnico humano recebem a notícia e escolhem eles); na carreira, a etapa 3 roda em 2002 (fora de 2001)',
          'the rule: at the first stage everyone\'s registration is cleared; at each stage each club in the Champions League, the UEFA Cup or the third cup [inference: the Cup Winners\' Cup] picks its registered players (human-managed clubs get the news and pick themselves); in the career, stage 3 runs in 2002 (not in 2001)'),
         ('ranking_fifa_inicial', 'FIFA_RANKINGS::create_default_table', '0x58dd50', 'fifa_rankings.cpp', 'thiscall, ret 0',
          'a regra do ranking inicial da FIFA: os pontos de cada nação são a força dela no banco ÷ 35, repetidos como os pontos de cada um dos 6 anos [inferência] (× 1000); a posição sai da ordem dos pontos',
          'the initial FIFA ranking rule: each nation\'s points are its database strength ÷ 35, repeated as the points of each of the 6 years [inference] (× 1000); the position comes from the points order'),
         ('lista_de_estadios_neutros', 'Stadium::build_venue_list', '0x85dfc0', 'stadium.cpp', 'thiscall, ret 0',
          'a regra dos campos neutros (finais e semifinais): servem os estádios com cidade, nação e continente, menos o City of Manchester Stadium (em obras) e, nas nações com competições, os de menos de 5000 lugares; o estádio nacional de cada nação fica marcado',
          'the neutral venue rule (finals and semi-finals): grounds with a city, nation and continent qualify, except the City of Manchester Stadium (under construction) and, in nations with competitions, those under 5000 seats; each nation\'s national stadium is flagged')])
CAMPOS = [('pessoa (registros de 0x6e bytes em [0xae23bc]) +0x6d marca de inscrição europeia; valor; grava 0 na etapa 1 (inscricao_europeia)',
           'person (0x6e-byte records at [0xae23bc]) +0x6d European registration mark; value; writes 0 at stage 1 (inscricao_europeia)'),
          ('clube +0x1db competição europeia; id; lê (inscricao_europeia); SQUAD_MANAGER +0 ano das inscrições; valor; grava (inscricao_europeia)',
           'club +0x1db European competition; id; reads (inscricao_europeia); SQUAD_MANAGER +0 registration year; value; writes (inscricao_europeia)'),
          ('nação (0x122 bytes em [0xae23a8]) +0xa8 força (double); valor; lê (ranking_fifa_inicial); FIFA_RANKINGS +0 a tabela (0x2c bytes por nação: +0 índice, +0xc pontos float, +0x10 posição, +0x12 posição anterior, +0x14..+0x28 6 dwords de pontos); ponteiro e valor; grava (ranking_fifa_inicial)',
           'nation (0x122 bytes at [0xae23a8]) +0xa8 strength (double); value; reads (ranking_fifa_inicial); FIFA_RANKINGS +0 the table (0x2c bytes per nation: +0 index, +0xc float points, +0x10 position, +0x12 previous position, +0x14..+0x28 6 dwords of points); pointer and value; writes (ranking_fifa_inicial)'),
          ('estádio (0x4e bytes em [0xae23b0], conta [0xae2358]) +4 nome, +0x38 cidade, +0x3c lugares; cidade +0x1f nação; nação +0x71 continente, +0x80 estádio nacional, +0x11c tem competições; valor e ponteiro; lê (lista_de_estadios_neutros)',
           'stadium (0x4e bytes at [0xae23b0], count [0xae2358]) +4 name, +0x38 city, +0x3c capacity; city +0x1f nation; nation +0x71 continent, +0x80 national stadium, +0x11c has competitions; value and pointer; reads (lista_de_estadios_neutros)'),
          ('Stadium +0xc a lista dos campos neutros (16 bytes: nação, continente, nacional, estádio), +0x10 conta, +0x14 capacidade, +0x18 pronta; ponteiro e valor; grava (lista_de_estadios_neutros)',
           'Stadium +0xc neutral venue list (16 bytes: nation, continent, national, stadium), +0x10 count, +0x14 capacity, +0x18 ready; pointer and value; writes (lista_de_estadios_neutros)')]
INTERESSA = ['CMRTE', 'SGE']
