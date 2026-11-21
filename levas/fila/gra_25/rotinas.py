# O jogo de copa jogado (Cup::process_fixture 0x51a150, Cup.cpp, 0x51a150..0x51c011, ~7,9 KB; leitor gra, rodada 4,
# 21/11/2026). Roda em toda partida de copa da carreira (slot +4 de todas as vtables de copa; também pela base das
# copas europeias 0x583b10 e da Intertoto 0x633290). A rotina inteira desviada num item só, partida em pedaços com nome:
# cup_first_leg_played (decisão 3), cup_end_on_draw (copa de uma rodada sem mais replays), cup_schedule_replay (0),
# cup_schedule_deciding_match (4), cup_advance_tie_winner (1/2) e cup_create_next_round_fixture (o jogo da rodada
# seguinte já devido, a notícia 0x14 e o jogo de volta).
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(copa, jogo, a2), ret 8, quadro SEH sem destrutor; jogo nulo: o original lê +0x42 antes da caixa
#   (linha 0x467) e cai ali: a cópia chama a original;
# - 0x519c90 thiscall(copa, jogo) ret 4; 0x521b30 thiscall(copa, time, rodada) ret 8 (o time e a rodada empilhados
#   como dwords com lixo em cima; a 0x521b30 lê as palavras); 0x522240 thiscall(copa, vencedor, perdedor, rodada,
#   jogo) ret 0x10; 0x521d60 thiscall(copa, jogo, vaga) ret 8; 0x51e870 thiscall(copa, rodada, confronto) ret 8;
#   0x5ac920 thiscall([0xae2a58], &ano, &dia, &fatia, 1, comp, rodada, id de fora, id da casa, -1, -1, -1) ret 0x2c;
#   virtuais: +4 do estágio (jogo, a2), +0x40 (clube, desfecho, estágio, jogo, a2) ret 0x14, +0x7c (&jogo), +0x98
#   (jogo) e +0x9c (confronto, jogo) da Cup (0x51c020/0x51f2f0);
# - 0x49f570 thiscall(+8 do objeto da competição em [0xadadfc], jogo, a2, 0, 0); 0x5a0590 thiscall([0xae2a38],
#   clube) e 0x5999a0 thiscall(finanças, valor); 0x549ef0 thiscall(data, dia, ano); 0x54c5b0 thiscall(data, saída, n)
#   (n como palavra: o gap da rodada vai com o dia do jogo nos 16 bits de cima); 0x54a3f0 thiscall(data);
#   0x4b0870 cdecl(dword: só a palavra); 0x5ab510 thiscall([0xae2a58], &jogo); 0x5aa680 thiscall([0xae2a58], &jogo, 1);
#   0x85c260 thiscall([0xdd7ef4], valor, &jogo, estádio da copa); 0x7875e0 thiscall([0xb7431c], clube);
#   0x77bbe0/0x785790 no NewsItem local; 0x7861e0 thiscall([0xb7431c], &notícia, clube); 0x944c9f (sprintf) cdecl.
# - o registro de partida local (0x4f) não é zerado pelo original no caminho do jogo da rodada seguinte (os bits 0xf0
#   do +0x4e ficam com sobra de pilha, docs/MAPA_SAVE.md): a cópia zera.
# A notícia do resultado do mata-mata (0x4b0b80, item gra_24) não é chamada daqui: vem pela virtual +0x40
# (Cup::add_team_result_news 0x519690, já em Rust) e pelo prêmio (0x522240).
# Comparador: só a original roda (cria jogos, notícias, prêmios, progresso: não roda duas vezes), conferida depois
# contra o plano feito antes pela cópia (cup_process_fixture_plan, com a decisão tomada em cópias do confronto e do
# jogo e os mesmos ajudantes da cópia): o retorno, o confronto inteiro (0x35 bytes), as marcas do jogo, os desfechos,
# o vencedor no confronto seguinte, o perdedor no de terceiro lugar e a marca de copa encerrada.
SAVE = False
ROTINAS = [('cup_process_fixture', 0x51a150, [0x6a, 0xff, 0x68, 0x89, 0x76, 0x95, 0x00])]
DOCS = ('o jogo de copa jogado',
        'a played cup match',
        [('cup_process_fixture', 'Cup::process_fixture', '0x51a150', 'Cup.cpp', 'thiscall, ret 8',
          'depois de cada jogo de copa: a copa principal passa o jogo de um estágio ao estágio; as estatísticas da competição registram o jogo; jogo de antes do ano da copa: -1; a taxa de jogo da rodada (+0x5c, acima de 0) vai para os dois clubes; o confronto do jogo guarda o placar no bloco do jogo (pelos jogos disputados: replays + próximo jogo) e a copa decide (a regra do confronto em rodada de ida e volta antes do terceiro jogo, senão a do jogo), com as marcas de resultado guardadas na ordem dos lados; jogo de ida: o confronto passa ao jogo de volta, que recebe o placar da ida invertido e as regras da rodada (com a regra 0x10 ou 0x2000, as marcas invertidas); empate em jogo único: replay (o primeiro LEG_GAP_DAYS dias depois, data livre do calendário, mando invertido, rodada base + 1 ou + 2, as regras de replay; terceiro jogo: caixa); empate no agregado: jogo de desempate (4 dias depois com a regra 0x2000, senão 7; base + 1, ou + 5 sem a marca de jogo único; mando invertido sem a regra 0x2000); numa copa de uma rodada sem mais replays o empate dá desfecho 1 aos dois; com vencedor: na final, desfechos 1 e 2 (3 e 4 na disputa de terceiro lugar), notícias, progresso, prêmio e a copa encerrada quando todos têm desfecho; antes, o vencedor vai ao seu lado do confronto seguinte e o perdedor sai (na semifinal de copa com terceiro lugar, vai ao confronto de terceiro); se o confronto seguinte já estava devido e tem os dois times, o mando dele é refeito e o jogo criado (hoje se a data da rodada não passa de depois de amanhã), com a notícia 0x14 aos técnicos humanos e, em ida e volta, o jogo de volta (o mais tarde entre a data da rodada + LEG_GAP_DAYS e três dias depois da ida)',
          "after every cup match: a top-level cup passes a stage's match to the stage; the competition statistics record the match; a match from before the cup's year: -1; the round's match fee (+0x5c, above 0) goes to both clubs; the match's tie keeps the score in the block of that match (by the matches played: replays + next match) and the cup decides (the tie rule in a two-legged round before its third match, otherwise the match rule), the result marks kept in side order; a first leg: the tie moves to the second leg, which takes the first-leg score reversed and the round rules (with rule 0x10 or 0x2000, the marks reversed); a single match drawn: a replay (the first LEG_GAP_DAYS days later, a free date from the calendar, the ground swapped, base round + 1 or + 2, the replay rules; a third match: the box); a tie level on aggregate: a deciding match (4 days later with rule 0x2000, else 7; base + 1, or + 5 without the single-leg mark; the ground swapped without rule 0x2000); in a one-round cup out of replays a draw gives both outcome 1; with a winner: in the final, outcomes 1 and 2 (3 and 4 in the third-place match), news, progress, prize money and the cup finished when every team has an outcome; before it, the winner takes its side of the next tie and the loser goes out (in the semi-final of a cup with a third-place match, to that tie); when the next tie was already due with both teams known, its home sides are set again and its match created (today when the round's date is not after the day after tomorrow), with news item 0x14 to human managers and, two-legged, the second leg (the later of the round's date + LEG_GAP_DAYS and three days after the first leg)")])
