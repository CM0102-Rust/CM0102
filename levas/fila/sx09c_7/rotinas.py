# A multa de um jogador (Fine::fine_player 0x5a5980, Fine.cpp, 0x5a5980..0x5a69a9, 4 KB; leitor sxA, rodada 5,
# 24/11/2026). Chamada pela AWOL (0x41a60a, 0x41b003: o técnico da IA multa quem falta ao treino; roda na carreira de
# férias) e pela tela da Disciplina (0x5a8ecf). Simulação, sem LIGAS. Junto com o sx09c_8 (is_fine_justified), que ela
# chama pelo endereço.
# Convenções conferidas na desmontagem:
# - thiscall(fine, jogador, período, motivo) ret 0xc, sem retorno; quadro SEH com uma NewsItem local de 0xe0 bytes
#   (0x77bbe0, estados 3, 5, 6) e duas datas locais (0x54a110); o motivo 0 ou 1 com o jogador ausente sem licença vira
#   5 só no byte de baixo do argumento (o dword segue para a 0x5a69b0 e a 0x5a74b0);
# - 0x4cf5a0 duas vezes (grava [0xae1080] nas duas); 0x4cf5f0 thiscall(cm, jogador, clube); 0x418f60
#   thiscall([0xad9c6c], jogador, 0) -> al; 0x5a69b0 thiscall(fine, jogador, período, motivo) -> al;
# - 0x5a74b0 post_fine_reaction_news é stdcall(jogador, motivo, reação, período, efeitos) (o ecx carregado não é
#   lido; num ramo o 5º argumento é empilhado antes do desvio, o mesmo valor 0); 0x56a100 thiscall([0xae2a30], jogador,
#   clube, a, b) ret 0x10 (sx09c_6, pelo endereço); 0x419840 thiscall([0xad9c6c], jogador, 0, 0);
# - 0x8676f0 -> al/eax, 0x8676d0 thiscall(contrato, 2), 0x867700 thiscall(contrato, marca, 0), 0x866f60 -> eax;
# - 0x600a50 cdecl(clube, 0, 1, 0, 0); 0x544410 cdecl(np, byte +0x19) -> al; 0x547e70, 0x547af0 cdecl(quem, de quem,
#   1); 0x547810 cdecl(jogador, colega);
# - a diretoria: [[0xb67a44]] + id do clube × 0x49, palavras +6, +0xa, +0xc somadas em 16 bits e presas em 1..10000;
#   o registro de 6 bytes (1, 8, id) na lista [[0xb67a44]+4] por 0x68d0d0 e o bit 2 de +2;
# - 0x54c5b0 Date::add_days e 0x54c770 Date::add_weeks thiscall(0xae2c90, &saída, n); 0x945640 _ftol do byte +0xa
#   da linha de 0x4f bytes ([0xae2d0c]) × 0,5 (0x968968);
# - os registros: 17 bytes em [fine+4] (assistente, jogador, mudança, data) e 14 bytes em [fine] (jogador, motivo,
#   resposta, data).
# Prólogo: push -1; push 0x95935d (7 bytes, sem relativo). Corpo igual no cm0102.exe e no cm0102-1080.exe.
# Comparador: só a original (a multa mexe no elenco, na diretoria, nas notícias e nas disputas); confere que a lista
# [fine] cresce no máximo um registro. A cópia é provada pelas mutações na carreira (modo Rust).
# Defeito do original: o byte dos efeitos vai à notícia num dword com os 3 bytes de cima sem valor (a notícia só lê o
# byte).
ROTINAS = [('fine_player', 0x5a5980, [0x6a, 0xff, 0x68, 0x5d, 0x93, 0x95, 0x00])]
DOCS = ('a multa de um jogador',
        'fining a player',
        [('fine_player', 'Fine::fine_player', '0x5a5980', 'Fine.cpp', 'thiscall, ret 0xc',
          'multa um jogador do clube por um período (semanas) e um motivo: jogador com ficha, id válido, contrato e contrato com o clube; motivo 0 ou 1 com o jogador ausente sem licença vira 5. Se ele acha justa: os colegas perdem a marca 2 do contrato (random(profissionalismo) ≠ 0); salvo quando o profissionalismo e o motivo a tornam certa (acima de 2 com motivo 2 ou 3, acima de 4 com motivo 1) e random(profissionalismo) = 0, ele dá de ombros (notícia, marca 0x200 e, conforme o perfil, a queixa (4, 0) ou a ausência); senão os efeitos por motivo (profissionalismo, empenho, temperamento, agressividade, determinação, o estado de treino), a notícia e, com mudança no treino num clube humano com assistente, o registro do assistente. Se acha injusta: a contagem de multas do clube sobe; o técnico humano perde lealdade; com a confiança da diretoria abaixo de 4250, acima do limite do presidente a diretoria age e a multa para (fila de saída do técnico, ou confiança menos 100 × disciplina do presidente ou 500, menos 250 do elenco e da quarta, e a notícia); no limite, menos 100 e a notícia; abaixo, menos 50. Depois o jogador: aceita (moral menos 15 ou 5 por semana) ou se ressente (moral menos 20 por semana; antipatias, a reação dos colegas, a notícia, ausência) e, com o perfil de estrela, a queixa (3, 1). Por fim o registro de 14 bytes da multa (jogador, motivo, resposta, hoje + período semanas)',
          "fines a player of the club for a period (weeks) and a reason: a player with a sheet, a valid id, a contract and a club contract; reason 0 or 1 for a player absent without leave becomes 5. If he finds it fair: the team-mates lose contract flag 2 (random(professionalism) ≠ 0); unless his professionalism and the reason make it certain (above 2 with reason 2 or 3, above 4 with reason 1) and random(professionalism) = 0, he shrugs it off (the news, flag 0x200 and, by his profile, the complaint (4, 0) or an absence); otherwise the effects by reason (professionalism, work rate, temperament, aggression, determination, the training state), the news and, after a training change at a human club with an assistant, the assistant's record. If he finds it unfair: the club's fine count goes up; the human manager loses loyalty; with the board's confidence below 4250, above the chairman's threshold the board acts and the fine stops (the manager's departure queued, or confidence less 100 × the chairman's discipline or 500, less 250 on the staff and fourth ones, and the news); at the threshold, less 100 and the news; below it, less 50. Then the player: he takes it (morale less 15 or 5 per week) or resents it (morale less 20 per week; dislikes, the team-mates' reaction, the news, an absence) and, with a star's profile, complains (3, 1). Finally the fine's 14-byte record (player, reason, verdict, today + period weeks)")])
