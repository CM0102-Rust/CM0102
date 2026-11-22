# As listas de indicados aos prêmios (2) (award_shortlist.cpp; leitor tel29, rodada das telas, 21/11/2026): carregar e
# gravar uma lista, os votos dos clubes do computador, o voto de um técnico humano e se ele pode votar. Date::operator=
# (0x417c10), que o mapa põe neste arquivo, já é Rust (rust/src/date.rs): fica de fora.
# Convenções conferidas na desmontagem (0x417590..0x41789c, 0x417c50..0x41813a, 0x418140..0x4182fa): todas thiscall,
# ecx = a lista; load ret 4 (arquivo; devolve 1 ou 0), save ret 4 (arquivo), pick_winners ret 8 (saída Staff**,
# contagem short), cast_vote ret 8 (técnico, jogador; devolve o endereço dos votos da entrada), get_vote_status ret 4
# (técnico; devolve ax: a entrada votada, -1 pode votar, -2 não pode). zread/zwrite (0x932c30/0x932e10) cdecl (dest,
# tamanho, quantidade, arquivo); utils_random (0x90cfc0) cdecl; human_manager_is_human_club (0x600a50) cdecl de 5;
# qsort (0x945333) com os comparadores 0x4183f0 (nota) e 0x418420 (votos).
# Quando rodam: load ao carregar um jogo salvo com uma lista de indicados em aberto, save ao gravar (prova do save);
# pick_winners na votação de um prêmio com lista (o de fim de temporada; a Premier o tem), cast_vote e get_vote_status
# na tela de votação do técnico humano (as notícias de indicados, botão de votar).
# Comparadores: load e save só na original (arquivo), load conferindo o plano; pick_winners lado a lado (as entradas,
# a saída e o gerador de aleatórios); cast_vote e get_vote_status lado a lado (os caminhos de erro só pela original).
TELAS = True
ROTINAS = [('award_shortlist_load', 0x417590, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_shortlist_save', 0x417720, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_shortlist_pick_winners', 0x417c50, [0xa1, 0x50, 0x23, 0xae, 0x00]),
           ('award_shortlist_cast_vote', 0x417fe0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00]),
           ('award_shortlist_get_vote_status', 0x418140, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as listas de indicados aos prêmios (2): carregar e gravar, os votos do computador, o voto do técnico humano e se ele pode votar',
        "the award shortlists (2): loading and saving, the computer's votes, the human manager's vote and whether he may vote",
        [('award_shortlist_load', 'AWARD_SHORTLIST::load', '0x417590', 'award_shortlist.cpp', 'thiscall, ret 4',
          'lê a lista de um jogo salvo: a contagem e a capacidade, as entradas, os 16 votos dos técnicos humanos e a data; leitura curta dá a caixa de erro e 0',
          'reads a shortlist from a saved game: the count and capacity, the entries, the 16 human managers\' votes and the date; a short read gives the error box and 0'),
         ('award_shortlist_save', 'AWARD_SHORTLIST::save', '0x417720', 'award_shortlist.cpp', 'thiscall, ret 4',
          'grava a lista no jogo salvo na mesma ordem da leitura; escrita curta dá a caixa de erro e para',
          'writes the shortlist to the saved game in the order load reads it; a short write gives the error box and stops'),
         ('award_shortlist_pick_winners', 'AWARD_SHORTLIST::pick_winners', '0x417c50', 'award_shortlist.cpp', 'thiscall, ret 8',
          'escolhe os vencedores: ordena pela nota, cada clube do computador no escopo do prêmio (divisão, nação, continente, seleções do continente; ou um voto por clube, por seleção, por registro) vota numa entrada sorteada abaixo da vez (a primeira vai para a maior nota), ordena pelos votos e devolve os primeiros',
          'picks the winners: sorts by score, each computer club in the award\'s scope (division, nation, continent, the continent\'s national teams; or one vote per club, per national team, per record) votes for an entry drawn below the turn (the first goes to the best score), sorts by votes and returns the first ones'),
         ('award_shortlist_cast_vote', 'AWARD_SHORTLIST::cast_vote', '0x417fe0', 'award_shortlist.cpp', 'thiscall, ret 8',
          'registra o voto de um técnico humano num indicado: a vaga dele guarda a entrada e a entrada ganha um voto',
          "records a human manager's vote for a shortlisted player: his slot keeps the entry and the entry gets one vote"),
         ('award_shortlist_get_vote_status', 'AWARD_SHORTLIST::get_vote_status', '0x418140', 'award_shortlist.cpp', 'thiscall, ret 4',
          'se o técnico humano pode votar: a entrada em que votou, -1 se ainda pode, -2 se não pode (pelo escopo); defeito: no escopo de divisão compara o id do clube com o da divisão, e nos de continente o id do continente com o da nação, então quase nunca deixa votar nesses prêmios',
          'whether the human manager may vote: the entry he voted for, -1 if he still may, -2 if not (by the scope); defect: the division scope compares the club\'s id with the division\'s, and the continent scopes the continent\'s id with the nation\'s, so it almost never lets him vote on those awards')])
