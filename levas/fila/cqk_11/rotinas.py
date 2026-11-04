# A Libertadores: os 32 classificados, o destino de cada clube (o campeão vai ao Mundial Interclubes) e a fase seguinte
# (conmebol_liber.cpp). Frente das competições (Agente D, 03/11/2026), carreira de 8 ligas (o Brasil é uma delas).
# - 0x4c3f40 (ConmebolLiber::select_qualified_clubs, 317 bytes; 2 chamadas até 2002-06-01 com as 8 ligas): os 32
#   clubes marcados para a Libertadores, embaralhados. Uma mutação.
# - 0x4c4080 (ConmebolLiber::update_staff_records_for_status, 400 bytes, virtual +0x44; 118 chamadas): os registros de
#   cada clube pelo desfecho e o campeão na vaga do Mundial Interclubes. Uma mutação (só roda se a final cair na
#   janela).
# - 0x4c3a80 (ConmebolLiber::advance_stage, 144 bytes, virtual +0x28; 2 chamadas): a fase seguinte e o mata-mata depois
#   dos sete grupos. Uma mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   0x4c3f40: quem a chama (a montagem da copa) mov ecx; sem push -> thiscall, ret 0. Dentro: 0x944e46 (push 0x80; add
#   esp, 4 -> cdecl), 0x4afb40 (push n (cx com lixo nos 16 bits altos; o chamado lê a palavra: a cópia passa limpo);
#   push lista; add esp, 8 -> cdecl), a caixa (aviso_em).
#   0x4c4080: virtual +0x44 (6 pushes -> thiscall, ret 0x18). Dentro: 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl),
#   0x86bdd0 / 0x86c000 (5 pushes; mov ecx, [0xdd7f00] -> thiscall, ret 0x14), 0x86c1d0 (4 pushes -> thiscall, ret
#   0x10), 0x86a800 / 0x86b0b0 (push a6; push jogo; push clube; mov ecx, [0xdd7f00] -> thiscall, ret 0xc), 0x4a1c50
#   (push 1; mov ecx, [mundial+8] -> thiscall, ret 4), a virtual +0x5c do Mundial (mov ecx -> thiscall, ret 0). Na
#   classificação do mata-mata, a rodada vai num registrador cujos 16 bits altos são os do ponteiro da fase 7: a cópia
#   reproduz; +0x30 na eliminação vai com lixo nos bits altos (o chamado lê a palavra): a cópia passa limpo.
#   0x4c3a80: virtual +0x28 (mov ecx; sem push -> thiscall, ret 0). Dentro: 0x4c3d10 (ecx = this -> thiscall, ret 0),
#   a caixa da linha 0x1f6.
# Comparadores: 0x4c3f40 lado a lado com as duas listas (a original aloca a dela e a cópia a sua, que é comparada e
# liberada; o estado do sorteador volta e a cópia sorteia sem contar); 0x4c4080 só a original (os registros) com o
# plano (o campeão na vaga do Mundial); 0x4c3a80 só a original (o mata-mata aloca e sorteia) com o plano (+0x30 sobe).
# Determinismo: nada lê pilha, heap ou endereço como valor (os bits altos da rodada são os do ponteiro, como na
# original, e o chamado só lê a palavra).
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: embaralhar 31 e os classificados trocados aos pares ACUSADAS; o campeão na outra vaga do Mundial
#   Interclubes ESCAPA (o ramo roda uma vez, mas o jogo do Mundial não cai na janela ou não muda nada lido).
#   O mata-mata uma fase antes derrubou o jogo (0xC0000005): trocada.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqk11_libertadores_classificados', 0x4c3f40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqk11_libertadores_registros', 0x4c4080, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqk11_libertadores_fase_seguinte', 0x4c3a80, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Libertadores: os classificados, o destino de cada clube e a fase seguinte',
        'the Copa Libertadores: the qualified clubs, each club\'s fate and the next stage',
        [('cqk11_libertadores_classificados', 'ConmebolLiber::select_qualified_clubs', '0x4c3f40', 'conmebol_liber.cpp', 'thiscall, ret 0',
          'regra: entram na Libertadores os 32 clubes marcados com ela como competição continental da temporada (a marca vem da classificação nas ligas), na ordem do banco, depois embaralhados (96 trocas sorteadas) para os grupos; com outro número de clubes, erro (a caixa)',
          'rule: the Libertadores takes the 32 clubs marked with it as the season\'s continental competition (the mark comes from league qualification), in database order, then shuffled (96 random swaps) for the groups; any other count is an error (the box)'),
         ('cqk11_libertadores_registros', 'ConmebolLiber::update_staff_records_for_status', '0x4c4080', 'conmebol_liber.cpp', 'thiscall, ret 0x18',
          'regra: nos sete grupos (fases 0 a 6), quem passa ganha o registro de classificação (peso 40) e quem não passa o de não classificação (peso 40); no mata-mata (fase 7), quem passa ganha a classificação pela rodada (peso de 50 a 100), quem cai a eliminação; o campeão ganha o registro de título e vai ao Mundial Interclubes, na vaga sul-americana (a 0, se lá já está um clube do continente [0x9cfa1c], senão a 1), com o arquivo da temporada do Mundial e as posições previstas; o vice ganha o registro de vice',
          'rule: in the seven groups (stages 0 to 6), a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7), a club going through gets the qualification by round (weight 50 to 100), one going out the knocked-out record; the winner gets the title record and goes to the Intercontinental Cup, in the South American place (place 0 if a club of continent [0x9cfa1c] is there, otherwise place 1), with the Intercontinental season archive and predicted positions; the runner-up gets the runner-up record'),
         ('cqk11_libertadores_fase_seguinte', 'ConmebolLiber::advance_stage', '0x4c3a80', 'conmebol_liber.cpp', 'thiscall, ret 0',
          'regra: a Libertadores tem sete fases de grupos (0 a 6) e o mata-mata na fase 7 (os dois primeiros de cada grupo, 16 clubes); passar da última fase é erro (a caixa)',
          'rule: the Libertadores has seven group stages (0 to 6) and the knockout at stage 7 (the top two of each group, 16 clubs); going past the last stage is an error (the box)')])
CAMPOS = [('clube +0x1db competição continental da temporada; id; lê (cqk11_libertadores_classificados)',
           'club +0x1db continental competition of the season; id; reads (cqk11_libertadores_classificados)'),
          ('Libertadores +0xee lista dos classificados (ponteiros), +0xf2 quantos (32); ponteiro/valor; grava (cqk11_libertadores_classificados)',
           'Libertadores +0xee qualified list (pointers), +0xf2 count (32); pointer/value; writes (cqk11_libertadores_classificados)'),
          ('Mundial Interclubes ([0x9cf6e4]) +0xa7 times (6 bytes; vaga 0 e 1), +8 estatísticas; nação +0x71 continente (+0 id), [0x9cfa1c] continente sul-americano; ponteiro/id; lê e grava a vaga (cqk11_libertadores_registros)',
           'Intercontinental Cup ([0x9cf6e4]) +0xa7 teams (6 bytes; places 0 and 1), +8 statistics; nation +0x71 continent (+0 id), [0x9cfa1c] South American continent; pointer/id; reads and writes the place (cqk11_libertadores_registros)'),
          ('competição +0x2c fases, +0x30 fase atual; valor; lê e grava +0x30 (cqk11_libertadores_fase_seguinte)',
           'competition +0x2c stages, +0x30 current stage; value; reads and writes +0x30 (cqk11_libertadores_fase_seguinte)')]
INTERESSA = ['CMRTE']
