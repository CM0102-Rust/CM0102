# A fase de mata-mata das copas (Cup_Stage): o construtor e o destrutor (cup_stage.cpp, em cups\). Frente das
# competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x522e00 (Cup_Stage::Cup_Stage, 700 bytes; 53 chamadas até 2002-06-01 com as 8 ligas): a fase de mata-mata de todas
#   as copas com fases (os mata-matas italianos, o Troféu da Liga, os play-offs das Séries...).
# - 0x523300 (Cup_Stage::~Cup_Stage, 122 bytes; 23 chamadas): o destrutor (passageira).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas (lição da 258a): 0x522e00 thiscall, ret 0x3c (15 argumentos, conferidos nos pontos de chamada
# portados: 0x63f870, 0x56bab0, 0x56bce0...); 0x523300 thiscall, ret 0. Dentro: Cup::Cup 0x518640 e Cup::~Cup 0x518690
# (mov ecx -> thiscall, ret 0); 0x944e46 / 0x9452ca (push; add esp, 4 -> cdecl); calloc 0x945501 (push 6; push n; add
# esp, 8 -> cdecl); Cup_Stage::map_fixture_tree 0x5241e0 e Cup::map_fixture_tree 0x518790 (mov ecx -> thiscall, ret 0;
# devolução com sinal); a caixa (aviso_em).
# Comparadores: só a original (a base, as alocações e a árvore sorteada; o destrutor libera), o construtor conferindo o
# plano feito antes: this, +0x3e, +0x3c, +0x40, as tabelas montadas e os mesmos clubes na lista (a árvore, no fim,
# reescreve partes da tabela e da lista: na comparação de 06/11 a conferência byte a byte acusou à toa).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs7_fase_de_copa', 0x522e00, [0x6a, 0xff, 0x68, 0x3b, 0x78, 0x95, 0x00]),
           ('cqs7_fim_da_fase_de_copa', 0x523300, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0xa7, 0x00, 0x00, 0x00])]
DOCS = ('a fase de mata-mata das copas (Cup_Stage): o construtor e o destrutor',
        'the cups\' knockout stage (Cup_Stage): the constructor and the destructor',
        [('cqs7_fase_de_copa', 'Cup_Stage::Cup_Stage', '0x522e00', 'cup_stage.cpp', 'thiscall, ret 0x3c',
          'regra: uma fase de mata-mata de uma copa leva a tabela das rodadas copiada do modelo da copa (datas, sorteios), o ano, a fase, o id e as regras do pai (+0x49, +0x4a); cada time entra com a situação 0 e a semente do pote (dada pela copa, ou a posição na lista quando a copa pede a ordem, ou 0); a árvore dos confrontos é montada na hora: a da fase (Cup_Stage::map_fixture_tree, que liga os vencedores e põe as sementes na primeira rodada) ou a da copa (Cup::map_fixture_tree, que liga ao acaso)',
          'rule: a cup knockout stage takes the round table copied from the cup\'s template (dates, draws), the year, the stage, the id and the parent\'s rules (+0x49, +0x4a); each team enters with status 0 and its pot seed (given by the cup, or the list position when the cup asks for order, or 0); the fixture tree is built at once: the stage one (Cup_Stage::map_fixture_tree, which links the winners and places the seeds in the first round) or the cup one (Cup::map_fixture_tree, which links at random)'),
         ('cqs7_fim_da_fase_de_copa', 'Cup_Stage::~Cup_Stage', '0x523300', 'cup_stage.cpp', 'thiscall, ret 0',
          'regra: o fim de uma fase de mata-mata: a lista dos times, a tabela das rodadas e os confrontos de cada rodada liberados, e a base destruída',
          'rule: the end of a knockout stage: the team list, the round table and each round\'s fixtures freed, and the base destroyed')])
CAMPOS = [('fase +4, +0x28, +0x3a id, +0x3c rodadas, +0x3e times, +0x40 ano, +0x42, +0x43, +0x44 fase, +0x49, +0x4a, +0x51, +0x2c, +0x30, +0xab, +0xac, +0xb1; valor; grava (construtor)',
           'stage +4, +0x28, +0x3a id, +0x3c rounds, +0x3e teams, +0x40 year, +0x42, +0x43, +0x44 stage, +0x49, +0x4a, +0x51, +0x2c, +0x30, +0xab, +0xac, +0xb1; value; writes (constructor)'),
          ('fase +0xa3 tabela das rodadas (0x68 bytes por rodada), +0xa7 times (6 bytes: clube, +4 semente, +5 situação), +0xad confrontos por rodada; ponteiro; grava (construtor) e libera (destrutor)',
           'stage +0xa3 round table (0x68 bytes per round), +0xa7 teams (6 bytes: club, +4 seed, +5 status), +0xad fixtures per round; pointer; writes (constructor) and frees (destructor)')]
INTERESSA = ['CMRTE']
