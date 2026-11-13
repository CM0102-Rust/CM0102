# O hall da fama dos técnicos humanos (HallOfFame, hall_of_fame.cpp; leitor mwg, rodada 2 do mundo inteiro,
# 12/11/2026): a entrada do ano civil, a da liga da nação e a tentativa de entrar numa das 28 tabelas. Chamadas pelos
# registros dos técnicos (staff_records.rs, Rust: os pontos do ano de cada técnico) e pelo fim de temporada das ligas:
# 19.518, 2.977 e 4 chamadas até 2002-08-01 no mundo inteiro (cobertura_mundo.txt); só o humano chega a
# try_add_entry. Rodam na carreira de férias só com a Inglaterra (o humano existe): LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x5ef270..0x5ef31e, 0x5ef330..0x5ef5ce, 0x5ef5e0..0x5ef963):
# - add_calendar_year_entry: thiscall(this, pessoa, pontos), ret 8 -> eax; add_league_entry: thiscall(this, pessoa,
#   id da nação, pontos), ret 0xc -> eax; try_add_entry: thiscall(this, tabela (lida como byte; o dword empurrado
#   tem lixo em cima), pessoa, pontos), ret 0xc -> eax 0/1, quadro SEH sem destrutor;
# - a tabela da liga: as 27 chaves de nação na ordem da original (Argentina 1 ... Gales 26, Coreia do Sul 27),
#   qualquer outra −1 (setne/dec/and/dec), que try_add_entry recusa com a caixa de erro;
# - cdecl: 0x53dba0 (pessoa, texto, 200, 1), 0x542100 (clube, 0, texto, 200, 1, 1), 0x9466d0 strncpy (50),
#   0x944c9f sprintf "%s/%s" (0x9ce448), 0x66f4e0 langlib_sprintf "No Club/Nation" (0x9ce438), 0x946ebb time(0),
#   0x945333 qsort(base, 100, 120, 0x5f0620); thiscall 0x54a110 Date() e 0x417c10 Date::operator=;
# - as caixas de erro (linhas 0xff, 0x11b e 0x13b de hall_of_fame.cpp, 0x9ce408) com o 0 em 0xb67a34.
# Fidelidade: a cópia do nome de clube ou de seleção sozinho é o strcpy em linha da original (a cópia usa strncpy de
# 200 no texto local, que só muda o resto do texto local); a repetição só olha as 99 primeiras entradas, como a
# original. O carimbo de hora é time(0): os dois lados do comparador o leem no mesmo segundo (salvo a virada de um
# segundo entre eles, rara: só 4 entradas em toda a carreira).
# Comparadores: lado a lado sobre as 28 tabelas (336.000 bytes) quando a pessoa é humana; senão pelo valor.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('add_calendar_year_entry', 0x5ef270, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('add_league_entry', 0x5ef330, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('try_add_entry', 0x5ef5e0, [0x6a, 0xff, 0x68, 0x1b, 0xac, 0x95, 0x00])]
DOCS = ('o hall da fama dos técnicos humanos',
        'the hall of fame of the human managers',
        [('add_calendar_year_entry', 'HallOfFame::add_calendar_year_entry', '0x5ef270', 'hall_of_fame.cpp', 'thiscall, ret 8',
          'leva os pontos de um técnico humano à tabela do ano civil (0) do hall da fama: sem pessoa, a caixa de erro (linha 0xff) e 0; quem não é um dos 16 técnicos humanos, 0; senão try_add_entry com a tabela 0',
          "takes a human manager's points to the calendar year table (0) of the hall of fame: without a person, the error box (line 0xff) and 0; someone who is not one of the 16 human managers, 0; otherwise try_add_entry with table 0"),
         ('add_league_entry', 'HallOfFame::add_league_entry', '0x5ef330', 'hall_of_fame.cpp', 'thiscall, ret 0xc',
          'leva os pontos de um técnico humano à tabela da liga da nação: sem pessoa ou com id de nação fora da tabela, a caixa de erro (linha 0x11b) e 0; quem não é humano, 0; a tabela é a posição da nação (1 Argentina, 2 Austrália, 3 Bélgica, 4 Brasil, 5 Croácia, 6 Dinamarca, 7 Inglaterra, 8 Finlândia, 9 França, 10 Alemanha, 11 Grécia, 12 Holanda, 13 Irlanda, 14 Itália, 15 Japão, 16 Irlanda do Norte, 17 Noruega, 18 Polônia, 19 Portugal, 20 Rússia, 21 Escócia, 22 Espanha, 23 Suécia, 24 Turquia, 25 Estados Unidos, 26 País de Gales, 27 Coreia do Sul), −1 para as outras',
          "takes a human manager's points to the table of the nation's league: without a person or with a nation id outside the table, the error box (line 0x11b) and 0; someone not human, 0; the table is the nation's position (1 Argentina, 2 Australia, 3 Belgium, 4 Brazil, 5 Croatia, 6 Denmark, 7 England, 8 Finland, 9 France, 10 Germany, 11 Greece, 12 Holland, 13 Ireland, 14 Italy, 15 Japan, 16 Northern Ireland, 17 Norway, 18 Poland, 19 Portugal, 20 Russia, 21 Scotland, 22 Spain, 23 Sweden, 24 Turkey, 25 United States, 26 Wales, 27 South Korea), −1 for the others"),
         ('try_add_entry', 'HallOfFame::try_add_entry', '0x5ef5e0', 'hall_of_fame.cpp', 'thiscall, ret 0xc',
          'põe os pontos de um técnico numa das 28 tabelas de 100 entradas de 120 bytes quando passam a última: tabela fora de 0..27 ou sem pessoa, a caixa de erro (linha 0x13b) e 0; pontos 0 ou não acima dos da 100ª, 0; a entrada nova: o nome (50 caracteres), "clube/seleção" pelos nomes que ele tem (ou "No Club/Nation"), a hora (time), os pontos, a semente da partida e a data de hoje; uma das 99 primeiras com o mesmo nome, semente e data é repetição: 0; senão substitui a última, a tabela é reordenada (pontos do maior para o menor, depois a mais antiga primeiro) e devolve 1',
          'puts a manager\'s points in one of the 28 tables of 100 entries of 120 bytes when they beat the last one: a table outside 0..27 or no person, the error box (line 0x13b) and 0; points 0 or not above the 100th entry\'s, 0; the new entry: the name (50 characters), "club/national team" from the names he has (or "No Club/Nation"), the time stamp (time), the points, the game\'s seed and today; one of the first 99 with the same name, seed and date is a repeat: 0; otherwise it replaces the last one, the table is sorted again (points high to low, then older first) and 1 is returned')])
