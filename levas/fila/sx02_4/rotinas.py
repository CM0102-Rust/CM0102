# As transferencias passadas entre duas datas (TRANSFER_MANAGER::get_past_transfers 0x8cc030, transfer_manager.cpp;
# leitor sx02, rodada 5, 23/11/2026). 227 chamadas ate 2002-06-01 na carreira ENG (manager_manager 0x6714, printouts,
# telas do clube).
# Convencoes conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, mes inicial, ano inicial, mes final, ano final, &vetor, &conta), ret 0x18; os
#   meses comparados como bytes com sinal (cmp al, [arg]), os anos como words sem sinal (ja/jb); eax na saida nao e
#   usado pelos chamadores (a copia nao devolve nada);
# - 0x549f70 (Date::Date): thiscall(&data, 1, mes, ano, -1) e (&data, 0x1c, mes, ano, -1), devolve a data;
#   0x54cfb0 (Date::get_month): thiscall(data), al; 0x417c10 (Date::operator=): thiscall(destino, &tmp, origem);
# - CRT: malloc 0x944e46 / realloc 0x945667 (vetor do chamador, +100 registros de 37 bytes), sprintf em 0xde1f64,
#   0x90bde0 (cdecl("CM3_TEMP", nome)), fopen "rb", fread(buf, 0x25, 1, f), fclose;
# - as caixas de memoria: as linhas 0x291f (memoria) e 0x295f (arquivo), texto 0xa8ecb8.
# Comparador: a copia num vetor proprio (malloc do jogo com o conteudo do chamador), depois a original no vetor real;
# confere a conta e os registros.
SAVE = False
ROTINAS = [('get_past_transfers', 0x8cc030, [0x6a, 0xff, 0x68, 0x7b, 0x3e, 0x96, 0x00])]
DOCS = ('as transferências passadas entre duas datas',
        'the past transfers between two dates',
        [('get_past_transfers', 'TRANSFER_MANAGER::get_past_transfers', '0x8cc030', 'transfer_manager.cpp', 'thiscall, ret 0x18',
          'regra: junta no vetor do chamador (que cresce de 100 em 100) as transferências do dia 1 do mês inicial ao fim do mês final: se o vetor já cobre o período (o início depois do primeiro registro e o dia 28 do mês final antes do último), nada muda; senão as da memória (em ordem de data, até a primeira depois do fim) e depois as dos arquivos transfers_<ano>.tmp de cada ano do período; a conta de entrada é a capacidade e a de saída, quantas juntou',
          "rule: collects into the caller's array (growing by 100) the transfers from the 1st of the start month to the end of the end month: if the array already covers the period (the start after its first record and the 28th of the end month before its last), nothing changes; otherwise those in memory (in date order, up to the first one past the end) and then those of the files transfers_<year>.tmp of each year of the period; the count on entry is the capacity and on exit how many it collected")])
CAMPOS = []
INTERESSA = []
