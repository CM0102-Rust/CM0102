# O relatorio do fisioterapeuta: a avaliacao, a noticia e o texto dela (physio.cpp; leitor rx11b, rodada 6,
# 25/11/2026). Telas: so o pedido do relatorio (o clique 0x7a2f10, tel36_3) gera a noticia; o texto, quando a noticia e
# aberta (ou escrita com CM_TEXTOS=1).
# Caminho no jogo: o perfil de um jogador do proprio clube -> pedir o relatorio do fisioterapeuta -> Confirmar; abrir a
# noticia "Physio ... reports on ..." na caixa de entrada (manchete e corpo). Para os ramos: um jogador com lesao
# recorrente, um lesionado agora, um de outro clube (em emprestimo) e um recem-chegado (pouco contato).
# Convencoes conferidas na desmontagem:
# - 0x7a2650 physio_assess_player_fitness thiscall(fisio, jogador, &bandeiras (palavra), &tipo (byte)) ret 0x10 (this =
#   0xb74344, passado ao 0x7a2900): 0x543fd0 cdecl(fisio, ficha, 10) -> al; 0x62ece0/0x62ed30 thiscall([0xb63d28],
#   jogador) ret 4 -> al; 0x54cdb0 thiscall(0xae2c90, +0x3e, +0x42) ret 8; 0x4cf5a0 thiscall([0xae1050], jogador) ret 4,
#   o resultado gravado em [0xae1080]; 0x7a2900 thiscall(this, fisio, jogador) ret 8; x87: fild, fmul [0x968e00] (0,4),
#   fcom com (x - 75) e _ftol 0x945640 (o menor dos dois, truncado); os idiv com sinal.
# - 0x7a1a90 physio_file_report_news thiscall(fisio, jogador) ret 8: 0x7a25c0 thiscall(this, jogador) ret 4 -> al;
#   0x7bd9d0 cdecl(jogador, fisio, &palavra1, &palavra2); 0x77bbe0 NewsItem(0x1392, 0) ret 8; 0x785790 x 16;
#   0x62ed70 thiscall([0xb63d28], jogador) ret 4; 0x7861e0 thiscall([0xb7431c], noticia, [fisio+0x39]) ret 8. O truque
#   setge/dec ebx/and/add 4 deixa 4 ou 2 em bl (os bytes de cima de ebx sao os do chamador; so bl e lido).
# - 0x7a1cd0 physio_write_report_news_text thiscall(this, noticia, completo, Text) ret 0xc, quadro de 0x1160 (chkstk):
#   0x785830 x 16; 0x53e140 cdecl(3 ids, buf, 0xc8, forma 1/5, id); 0x66f4e0 langlib_sprintf; 0x944c9f sprintf da CRT
#   para juntar os paragrafos; 0x62ef30 thiscall([0xb63d28], tipo, buf, 0x64, 1) ret 0x10; 0x6306e0 thiscall([0xb63d28],
#   tipo) ret 4 -> al (pedido duas vezes); 0x7e5780 cdecl(dias, buf, 0x64, 1); tabela de salto 0x7a2568/0x7a2570 (tipo -
#   0xb <= 0x41: 0 especialista, 1 cirurgia); 0x53e990 cdecl(pessoa, 3 ids, 0); 0x4ae660, 0x4ae8a0 (0x7d7).
# Prologo do 0x7a1cd0: mov eax, 0x1160 (5 bytes, absoluto; o call _chkstk vem depois).
# Comparadores: a avaliacao lado a lado (as duas saidas, [0xae1080] e o byte +0xc do registro de 0x4f bytes do
# jogador); a noticia so a original, com o plano (a noticia guardada na caixa do tecnico humano do clube do fisio); o
# texto lado a lado (compare_text_writer).
TELAS = True
ROTINAS = [('physio_assess_player_fitness', 0x7a2650, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x18]),
           ('physio_file_report_news', 0x7a1a90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('physio_write_report_news_text', 0x7a1cd0, [0xb8, 0x60, 0x11, 0x00, 0x00])]
DOCS = ('o relatório do fisioterapeuta: a avaliação, a notícia e o texto dela',
        "the physio report: the assessment, the news item and its text",
        [('physio_assess_player_fitness', 'physio_assess_player_fitness', '0x7a2650', 'physio.cpp', 'thiscall, ret 0x10',
          'regra: o juízo do fisioterapeuta (os ids somados mod 120 − 60, mais 10 x o atributo 10 dele) decide: a lesão recorrente (acima de id mod 40 − propensão + 75 com nível de recorrência, ou nível acima de 1 e reputação mundial acima de 4750), o tratamento (nível acima de 1 e propensão + nível acima de 15; curandeiro quando o id do fisio é múltiplo de 146); jogador de outro clube: juízo min(0,4 x, x − 75); mesmo clube com pouco tempo de casa: min(x/2, x − 50) e "pouco contato"; propensão abaixo de 4: raramente se machuca, acima de 17: propenso forte, senão propenso; físico natural acima de 15: dificuldade de manter a forma, abaixo de 6: saudável',
          "rule: the physio's judgement (the ids summed mod 120 − 60, plus 10 x his attribute 10) decides: the recurring injury (above id mod 40 − proneness + 75 with a recurrence level, or a level above 1 and a world reputation above 4750), the treatment (a level above 1 and proneness + level above 15; a faith healer when the physio id is a multiple of 146); a player at another club: judgement min(0.4 x, x − 75); same club, little time there: min(x/2, x − 50) and \"not enough contact\"; proneness below 4: rarely injured, above 17: strongly prone, else prone; natural fitness above 15: trouble sustaining fitness, below 6: healthy"),
         ('physio_file_report_news', 'physio_file_report_news', '0x7a1a90', 'physio.cpp', 'thiscall, ret 8',
          'regra: o relatório vai como notícia ao técnico do clube do fisioterapeuta, com os dois, a avaliação, o estágio da idade, a familiaridade (1 com mais de 1000 dias no clube e lealdade acima de 10; 4 com 5 x (21 − adaptabilidade) + 90 dias; senão 2), os traços do jogador e a lesão atual',
          "rule: the report goes as news to the manager of the physio's club, with both people, the assessment, the age stage, the familiarity (1 after more than 1000 days at the club and a loyalty above 10; 4 after 5 x (21 − adaptability) + 90 days; else 2), the player's traits and the current injury"),
         ('physio_write_report_news_text', 'physio_write_report_news_text', '0x7a1cd0', 'physio.cpp', 'thiscall, ret 0xc',
          'regra: a manchete varia pela soma dos ids mod 3; o corpo junta o veredito do físico, as lesões (a atual com o tempo, a recorrente) e, com tratamento, curandeiro, especialista (8 tipos de lesão) ou cirurgia; sem nenhuma marca, a atitude do jogador com a saúde',
          "rule: the headline varies with the ids' sum mod 3; the body joins the fitness verdict, the injuries (the current one with its time, the recurring one) and, with a treatment, a faith healer, a specialist (8 injury types) or surgery; with no flags at all, the player's attitude to his health")])
CAMPOS = []
INTERESSA = []
