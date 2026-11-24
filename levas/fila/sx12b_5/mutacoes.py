# Mutacoes (sx12b_5). O hall_of_fame.bin fica fora do .qst: a prova do save (que compara o .qst) nao ve o arquivo; as
# mutacoes sao acusadas pelo comparador (o arquivo escrito pela original e pela copia, byte a byte), que roda quando o
# jogo e salvo comparando (save_game_data chama HallOfFame::save no fim). Pela prova do save, escapam.
# - a soma pelos bytes ja embaralhados (a carga seguinte acusa a soma errada e zera o hall da fama);
# - a versao 2 no cabecalho.
MUTACOES = [
    ('sx12b_soma_embaralhada',
     '        checksum = checksum.wrapping_add(b as u32); // sx12b_5: the checksum of the plain bytes',
     '        checksum = checksum.wrapping_add((b ^ key(i) as u8) as u32); // mutation: the scrambled bytes summed',
     'sx12b_5 hall of fame saved'),
    ('sx12b_versao_2',
     '    let mut header = [0xb2u8, 0xbf, 0x3a, 0x00, 1]; // the magic 0x3abfb2 and the version 1',
     '    let mut header = [0xb2u8, 0xbf, 0x3a, 0x00, 2]; // mutation: version 2',
     'sx12b_5 hall of fame saved'),
]
