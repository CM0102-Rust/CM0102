# O dinheiro como o jogo o mostra (cash.cpp; leitor sx05, rodada 5, 23/11/2026): o passo de arredondamento
# (cash_get_rounding_step 0x4426e0), o valor na moeda da tela (Cash::get_display_amount 0x43f460) e o texto do valor
# (cash_format_money 0x441c10, ret em 0x44265b, depois as duas tabelas de salto de 15 entradas 0x44265c e 0x442698).
# Chamadas pelas noticias (contratos, transferencias, premios), pelos textos da midia e pelas telas.
# Convencoes conferidas na desmontagem:
# - 0x4426e0 cdecl(i64 valor, bandeiras), devolve edx:eax (ponto de chamada 0x43f480: push flags; push hi; push lo;
#   add esp, 0xc); a escada em x87: `fcom; test ah, 0x41; je` = segue so quando o valor passa o limiar;
# - 0x43f460 thiscall(&dinheiro (i32), bandeiras) ret 4, devolve edx:eax; 0x945e80 __alldiv, 0x945e40 __allmul,
#   0x945f30 __allrem, 0x945640 _ftol (trunca);
# - 0x441c10 cdecl(buf, tamanho (word com sinal), i64 valor, bandeiras), devolve eax (1 ou 0); os digitos em
#   [esp+0x10] (16 bytes), o indice do caso = strlen - 1; cada caso um sprintf 0x944c9f com os %c dos digitos.
# Comparadores: o valor lado a lado (0 e 1) e o texto lado a lado (2, os primeiros 0x80 bytes do buffer).
SAVE = False
ROTINAS = [('cash_get_rounding_step', 0x4426e0, [0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x18]),
           ('cash_get_display_amount', 0x43f460, [0xdb, 0x01, 0x53, 0x55, 0xdc, 0x0d, 0xb8, 0x9c, 0xad, 0x00]),
           ('cash_format_money', 0x441c10, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00])]
DOCS = ('o dinheiro como o jogo o mostra',
        'money as the game shows it',
        [('cash_get_rounding_step', 'cash_get_rounding_step', '0x4426e0', 'cash.cpp', 'cdecl',
          'o passo de arredondamento de um valor: com a bandeira 4 (salário por ano, valores de transferência), a escada larga: até 10.000 de 1.000 em 1.000, 30.000 de 2.000, 100.000 de 5.000, 250.000 de 10.000, 1 milhão de 25.000, 5 milhões de 100.000, 20 milhões de 250.000, 50 milhões de 500.000, acima de 1 milhão; com a 8 (salário por semana ou mês), a fina, de 1 em 1 até 10 a 1 milhão acima de 50 milhões (com um salto de 500 para 5.000 aos 100.000); limiares e passos vezes a escala da moeda, o passo truncado; a 4 vale sobre a 8; sem nenhuma, 1.000',
          'the rounding step of an amount: with flag 4 (yearly wages, transfer fees), the coarse ladder: up to 10,000 in steps of 1,000, 30,000 of 2,000, 100,000 of 5,000, 250,000 of 10,000, 1 million of 25,000, 5 million of 100,000, 20 million of 250,000, 50 million of 500,000, above that of 1 million; with 8 (weekly or monthly wages), the fine one, from steps of 1 up to 10 to 1 million above 50 million (with a jump from 500 to 5,000 at 100,000); thresholds and steps times the currency scale, the step truncated; 4 wins over 8; with neither, 1,000'),
         ('cash_get_display_amount', 'Cash::get_display_amount', '0x43f460', 'cash.cpp', 'thiscall, ret 4',
          'o valor na moeda da tela: o valor guardado vezes o câmbio, truncado; sem a bandeira 0x20, arredondado ao passo mais próximo (meio passo acima e para baixo, divisão inteira de 64 bits)',
          'the amount in the display currency: the stored amount times the exchange rate, truncated; without flag 0x20, rounded to the nearest step (half a step up, then down, in 64-bit integer division)'),
         ('cash_format_money', 'cash_format_money', '0x441c10', 'cash.cpp', 'cdecl',
          'escreve o valor como texto: o prefixo da moeda, o valor e o sufixo. Com a bandeira 2, o valor inteiro com o separador de milhar (nenhum em quatro dígitos quando a moeda pede); sem ela, de quatro dígitos para cima abreviado em milhares, milhões, bilhões ou milhares de bilhões com a unidade: os um a três dígitos da frente e até duas decimais depois do separador decimal, sem os zeros finais; moeda sem unidade de bilhões escreve os bilhões em milhões inteiros com o separador. Zero e negativo saem "0"; os espaços viram "|". Devolve 1, ou 0 com a caixa de erro (linha 0x2f3) com buffer nulo ou menor que 25. Defeito do original: um valor de um dígito sai "0" (a tabela de saltos começa nos dois dígitos)',
          'writes the amount as text: the currency prefix, the amount and the suffix. With flag 2, the whole amount with the thousands separator (none in four digits when the currency says so); without it, from four digits up abbreviated to thousands, millions, billions or thousands of billions with the unit: the leading one to three digits and up to two decimals after the decimal separator, trailing zeros dropped; a currency without a billions unit writes billions as whole millions with the separator. Zero and negative amounts are "0"; spaces become "|". Returns 1, or 0 with the error box (line 0x2f3) with a null buffer or one shorter than 25. Defect of the original: a single-digit amount is written "0" (the jump table starts at two digits)')])
CAMPOS = []
INTERESSA = []
