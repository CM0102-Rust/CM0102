# A tabela das 35 nacoes-chave (key_nation.cpp; leitor sx12b, rodada 5, 23/11/2026).
# key_nation_build_table 0x667150 (8 KB, 0x667150..0x669138 ret; 0x669139..0x66913f enchimento): roda uma vez na
# criacao do jogo (setup 0x81ce00, 0x827110, 0x82a9c0); a carreira ENG cria o jogo do zero a cada execucao.
# Conferido na desmontagem por uma execucao simbolica da rotina inteira (as escritas absolutas na tabela 0xb63d60, as
# chamadas a Date::Date 0x549f70 thiscall(data, dia, mes, ano, dia da semana) ret 0x10 e Date::operator= 0x417c10
# thiscall(destino, temporaria, origem) ret 8): a pilha fecha (cada push consumido), nenhuma outra escrita.
# Convencao: cdecl, sem argumentos, eax 1 (0 com a caixa, linha 0x36, sem a tabela de nacoes [0xae23a8]). Quadro SEH so
# pelas datas temporarias (sem destrutor). O ano e empilhado como dword com a metade alta de residuo de pilha; o
# Date::Date le so a palavra (mov word [esi+2], cx; and ecx, 0xffff): sem efeito.
# Ponteiros: nacao = [0xae23a8] + chave x 0x122; continente = [0xae23a4] + chave x 0xc6; copa = [0xae23d0] + chave x 0x6b.
# Comparador: lado a lado sobre a tabela (0x9d8 bytes) e o retorno; sem a tabela de nacoes, so a original.
# Prologo: mov eax, fs:[0] (6 bytes).
SAVE = False
ROTINAS = [('key_nation_build_table', 0x667150, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tabela das nações-chave',
        'the key nation table',
        [('key_nation_build_table', 'key_nation_build_table', '0x667150', 'key_nation.cpp', 'cdecl',
          'preenche as 35 linhas da tabela das nações-chave para um jogo novo: sem a tabela de nações, a caixa "Error" e 0; senão zera a tabela e escreve em cada linha a nação ou o continente, a função que cria as competições, o dia do fim da temporada (dia do ano em 2001), a marca de temporada dividida, a copa e a copa da liga, e as datas de início e fim da temporada (ano atual ou o seguinte, com o dia da semana); as linhas: eliminatórias da Copa do Mundo, as outras competições internacionais, as mundiais de clubes, as dos cinco continentes e as 27 nações com liga, da Argentina ao País de Gales; devolve 1',
          'fills the 35 rows of the key nation table for a new game: without the nation table, the "Error" box and 0; otherwise clears the table and writes in each row the nation or the continent, the function that creates the competitions, the season end day (day of the year in 2001), the split season flag, the cup and the league cup, and the season start and end dates (this year or the next, with the weekday); the rows: World Cup qualifying, the other international competitions, the world club ones, the five continents ones and the 27 league nations, from Argentina to Wales; returns 1')])
