# Seleções (national_teams.cpp), 2º item: pôr no elenco (add_player_to_squad 0x776250, 106 chamadas na carreira até
# 2002-06-01; 0x776218, 0x779813), tirar do elenco (remove_player_from_squad 0x7762f0, 142; 0x775de4, 0x776200...) e
# dispensar os acima da idade do sub-21 (release_overage_u21_players 0x76d880, 228; 0x76bc37, 0x76c46b...). Simulação:
# as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio, sem local lida
# sem valor (a 0x776250 reaproveita o slot do tipo para o elenco, gravado antes; a data devolvida pela 0x417c10 só é
# escrita). Caixa só com time ou pessoa nulos (0x896c60, 0x53b3d0), fora do caminho comum. Comparadores lado a lado
# pelas 50 vagas do time (+0xd7) e pelas pessoas (+0x24 a +0x5e); a tática (0x896c60 de [0xde1d08]) fica fora das
# regiões: a cópia roda depois da original e a 0x896c60 não acha mais a pessoa (não muda nada). Nenhuma em laço quente.
SAVE = False
ROTINAS = [('por_no_elenco_da_selecao', 0x776250, [0x83, 0xec, 0x08, 0x53, 0x8a, 0x5c, 0x24, 0x14]),
           ('tirar_do_elenco_da_selecao', 0x7762f0, [0x8a, 0x44, 0x24, 0x08, 0x56]),
           ('dispensar_acima_da_idade_do_sub21', 0x76d880, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21',
        'national teams: adding to and removing from the squad and releasing the over-age under-21 players',
        [('por_no_elenco_da_selecao', 'add_player_to_squad', '0x776250', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a primeira das 50 vagas livres do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe a pessoa, que ganha o time (+0x24), a data de hoje (+0x29), a marca 0xb (+0x28) e o elenco 4 ou 8 (0x540980)',
          "the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (0x540980)"),
         ('tirar_do_elenco_da_selecao', 'remove_player_from_squad', '0x7762f0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'tira a pessoa da tática do time (0x896c60) e, achando-a numa vaga, esvazia a vaga e limpa +0x24, +0x28 e os bits de seleção de +0x5e',
          'removes the person from the team tactics (0x896c60) and, if found in a slot, empties it and clears +0x24, +0x28 and the national bits of +0x5e'),
         ('dispensar_acima_da_idade_do_sub21', 'release_overage_u21_players', '0x76d880', 'national_teams.cpp', 'thiscall, ret 4',
          'passada a data do registro da nação (+0x62), tira do sub-21 quem tem mais de 21 anos (+0x18) e zera o +0x6b do registro',
          'once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and clears record +0x6b')])
CAMPOS = [('time (TClub) +0xd7: as 50 vagas da seleção; lê e grava (as três)', 'team (TClub) +0xd7: the 50 national squad slots; reads and writes (all three)'),
          ('pessoa (0x6e bytes) +0x18 idade, +0x24 time da seleção, +0x28 marca de convocado, +0x29 data, +0x5e bits de elenco; lê e grava (as três)',
           'person (0x6e bytes) +0x18 age, +0x24 national team, +0x28 called-up mark, +0x29 date, +0x5e squad bits; reads and writes (all three)'),
          ('registro da nação no gerenciador das seleções ([this+4] + 0x6e × id) +0x62 data, +0x6b; lê e grava (dispensar_acima_da_idade_do_sub21)',
           'nation record in the national team manager ([this+4] + 0x6e × id) +0x62 date, +0x6b; reads and writes (dispensar_acima_da_idade_do_sub21)')]
INTERESSA = ['SGE', 'CMRTE']
