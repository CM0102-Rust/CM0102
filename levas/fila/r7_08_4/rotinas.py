# Se um clube da IA quer um jogador oferecido numa troca (shortlist_manager.cpp; leitor r7_08, rodada 7, 28/11/2026):
# estava adiada no levas/fila/rx11b_5/FORA.md e rxf_7/FORA.md so por tempo. Rotina inteira:
# - 0x843800 Shortlist_Manager::club_wants_player (1552 bytes, ate o ret 0x10 em 0x843e05; sem quadro SEH).
# TELAS: a troca oferecida pelo humano numa proposta (open_add_part_exchange_player_screen 0x47c0b0, gravar 0) e a
# resposta da IA a essa proposta (ai_respond_to_bid 0x8c8090, gravar 1). A carreira automatica nao passa (0 chamadas na
# cobertura do mundo).
# Caminho no jogo para o Lucas, jogando comparando: fazer uma proposta por um jogador de um clube da IA oferecendo um
# jogador do proprio elenco na troca (a tela de acrescentar jogador na troca), e enviar a proposta.
# Prologo: sub esp, 0x20 / push ebx / push ebp (5 bytes), sem relativas. Igual no cm0102-1080.exe (a rotina inteira,
# conferido byte a byte).
# Convencoes conferidas na desmontagem:
#   0x843800 thiscall(sm, jogador, clube, valor, gravar) ret 0x10 -> eax 1/0.
#   Dentro: 0x4cf5a0 thiscall([0xae1050], pessoa); 0x5b7d20 cdecl(ficha) -> ax; 0x946464 bsearch(&par, [sm],
#   [sm+0x10], 8, 0x757ea0); 0x84b180 thiscall(sm, jogador, clube do contrato, clube) -> al; 0x8ce990
#   thiscall([0xde1f54], jogador, clube, 0, 1); 0x456740 Date::operator>; 0x5407e0 cdecl(jogador, tecnico, 0) -> ax;
#   0x90cfc0 cdecl(1000); 0x8c7560 thiscall([0xde1f54], jogador, clube, 1); 0x840d80 thiscall(sm, clube, pessoa,
#   vaga, &nota, avaliacao, 1 no alvo / 0 no jogador); 0x600a50 cdecl(clube, 0, 1, 0, 0); 0x4d42d0 / 0x4d1600
#   thiscall([0xae1050], ...); 0x5a0590 thiscall([0xae2a38], clube); 0x4d0c60 thiscall([0xae1050], jogador, vendedor,
#   0, -1, 0); 0x62ed70 thiscall([0xb63d28], pessoa); 0x945640 _ftol; 0x846920 thiscall(sm, jogador, clube, 1, 0,
#   status, 0) -> st0; 0x843e10 thiscall(sm, alvo antigo, clube, 3, 0); 0x8423f0 thiscall(sm, jogador, clube) (ja
#   Rust, pelo endereco como no irmao 0x842e10).
# O x87: y = 1 - (preco - valor) / (reputacao^2 / 10, inteiro); y < 0,01 da 0,01; y > 3 (ou NaN) da 3; acima de 1,
# y / 3 + 0,67. Com reputacao de -3 a 3 a divisao e por 0 (inf ou NaN): a copia segue as comparacoes do x87 (NaN cai em
# 3), por isso nao usa o shortlist_offer_margin do irmao.
# Leitura de memoria: o gerente das listas curtas (as vagas de 0x25 bytes, os pares), pessoas, clubes, financas,
# lesoes; nenhuma local lida antes de gravada.
# Comparador: gravar = 0 lado a lado (resposta, 0xae1080 e o estado do gerador de sorteios); gravar = 1 so a original
# (muda o alvo da vaga, o status do alvo antigo e os pares).
TELAS = True
SAVE = False
ROTINAS = [('club_wants_player', 0x843800, [0x83, 0xec, 0x20, 0x53, 0x55])]
DOCS = ('se um clube da IA quer um jogador oferecido numa troca, pela lista curta dele',
        'whether a computer club wants a player offered in an exchange, by its shortlist',
        [('club_wants_player', 'Shortlist_Manager::club_wants_player', '0x843800', 'shortlist_manager.cpp', 'thiscall, ret 0x10',
          'devolve 1 quando o clube quer o jogador pelo valor oferecido. 0 com a lista curta do clube inativa ou com 100 dias ou mais; 1 de cara se o par (jogador, clube) já está na lista; 0 pelas regras de visto ou se o clube não pode fazer proposta por ele. Em cada uma das 40 vagas: vaga de jogador (cargo 11, não de empréstimo), sem proposta, cuja posição o jogador cobre (as derivadas 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400; vaga sem posição também serve), que não seja a do alvo que é o próprio substituto, com data antes de hoje; vaga de status 5 só até 24 anos, e estrangeiro (nação dele e do clube em que trabalha diferentes da do clube) só com reputação 5000 e a reputação vista pelo técnico do clube de pelo menos 500 + sorteio(1000); visto necessário só nos status 1 a 3. A nota a bater: a do alvo atual (nota × margem pelo preço pedido do clube dele; se chega à avaliação, o alvo fica e a vaga passa; senão a avaliação menos a lesão dele) ou, sem alvo, a avaliação. O jogador: nota e valor para o clube; vendedor em crise limita o valor; nota × margem (valor oferecido − valor) menos a lesão tem de chegar à nota a bater, e o interesse dele em vir, a 2,75. Margem: 1 − diferença / (reputação² / 10), entre 0,01 e 3, acima de 1 vira y/3 + 0,67. Querido: com gravar, vira o alvo da vaga, o alvo antigo recebe o status 3 e o par entra na lista; 1. Nenhuma vaga: 0',
          'returns 1 when the club wants the player at the value offered. 0 with the club\'s shortlist inactive or at 100 days or more; 1 at once if the (player, club) pair is already listed; 0 by the work permit rules or if the club cannot bid for him. In each of the 40 slots: a player slot (job 11, not a loan slot), without an offer, whose position the player covers (derived 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400; a slot without a position also fits), not one whose target is its own replacement, dated before today; a status 5 slot only up to 24, and a foreigner (his and his job club\'s nation not the club\'s) only with reputation 5000 and the reputation the club\'s manager sees at least 500 + random(1000); a work permit needed only in statuses 1 to 3. The score to beat: the current target\'s (score × margin by his club\'s asking price; reaching the valuation, the target stays and the slot is skipped; otherwise the valuation less his injury) or, without a target, the valuation. The player: score and value for the club; a seller in crisis caps the value; score × margin (value offered − value) less his injury must reach the score to beat, and his interest in joining, 2.75. Margin: 1 − difference / (reputation² / 10), within 0.01..3, above 1 it becomes y/3 + 0.67. Wanted: with commit, he becomes the slot\'s target, the old target gets status 3 and the pair is listed; 1. No slot: 0')])
INTERESSA = []
