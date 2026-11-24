# O texto da notícia de convocação dos jogadores de um clube (NationalTeams::write_call_up_news_text 0x76e3b0,
# national_teams.cpp, 3.568 bytes; leitor sx15, rodada 5, 23/11/2026). Escritor do tipo 0x1d das notícias (0x77bc80),
# 157 chamadas na carreira ENG até 2002-06-01. Partida em pedaços com nome: os jogadores (read_call_up_players), os
# adversários (write_call_up_opponents), o texto sem adversários, a manchete e o corpo.
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(objeto das seleções, notícia, corpo, Text), ret 0xc; o this não é lido;
# - 0x785830 get_slot lê só o byte do índice; 0x53e140 cdecl(primeiro, segundo, comum, buf 200, modo 1/5, id);
#   0x53e990 cdecl(pessoa, primeiro, segundo, comum, 0); 0x542100 cdecl(seleção, segunda (slot 5 com sinal), buf, 100,
#   2, 1); 0x5409b0 cdecl(seleção); 0x90c520 cdecl(n, buf, 0x32, 1) e 0x90c2d0 com o dword do buf (lê o byte);
# - os argumentos de cada formato (0x66f4e0) conferidos push a push, inclusive os três que dividem a chamada final
#   (0xa5c050, 0xa5bd60, 0xa5b768);
# - na manchete de um jogador o vínculo do nome vem ANTES do 0x4ae660 (o texto), como no original.
# Defeito do original: com adversários (slot 2 zero) e nenhum a ler (slot 6 <= 0), os corpos que citam os adversários
# juntam o buffer de 300 bytes nunca escrito (lixo da pilha); a cópia usa frase vazia e o comparador deixa esse caso só
# com a original.
# Comparador: compare_text_writer (texto, vínculos e gerador).
ROTINAS = [('write_call_up_news_text', 0x76e3b0, [0x81, 0xec, 0x78, 0x04, 0x00, 0x00])]
DOCS = ('o texto da notícia de convocação dos jogadores de um clube',
        "the news text of a club's players called up by a national team",
        [('write_call_up_news_text', 'NationalTeams::write_call_up_news_text', '0x76e3b0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia (tipo 0x1d) de que jogadores de um clube foram convocados por uma seleção: com até três jogadores, os nomes (repetidos fora da conta); sem adversários (slot 2), a manchete cita a seleção e a competição e o corpo os jogadores (um, dois, três pelo nome; mais, pelo número por extenso e o clube); com adversários, o nome da seleção (com a marca de segunda seleção) e os adversários numa frase (um, "A e B", "A, B e C", repetidos dobrados); a manchete: um jogador "selecionado por" (primeira, segunda seleção ou seleção especial), mais jogadores "jogadores do clube convocados"; o corpo pelo número de jogadores e de adversários (um adversário com o gênero dele, até três com a frase, mais sem eles); depois os vínculos dos jogadores (ou do clube), da seleção e dos adversários',
          "the headline or the body of the news (type 0x1d) that a club's players were called up by a national team: with up to three players, their names (repeats out of the count); without opponents (slot 2), the headline names the national team and the competition and the body the players (one, two, three by name; more by the count in words and the club); with opponents, the national team's name (with the second-team flag) and the opponents in one phrase (one, \"A and B\", \"A, B and C\", repeats folded); the headline: one player \"selected by\" (first, second or special national team), more players \"club players called up\"; the body by the number of players and of opponents (one opponent with its gender, up to three with the phrase, more without them); then the links of the players (or the club), the national team and the opponents")])
