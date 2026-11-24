# A moeda da tela (cash.cpp; leitor sx05b, rodada 5, 23/11/2026): cash_set_currency, a rotina de 7,6 KB (0x43f7f0 a
# 0x441576 mais a tabela de saltos de 26 entradas em 0x441578), sozinha no item. Usa as constantes CURRENCY_* e
# CASH_CPP do sx05_1 (integrar depois dele). Define CURRENCY_PREFIX, CURRENCY_ROUND_RATE e CURRENCY_ID (usados pelo
# sx05b_23 e sx05b_24).
# Roda na carga das preferencias do jogo (game_config.cpp: 0x5d76ce e 0x5d85e0; duas chamadas na carreira do mundo),
# e na tela de preferencias quando o Lucas troca a moeda. Item de simulacao sem LIGAS.
# Convencoes conferidas na desmontagem: cdecl(id (byte)) -> eax (1, ou 0 com a caixa "Error" da linha 0x265);
# movsx do byte e `cmp 0x19; ja`; cada caso grava a taxa (f64 em dois dwords, 0xad9cb8), a taxa redonda (0xad9da0), a
# escala (0xad9cc0), copia (repne scasb + rep movsd/movsb) o prefixo (0xad9ca8), o sufixo (0xad9c70; 0xad9c64 e a
# cadeia vazia) e as unidades (0xad9d00 milhares, 0xad9d68 milhoes, 0xad9ccc bilhoes, 0xad9d34 milhares de bilhoes),
# grava os separadores (0xad9ca2, 0xad9cc8), a marca sem separador abaixo de 10.000 (0xad9ca4, 0 ou 1) e o id
# (0x98b140). Os 26 casos tirados da desmontagem por um script (os valores gravados por caso, com os destinos e as
# origens das copias), conferidos a mao nos casos 1 e 5; todos gravam os mesmos campos.
# Comparador: lado a lado sobre 0xad9c70..0xad9da8 e o id.
SAVE = False
ROTINAS = [('cash_set_currency', 0x43f7f0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('a moeda da tela',
        'the display currency',
        [('cash_set_currency', 'cash_set_currency', '0x43f7f0', 'cash.cpp', 'cdecl',
          'troca a moeda da tela (0 a 25: libra, peso, dólar australiano, franco belga, real, kuna, coroa dinamarquesa, euro, marco finlandês, franco francês, marco alemão, florim, dracma, libra irlandesa, lira, iene, coroa norueguesa, zloty, escudo, rublo, peseta, coroa sueca, franco suíço, lira turca, dólar, won): o câmbio, o câmbio redondo dos valores prontos, a escala, o prefixo e o sufixo, as unidades de milhares, milhões, bilhões e milhares de bilhões, os separadores de milhar e decimal, a marca de sem separador abaixo de 10.000 e o id; 1. Outro id: a caixa "Error" e 0, nada muda',
          'sets the display currency (0 to 25: pound, peso, Australian dollar, Belgian franc, real, kuna, Danish krone, euro, markka, French franc, Deutschmark, guilder, drachma, Irish pound, lira, yen, Norwegian krone, zloty, escudo, ruble, peseta, Swedish krona, Swiss franc, Turkish lira, US dollar, won): the exchange rate, the round rate of the preset amounts, the scale, the prefix and suffix, the units of thousands, millions, billions and thousands of billions, the thousands and decimal separators, the no-separator-under-10,000 flag and the id; 1. Any other id: the "Error" box and 0, nothing changed')])
CAMPOS = [('[0xad9ca8] prefixo da moeda, [0xad9da0] câmbio redondo (f64), [0x98b140] id da moeda; texto, valor; grava (cash_set_currency)',
           '[0xad9ca8] currency prefix, [0xad9da0] round rate (f64), [0x98b140] currency id; text, value; writes (cash_set_currency)')]
INTERESSA = []
