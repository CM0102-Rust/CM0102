# Recordes do clube (club_records.cpp), 3º item: as cópias do recorde de valor de transferência
# (transfer_fee_record_assign 0x448550, 102 chamadas na carreira até 2002-06-01) e do recorde de gols numa partida
# (goals_in_match_record_assign 0x451570, 1). As datas pela Date::operator= 0x417c10 (retorno numa local que ela grava);
# o recordista (0x448490) e o registro de partida (0x448110) já em Rust (216ª). Simulação: as mutações pela carreira até
# 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio, sem local lida sem valor.
# Comparadores lado a lado pelo registro inteiro. Nenhuma em laço quente.
SAVE = False
ROTINAS = [('copiar_recorde_de_valor', 0x448550, [0x83, 0xec, 0x08, 0x56, 0x57]),
           ('copiar_recorde_de_gols_na_partida', 0x451570, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os recordes do clube: as cópias do recorde de valor de transferência e do recorde de gols numa partida',
        'club records: the copies of the transfer fee record and the goals in a match record',
        [('copiar_recorde_de_valor', 'transfer_fee_record_assign', '0x448550', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de valor de transferência (0x29 bytes): cinco duplas, as datas (+0x14, +0x20), +0x1c e a marca de troca (+0x28)',
          'copies the transfer fee record (0x29 bytes): five dwords, the dates (+0x14, +0x20), +0x1c and the part-exchange flag (+0x28)'),
         ('copiar_recorde_de_gols_na_partida', 'goals_in_match_record_assign', '0x451570', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de gols numa partida (0x38 bytes): os gols (+0), o recordista (+1) e a partida (+0x19)',
          'copies the goals in a match record (0x38 bytes): the goals (+0), the holder (+1) and the match (+0x19)')])
CAMPOS = [('recordes do clube: recorde de valor de transferência (0x29 bytes, datas em +0x14 e +0x20, troca em +0x28) e de gols numa partida (0x38: gols + recordista + registro de partida); escreve (copiar_recorde_de_valor, copiar_recorde_de_gols_na_partida)',
           'club records: transfer fee record (0x29 bytes, dates at +0x14 and +0x20, part-exchange at +0x28) and goals in a match (0x38: goals + holder + match record); writes (copiar_recorde_de_valor, copiar_recorde_de_gols_na_partida)')]
INTERESSA = ['SGE', 'CMRTE']
