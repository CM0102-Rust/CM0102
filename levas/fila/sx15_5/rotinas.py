# O aviso dos classificados para uma fase final e o texto dele (NationalTeams::post_qualified_nations_news 0x7779e0 e
# NationalTeams::write_qualified_nations_text 0x777ca0, national_teams.cpp, 704 e 4.592 bytes; leitor sx15, rodada 5,
# 23/11/2026). O aviso roda no fim do dia das seleções (0x76b100 daily_update, chamada em 0x76b5dc, toda carreira); só
# manda notícia quando NationalTeams::add_qualified_nation (0x7779b0, em Rust) pôs classificados na lista (+0x28): as
# vagas da Copa de 2002 decididas em out/nov de 2001 na carreira ENG [inferência: não há contador de 2002 para estas
# duas]. O texto é o escritor do tipo 0x2c do 0x77bc80.
# Convenções conferidas na desmontagem:
# - 0x7779e0: thiscall(objeto) sem argumentos, ret 0, SEH só para a notícia local (0x77bbe0, 0x2c, 0; sem destrutor);
#   0x5fa7d0 thiscall([0xb5e134], id da final, &sede (0), &segunda sede (-1)), ret 0xc; 0x90cfc0 cdecl(4) e (2);
#   0x786640 thiscall([0xb7431c], &notícia, continente), 0x786780 thiscall([0xb7431c], &notícia); 0x68d080 limpa a
#   lista; a ordenação compara +0x80 do clube como palavra com sinal (jge);
# - 0x777ca0: thiscall(objeto, notícia, corpo, Text), ret 0xc; o this não é lido; 0x90c520 cdecl(n, buf 0x78, 0x78, 0);
#   0x53b3d0 cdecl(nação); os argumentos de cada formato conferidos push a push (os de 15 a 17 argumentos inclusive).
# Defeito do original: a ordenação por reputação troca só pares (i, j) com j < n - 1, então o último classificado nunca
# sai do lugar (com dois, nada é ordenado).
# Comparadores: 0x7779e0 só a original (sorteia, manda notícias e esvazia a lista), com o plano conferido depois (a lista
# vazia); 0x777ca0 compare_text_writer.
ROTINAS = [('post_qualified_nations_news', 0x7779e0, [0x6a, 0xff, 0x68, 0xbb, 0xdf, 0x95, 0x00]),
           ('write_qualified_nations_text', 0x777ca0, [0x81, 0xec, 0x20, 0x06, 0x00, 0x00])]
DOCS = ('o aviso dos classificados para uma fase final e o texto dele',
        'the news of the nations that qualified for a finals and its text',
        [('post_qualified_nations_news', 'NationalTeams::post_qualified_nations_news', '0x7779e0', 'national_teams.cpp', 'thiscall',
          'com classificados novos na lista, uma notícia (tipo 0x2c) por fase final: a competição, quantos, duas redações sorteadas (4 e 2), as próximas sedes, o ano e as seleções ordenadas por reputação (o último nunca sai do lugar); a Eurocopa vai aos técnicos da Europa, as outras a todos; no fim a lista é esvaziada',
          'with new qualifiers in the list, one news item (type 0x2c) per finals: the competition, how many, two drawn wordings (4 and 2), the next hosts, the year and the national teams ordered by reputation (the last one never moves); the European Championship goes to the managers of Europe, the others to all; at the end the list is emptied'),
         ('write_qualified_nations_text', 'NationalTeams::write_qualified_nations_text', '0x777ca0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete pela redação sorteada (pelo número: um, dois ou o número por extenso; a competição; a sede; a frase comum) ou o corpo pela outra redação e pelo número (mais de seis, quatro a seis, um, dois ou três), com uma ou duas sedes: a primeira seleção, a competição, as sedes, o ano e a frase das outras (com quatro ou mais, a partir da segunda, até a sexta); depois os vínculos das seleções, das sedes (à seleção delas) e da competição',
          'the headline by the drawn wording (by the count: one, two or the count in words; the competition; the host; the common sentence) or the body by the other wording and the count (more than six, four to six, one, two or three), with one or two hosts: the first national team, the competition, the hosts, the year and the phrase of the others (with four or more, from the second to the sixth); then the links of the national teams, the hosts (to their national teams) and the competition')])
