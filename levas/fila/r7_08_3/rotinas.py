# O status no elenco de um jogador mudado pelas telas de transferencia (transfer_manager.cpp; leitor r7_08, rodada 7,
# 28/11/2026): estava adiada no levas/fila/rxs1_12/FORA.md so por tamanho. Rotina inteira:
# - 0x8ce0e0 TRANSFER_MANAGER::set_player_squad_status (2112 bytes, ate o ret 0x1c em 0x8ce8ec, mais as tabelas de
#   saltos 0x8ce8f0 (status 1..7) e 0x8ce90c (status merecido 1..5); sem quadro SEH).
# TELAS: as telas de transferencias (0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc; ja Rust em transfer_screens.rs,
# que a chamam pelo endereco TS_FN_SET_PLAYER_SQUAD_STATUS). A carreira automatica nao passa (0 chamadas na cobertura
# do mundo).
# Caminho no jogo para o Lucas, jogando comparando: no proprio clube, perfil de um jogador -> Transferencias/Contrato:
# mudar o status no elenco (titular, rodizio, reserva, jovem, nao desejado...) com e sem pedir preco, e listar/tirar da
# lista; uma vez com o jogo com mais de uma semana, rebaixando um titular jovem.
# Prologo: sub esp, 0x208 (6 bytes), sem relativas. Igual no cm0102-1080.exe (a rotina inteira, conferido byte a byte).
# Convencoes conferidas na desmontagem:
#   0x8ce0e0 thiscall(tm, pessoa, clube, valor, &saida, status, marcas, tirar_da_lista) ret 0x1c (o status lido como
#   byte; das marcas so os bits 1 e 3).
#   Dentro: 0x5a0590 thiscall([0xae2a38], clube); 0x4cf5f0 thiscall([0xae1050], pessoa, clube); a caixa "Error" linha
#   0x2f0c (show_error_box); 0x4d0c60 thiscall([0xae1050], pessoa, clube do tecnico da tela, 0, -1, 1);
#   0x54cdb0 thiscall(&data, data por valor); 0x8674e0 thiscall(contrato, 0x4000); 0x867700 thiscall(contrato, marca,
#   0); 0x90cfc0 cdecl(n); 0x62ed70 thiscall([0xb63d28], pessoa); 0x5882d0 cdecl(pessoa, clube)
#   (news_post_player_unhappy_with_role, pelo endereco); 0x4ca470 thiscall([0xae1050], pessoa, contrato) -> al;
#   0x842b00 / 0x842c00 thiscall([tm+0x856], pessoa, 0).
# Leitura de memoria: a pessoa, a ficha, o contrato, o clube, a lesao; nenhuma local lida antes de gravada.
# Comparador: so a original (sorteia, posta noticia, marca o contrato e refaz as listas curtas); o plano conferido na
# saida: com contrato e status diferente de -1, o nibble alto do status no elenco e o status pedido.
TELAS = True
SAVE = False
ROTINAS = [('set_player_squad_status', 0x8ce0e0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o status no elenco de um jogador mudado pelas telas de transferência: preço pedido, felicidade, moral e a reação dele',
        'a player\'s squad status set from the transfer screens: asking price, happiness, morale and his reaction',
        [('set_player_squad_status', 'TRANSFER_MANAGER::set_player_squad_status', '0x8ce0e0', 'transfer_manager.cpp', 'thiscall, ret 0x1c',
          'sem contrato com o clube, a caixa "Error" (linha 0x2f0c) e nada mais. Jogador com contrato do tipo 2 recebe o preço pedido como valor; num clube em crise, só se não passa do que o clube do técnico na tela pagaria. Status 7 (não desejado) tira 100 da felicidade, se ele já não o tinha. A moral: sem status anterior, 1 +100, 2 +50, 3 +25 (pedindo preço: -100 e a marca de recusa), 4 só com pedido -100, 5 só com a marca 1 -50, 6 -20, 7 -100; de um status melhor, 25 por posição perdida (no máximo -125; de 3 para 4, -50); sempre entre -100 e 100. Com o jogo com uma semana ou mais, a reação: status 1 tira a queixa 0x4000 e, por caráter (sorteio de profissionalismo ou lealdade dando 0), o deixa chateado; rebaixado de titular abaixo da idade-limite (32, mudada pela lealdade, profissionalismo, goleiro ou atacante acima de 15 e lesões longas): por caráter ou 1 em 5 a notícia de jogador insatisfeito, a queixa 0x4000 e -5 de moral (sem a notícia, ou para o status 2: 1 em 5 e sorteio de ambição diferente de 0, chateado); senão (menos para rodízio num clube de reputação 5750 ou mais) contratado há menos de (ambição + 6) × 5 dias fica chateado, e pelo status que ele acha merecer: 1 por caráter ou 1 em 3 chateado e -10 de moral; 2 com 9 em 10 por caráter; 3 e 4 abaixo da idade, 4 em 5, ambição acima de 10 e por caráter; 5 ambição acima de 15, 2 em 3 e por caráter. O status entra no nibble alto. No fim, tirar da lista (fora de crise) apaga o bit 2 da situação de transferência, e as listas curtas da IA param e recomeçam para ele',
          'without a contract at the club, the "Error" box (line 0x2f0c) and nothing else. A player under a type 2 contract gets the asking price as his value; at a club in crisis, only when it is no more than the on-screen manager\'s club would pay. Status 7 (not wanted) takes 100 off the happiness unless he already had it. The morale: from no status, 1 +100, 2 +50, 3 +25 (asking: -100 and the refusal mark), 4 only when asking -100, 5 only with flag 1 -50, 6 -20, 7 -100; from a better status, 25 per place lost (at most -125; from 3 to 4, -50); always within -100..100. Once the game is a week old, the reaction: status 1 clears grievance 0x4000 and, by character (a professionalism or loyalty draw giving 0), upsets him; a key player dropped under the age limit (32, moved by loyalty, professionalism, goalkeeping or attacking above 15 and long injuries): by character or 1 in 5 the "unhappy with his role" story, grievance 0x4000 and -5 morale (without the story, or to status 2: 1 in 5 and a non-zero ambition draw, upset); otherwise (except rotation at a club of reputation 5750 or more) one signed less than (ambition + 6) × 5 days ago is upset, and by the status he thinks he deserves: 1 by character or 1 in 3 upset and -10 morale; 2 9 in 10 and by character; 3 and 4 under the age, 4 in 5, ambition above 10 and by character; 5 ambition above 15, 2 in 3 and by character. The status goes into the high nibble. At the end, unlisting (outside a crisis) clears bit 2 of the transfer status, and the AI shortlists stop and restart for him')])
INTERESSA = []
