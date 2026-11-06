# O Troféu da Liga inglesa (LDV Vans Trophy): os 56 clubes das duas chaves, a fase das chaves e a fase seguinte
# (eng_auto_cup.cpp, em cups\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de
# 2001).
# - 0x56b730 (EngAutoCup::select_section_clubs, 889 bytes): os 56 clubes (a segunda e a terceira divisões, o rebaixado
#   para a Conference e os clubes da Conference ou ingleses de fora, embaralhados), ordenados em Norte e Sul.
# - 0x56bab0 (EngAutoCup::create_section_stage, 405 bytes): a fase das chaves (Cup_Stage) com os 56.
# - 0x56bc50 (EngAutoCup::advance_stage, 142 bytes): a fase seguinte; depois das chaves, a final.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três thiscall, ret 0. Dentro: 0x944e46 / 0x944cf1 / 0x9452ca (push; add
# esp, 4 -> cdecl); comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl; as palavras vão em
# dwords com lixo em cima); qsort 0x945333 (4 pushes; add esp junto -> cdecl) com 0x53fd40 (latitude) e 0x4b0810
# (reputação, crescente); o método +0x3c da copa (push 0; push &extra; push &rodadas; push 0; mov ecx -> thiscall, ret
# 0x10); Cup_Stage::Cup_Stage 0x522e00 (15 pushes; mov ecx, nova -> thiscall, ret 0x3c); a final 0x56bce0 (call direto com
# o ecx da copa -> thiscall, ret 0); a caixa (aviso_em).
# Comparadores: os clubes lado a lado a partir do mesmo estado (o sorteador, +0xb6, [0xb67a34]), a lista da cópia (sem
# contar os sorteios) comparada nas 56 vagas e liberada; sem rebaixado (a original lê uma dword da pilha sem valor; a
# cópia usa 0), só a original. A fase das chaves e a fase seguinte, só a original (alocam e sorteiam), com o plano.
# Determinismo: nada lê pilha, heap ou endereço como valor (o caso sem rebaixado vai à original no comparador e usa 0 na
# cópia).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs5_trofeu_clubes', 0x56b730, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('cqs5_trofeu_fase_das_chaves', 0x56bab0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqs5_trofeu_fase_seguinte', 0x56bc50, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('o Troféu da Liga inglesa: os 56 clubes das chaves Norte e Sul, a fase das chaves e a fase seguinte',
        'the English Football League Trophy: the 56 clubs of the North and South sections, the section stage and the next stage',
        [('cqs5_trofeu_clubes', 'EngAutoCup::select_section_clubs', '0x56b730', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: o Troféu da Liga (LDV Vans) tem 56 clubes: todos os da segunda e da terceira divisões, o rebaixado da terceira para a Conference e, para completar, clubes da Conference (com a regra da nação ligada, os que já estavam nela pela categoria, 6 ou menos primeiro; senão até 30 da Conference ou outros ingleses fora da divisão genérica, ordenados pela reputação e com os melhores embaralhados); a lista é dividida em Norte e Sul pela latitude da cidade do estádio e cada metade vai da menor reputação para a maior',
          'rule: the League Trophy (LDV Vans) has 56 clubs: all of the second and third divisions, the club relegated from the third to the Conference and, to complete it, Conference clubs (with the nation rule on, those already in it by category, 6 or less first; otherwise up to 30 Conference clubs or other English clubs outside the generic division, sorted by reputation with the best shuffled); the list is split into North and South by the latitude of the stadium city and each half goes from lowest reputation to highest'),
         ('cqs5_trofeu_fase_das_chaves', 'EngAutoCup::create_section_stage', '0x56bab0', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: a fase das chaves do Troféu é um mata-mata (Cup_Stage) com os 56 na ordem da lista (Norte, depois Sul), com as rodadas da fase 0 da tabela da copa, sem cabeças de chave',
          'rule: the Trophy section stage is a knockout (Cup_Stage) with the 56 in list order (North, then South), with the rounds of stage 0 of the cup table, no seeding'),
         ('cqs5_trofeu_fase_seguinte', 'EngAutoCup::advance_stage', '0x56bc50', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: a fase do Troféu avança uma; quando as chaves acabam (fase 1), monta a final entre o campeão do Norte e o do Sul; na última fase, a caixa de erro',
          'rule: the Trophy stage moves on by one; when the sections end (stage 1), it builds the final between the North and South winners; at the last stage, the error box')])
CAMPOS = [('copa +0xb2 lista dos clubes, +0xb6 quantos (56); ponteiro/valor; grava (clubes) e lê (fase das chaves)',
           'cup +0xb2 club list, +0xb6 how many (56); pointer/value; writes (clubs) and reads (section stage)'),
          ('clube +0x57 divisão, +0x5b divisão da temporada anterior [inferência], +0x53 nação; ponteiro; lê',
           'club +0x57 division, +0x5b previous season division [inference], +0x53 nation; pointer; reads'),
          ('clube +0x5f categoria (byte com sinal) [inferência]; valor; lê', 'club +0x5f category (signed byte) [inference]; value; reads'),
          ('nação +0x11c bit 4 (a regra da Conference por categoria) [inferência]; valor; lê', 'nation +0x11c bit 4 (the Conference-by-category rule) [inference]; value; reads'),
          ('[0x9cf5c4], [0x9cf5c8], [0x9cf69c], [0x9cf764], [0x9cf2e4]: a segunda e a terceira divisões, a Conference, a divisão genérica e a Inglaterra [inferência]; id; lê',
           '[0x9cf5c4], [0x9cf5c8], [0x9cf69c], [0x9cf764], [0x9cf2e4]: the second and third divisions, the Conference, the generic division and England [inference]; id; reads'),
          ('copa +0xc fases, +0x2c fases, +0x30 fase atual; ponteiro/valor; grava', 'cup +0xc stages, +0x2c stages, +0x30 current stage; pointer/value; writes')]
INTERESSA = []
