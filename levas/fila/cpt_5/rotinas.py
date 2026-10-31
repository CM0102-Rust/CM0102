# O tempo das cidades (weather.cpp): Weather::update_city_weather 0x922ee0 (todo dia pelo laço do dia do jogo, 0x5cd9aa,
# [0xde2f74]; e em 0x5cca3b), que sorteia 3 ou 4 vezes por cidade, e a leitura do weather.cfg pela Weather::load_config
# 0x922810 (na criação do jogo, Setup.cpp 0x9226c0): a região Weather::read_region 0x922c40 e o período da região
# Weather::read_region_period 0x922d20 (sscanf 0x946fd8 sobre a linha de 0x90cba0). Iguais no cm0102-1080.exe. Sem SEH e
# sem local lida antes de gravada (os inteiros do sscanf só são lidos com os 19 campos lidos). x87 só na média da
# medida (× 0,5 e _ftol, igual em f64). Comparadores: só a original (sorteios; arquivo que não se lê duas vezes); a
# cópia leva os ramos.
SAVE = False
ROTINAS = [('tempo_das_cidades', 0x922ee0, [0x51, 0x56, 0x57, 0x8b, 0xf9]),
           ('ler_regiao_do_clima', 0x922c40, [0x81, 0xec, 0xd0, 0x07, 0x00, 0x00]),
           ('ler_periodo_do_clima', 0x922d20, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xe0, 0x0a, 0x00, 0x00])]
DOCS = ('o tempo das cidades: o tempo do dia, a região do weather.cfg e o período da região',
        'the city weather: the daily weather, the weather.cfg region and the region period',
        [('tempo_das_cidades', 'Weather::update_city_weather', '0x922ee0', 'weather.cpp', 'thiscall',
          'cada cidade: as três medidas viram (medida + sorteio(100)) × 0,5, presas em 0..100; uma vez em dez (sorteio(10) zerado), o vento novo = sorteio(3)',
          'each city: the three readings become (reading + random(100)) × 0.5, clamped to 0..100; one time in ten (random(10) zero), the new wind = random(3)'),
         ('ler_regiao_do_clima', 'Weather::read_region', '0x922c40', 'weather.cpp', 'thiscall, ret 8',
          'uma linha do weather.cfg com o id e o nome da região ("%ld %s") e os quatro períodos dela (0x922d20) em +0x1e, +0x31, +0x44 e +0x57',
          'one weather.cfg line with the region id and name ("%ld %s") and its four periods (0x922d20) at +0x1e, +0x31, +0x44 and +0x57'),
         ('ler_periodo_do_clima', 'Weather::read_region_period', '0x922d20', 'weather.cpp', 'thiscall, ret 8',
          'uma linha com o nome e 18 inteiros (o dia, cinco W, cinco P, sete H): o período recebe o dia (palavra) e os 17 bytes; arquivo nulo é a caixa "Error"',
          'one line with the name and 18 integers (the day, five W, five P, seven H): the period gets the day (word) and the 17 bytes; a null file is the "Error" box')])
CAMPOS = [('Weather [0xde2f74] +4 tabela das cidades (4 bytes por cidade: +0..+2 medidas 0..100, +3 vento 0..2); ponteiro/valores; lê e grava (tempo_das_cidades)',
           'Weather [0xde2f74] +4 city table (4 bytes per city: +0..+2 readings 0..100, +3 wind 0..2); pointer/values; reads and writes (tempo_das_cidades)'),
          ('[0xae2354] quantas cidades; valor; lê (tempo_das_cidades)',
           '[0xae2354] city count; value; reads (tempo_das_cidades)'),
          ('região do clima (0x6e bytes) +0 id, +4 nome, +0x1e/+0x31/+0x44/+0x57 os quatro períodos; valores; grava (ler_regiao_do_clima)',
           'weather region (0x6e bytes) +0 id, +4 name, +0x1e/+0x31/+0x44/+0x57 the four periods; values; writes (ler_regiao_do_clima)'),
          ('período do clima (0x13 bytes) +0 dia (palavra), +2..+6 W, +7..+0xb P, +0xc..+0x12 H; valores; grava (ler_periodo_do_clima)',
           'weather period (0x13 bytes) +0 day (word), +2..+6 W, +7..+0xb P, +0xc..+0x12 H; values; writes (ler_periodo_do_clima)')]
INTERESSA = ['GPF2']
