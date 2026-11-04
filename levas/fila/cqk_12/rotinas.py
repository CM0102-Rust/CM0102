# A Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico da temporada (african_nations.cpp).
# Frente das competições (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x401e10 (AfricanNations::advance_stage, 144 bytes, virtual +0x28; 1 chamada até 2002-06-01 com as 8 ligas): a fase
#   seguinte e o mata-mata depois das três fases de grupos. Uma mutação.
# - 0x4019c0 (AfricanNations::init_group_a_table, 227 bytes; 1 chamada): a tabela do grupo A com as quatro primeiras
#   seleções sorteadas e a notícia do sorteio. Duas mutações.
# - 0x401b90 (AfricanNations::record_season_honours, 11 bytes, virtual +0x30; 4 chamadas): o salto para o histórico do
#   mata-mata. Passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x401e10 virtual +0x28 (mov ecx, copa; sem push -> thiscall, ret 0; chama
# 0x402090 com ecx = this -> thiscall, ret 0); 0x4019c0 (quem chama a monta pela fase de grupos: mov ecx; sem push ->
# thiscall, ret 0; dentro: 0x944e46 cdecl com push 0xec; 0x687430 push 0; push 0; push seleção; push i; mov ecx, this
# -> thiscall, ret 0x10, o i num dword cujos 3 bytes altos são sobra de pilha: o chamado lê o char, a cópia passa
# limpo; 0x684230 mov ecx, this -> thiscall, ret 0); 0x401b90 virtual +0x30 (mov ecx; sem push -> thiscall; salta
# para a virtual +0x30 da fase 3 com ecx = a fase).
# Comparadores: só a original nas três (o mata-mata aloca e sorteia; a tabela aloca e a notícia vai aos técnicos; o
# histórico tem o comparador dele), com o plano: +0x30 sobe um; 4 times e as linhas com as seleções sorteadas na ordem.
# Determinismo: nada lê pilha, heap ou endereço como valor (o índice da linha vai limpo).
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: o grupo A com três e as linhas trocadas ACUSADAS; o mata-mata uma fase antes derruba o jogo
#   (0xC0000029, o jogo sai sem chegar à data): acusada pela saída, não pela impressão.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqk12_africana_fase_seguinte', 0x401e10, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqk12_africana_grupo_a', 0x4019c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqk12_africana_historico', 0x401b90, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x0c])]
DOCS = ('a Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico',
        'the African Nations Cup: the next stage, the group A table and the season honours',
        [('cqk12_africana_fase_seguinte', 'AfricanNations::advance_stage', '0x401e10', 'african_nations.cpp', 'thiscall, ret 0',
          'regra: a Copa Africana tem três fases de grupos (as fases 0 a 2, uma por rodada do grupo [inferência]) e, na fase 3, o mata-mata; passar da última fase é erro (a caixa)',
          'rule: the African Nations Cup has three group stages (stages 0 to 2, one per group round [inference]) and, at stage 3, the knockout; going past the last stage is an error (the box)'),
         ('cqk12_africana_grupo_a', 'AfricanNations::init_group_a_table', '0x4019c0', 'african_nations.cpp', 'thiscall, ret 0',
          'regra: o grupo A tem quatro seleções, as quatro primeiras da lista do sorteio, na ordem; os técnicos recebem a notícia do sorteio',
          'rule: group A has four national teams, the first four of the draw list, in order; the managers get the draw news'),
         ('cqk12_africana_historico', 'AfricanNations::record_season_honours', '0x401b90', 'african_nations.cpp', 'thiscall, ret 0',
          'regra: o histórico da temporada (campeão, vice, terceiro) é o do mata-mata',
          'rule: the season honours (winner, runner-up, third) are the knockout ones')])
CAMPOS = [('competição +0x2c fases, +0x30 fase atual, +0xc fases (a 3: o mata-mata); valor/ponteiro; lê e grava +0x30 (cqk12_africana_fase_seguinte, cqk12_africana_historico)',
           'competition +0x2c stages, +0x30 current stage, +0xc stages (stage 3: the knockout); value/pointer; reads and writes +0x30 (cqk12_africana_fase_seguinte, cqk12_africana_historico)'),
          ('Copa Africana +0xee lista das seleções sorteadas (ponteiros), +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 seleção, +4 posição); ponteiro/valor; lê e grava (cqk12_africana_grupo_a)',
           'African Nations Cup +0xee drawn teams list (pointers), +0x3e teams, +0xb1 table (0x3b-byte rows: +0 team, +4 position); pointer/value; reads and writes (cqk12_africana_grupo_a)')]
INTERESSA = []
