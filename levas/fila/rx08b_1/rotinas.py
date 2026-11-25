# O relatorio do treinador sobre um jogador: a fase da carreira, os dias de que ele precisa, os vereditos e a noticia
# (coach.cpp; leitor rx08b, rodada 6, 25/11/2026). TELAS: so a tela "Request Coach Report" (coach_report_request_handle_action
# 0x48c290, ja Rust, chama a 0x487c70 no Confirm) leva a elas; a cobertura do mundo viu 0 chamadas de cada.
# Caminho no jogo: ficha de um jogador do proprio clube -> pedir relatorio ao treinador (Request Coach Report) -> marcar
# os treinadores -> Confirm; a noticia do relatorio chega no dia seguinte (o texto e da 0x487f60, item rx08b_2).
# Quem chama (desmontagem): 0x487c70 <- 0x48c500 (handle_action); 0x48af30 e 0x48aff0 <- 0x487c70; 0x48ba10 <- 0x48aff0.
# - 0x487c70 coach_post_report_news thiscall(tecnico, jogador) ret 8, this = 0xadadf4 (nao lido); quadro SEH so para o
#   NewsItem local (sem destrutor no caminho normal); prologo mov eax, fs:[0] (6).
# - 0x48af30 coach_get_player_career_stage thiscall(jogador) ret 4, so al; prologo push esi; mov esi, [esp+8] (5).
# - 0x48aff0 coach_assess_player thiscall(tecnico, jogador, &veredito, &potencial) ret 0x10; quadro SEH so de um Date
#   local (0x54a110) que nao e lido; prologo push -1; push 0x955fdb (7). A conta da ambicao (0x83bf40 e a segunda
#   0x7bca90, ftol guardado em [esp+0x28] da rotina) nunca e lida: as duas chamadas ficam, a conta nao.
# - 0x48ba10 coach_get_assessment_days thiscall(tecnico, jogador) ret 8; prologo sub esp, 0x200 (6).
# As quatro iguais no cm0102-1080.exe (inteiras). Sem relativo nos prologos.
# Comparadores: a fase e os dias lado a lado (resposta e o G_CONTRACT_LOOKUP_RESULT); os vereditos lado a lado nas duas
# palavras, no G_CONTRACT_LOOKUP_RESULT e no gerador (sorteia quatro vezes 500); a noticia so a original.
TELAS = True
SAVE = False
ROTINAS = [('coach_post_report_news', 0x487c70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('coach_get_player_career_stage', 0x48af30, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('coach_assess_player', 0x48aff0, [0x6a, 0xff, 0x68, 0xdb, 0x5f, 0x95, 0x00]),
           ('coach_get_assessment_days', 0x48ba10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o relatório do treinador sobre um jogador', "a coach's report on a player",
        [('coach_post_report_news', 'coach_post_report_news', '0x487c70', 'coach.cpp', 'thiscall, ret 8',
          'regra: o relatório do treinador vai ao clube dele como notícia (0x1391): quem, quão adaptado o jogador está (1 depois de 1000 dias com lealdade acima de 10, 4 depois de (21 − adaptabilidade) × 5 + 90 dias, 2 antes), a fase da carreira, os dois vereditos, as características, os pontos fortes e fracos, as posições, a idade, a reputação do clube dele e se há transferência marcada, se está ausente sem licença e se tem ficha de comissão',
          "rule: a coach's report goes to his club as a news item (0x1391): who, how settled the player is (1 after 1000 days with loyalty above 10, 4 after (21 − adaptability) × 5 + 90 days, 2 before), the career stage, the two verdicts, his traits, strengths and weaknesses, positions, age, his club's reputation and whether he has a future transfer, is absent without leave or has a staff sheet"),
         ('coach_get_player_career_stage', 'coach_get_player_career_stage', '0x48af30', 'coach.cpp', 'thiscall, ret 4',
          'regra: a fase da carreira: 4 (veterano) passados 115% do limite da posição (33 goleiro, 31 zagueiro ou volante, 29 meia, 28 o resto) ou 34 anos; 1 abaixo de 18; 2 abaixo de 24; 16 (consagrado) com jogos pela seleção ou reputação acima de 6000; 8 (rodado) senão',
          "rule: the career stage: 4 (veteran) past 115% of his position's age limit (33 goalkeeper, 31 defender or defensive midfielder, 29 midfielder, 28 the rest) or past 34; 1 under 18; 2 under 24; 16 (established) with caps or a reputation above 6000; 8 (journeyman) otherwise"),
         ('coach_assess_player', 'coach_assess_player', '0x48aff0', 'coach.cpp', 'thiscall, ret 0x10',
          'regra: os dois vereditos do treinador: o atual pela nota do jogador na função dele contra o nível que o clube procura (8 craque, 4 bom, 0x200 quase, 2 elenco, 1 fraco) e o potencial até 24 anos (8, 4, 0x100, 2 ou 1 pelo tempo no clube e pela idade; 0x10 acima de 24); suavizados para reserva com treinador firme, jogador encostado, em boa fase ou elenco curto; marcas de ainda não o conhecer (0x10 e 0x20), lista de transferências (0x40), não jogador do clube (0x80), emprestado ou da base (0x40) e polivalente (0x80)',
          "rule: the coach's two verdicts: the current one by the player's score in his role against the level the club looks for (8 star, 4 good, 0x200 nearly, 2 squad, 1 weak) and the potential up to 24 (8, 4, 0x100, 2 or 1 by the time at the club and the age; 0x10 past 24); softened for a reserve with a resilient coach, a player left out, in form or a short squad; marks for not knowing him yet (0x10 and 0x20), transfer listed (0x40), not a player at the club (0x80), on loan or youth (0x40) and versatile (0x80)"),
         ('coach_get_assessment_days', 'coach_get_assessment_days', '0x48ba10', 'coach.cpp', 'thiscall, ret 8',
          'regra: quantos dias o treinador precisa para avaliar o jogador: nenhum para um craque (habilidade acima de 160, ou potencial acima de 160 com habilidade acima de 100); emprestado a um clube, 5, 10 ou 15 pela habilidade do treinador; da base, 0 a 9; senão 30 + 0 a 34 + essa base',
          'rule: how many days the coach needs to judge the player: none for a star (ability above 160, or potential above 160 with ability above 100); on loan at a club, 5, 10 or 15 by the coach\'s ability; a youth player, 0 to 9; otherwise 30 + 0 to 34 + that base')])
CAMPOS = []
INTERESSA = []
