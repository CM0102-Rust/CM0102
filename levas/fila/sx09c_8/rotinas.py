# Se a multa é justa para o jogador (Fine::is_fine_justified 0x5a69b0, Fine.cpp, 0x5a69b0..0x5a7385 e a tabela de
# salto 0x5a7388 de 5 entradas, 2,5 KB; leitor sxA, rodada 5, 24/11/2026). Chamada só pela Fine::fine_player
# (0x5a5ab9, sx09c_7), que roda pela AWOL (0x41a60a, 0x41b003: o técnico da IA multa quem falta ao treino). Simulação,
# sem LIGAS.
# Convenções conferidas na desmontagem:
# - thiscall(fine, jogador, período, motivo) ret 0xc -> al; o período não é lido; o motivo como byte com sinal;
# - quadro SEH de 0xe10 bytes: o registro de partida local com os dois blocos de time de 0x625 bytes construídos pelo
#   eh vector constructor (0x9463af com 0x453c30 e 0x453c10) e destruídos (0x9462bb) em toda saída, e uma LLIST local
#   (0x68d020 / 0x68d040);
# - a lista [fine] rebobinada à mão (+0xa = +2) e lida por 0x68d1b0; registro de 14 bytes (+0 jogador, +5 aceitou,
#   +6 data);
# - tabela 0x5a7388 (motivo - 1 em 0..4): 0x5a6b45 (1 e 5), 0x5a6e49 (2), 0x5a702f (3), 0x5a71d4 (4);
# - 0x7357c0 thiscall([0xb6b7c8], jogador) -> ax; 0x418f60 thiscall([0xad9c6c], jogador, 0) -> al; 0x418ff0 e
#   0x419070 thiscall([0xad9c6c], jogador) -> al (0x7357c0 e 0x419070 pelo endereço, FN no item);
# - 0x54cab0 Date::sub_weeks thiscall(0xae2c90, &saída, 8), chamada duas vezes (o ano da primeira, o dia da segunda);
#   0x5acbe0 thiscall(fm, dia, ano, dia de hoje, ano de hoje, id do clube, &lista, -1, -1, -1) ret 0x24 (o ano de hoje
#   num dword com a palavra de cima do ponteiro do contrato, mantida; o dia e o ano de início em registradores com
#   resto em cima: a cópia passa as palavras, que é o que a busca lê); a lista andada para trás (+0xa = +6, 0x68d1d0)
#   até o jogo com +0x43 >= 0; 0x729600 thiscall([0xb6b7c0], fixture +8, &registro) com o retorno não lido;
# - as 20 linhas de 0x49 bytes do bloco do time do clube (+0x71): cartões amarelos +0x2c, vermelho +0x2d, as infrações
#   +0x2e e +0x2f;
# - motivo 4: [[0xb74350]+0x30] + id da ficha x 0x27, byte +0x26 & 0xf.
# Prólogo: push -1; push 0x9593c8 (7 bytes, sem relativo). Corpo igual no cm0102.exe e no cm0102-1080.exe.
# Comparador: lado a lado com o gerador restaurado (a resposta e o estado do gerador).
ROTINAS = [('is_fine_justified', 0x5a69b0, [0x6a, 0xff, 0x68, 0xc8, 0x93, 0x95, 0x00])]
DOCS = ('se a multa é justa para o jogador',
        'whether a fine is justified for the player',
        [('is_fine_justified', 'Fine::is_fine_justified', '0x5a69b0', 'Fine.cpp', 'thiscall, ret 0xc',
          'se o jogador multado aceita a multa (1) ou a contesta (0): sem clube ou contrato, 1. Uma multa ainda valendo do mesmo jogador: a que ele contestou torna esta injusta; a que aceitou, injusta com random(profissionalismo) = 0. Por motivo: 1 e 5, matéria na imprensa, ausência, falta ao treino (motivo até 3), incidente físico (1, 2) ou estrela emburrada (moral do contrato abaixo de −25, empenho e profissionalismo abaixo de 10, reputação acima da do clube + 250) são justas; senão o último jogo em 8 semanas: cartão amarelo por infração 1, 2, 4, 5, 6 ou 8 (motivo 1), expulsão violenta (0xb, 0xc), dois amarelos (até 2) ou vermelho direto (até 3) são justas; jogo de outros clubes, injusta; depois, motivo 1 com temperamento + pressão acima de random(50), justa; por fim random(100) = 0, justa. 2: expulsão violenta ou vermelho direto no último jogo. 3: expulsão violenta no último jogo. 4: a última nota (nenhuma, injusta; abaixo de 4, justa; 4 até o motivo 2 e 5 até o motivo 1, justas). Outro motivo: injusta',
          "whether the fined player accepts the fine (1) or contests it (0): without a club or a contract, 1. A fine of the same player still running: one he contested makes this one unfair; one he accepted, unfair with random(professionalism) = 0. By reason: 1 and 5, a press story, absence, missed training (reason up to 3), a fitness incident (1, 2) or a sulking star (contract morale below -25, work rate and professionalism below 10, reputation above the club's + 250) are fair; otherwise the last match in 8 weeks: a booking for offence 1, 2, 4, 5, 6 or 8 (reason 1), a violent sending-off (0xb, 0xc), two bookings (up to 2) or a straight red (up to 3) are fair; a match of other clubs, unfair; then reason 1 with temperament + pressure above random(50), fair; finally random(100) = 0, fair. 2: a violent sending-off or straight red in the last match. 3: a violent sending-off in the last match. 4: the last rating (none, unfair; below 4, fair; 4 up to reason 2 and 5 up to reason 1, fair). Any other reason: unfair")])
