# As escapadas da 334a leva (itens de save: corte das pessoas, nevoeiro, media, ausencias, selecoes; prova do save).
# Evidencia lida dos itens do base1.qst da prova (%TEMP%\cm0102_prova_save, o jogo novo do inicio rapido, DATABASE=MINIMUM,
# sem tecnico humano) e do jogo\FeriasTesteSave.sav (tipo 3, data 24/07/2001; a carga anda ate 2001-08-28).

ESCAPADAS = {
    'sx08b_2_clube_grande_6000': (
        'LACUNA EXPLICADA. A mutacao troca so o teste "big_club && !wide" de trim_keeps_person, que vale para os tamanhos '
        'de base que nao sao 1 e 2. A prova cria o jogo com DATABASE=MINIMUM, que o inicio rapido traduz em tamanho 2 '
        '(0x827317 push 2 -> 0x5d7c60, bits 0..2 de [0xae3138]+4): wide = true e o teste nunca decide. O teste dos '
        'tamanhos 1 e 2 (reputacao >= 7500, liga acima de 10 e 1 em 3) e outra linha, nao mutada. Ha clubes no alcance '
        '(o club.dat do .qst tem 126 clubes de reputacao 6000..7499 fora da Inglaterra, ex. 1.FC Koln, AEK): com '
        'DATABASE=NORMAL a mutacao mudaria a tabela de pessoas. A carga nao cria jogo, entao nao corta.',
        'EXPLAINED GAP. The mutation changes only the "big_club && !wide" test of trim_keeps_person, used by database sizes '
        'other than 1 and 2. The proof creates the game with DATABASE=MINIMUM, which the quick start turns into size 2 '
        '(0x827317 push 2 -> 0x5d7c60, bits 0..2 of [0xae3138]+4): wide = true and the test never decides. The test of '
        'sizes 1 and 2 (reputation >= 7500, league above 10 and 1 in 3) is another, unmutated line. Clubs in range exist '
        '(the .qst club.dat has 126 clubs of reputation 6000..7499 outside England, e.g. 1.FC Koln, AEK): with '
        'DATABASE=NORMAL the mutation would change the people table. The load creates no game, so it does not trim.'),
    'sx10_1_load_fails': (
        'LACUNA EXPLICADA. A mutacao so troca o "return 1" do ramo "mascaramento desligado" de FOG_OF_WAR::load. O fog.dat '
        'do FeriasTesteSave comeca com o byte do mascaramento = 1 (e um mapa, do tecnico de ferias, id 0xd383, 487.401 '
        'bytes): a carga segue o ramo dos mapas e nunca passa pelo return mutado. O jogo novo da gravacao nao carrega '
        'nevoeiro (save 0).',
        'EXPLAINED GAP. The mutation changes only the "return 1" of the "masking off" branch of FOG_OF_WAR::load. The '
        'FeriasTesteSave fog.dat starts with the masking byte = 1 (and one map, the holiday manager\'s, id 0xd383, 487,401 '
        'bytes): the load takes the maps branch and never reaches the mutated return. The new game of the write loads no '
        'fog (save 0).'),
    'sx10_2_masking_byte': (
        'LACUNA EXPLICADA. A mutacao grava a palavra "present" (1) no lugar do byte do mascaramento. No jogo da prova o '
        'mascaramento esta LIGADO: o fog.dat do .qst tem 65 bytes, 01 e 16 palavras 0 (nenhum mapa, porque o .qst nao '
        'tem tecnico humano). O byte gravado e 1 nos dois casos. A nota do item supunha mascaramento desligado; com ele '
        'desligado a mutacao seria acusada.',
        'EXPLAINED GAP. The mutation writes the "present" word (1) in place of the masking byte. In the proof game masking '
        'is ON: the .qst fog.dat has 65 bytes, 01 and 16 zero words (no map, since the .qst has no human manager). The '
        'byte written is 1 either way. The item note assumed masking off; with it off the mutation would be caught.'),
    'sx10_2_presence_words': (
        'LACUNA EXPLICADA. A mutacao grava as 16 palavras so no ramo "mascaramento desligado" de FOG_OF_WAR::save. O '
        'fog.dat do .qst tem 65 bytes = 1 (mascaramento ligado) + 16 x 4 (palavras de presenca 0): o ramo mutado nao '
        'roda. Nao e mascara de ignorados: o fog.dat nao esta no IGNORAR nem no mascara_save.json (so general.dat, '
        'human_manager.dat, index.dat e staff history.tmp).',
        'EXPLAINED GAP. The mutation writes the 16 words only in the "masking off" branch of FOG_OF_WAR::save. The .qst '
        'fog.dat has 65 bytes = 1 (masking on) + 16 x 4 (presence words 0): the mutated branch does not run. It is not '
        'the ignore mask: fog.dat is in neither IGNORAR nor mascara_save.json (only general.dat, human_manager.dat, '
        'index.dat and staff history.tmp).'),
    'sx01d_gravacao_registro_curto': (
        'LACUNA EXPLICADA. O media.dat do .qst tem 58 bytes: 2 interesses (4 + 2 x 25) e a contagem do registro de '
        'historias = 0. O zwrite de 0 registros nao escreve nada com 23 nem com 22 bytes e devolve 0 = contagem, sem '
        'caixa. O FeriasTesteSave tem 9 historias, mas a carga nao grava de novo.',
        'EXPLAINED GAP. The .qst media.dat has 58 bytes: 2 interests (4 + 2 x 25) and a story log count of 0. The zwrite '
        'of 0 records writes nothing with 23 or with 22 bytes and returns 0 = the count, no box. The FeriasTesteSave has '
        '9 stories, but the load does not save again.'),
    'sx05b_8_ausencias_vazias': (
        'LACUNA EXPLICADA. A mutacao so deixa de pôr na lista as ausencias (lista 0, 12 bytes) lidas do awol.dat. O '
        'awol.dat do FeriasTesteSave (54.822 bytes, 54.148 pessoas) tem 0 ausencias, 14 treinos faltados e 28 incidentes '
        'de forma: o laco da lista 0 nao roda nenhuma vez. A gravacao nao carrega awol.dat.',
        'EXPLAINED GAP. The mutation only stops appending the absences (list 0, 12 bytes) read from awol.dat. The '
        'FeriasTesteSave awol.dat (54,822 bytes, 54,148 people) has 0 absences, 14 missed trainings and 28 fitness '
        'incidents: the list 0 loop runs zero times. The write does not load awol.dat.'),
    'sx05b_9_conta_dobrada': (
        'LACUNA EXPLICADA. O awol.dat do .qst tem 9.502 bytes = 4 + 9.486 marcas + 3 contagens, as tres = 0 (jogo novo, '
        'nenhuma ausencia, treino faltado nem incidente). 0 x 2 = 0: o item sai igual. O FeriasTesteSave tem 14 e 28 '
        'registros, mas a carga nao grava de novo.',
        'EXPLAINED GAP. The .qst awol.dat has 9,502 bytes = 4 + 9,486 flags + 3 counts, all three 0 (new game: no '
        'absence, missed training or incident). 0 x 2 = 0: the item comes out equal. The FeriasTesteSave has 14 and 28 '
        'records, but the load does not save again.'),
    'sx05b_9_incidente_curto': (
        'LACUNA EXPLICADA. A mutacao encurta a escrita de cada registro (min 13 bytes), mas as tres listas do awol.dat do '
        '.qst estao vazias (contagens 0, 9.502 bytes): nenhum registro e escrito. Os 28 incidentes do FeriasTesteSave so '
        'seriam gravados se a prova gravasse depois da carga.',
        'EXPLAINED GAP. The mutation shortens each record write (min 13 bytes), but the three lists of the .qst awol.dat '
        'are empty (counts 0, 9,502 bytes): no record is written. The 28 incidents of the FeriasTesteSave would only be '
        'written if the proof saved after the load.'),
    'sx158_pool_deslocado': (
        'LACUNA MEDIDA. Na carga do national_teams.dat, cada vaga dos dois pools de reservas aponta para a pessoa '
        'seguinte. O laco rodou com o FeriasTesteSave (pools de 12.621 vagas, 10.607 e 4.266 ocupadas; 0 convocacoes), e a '
        'carga ate 2001-08-28 (35 dias desde 24/07) deu a impressao IGUAL a base (717644320 sorteios, 24d8a399559dc979): '
        'a impressao e so a contagem e o estado dos sorteios, e quem le os pools (call_up_replacement, add_to_reserves, '
        'remove_player) escolhe e compara ponteiros sem sortear; trocar a pessoa nao mudou nenhum sorteio nesse trecho. '
        'A mesma carga devolvendo 0 (sx158_carga_refeita, init_tables com sorteios) foi acusada.',
        'MEASURED GAP. When loading national_teams.dat, each slot of the two reserve pools points at the next person. The '
        'loop ran with the FeriasTesteSave (pools of 12,621 slots, 10,607 and 4,266 filled; 0 call-ups), and the load to '
        '2001-08-28 (35 days from 24/07) gave a fingerprint EQUAL to the base (717644320 draws, 24d8a399559dc979): the '
        'fingerprint is only the draw count and generator state, and the pool readers (call_up_replacement, '
        'add_to_reserves, remove_player) pick and compare pointers without drawing; swapping the person changed no draw '
        'in that stretch. The same load returning 0 (sx158_carga_refeita, init_tables with draws) was caught.'),
}  # nome: (explicacao pt, explicacao en)
