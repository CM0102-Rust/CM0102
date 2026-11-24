# O dia a dia da diretoria e as nomeacoes de tecnico (MANAGER_MANAGER::daily_update 0x68f9c0, manager_manager.cpp;
# leitor sx14, rodada 5, 23/11/2026). Roda todo dia (516 chamadas na carreira ENG ate 2002-06-01; chamada pelo laco do
# dia 0x5cc850/0x5cccd0, originais). Convencoes conferidas na desmontagem (rotina inteira 0x68f9c0..0x6906a1, tabelas
# de salto 0x6906a4, 0x6906d0 e 0x6906fc de 11 entradas cada, pelo cargo - 5):
# - a rotina: thiscall(this), ret 0; quadro SEH so pelo Date do registro da transferencia (sem destrutor que faca algo);
# - 0x54c8e0 thiscall(hoje, &data, 0xb4) ret 8; 0x54cfb0 thiscall(hoje); 0x6a4c40, 0x69d060, 0x697c40, 0x69b650,
#   0x6a3770, 0x69ba50, 0x690890, 0x690730, 0x6a2e00, 0x69fd50, 0x694c80, 0x6abef0: Rust, chamados direto;
#   0x6a4f80 thiscall(this, clube, pessoa) ret 8 (original, pelo endereco); 0x68d1b0/0x68d1f0 thiscall da LLIST;
#   0x549f70 thiscall(data, 23, 7, 1971, -1) e 0x417c10 thiscall(destino, temp, origem) ret 8 (a data "nunca" vai
#   para board_record +0x20 da linha do clube da vaga; o Ghidra mostra como local);
#   0x54cdb0 thiscall(hoje, d0, d1) ret 8; 0x4cf6d0, 0x4d0b50, 0x4c7950 thiscall([0xae1050], ..);
#   0x8cde70 thiscall([0xde1f54], pessoa, 0); 0x8cb810 thiscall([0xde1f54], &registro de 0x25 bytes);
#   0x85a850/0x85add0/0x85a470 thiscall([0xdd7ef0], ..); 0x5a0590 thiscall([0xae2a38], clube); 0x5986b0 thiscall;
#   0x53b900, 0x5409b0, 0x544820, 0x53ca90, 0x53c590 cdecl; a caixa de erro (linhas 0x5e2 e 0x6bb) = show_error_at.
# Prologo `push -1; push 0x95cc56` (7 bytes, fronteira de instrucao, sem relativo). Sem x87 e sem sorteio proprio.
# Comparador: so a original (nomeia tecnicos, posta noticias, sorteia nas rotinas chamadas); a prova e a carreira.
# Pilha particular: chamada pelo laco do dia, nao pela criacao do jogo; nao le residuo de pilha (todos os locais sao
# escritos antes de lidos).
SAVE = False
ROTINAS = [('daily_update', 0x68f9c0, [0x6a, 0xff, 0x68, 0x56, 0xcc, 0x95, 0x00])]
DOCS = ('o dia a dia da diretoria e as nomeações de técnico',
        "the board's daily pass and the manager appointments",
        [('daily_update', 'MANAGER_MANAGER::daily_update', '0x68f9c0', 'manager_manager.cpp', 'thiscall, ret 0',
          'todo dia: na virada do mês (o primeiro dia depois da carga só registra o mês), a notícia da confiança da diretoria de cada clube (e das seleções cuja nação tem +0x71) e o conserto dos cargos vagos; os técnicos humanos com mais de 180 dias no clube perdem a marca de recém-chegados; aos domingos (dia do ano múltiplo de 7) a confiança semanal; as saídas pendentes. Depois, cada vaga da lista: passada a data de chegada, a nomeação (o humano sem classificação reabre a vaga; sem clube ou há 90 dias parado recebe a oferta; a pessoa sem registro de técnico, emprestada ou aposentando reabre a vaga; clube que já tem técnico ou pessoa com proposta só tiram a vaga; senão o clube que ele deixa ganha expectativas novas e vaga, a data de candidatura do clube volta a "nunca", ele é liberado pelo cargo no clube ou na seleção com a notícia da nomeação, o jogador entre dois clubes vai para as transferências passadas, assume o clube (jogador-técnico com vaga no elenco, reputação + 750 acima da do clube, menos de 45 no elenco e fora da Itália; senão técnico) com a faixa de riqueza, e a diretoria recomeça); antes da data, o boato da vaga uma vez só, depois da data do boato, se o clube continua sem técnico. Por fim as apelações de suspensão',
          'every day: when the month changes (the first day after loading only records the month), the board confidence news of every club (and of the national teams whose nation has +0x71) and the fix of the missing jobs; the human managers with more than 180 days at the club lose the newcomer mark; on Sundays (day of the year divisible by 7) the weekly confidence; the pending departures. Then each vacancy of the list: past its arrival date, the appointment (a human without a classification reopens the vacancy; without a club or out of work for 90 days he gets the offer; a person without a manager record, on loan or retiring reopens it; a club that already has a manager or a person under offer just drop it; otherwise the club he leaves gets new expectations and a vacancy, the club job application date goes back to "never", he is released by his club or national job with the appointment news, a player between two clubs goes into the past transfers, he takes the club (player-manager with a squad place, reputation + 750 above the club\'s, fewer than 45 in the squad and outside Italy; else manager) with the wealth band, and the board starts over); before the date, the vacancy rumour once, after the rumour date, while the club has no manager. Finally the ban appeals')])
