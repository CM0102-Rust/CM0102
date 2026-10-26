# Do Database.cpp: as comparações do qsort dos clubes pela latitude (0x53fd40) e pela longitude (0x53fe50) da cidade
# do estádio, que as ligas e copas usam para dividir os clubes por região (latitude: eng_auto_cup 0x56b730, cro_a1,
# fin_prm, ita_ser_a, nor_prm, por_prm, spa_first, swe_cup; longitude: bel_first, hol_cup, ita_c_cup, spa_first,
# tur_first). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH,
# sem sorteio, sem chamada; a local de 8 bytes (a coordenada de b) é zerada antes de lida; a de 0x200 só serve à
# mensagem da caixa (clube nulo), caminho que fica com a original.
SAVE = False
ROTINAS = [('ordem_dos_clubes_pela_latitude', 0x53fd40, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('ordem_dos_clubes_pela_longitude', 0x53fe50, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as ordens dos clubes pela latitude e pela longitude da cidade do estádio',
        'the club order by latitude and by longitude of the stadium city',
        [('ordem_dos_clubes_pela_latitude', 'database_compare_clubs_by_latitude', '0x53fd40', 'Database.cpp', 'cdecl',
          'comparação do qsort de clubes pela latitude (+0x23, f64) da cidade (+0x38) do estádio (+0x69), a menor primeiro; sem cidade: 0',
          'qsort comparison of clubs by the latitude (+0x23, f64) of the city (+0x38) of the stadium (+0x69), lowest first; no city: 0'),
         ('ordem_dos_clubes_pela_longitude', 'database_compare_clubs_by_longitude', '0x53fe50', 'Database.cpp', 'cdecl',
          'a mesma comparação pela longitude (+0x2b, f64)',
          'the same comparison by longitude (+0x2b, f64)')])
CAMPOS = [('TClub +0x69: estádio; ponteiro; estádio +0x38 cidade; cidade +0x23 latitude, +0x2b longitude (f64); lê (as duas)',
           'TClub +0x69: stadium; pointer; stadium +0x38 city; city +0x23 latitude, +0x2b longitude (f64); reads (both)')]
INTERESSA = ['SGE', 'CMRTE']
