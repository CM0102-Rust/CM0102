# O texto da notícia de contratação concluída (TRANSFER_MANAGER::write_transfer_completed_news_text 0x8dc370,
# transfer_manager.cpp; leitor gra, rodada 4, 21/11/2026). Chamado pelas notícias (0x77bc80, tipo 0xfa8). Convenções
# conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, notícia, corpo, Text), ret 0xc (ponto de chamada 0x78238c; o ecx não é lido);
# - os slots a partir do 15 lidos por um contador (byte em [esp+0x14]): 4 por jogador trocado (até 3), o tipo do
#   movimento, a bandeira e, com ela, os dois seguintes (o segundo lido sem avançar o contador); sem a bandeira o
#   "gênero" do tipo de jogador fica sendo o próprio contador (como no original);
# - 0x5461b0 (write_position_name) cdecl(posições, buf, 0xc8, 1) e 0x5465a0 (write_player_description)
#   cdecl(pessoa, posições, pontos fortes, buf, 0xc8, gênero, 1), no mesmo buffer; 0x53ad00 (database_get_job_name)
#   cdecl(cargo, 0, 0, &gênero); 0x549ef0 Date::Date(dia, ano), ret 8; 0x54ba20 thiscall(data, buf, 0xc8, 0xff), ret 0xc;
#   0x90c520 cdecl(n, buf, 0x32, 1); 0x43f460/0x441c10 (0x32 bytes);
# - defeito do original: o texto do valor da cláusula de jogos é comparado (strcmp) com o texto da taxa, que só foi
#   escrito no caminho da taxa; nos outros caminhos compara com o que a pilha tinha (a cópia compara com vazio);
# - os argumentos dos 32 formatos (0x66f4e0) conferidos push a push.
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('write_transfer_completed_news_text', 0x8dc370, [0x6a, 0xff, 0x68, 0x9c, 0x45, 0x96, 0x00])]
DOCS = ('o texto da notícia de contratação concluída',
        'the news text of a completed signing',
        [('write_transfer_completed_news_text', 'TRANSFER_MANAGER::write_transfer_completed_news_text', '0x8dc370', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia de que um clube contratou ou nomeou uma pessoa, por uma taxa, de graça ou numa troca de jogadores: a nomeação para um cargo que não é de jogador (com o nome do cargo e a data de chegada), a transferência livre (com o tipo de jogador, de outro clube ou não, com ou sem a data), a taxa (com os jogadores da troca: um, dois, três ou o número por extenso), o valor que pode subir pela cláusula de jogos; os vínculos dos clubes, da pessoa e dos jogadores trocados; a manchete por (slot 15 × clube + pessoa) módulo 4: leva, vai para, fecha negócio ou contrata',
          "the headline or body of the news that a club signed or appointed a person, for a fee, for free or in a player swap: the appointment to a job that is not a player's (with the job name and the joining date), the free transfer (with the player type, from another club or not, with or without the date), the fee (with the swapped players: one, two, three or the number in words), the amount that may rise through the appearance clause; the links of the clubs, the person and the swapped players; the headline by (slot 15 × club + person) mod 4: get, moves to, seal deal or sign")])
