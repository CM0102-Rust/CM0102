# A troca da Conference com as divisões que a alimentam (eng_prm.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação
# da Inglaterra com a base NORMAL (a Conference e as divisões abaixo dela só existem com ela) e ATE = '2002-08-01': a
# virada inglesa (EngPrm::start_new_season 0x574c10, item cx09_15) chama esta rotina quando a Conference é jogada (senão
# a 0x574e60, já Rust).
# - 0x5750a0 EngPrm::exchange_conference_feeder_clubs thiscall ret 0: a Conference [[0xadadfc] + 4 x [0x9cf69c]]; três
#   vagas locais zeradas; malloc(4 x [0xae235c]) (nulo: caixa 0x2f9 e fim); os clubes com divisão [+0x57] e nação
#   [+0x53] não nulas, a divisão fora de [0x9cf764], [0x9cf5bc], [0x9cf5c0], [0x9cf5c4], [0x9cf5c8], [0x9cf69c] e a nação
#   [0x9cf2e4]; nenhum: free e fim; comp_util_sort_and_shuffle_top_clubs 0x4afbd0 cdecl(lista, n, 3); em ordem, a
#   divisão já numa vaga: pula; senão a primeira vaga livre recebe a divisão e League::move_club_to_division 0x6830b0
#   thiscall(this, clube, [0xae23d0] + [0x9cf69c] x 0x6b, 1) ret 0xc; a vaga 2 preenchida encerra. Depois, enquanto
#   houver vaga (< 3), cada linha da Conference com +0x37 = 3: League::relegate_club_to_division 0x6831a0 thiscall(this,
#   clube, vaga, 1) ret 0xc. free. DEFEITO: com menos de três divisões achadas, um rebaixado recebe a vaga vazia
#   (divisão 0). O edi guarda a divisão do primeiro candidato e não é lido. Prólogo sub esp, 0x21c (6 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (sorteia na ordenação e move clubes), conferindo depois que os
# três primeiros rebaixados da Conference saíram dela. Depende do cx09_15 (a virada que a chama) só para rodar: não
# usa nada dele.
SAVE = False
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('eng_prm_exchange_conference_feeder_clubs', 0x5750a0, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('a troca da Conference com as divisões que a alimentam',
        "the Conference's exchange with its feeder divisions",
        [('eng_prm_exchange_conference_feeder_clubs', 'EngPrm::exchange_conference_feeder_clubs', '0x5750a0', 'eng_prm.cpp', 'thiscall, ret 0',
          'regra: na virada, com a Conference jogada, o melhor clube inglês de cada uma de até três divisões abaixo dela sobe para a Conference (um por divisão, o mais forte primeiro), e os rebaixados da Conference descem para essas divisões, o primeiro para a primeira divisão que mandou um clube, e assim por diante, três no máximo',
          'rule: at the season turnover with a played Conference, the best English club of each of up to three feeder divisions below it goes up to the Conference (one per division, the strongest first), and the Conference\'s relegated clubs go down into those divisions, the first relegated to the first division that sent a club up, and so on, three at most')])
CAMPOS = []
INTERESSA = []
