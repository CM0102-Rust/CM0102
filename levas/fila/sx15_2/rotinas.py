# As brigas e a reclamação de um jogador em campo (match_pl.cpp; leitor sx15, rodada 5, 23-24/11/2026). Motor de
# partidas: as três estavam no original atrás de portões que só gravavam a fita na comparação (match_eng.rs,
# gate_start_fight/gate_react_to_foul_decision/gate_handle_suspected_dive); o comparador de cada uma faz agora esse
# papel (dentro da fita de outra rotina com RECORD_716: uma unidade, comparada por dentro; numa fita sem RECORD_716:
# a original passa; fora de fita: compare_com_fita). Ver INTEGRAR_ANTES.txt (tirar os portões do instalar()).
# Convenções conferidas na desmontagem:
# - 0x716630 thiscall(jogador, profundidade) ret 4, devolve al; 0x716c90 thiscall(jogador) ret 0; 0x716e10
#   thiscall(jogador, flag) ret 4;
# - 0x6d6f70 (narração, unidade da fita) thiscall(narrador, código, motor, a3, a4, a5, a6, a7) ret 0x1c; os bytes
#   empilhados com lixo em cima (cl/dl de registradores com ponteiro) vão mascarados em 0xff na fita;
# - 0x7164b0 thiscall(jogador, nível, alvo) ret 8; 0x6cfd40 thiscall(motor, jogador, &cartão, gravidade, flag);
#   0x6cfc60 thiscall(motor, jogador, outro, lance, -1, &cartão, &gravidade, &vantagem) ret 0x1c; 0x6bb950
#   thiscall(motor, lado, f32); 0x6bb8d0 thiscall(motor, jogador, f32); 0x6cb960 stdcall(lado, x, y) ret 0xc;
#   0x6f46b0 thiscall(jogador, x, y) (unidade da fita no topo); 0x6cda20 pelo player_left_pitch_with injetado;
# - x87: 20.0f − agressividade (f32) truncado; (4.0 − temperamento × 0.16) × confiança, piso 2 × (25 − t), teto 100;
# - pilha: o byte do cartão começa em 0 (0x716c90, 0x716e10); em 0x716630 o cartão e a gravidade da 0x6cfc60 não são
#   iniciados pelo original (a 0x6cfc60 os escreve quando dá veredito; a cópia usa 0; o comparador zera a pilha abaixo
#   antes da original, zero_stack_below).
MOTOR = True
ROTINAS = [('start_fight', 0x716630, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('handle_suspected_dive', 0x716c90, [0x51, 0x56, 0x8b, 0xf1, 0x57]),
           ('react_to_foul_decision', 0x716e10, [0x83, 0xec, 0x08, 0x53, 0x56])]
DOCS = ('as brigas e a reclamação de um jogador em campo',
        "a match player's fights and dissent",
        [('start_fight', 'MATCH_PLAYER::start_fight', '0x716630', 'match_pl.cpp', 'thiscall, ret 4',
          'o jogador irritado procura briga com um adversário da mesma casa da grade, pelo teste de raiva: nível 4 cabeçada, nível 3 cusparada (sorteio de 1000, ou 300 se a nação dele não fala inglês, abaixo de 10) ou chute/soco/cotovelada por sorteio de 10, nível 2 empurrão ou encarada; a raiva do lado do adversário sobe (×2,25, ×2,5, ×2,0 ou ×1,15) e a confiança dele vai a (4,0 − temperamento × 0,16) × confiança (3,1 − t × 0,1 no nível 2), com piso 2 × (25 − t) e teto 100; sem adversário devolve 0. Com profundidade > 1, sorteio de 5 ≠ 0 e incidentes pendentes ≥ sorteio de 3 + 2: "pancadaria geral" e a corrente segue de −50 a −40; senão narra o ato (profundidade ≥ 0). Quem está em campo, livre e passa no teste de nível 1 corre para a casa; o juiz julga o ato e a gravidade soma nos incidentes pendentes dele ou entra um incidente novo; o adversário reage (profundidade + 1) enquanto a profundidade estiver em 0..9 ou abaixo de −40 e houver menos de 5 incidentes; devolve 1',
          'the angry player picks a fight with an opponent in his grid cell by the anger check: level 4 headbutt, level 3 spit (draw of 1000, or 300 when his nation does not speak English, below 10) or kick/punch/elbow by a draw of 10, level 2 shove or square up; the opponent side anger rises (×2.25, ×2.5, ×2.0 or ×1.15) and his confidence becomes (4.0 − temperament × 0.16) × confidence (3.1 − t × 0.1 at level 2), floored at 2 × (25 − t) and capped at 100; no opponent returns 0. At depth > 1, a draw of 5 ≠ 0 and pending incidents ≥ a draw of 3 + 2: "a big punch-up" and the chain goes on from −50 to −40; otherwise the act is narrated (depth ≥ 0). Free players on the pitch passing the level-1 check run to the cell; the referee judges the act and the severity adds to his pending incidents or a new incident is queued; the opponent reacts (depth + 1) while the depth is 0..9 or below −40 and fewer than 5 incidents are pending; returns 1'),
         ('handle_suspected_dive', 'MATCH_PLAYER::handle_suspected_dive', '0x716c90', 'match_pl.cpp', 'thiscall',
          'narra "pareceu simulação", pede ao juiz o cartão de gravidade 6 e o anuncia: amarelo, segundo amarelo, vermelho ou advertência; com sorteio de 8 ≠ 0 o juiz o chama antes (e, com sorteio de 3 ≠ 0 e clube adversário de língua inglesa, "off, off, off"); segundo amarelo ou vermelho: sai de campo',
          'narrates "looked like a dive", asks the referee for the card of severity 6 and announces it: yellow, second yellow, red or ticking-off; with a draw of 8 ≠ 0 the referee calls him over first (and, with a draw of 3 ≠ 0 and an English-speaking opposing club, "off, off, off"); a second yellow or a red: he leaves the pitch'),
         ('react_to_foul_decision', 'MATCH_PLAYER::react_to_foul_decision', '0x716e10', 'match_pl.cpp', 'thiscall, ret 4',
          'a reação do jogador a uma decisão do juiz, pelos testes de raiva 5, 4, 2, 2 e 1: agride (5) ou empurra (4) o juiz se perde a cabeça (sorteio de 20 − agressividade = 0 com agressividade ≥ 5), senão chuta a bola para longe (gravidade 3, e o juiz pode chamá-lo); o primeiro nível 2 também chuta a bola; o segundo nível 2 (gravidade 2) e o nível 1 (gravidade 1) reclamam; o cartão vem do juiz, é anunciado e a raiva dos companheiros muda pelo cartão (×0,85, ×0,75, ×0,5, ×0,95). Numa competição inglesa, com a bola do outro lado, cartão que não 0 nem 9, a bola nas linhas 1..10 e falta pendente: a falta avança uma linha para o gol dele ("10 jardas") e, na área dele, vira indireta',
          'the player reaction to a referee decision, by the anger checks 5, 4, 2, 2 and 1: strikes (5) or pushes (4) the referee when he loses his head (a draw of 20 − aggression = 0 with aggression ≥ 5), otherwise kicks the ball away (severity 3, and the referee may call him over); the first level 2 also kicks the ball away; the second level 2 (severity 2) and level 1 (severity 1) argue; the referee gives the card, it is announced and his team-mates anger scales by the card (×0.85, ×0.75, ×0.5, ×0.95). In an English competition, with the ball with the other side, a card other than 0 and 9, the ball on rows 1..10 and a free kick pending: the free kick moves one row towards his goal ("10 yards") and, inside his area, becomes indirect')])
