# Os tratadores da diretoria: o ultimato do tecnico (a resposta e o tratamento), os pedidos a diretoria, o recurso de
# suspensao a federacao e a candidatura a um cargo (manager_manager.cpp; leitor rx08b, rodada 6, 25/11/2026). TELAS: a
# cobertura do mundo viu 0 chamadas de cada; so as telas do tecnico humano as disparam. Quem chama (desmontagem):
# - 0x6aca50 <- 0x6ad78e (o tratador do ultimato); 0x6ad610 <- 0x9068ef (ultimatum.cpp 0x906130, a tela do ultimato);
# - 0x6a8810 <- 0x42592e (a tela dos pedidos a diretoria, 0x42554c);
# - 0x6ab800 <- 0x55eea9 (discipline_appeal_dialog_handle_click 0x55ed00, ja Rust, a tela do recurso);
# - 0x69ca00 <- 0x69c50b (add_job_vacancy 0x69ba50, Rust: os humanos de clubes menores se candidatam quando nao estao
#   de ferias, por isso 0 na carreira automatica) e 0x6b1ce7 (manager_screens 0x6b19d0, "Apply for job").
# Caminho no jogo: Clube -> Diretoria (Board): dar um ultimato (salario, estadio, centro de treinamento, verba) e os
# pedidos (estadio, treinamento, verba, salarios, mais tempo, dinheiro); a noticia de suspensao de um jogador ->
# recorrer (o resultado chega dois dias depois); perfil de um clube com vaga -> candidatar-se (Apply for job).
# Convencoes e quadros:
# - 0x6aca50 MANAGER_MANAGER::decide_ultimatum_response thiscall(registro, clube, tecnico) ret 0xc; SEH so de um Date
#   local; tabela 0x6ad5f4 (a faixa de riqueza -1..2). Le o +8 do registro (a exigencia), grava o +9 (a resposta), a
#   data do ultimo ultimato (+0x41 da linha da diretoria), o +5, o +0x28 e o teto salarial (adjust_max_wage).
# - 0x6ad610 handle_manager_ultimatum thiscall(tecnico, clube, exigencia) ret 0xc; SEH so de um Date local. O registro
#   local tem na frente a data de amanha (0x54c5b0, 0x417c10), que ninguem le: a copia nao a monta. As noticias
#   (0x6ad930, rx08_10, pelo exe) levam (exigencia, 1, 2 ou 1, 0) e (exigencia, 3, resposta, boato) (a nota do rx08
#   trocava a ordem).
# - 0x6a8810 handle_board_request thiscall(clube, pedido) ret 8; sem SEH; tabelas 0x6a9328 (6 pedidos) e 0x6a9340 (a
#   faixa de riqueza); a resposta pela 0x6a9350 (sx14b_4, pelo exe).
# - 0x6ab800 submit_ban_appeal thiscall(pessoa, contestar, 4 regras, revogar, jogos, dias, tipo, arbitro) ret 0x2c, al;
#   o item de 0x19 bytes vai para a lista +0x14. Em dois caminhos (corte de dias e corte de jogos) o original deixa o
#   outro corte com o que havia na pilha (DEFEITOS); a copia deixa 0.
# - 0x69ca00 apply_for_job thiscall(clube, candidato) ret 8; SEH so dos Dates e das noticias locais (sem destrutor no
#   caminho normal). A historia do clube (0x86df90) e destruida e liberada como no original.
# Prologos: push -1; push imm32 (7) nas 0x6aca50, 0x6ad610, 0x6ab800, 0x69ca00; sub esp, 0x14; push ebx; push ebp (5)
# na 0x6a8810. Sem relativo. Iguais no cm0102-1080.exe (inteiras).
# Comparadores: a decisao do ultimato lado a lado (a resposta, a linha da diretoria, as financas, o gerador); as
# outras quatro so a original (mandam noticias, mexem em listas).
TELAS = True
SAVE = False
ROTINAS = [('decide_ultimatum_response', 0x6aca50, [0x6a, 0xff, 0x68, 0xcb, 0xd0, 0x95, 0x00]),
           ('handle_manager_ultimatum', 0x6ad610, [0x6a, 0xff, 0x68, 0xe8, 0xd0, 0x95, 0x00]),
           ('handle_board_request', 0x6a8810, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('apply_for_job', 0x69ca00, [0x6a, 0xff, 0x68, 0x07, 0xcd, 0x95, 0x00]),
           ('submit_ban_appeal', 0x6ab800, [0x6a, 0xff, 0x68, 0x38, 0xd0, 0x95, 0x00])]
DOCS = ('os tratadores da diretoria: ultimato, pedidos, recurso de suspensão e candidatura', "the board's handlers: ultimatum, requests, ban appeal and job application",
        [('decide_ultimatum_response', 'MANAGER_MANAGER::decide_ultimatum_response', '0x6aca50', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'regra: a diretoria responde ao ultimato: um segundo em até 180 dias do último (ou em até 90 dias no cargo) é recusado (recusa firme para técnico de reputação 5250 ou mais depois dos 180 primeiros dias do jogo, branda senão); fora isso cada exigência tem o seu teste (salário, estádio maior, centro de treinamento, verba de transferências), pesando a reputação dele e a do clube, o tempo no cargo, a confiança da diretoria, o presidente, o dinheiro e o rumo do clube; a resposta é concedido, recusado ou "ele sai"',
          "rule: the board answers the ultimatum: a second one within 180 days of the last (or within 90 days in the job) is refused (firmly for a manager of reputation 5250 or more after the first 180 days of the game, mildly otherwise); otherwise each demand has its own test (wages, a bigger stadium, better training facilities, transfer funds), weighing his reputation and the club's, his time in the job, the board's confidence, the chairman, the club's money and direction; the answer is granted, refused or 'he goes'"),
         ('handle_manager_ultimatum', 'MANAGER_MANAGER::handle_manager_ultimatum', '0x6ad610', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'regra: o ultimato do técnico: primeiro um boato quando ele é conhecido (reputação acima de 6750, ou de 2000 + um sorteio de 6750), depois a resposta da diretoria como notícia; se ele sai, a saída entra na fila (o motivo: a reputação dele, o pouco tempo no clube ou o ultimato público) e o presidente pode passar a não gostar dele; a recusa custa 1250 de confiança da diretoria, a branda 500',
          "rule: the manager's ultimatum: first a rumour when he is well known (reputation above 6750, or above 2000 + a draw of 6750), then the board's answer as news; if he goes, his departure is queued (the reason: his reputation, his short time there or the public ultimatum) and the chairman may take against him; a refusal costs 1250 of board confidence, a mild one 500"),
         ('handle_board_request', 'MANAGER_MANAGER::handle_board_request', '0x6a8810', 'manager_manager.cpp', 'thiscall, ret 8',
          'regra: a diretoria responde ao pedido do técnico (estádio maior, centro de treinamento, verba, aumento salarial, mais tempo, dinheiro): pedidos demais (mais que a paciência do presidente / 5 + 2, 3 sem presidente) são negados e custam 250 de confiança; confiança abaixo de 3500 ou o mesmo pedido de novo são negados; técnico abaixo de 5000 de reputação (4250 no treinamento) ou 1500 abaixo da do clube não é ouvido; cada pedido tem o seu teste de dinheiro, necessidade e humor do presidente',
          "rule: the board answers the manager's request (a bigger stadium, training facilities, transfer funds, a wage rise, more time, money): too many requests (more than the chairman's patience / 5 + 2, 3 without a chairman) are turned down and cost 250 of confidence; a confidence below 3500 or the same request again is turned down; a manager below 5000 reputation (4250 for training) or 1500 below the club's is not listened to; each request has its own test of money, need and the chairman's mood"),
         ('apply_for_job', 'MANAGER_MANAGER::apply_for_job', '0x69ca00', 'manager_manager.cpp', 'thiscall, ret 8',
          'regra: o técnico se candidata a um cargo: a diretoria dele quase sempre fica sabendo e perde confiança (e pode avisá-lo); a reputação de um técnico humano ganha um piso; a nota dele para o cargo é calculada, o clube abre a vaga se não tinha, e ele fica sabendo se o clube o recusou nos últimos seis meses; nota acima da melhor até agora o põe na frente',
          'rule: a manager applies for a job: his own board almost always finds out and loses confidence (and may warn him); a human manager\'s reputation gets a floor; his score for the job is worked out, the club opens the vacancy if it had none, and he hears whether the club turned him down in the last six months; a score above the best so far puts him in front'),
         ('submit_ban_appeal', 'MANAGER_MANAGER::submit_ban_appeal', '0x6ab800', 'manager_manager.cpp', 'thiscall, ret 0x2c',
          'regra: a federação julga o recurso de suspensão dois dias depois: negado de cara quando o árbitro do jogo foi bem e o recurso contesta, quando o jogador não tem clube ou técnico, ou quando o clube recorreu mais vezes que a paciência da federação e perde um sorteio contra a reputação; senão aceito por uma chance da reputação do clube e do técnico contra a conduta do jogador, nunca para clubes da Grécia ou da Austrália, com uma segunda chance de reduzir a pena; clubes que recorrem muito atraem matérias de reclamação',
          'rule: the FA hears the ban appeal two days later: refused outright when the referee had a good game and the appeal is contested, when the player has no club or manager, or when the club has appealed more often than the FA\'s patience and loses a draw against its reputation; otherwise upheld by a chance from the club\'s and manager\'s reputation against the player\'s conduct, never for clubs of Greece or Australia, with a second chance to cut the ban; clubs that appeal a lot draw complaint stories')])
CAMPOS = []
INTERESSA = []
