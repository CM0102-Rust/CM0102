# As datas internacionais dos amistosos (friendly.cpp, o que restava fora da fila): Friendly::build_international_date_table
# (0x5ca4f0, 0x5ca4f0..0x5cb9a0, 1.271 instruções quase todas de gravação de constante: as 110 datas extraídas da
# desmontagem para a tabela CQB3_DATAS) e Friendly::resolve_international_dates (0x5cc510, 0xda bytes), as duas uma vez
# por carreira, no construtor dos amistosos (0x5c1f80, na criação do jogo); passageira: Friendly::find_human_club_tournament
# (0x5c6830, 0x76 bytes, pelas telas dos amistosos do humano). Sem SEH, sem sorteio. Iguais no cm0102-1080.exe (corpo
# inteiro das três).
# As datas que a tabela dá em 2001 (as de ano +3, que a 0x5cc510 traz 4 anos para trás) são os dias em que a
# 0x5cbb00 (cpy_4, "amistosos marcados") marca os amistosos de seleções no segundo semestre; em 1/1/2002 a 0x5cbb00
# refaz os anos das mesmas datas.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x944e46 (malloc): push 0xfa0; call; add esp, 4 -> cdecl(n).
# - 0x5cb9b0 (friendly_international_date_set): mov ecx, entrada; 6 pushes; sem add esp -> thiscall, ret 0x18.
# - 0x945667 (realloc): push 0x898; push tabela; call; add esp, 8 -> cdecl(ponteiro, n).
# - 0x5cc510: mov ecx, esi; sem push; sem add esp -> thiscall(this), ret 0.
# - 0x54a130 (Date): lea ecx, [entrada + 3]; 4 pushes (dia da semana, ano, dia, mês na ordem dos argumentos: dia, mês,
#   ano, dia da semana); sem add esp -> thiscall, ret 0x10.
# - 0x945333 (qsort): 4 pushes (0x5cc610, 0x14, conta, tabela); add esp, 0x10 -> cdecl.
# - 0x9452ca (free, só no comparador): cdecl(ponteiro), como no fim da 0x5cbdf0 (push; call; add esp, 4).
# Comparadores: a 0x5ca4f0 em ensaio (a cópia aloca a tabela dela, copiada e liberada; depois a original a partir dos
# mesmos campos; confere as 110 datas, a conta e o último dia feito); a 0x5cc510 lado a lado nas datas; a 0x5c6830 pela
# devolução (pura).
SAVE = False
ROTINAS = [('tabela_das_datas_internacionais', 0x5ca4f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('resolver_datas_internacionais', 0x5cc510, [0x51, 0x66, 0xa1, 0x92, 0x2c, 0xae, 0x00]),
           ('torneio_do_clube_humano', 0x5c6830, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as datas internacionais dos amistosos: a tabela das datas, as datas no ciclo da Copa e o torneio do clube humano',
        'the international friendly dates: the date table, the dates in the World Cup cycle and the human club tournament',
        [('tabela_das_datas_internacionais', 'Friendly::build_international_date_table', '0x5ca4f0', 'friendly.cpp', 'thiscall',
          'aloca a tabela (+0x34ec) e grava as 110 datas internacionais de 20 bytes (dia da semana, dia, mês, ano a somar ao da Copa, continente e competição de eliminatória), reduz a tabela a 110, resolve as datas (0x5cc510) e marca o último dia feito (+0x34f4 = 400)',
          'allocates the table (+0x34ec) and writes the 110 international dates of 20 bytes (weekday, day, month, year offset from the World Cup year, confederation and qualifying competition), shrinks the table to 110, resolves the dates (0x5cc510) and marks the last day done (+0x34f4 = 400)'),
         ('resolver_datas_internacionais', 'Friendly::resolve_international_dates', '0x5cc510', 'friendly.cpp', 'thiscall',
          'dá a cada data a data do ciclo da Copa (o primeiro ano com (ano − 2) múltiplo de 4, mais o deslocamento, menos 4 se passar de 3 anos à frente) pela 0x54a130 e ordena a tabela pela data',
          'gives each entry its date in the World Cup cycle (the first year with (year - 2) a multiple of 4, plus the offset, less 4 when more than 3 years ahead) through 0x54a130 and sorts the table by date'),
         ('torneio_do_clube_humano', 'Friendly::find_human_club_tournament', '0x5c6830', 'friendly.cpp', 'thiscall',
          'o torneio em uso (de 127, 0x69 bytes a partir de +0xb1) que o clube do técnico humano da vez organiza, ou 0',
          'the tournament in use (of 127, 0x69 bytes from +0xb1) that the current human manager club hosts, or 0')])
CAMPOS = [('Friendly +0x34ec / +0x34f0: a tabela das datas internacionais (20 bytes: +0 dia da semana, +1 dia, +2 mês, +3 a data de 8 bytes, +0xb o ano a somar, +0xc continente, +0x10 competição) e a conta; ponteiro e valor; aloca, grava e ordena (tabela_das_datas_internacionais, resolver_datas_internacionais)',
           'Friendly +0x34ec / +0x34f0: the international date table (20 bytes: +0 weekday, +1 day, +2 month, +3 the 8-byte date, +0xb year offset, +0xc confederation, +0x10 competition) and the count; pointer and value; allocates, writes and sorts (tabela_das_datas_internacionais, resolver_datas_internacionais)'),
          ('Friendly +0x34f4: o último dia feito dos amistosos internacionais (palavra); valor; grava 400 (tabela_das_datas_internacionais)',
           'Friendly +0x34f4: the last day done of the international friendlies (word); value; writes 400 (tabela_das_datas_internacionais)'),
          ('Friendly +0xb1: os 127 torneios (0x69 bytes: +0 o anfitrião, +0x29 em uso); ponteiro; lê (torneio_do_clube_humano)',
           'Friendly +0xb1: the 127 tournaments (0x69 bytes: +0 the host, +0x29 in use); pointer; reads (torneio_do_clube_humano)'),
          ('[0x9cfa08..0x9cfa1c] os continentes e [0x9cf76c..0x9cf784] as eliminatórias das datas; id; lê (tabela_das_datas_internacionais)',
           '[0x9cfa08..0x9cfa1c] the confederations and [0x9cf76c..0x9cf784] the qualifying competitions of the dates; id; reads (tabela_das_datas_internacionais)'),
          ('[0xae2c92] o ano de hoje; valor; lê (resolver_datas_internacionais)',
           '[0xae2c92] the current year; value; reads (resolver_datas_internacionais)'),
          ('staff +0x39: o clube do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); ponteiro; lê (torneio_do_clube_humano)',
           'staff +0x39: the club of the current human manager ([0xb74c26 + 0x300 × [0xb77c7a]]); pointer; reads (torneio_do_clube_humano)')]
INTERESSA = []
