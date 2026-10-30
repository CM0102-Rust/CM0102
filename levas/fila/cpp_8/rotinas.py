# As doenças (injury.cpp): acrescentar o surto de doença de um clube (Injury::add_illness_outbreak 0x62e820, chamada
# pela lesão nova 0x62d250 e pela partida 0x5ab4xx: sorteia se entra, cresce o vetor e reordena pela data), espalhar
# os surtos pelos elencos (Injury::spread_illness_outbreaks 0x62e9e0, todo dia pela 0x62b7b0: sorteia as vagas do
# elenco e adoece pela 0x62d250; os de mais de cinco dias saem) e o jogador fora da temporada
# (injury_player_in_close_season 0x62ce30, pela 0x62b7b0 nas lesões de treino: clube fora da temporada da nação e
# faltando menos de 40 dias, sem convocação). Iguais no cm0102-1080.exe. As duas com quadro SEH só constroem datas
# (0x54a110, sem destrutor). Comparadores: o surto novo pelo plano (os sorteios com o estado guardado e devolvido; a
# conta e o surto no vetor); o espalhar só a original (a 0x62d250 sorteia e grava) com o plano dos que saem; o fora
# da temporada pelo valor (quente: sem contador).
SAVE = False
ROTINAS = [('acrescentar_surto_de_doenca', 0x62e820, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('espalhar_surtos_de_doenca', 0x62e9e0, [0x6a, 0xff, 0x68, 0x88, 0xb1, 0x95, 0x00]),
           ('jogador_fora_da_temporada', 0x62ce30, [0x6a, 0xff, 0x68, 0x20, 0xb1, 0x95, 0x00])]
DOCS = ('as doenças e a pré-temporada: acrescentar o surto de doença no clube, espalhar os surtos pelos elencos e o jogador fora da temporada',
        'illnesses and the close season: adding a club illness outbreak, spreading the outbreaks through the squads and the player in the close season',
        [('acrescentar_surto_de_doenca', 'Injury::add_illness_outbreak', '0x62e820', 'injury.cpp', 'thiscall, ret 0x10',
          'tipo 4 nunca; tipo 3 com 1 em 15, os outros com 3 em 4; fora das seleções, grava o surto (data, tipo, clube, partida) no vetor (que cresce de 30 em 30) e reordena pela data',
          'type 4 never; type 3 with 1 in 15, the others with 3 in 4; outside national teams, stores the outbreak (date, type, club, fixture) in the array (grown by 30) and re-sorts it by date'),
         ('espalhar_surtos_de_doenca', 'Injury::spread_illness_outbreaks', '0x62e9e0', 'injury.cpp', 'thiscall',
          'até o primeiro surto depois de hoje: o de mais de cinco dias sai; os outros adoecem jogadores sorteados do elenco do clube (tipo 3 só no dia, 1 a 4; os outros 0 a 2; até 20 tentativas) pela lesão nova 0x62d250',
          'up to the first outbreak after today: one older than five days is removed; the others make random squad players of the club ill (type 3 only on the day, 1 to 4; the others 0 to 2; up to 20 tries) through the new injury 0x62d250'),
         ('jogador_fora_da_temporada', 'injury_player_in_close_season', '0x62ce30', 'injury.cpp', 'stdcall, ret 4',
          '1 sem pessoa; 1 se o clube (sem marca +0x1db) está fora da temporada da nação e faltam menos de 40 dias, sem convocação futura da seleção; senão 0',
          '1 without a person; 1 when the club (without the +0x1db mark) is out of its nation\'s season with less than 40 days to go and there is no future national call-up; otherwise 0')])
CAMPOS = [('Injury +0x1c, +0x20, +0x24: o vetor de surtos (17 bytes), a capacidade e a conta; ponteiro e valor; lê e grava (acrescentar_surto_de_doenca, espalhar_surtos_de_doenca)',
           'Injury +0x1c, +0x20, +0x24: the outbreak array (17 bytes), its capacity and count; pointer and value; reads and writes (acrescentar_surto_de_doenca, espalhar_surtos_de_doenca)'),
          ('surto +0, +8, +9, +0xd: a data, o tipo da doença, o índice do clube e o id da partida (−1 sem); valor e id; grava (acrescentar_surto_de_doenca), lê (espalhar_surtos_de_doenca)',
           'outbreak +0, +8, +9, +0xd: the date, the illness type, the club index and the fixture id (−1 none); value and id; writes (acrescentar_surto_de_doenca), reads (espalhar_surtos_de_doenca)'),
          ('Injury +0: os registros de 31 bytes por jogador; +0x1d a doença atual; ponteiro e valor; lê (espalhar_surtos_de_doenca)',
           'Injury +0: the 31-byte records per player; +0x1d the current illness; pointer and value; reads (espalhar_surtos_de_doenca)'),
          ('clube +0xd7: as 50 vagas do elenco; ponteiro; lê (espalhar_surtos_de_doenca)',
           'club +0xd7: the 50 squad slots; pointer; reads (espalhar_surtos_de_doenca)'),
          ('clube +0x1db: marca (−1 = sem); valor; lê (jogador_fora_da_temporada)',
           'club +0x1db: mark (−1 = none); value; reads (jogador_fora_da_temporada)'),
          ('pessoa +0x24, +0x39, +0x5e (bit 3): seleção, clube e a classe da convocação; ponteiro e valor; lê (jogador_fora_da_temporada)',
           'person +0x24, +0x39, +0x5e (bit 3): national team, club and the call-up class; pointer and value; reads (jogador_fora_da_temporada)'),
          ('[0xae2c90] a data do jogo; valor; lê', '[0xae2c90] the game date; value; reads')]
INTERESSA = ['CMRTE']
