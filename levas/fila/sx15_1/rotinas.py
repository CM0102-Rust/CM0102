# A preparação de um jogador para a partida (MATCH_PLAYER::setup 0x6eaea0, match_pl.cpp, 3.776 bytes; leitor sx15,
# rodada 5, 23-24/11/2026). Motor de partidas: roda para cada jogador de cada partida (MATCH_MAN::place_player_in_team
# 0x6e0dde). Estava no original atrás do portão init_player_hook (match_man.rs), que só gravava a fita na comparação;
# o comparador faz agora esse papel (dentro de uma fita: unidade, comparada por dentro). Ver INTEGRAR_ANTES.txt.
# A rotina inteira num item, em pedaços com nome (identidade, condição e sentimentos, registro para a narração,
# frustração e raiva com a pressão da liga e o bônus do jogador assentado).
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(jogador, motor, pessoa, lado, índice no bloco do lado, vaga/posição), ret 0x14;
# - cdecl: 0x944c9f sprintf (6 args), 0x547790/0x5477d0 (pessoa, clube), 0x5409b0/0x53b900 (clube), 0x4b0700 (id),
#   0x547770 (cargo, data), 0x600a50 (clube, 0, 1, 0, 0), 0x547840 (técnico, pessoa), 0x544410 (não-jogador, byte),
#   0x544520 (pessoa, 0), 0x53b640 (clube, uniforme, &frente, &fundo: os ponteiros são as vagas dos args 5 e 4, e o
#   fundo vai como palavra para +0x1ba); stdcall: 0x7c3d20 (pessoa), 0x7c3bb0 (pessoa, clube); thiscall:
#   0x4cf5f0, 0x62c590, 0x8b1750, 0x62c400, 0x6c38c0, 0x7c47b0 (devolve no x87), 0x7c3a40, 0x7c1e30 (devolve no
#   x87), 0x6d4280, 0x48f150, 0x4a4850, 0x7a7eb0; 0x945ff0 _CIpow pelo game_pow;
# - x87 (53 bits): importância × fator da aposta ((aposta − 10) × 0,05 + 1,0)^6, a pressão base × P × P × importância
#   × fator ÷ capacidade; o fator acima de 1 vira fator × 1/7 + 0,858.
# Defeitos do original (no cabeçalho da rotina): o contrato é buscado com o +0x27 de antes de ser escrito; a
# frustração fica com o valor da partida anterior quando nenhum sorteio dispara; o teste da reputação 5750 testa o
# endereço do objeto do técnico de partida (nunca nulo) e nunca dispara (não portado; a bandeira dá sempre random(6)).
MOTOR = True
ROTINAS = [('match_player_setup', 0x6eaea0, [0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c])]
DOCS = ('a preparação de um jogador para a partida',
        'setting up a match player',
        [('match_player_setup', 'MATCH_PLAYER::setup', '0x6eaea0', 'match_pl.cpp', 'thiscall, ret 0x14',
          'preenche o jogador da partida a partir da pessoa: motor, pessoa, índice no bloco, id da narração e ficha (sem ficha: mensagem no buffer global e uma ficha sorteada); forma 10 fora da tabela de pessoas, senão o moral da ficha; o contrato com o clube; as reputações; o clube pelo lado; a condição (estamina e teto) e o treino da semana; a posição (−2 lesionado, 0..10 com a posição inicial, senão −1 no banco com o desempenho 6400); os minutos −1; a confiança 2 × (25 − temperamento); o temperamento da partida (+10 se o adversário o favorece, −20 se o rejeita, −10 se o clube dele é rival do adversário) e o sentimento pelo próprio clube (10, +30 favorito, −10 rejeitado, +5 no clássico); o registro para a narração (estreia e gols, internacionais ou do clube, −1000 sem); o pé bom e a deficiência do outro; os ids do nome; o lado; os sorteios de regularidade; a frustração (pela pressão da partida: campo, profissionalismo², importância pelo contrato, aposta pela reputação da competição e do adversário, clássico e chance no campeonato, contra a capacidade do estádio; jogos grandes; habilidade; técnico adversário que o rejeita; motivação do técnico; sequência na liga; técnico se aposentando; goleiro de clube humano; jogador assentado) e a raiva (sujeira + frustração / 2 + 10) / 4; a cor do uniforme do lado; a direção (180 no lado 1); livre, casa (0, 0) e a ação 0x65',
          "fills the match player from the person: engine, person, index in the block, commentary id and player record (none: a message in the global buffer and a random record); form 10 outside the staff table, else the record's morale; the contract with the club; the reputations; the club by side; the condition (stamina and ceiling) and the week's training; the position (−2 injured, 0..10 with the starting position, else −1 on the bench with the rating 6400); the minutes −1; the confidence 2 × (25 − temperament); the match temper (+10 when the opponent favours him, −20 when it dislikes him, −10 when his club is the opponent's rival) and the feeling for his own club (10, +30 favoured, −10 disliked, +5 in a derby); the commentary record (debut and goals, international or for the club, −1000 none); the strong foot and the other foot's deficit; the name ids; the side; the consistency rolls; the frustration (by the match pressure: pitch, professionalism², importance by the contract, stake by the competition's and opponent's reputation, derby and title chance, against the stadium capacity; big matches; ability; an opposing manager who dislikes him; the manager's motivation; the league run; a retiring manager; a human club's goalkeeper; a settled player) and the anger (dirtiness + frustration / 2 + 10) / 4; the side's kit colour; the facing (180 for side 1); not busy, cell (0, 0) and action 0x65")])
