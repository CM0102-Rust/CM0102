# Três comparações de qsort (puras: comparador lado a lado sem regiões, original x cópia na resposta). Na
# criação: o vetor de clubes pela reputação (0x53db40 com 0x540020; o vetor vai para o save) e o corte dos
# conjuntos A+B a 48 (0x541e60 com 0x5402f0); na temporada: as seleções da MLS (0x5402f0) e a força do
# elenco da IA (0x549a30 com 0x549c30, todo dia). A gravação do .qst da prova do save cria o jogo.
# Troca: a poda 0x539ae0 saiu (está no dbk_2, escrito ao mesmo tempo por outro agente, junto com 0x53db40);
# entrou 0x549c30, a outra comparação de elenco do Database.cpp.
SAVE = True
ROTINAS = [('ordem_dos_clubes_pela_reputacao', 0x540020, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('ordem_do_elenco_pela_habilidade', 0x5402f0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('ordem_do_elenco_pela_habilidade_atual', 0x549c30, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('as ordens dos clubes pela reputação e dos elencos pela habilidade',
        'the club-by-reputation and squad-by-ability orders',
        [('ordem_dos_clubes_pela_reputacao', 'qsort comparator (club ids by reputation)', '0x540020', 'Database.cpp', 'cdecl',
          'comparação do qsort dos ids de clube pela reputação (+0x80), a maior primeiro; id negativo = 0',
          'qsort comparison of club ids by reputation (+0x80), highest first; negative id = 0'),
         ('ordem_do_elenco_pela_habilidade', 'qsort comparator (squad by player +0xb)', '0x5402f0', 'Database.cpp', 'cdecl',
          'comparação do qsort dos elencos: pessoa nula no fim, depois a palavra +0xb do jogador, a maior primeiro',
          'qsort comparison of squads: null staff last, then the player word +0xb, highest first'),
         ('ordem_do_elenco_pela_habilidade_atual', 'compare_staff_by_player_ability_desc', '0x549c30', 'Database.cpp', 'cdecl',
          'comparação do qsort do elenco: sem jogador no fim, depois a habilidade atual (+5) e, empatada, a palavra +0xb, a maior primeiro',
          'qsort comparison of a squad: no player last, then current ability (+5) and, when tied, the word +0xb, highest first')])
CAMPOS = [('clube +0x80: reputação; palavra com sinal; lê (ordem_dos_clubes_pela_reputacao)', 'club +0x80: reputation; signed word; reads (ordem_dos_clubes_pela_reputacao)'),
          ('pessoa +0x61: jogador; ponteiro; lê. Jogador +5: habilidade atual; +0xb: a palavra do desempate; palavras com sinal; lê (as duas ordens de elenco)', 'staff +0x61: player; pointer; reads. Player +5: current ability; +0xb: the tie-break word; signed words; reads (both squad orders)')]
INTERESSA = ['SGE', 'CMRTE']
