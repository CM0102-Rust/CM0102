# Os valores em dinheiro das telas (cash.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa CASH_CPP (sx05_1),
# CURRENCY_PREFIX, CURRENCY_ROUND_RATE (sx05b_21) e cash_from_display (sx05b_22): integrar depois deles.
# So rodam nas telas: o comprimento do prefixo na tela de finanças do clube (club_screens 0x462070), o valor digitado
# nas telas de finanças e de transferências (club_screens 0x462070, transfer_screens 0x8e9550), os valores prontos
# dos filtros da busca de jogadores (search_filters 0x807450, search_screens 0x813210), a tabela de valores das telas
# de contrato e de transferências (contract_screens 0x4de5d0, transfer_screens 0x8e9550/0x8f6770) e os nomes das
# moedas na tela de preferências (Setup.cpp 0x823890). TELAS = True.
# Caminho para o Lucas provar (jogando comparando): Clube -> Finanças (o valor com o prefixo); fazer uma oferta por um
# jogador e digitar o valor; Busca de jogadores -> filtros de valor e salário; Oferecer contrato (a lista de salários);
# Opções/Preferências -> Moeda (a lista dos nomes).
# Convencoes conferidas na desmontagem:
# - 0x43f440 cdecl() -> eax: strlen de 0xad9ca8 (repne scasb, not, dec);
# - 0x43f4c0 thiscall(&dinheiro, valor (i64)) ret 8, devolve eax: fild qword / [0xad9cb8], _ftol, o dword baixo;
# - 0x43f620 thiscall(&dinheiro, indice (byte)) ret 4: fora de 0..6 a caixa "Error" da linha 0xaa e devolve o valor
#   atual; a tabela 0x98b1e0 (o teste de nulo dela, linha 0xc0, nunca acontece: endereco fixo); fild dword x
#   [0xad9da0], _ftol de 64 bits, fild qword / [0xad9cb8], _ftol;
# - 0x43f720 thiscall(&dinheiro, indice (byte), tabela, conta (word)) ret 0xc: tabela nula, indice (movsx para word)
#   >= conta (com sinal) ou indice < 0: a caixa da linha 0xc0;
# - 0x4415f0 cdecl(buf, tamanho (word com sinal), id (byte)) -> eax: buf nulo ou tamanho < 0x32 a caixa da linha 0x285,
#   id fora de 0..0x19 a da linha 0x2db, 0; senao a copia (repne scasb + rep movsd/movsb) do nome da tabela de saltos
#   0x441ba4 e 1.
# Comparadores: lado a lado (o valor, o buffer); os caminhos da caixa de erro so a original.
TELAS = True
SAVE = False
ROTINAS = [('cash_currency_prefix_length', 0x43f440, [0x57, 0xbf, 0xa8, 0x9c, 0xad, 0x00]),
           ('cash_set_from_display', 0x43f4c0, [0xdf, 0x6c, 0x24, 0x04, 0x56]),
           ('cash_set_from_preset_index', 0x43f620, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('cash_set_from_amount_table', 0x43f720, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('cash_get_currency_name', 0x4415f0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os valores em dinheiro das telas',
        'the money values of the screens',
        [('cash_currency_prefix_length', 'cash_currency_prefix_length', '0x43f440', 'cash.cpp', 'cdecl',
          'o comprimento do prefixo da moeda da tela',
          'the length of the display currency prefix'),
         ('cash_set_from_display', 'Cash::set_from_display', '0x43f4c0', 'cash.cpp', 'thiscall, ret 8',
          'guarda um valor na moeda da tela como dinheiro: dividido pelo câmbio e truncado; devolve-o',
          'stores a display currency amount as cash: divided by the exchange rate and truncated; returns it'),
         ('cash_set_from_preset_index', 'Cash::set_from_preset_index', '0x43f620', 'cash.cpp', 'thiscall, ret 4',
          'guarda um dos sete valores prontos (de 100 mil a 10 milhões de libras) como a moeda da tela o arredonda: vezes o câmbio redondo, de volta pelo câmbio; índice fora de 0 a 6 mostra a caixa "Error" e deixa o valor',
          'stores one of the seven preset amounts (100 thousand to 10 million pounds) as the display currency rounds it: times the round rate, back by the exchange rate; an index outside 0 to 6 shows the "Error" box and leaves the value'),
         ('cash_set_from_amount_table', 'Cash::set_from_amount_table', '0x43f720', 'cash.cpp', 'thiscall, ret 0xc',
          'guarda o valor de um índice de uma tabela do chamador como a moeda da tela o arredonda; tabela nula ou índice fora da conta mostra a caixa "Error" e deixa o valor',
          "stores the amount at an index of a caller's table as the display currency rounds it; a null table or an index outside the count shows the \"Error\" box and leaves the value"),
         ('cash_get_currency_name', 'cash_get_currency_name', '0x4415f0', 'cash.cpp', 'cdecl',
          'copia o nome curto de uma moeda (£, Peso, Aus$... Pta, SKr, SwF, TLira, $ e o won) num buffer de ao menos 50 caracteres; 1, ou 0 com a caixa "Error" (buffer nulo ou curto, id fora de 0 a 25)',
          'copies the short name of a currency (£, Peso, Aus$... Pta, SKr, SwF, TLira, $ and the won) into a buffer of at least 50 characters; 1, or 0 with the "Error" box (null or short buffer, id outside 0 to 25)')])
CAMPOS = []
INTERESSA = []
