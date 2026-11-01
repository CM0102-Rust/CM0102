# O jogo da Liga dos Campeões (european_cup.cpp) e o conflito de datas da competição (comp.cpp):
# EuropeanCup::process_fixture (0x583b10, a virtual +4 da Liga dos Campeões, a cada jogo: os pontos do coeficiente
# da UEFA pelo placar e o prêmio da fase de grupos, antes da base da copa 0x51a150), EuropeanCup::set_club_progress
# (0x586c00, a virtual +0x68, da base 0x521b30: os três códigos de progresso do clube na estatística da competição;
# passageira) e Comp::counts_for_fixture_clash (0x48e360, a virtual +0x60 das competições, do conflito de datas
# 0x5adba0; passageira). Iguais no cm0102-1080.exe; sem SEH; sem x87. european_cup.cpp e comp.cpp não foram
# instrumentados na cobertura de 17/10; a 0x5adba0, que chama a +0x60, rodou 24.387 vezes.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x9058b0 (coeficiente): push 2 (ou 1); push [clube+0x53]; mov ecx, [0xde1f58]; call; sem add esp ->
#   thiscall(tabela, nação, pontos), ret 8 (0x9058da).
# - 0x5a0590 (finanças do clube): push valor; push clube; mov ecx, [0xae2a38]; call -> thiscall(tabela, clube),
#   ret 4 (o valor fica na pilha para a seguinte).
# - 0x5999a0 (somar): mov ecx, eax (as finanças); call -> thiscall(finanças, valor), ret 4 (0x5999d1).
# - 0x48e3e0 (notícia do prêmio): 7 push (−2, jogo, 0, −1, 0, valor, clube); mov ecx, ebx (this); sem add esp ->
#   thiscall(this, clube, valor, 0, −1, 0, jogo, −2), ret 0x1c (a mesma da cpx_11).
# - 0x51a150 (base da copa): push [esp+0x14] (arg); push jogo; mov ecx, ebx; sem add esp -> thiscall(this, jogo,
#   arg), ret 8.
# - 0x4a4850 (registro do clube na estatística): mov ecx, [ecx+8]; push clube; call; sem add esp -> thiscall(estat,
#   clube), ret 4 (o [esp+0x10] seguinte só fecha com ret 4).
# - 0x4b0850 (unidades): push rodada; call; add esp, 4 -> cdecl(n).
# - 0x53b3d0 (seleção da nação, só no comparador, como dentro da 0x9058b0): push; call; add esp, 4 -> cdecl(nação).
# Comparadores: a 0x583b10 em ensaio (a cópia sem a notícia e sem a base) contra a original, nas palavras do
# coeficiente das duas nações (as finanças são restauradas, mas não conferidas: a base paga o prêmio da rodada nos
# mesmos campos); quando a fase anda na base (o sorteio soma coeficiente), sem comparação. A 0x586c00 lado a lado em
# +0x73..+0x75 do registro do clube; a 0x48e360 pela devolução.
SAVE = False
ROTINAS = [('processar_jogo_da_liga_dos_campeoes', 0x583b10, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('progresso_do_clube_na_liga_dos_campeoes', 0x586c00, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('conta_para_conflito_de_datas', 0x48e360, [0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbf, 0x48, 0x30])]
DOCS = ('o jogo da Liga dos Campeões: o prêmio e o coeficiente do jogo, o progresso do clube e o conflito de datas da competição',
        "the Champions League match: the match prize and coefficient, the club's progress and the competition's date clash",
        [('processar_jogo_da_liga_dos_campeoes', 'EuropeanCup::process_fixture', '0x583b10', 'european_cup.cpp', 'thiscall, ret 8',
          'pelo placar (+0x47/+0x48, ou +0x43/+0x44), 2 pontos no coeficiente da UEFA da nação do vencedor ou 1 para cada no empate (0x9058b0); na fase de grupos (+0x42 de 0 a 11), 200 mil ao vencedor ou 100 mil a cada um, com a notícia do prêmio (0x48e3e0); depois a base da copa (0x51a150)',
          "by the score (+0x47/+0x48, or +0x43/+0x44), 2 UEFA coefficient points to the winner's nation or 1 each on a draw (0x9058b0); in the group stage (+0x42 from 0 to 11), 200 thousand to the winner or 100 thousand each, with the prize news item (0x48e3e0); then the cup base (0x51a150)"),
         ('progresso_do_clube_na_liga_dos_campeoes', 'EuropeanCup::set_club_progress', '0x586c00', 'european_cup.cpp', 'thiscall, ret 0x14',
          'converte os três códigos de progresso pela fase (qualificação, fase 1, fase 2) e grava em +0x73..+0x75 do registro do clube na estatística da competição',
          "converts the three progress codes by stage (qualifying, phase 1, phase 2) and writes them to +0x73..+0x75 of the club's competition stats entry"),
         ('conta_para_conflito_de_datas', 'Comp::counts_for_fixture_clash', '0x48e360', 'comp.cpp', 'thiscall, ret 8',
          '1 se a rodada do jogo (+0x30) termina em 0 ou passa de 999, senão 0',
          "1 when the match round (+0x30) ends in 0 or is above 999, else 0")])
CAMPOS = [('jogo +0x47 / +0x48 (ou +0x43 / +0x44 com +0x47 = −1): o placar; valor; lê (processar_jogo_da_liga_dos_campeoes)',
           'match +0x47 / +0x48 (or +0x43 / +0x44 when +0x47 = -1): the score; value; reads (processar_jogo_da_liga_dos_campeoes)'),
          ('jogo +0x42: a fase (0 a 11 a fase de grupos); valor; lê (processar_jogo_da_liga_dos_campeoes)',
           'match +0x42: the stage (0 to 11 the group stage); value; reads (processar_jogo_da_liga_dos_campeoes)'),
          ('jogo +0x30: a rodada; valor; lê (conta_para_conflito_de_datas)',
           'match +0x30: the round; value; reads (conta_para_conflito_de_datas)'),
          ('[0xde1f58] +0xf + 18 × índice: os pontos do coeficiente da UEFA da nação (o índice é +0x1df da seleção); valor; grava pela 0x9058b0 (processar_jogo_da_liga_dos_campeoes)',
           "[0xde1f58] +0xf + 18 x index: the nation's UEFA coefficient points (the index is +0x1df of the national team); value; writes through 0x9058b0 (processar_jogo_da_liga_dos_campeoes)"),
          ('finanças do clube +0 (64 bits), +0xa8, +0x148: o saldo e os acumulados de prêmios; valor; grava pela 0x5999a0 (processar_jogo_da_liga_dos_campeoes)',
           'club finance +0 (64 bits), +0xa8, +0x148: the balance and the prize totals; value; writes through 0x5999a0 (processar_jogo_da_liga_dos_campeoes)'),
          ('estatística da competição, registro do clube +0x73 / +0x74 / +0x75: os códigos de progresso; valor; grava (progresso_do_clube_na_liga_dos_campeoes)',
           'competition stats, club entry +0x73 / +0x74 / +0x75: the progress codes; value; writes (progresso_do_clube_na_liga_dos_campeoes)'),
          ('Liga dos Campeões +0x30: a fase atual; valor; lê no comparador (processar_jogo_da_liga_dos_campeoes)',
           'Champions League +0x30: the current stage; value; read by the comparer (processar_jogo_da_liga_dos_campeoes)')]
INTERESSA = ['CMRTE']
