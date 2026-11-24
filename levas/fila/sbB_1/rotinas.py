# A frase da confianca da diretoria (ou da federacao) no tecnico, do manager_manager.cpp (leitor sbB, rodada 5,
# 24/11/2026): 0x690d80, 11 KB, inteira num desvio. Escreve num buffer (nao num Text): a usam o corpo da noticia da
# confianca da diretoria (news_board_confidence_update_text 0x6a4d40, noticia (6, 0x1772), ja em Rust, chamada pelo
# endereco do exe em 0x6a4e39) e a tela da confianca da diretoria (open_board_confidence_screen 0x6b2080, em 0x6b22b7).
# Na carreira automatica roda pelo modo CM_TEXTO_NOTICIAS: a noticia 0x1772 e postada por post_board_confidence_update_news
# 0x6a4c40 (cerca de 40 mil chamadas ate 2002-06-01 na cobertura da simulacao), por alcance, e o modo escreve o texto de
# toda noticia enviada.
# Fronteiras conferidas na desmontagem: 0x690d80..0x693406 (os ret 0x10 em todo o corpo, o ultimo em 0x693403); depois as
# tabelas de salto 0x693408..0x6934c4 (ate a proxima rotina). Prologo sub esp, 0xa2c (6 bytes, sem relativas).
# Convencao: thiscall(MANAGER_MANAGER, clube, dias no cargo, buf, tamanho), ret 0x10; o tamanho nao e lido. Chamadas
# conferidas: 0x690bd0/0x690b70 thiscall(clube) ret 4 (o al); 0x690c30 thiscall(clube, &tipo, &tamanho) ret 0xc;
# 0x53b900 cdecl(clube); 0x5a0590 thiscall([0xae2a38], clube) ret 4; 0x5986b0 thiscall(financas) (al com sinal);
# 0x6181d0 cdecl(nacao, buf, 100, 9); 0x54cdb0 thiscall(0xae2c90, d0, d1) ret 8; 0x66f4e0 langlib_sprintf e 0x944c9f
# sprintf, cdecl. O tamanho da serie vai com movsx (byte com sinal).
# As tres tabelas de modelos (5 + 175 + 700 entradas, 289 modelos distintos) foram geradas emulando a original (unicorn,
# com as chamadas trocadas por stubs) em todas as entradas, e o codigo.rs foi conferido de volta pelo mesmo emulador:
# 7782 casos (todas as faixas, tipos de serie, faixas de dias, e uma grade da confianca, do humor da torcida, dos dias
# desde o ultimato, com e sem selecao e nacao), 0 diferencas. Os textos "UNKNOWN ... STATE" (3) sao inalcancaveis.
# Defeito do original: selecao sem nacao recebe o buffer do adjetivo nunca escrito (a copia o deixa vazio).
SAVE = False
ROTINAS = [('build_board_confidence_text', 0x690d80, [0x81, 0xec, 0x2c, 0x0a, 0x00, 0x00])]
DOCS = ('a frase da confianca da diretoria no tecnico', "the board's confidence in the manager",
        [('build_board_confidence_text', 'MANAGER_MANAGER::build_board_confidence_text', '0x690d80', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'escreve num buffer a opiniao da diretoria (ou da federacao) sobre o tecnico do clube, a frase da noticia e da tela da confianca da diretoria. Pelos dias no cargo: menos de 30, uma frase pela faixa de riqueza (-2..2); menos de 60, pela riqueza, pela faixa da confianca da diretoria (0..6) e pela serie atual (vitorias, invicto, derrotas, sem vencer, nenhuma, com o tamanho da serie quando o modelo o conta); a partir de 60, pela riqueza, pela faixa da confianca na liga (0, 1, 2, 3 a 6), pela da diretoria e pela serie. Selecao: o veredito da federacao pela confianca (acima de 9500, 8500, 6500, 4275, 3500, ou "muito desapontada"), com o adjetivo da nacao. A partir de 60 dias e na selecao, o fecho: com ultimato ha menos de 120 dias e confianca abaixo de 6500, a frase vira a da arrogancia com a diretoria (acima de 4275 "um tanto insatisfeita", acima de 3500 "desapontada", com o apoio da torcida quando o humor dela passa de 1500); senao um segundo paragrafo pelo humor da torcida (abaixo de -750 os pedidos de demissao, pela confianca abaixo de 3750 ou de 5500; abaixo de -500 o apoio que esfria; acima de 1500 a boa relacao com a torcida). Defeito do original: selecao sem nacao recebe o adjetivo nunca escrito',
          "writes into a buffer the board's (or the FA's) opinion of the club's manager, the sentence of the board confidence news and screen. By the days in charge: under 30, one sentence by the wealth band (-2..2); under 60, by the wealth, the board confidence band (0..6) and the current run (won, unbeaten, lost, without a win, none, with the run's length when the template counts it); from 60, by the wealth, the league confidence band (0, 1, 2, 3 to 6), the board band and the run. National team: the FA's verdict by the confidence (above 9500, 8500, 6500, 4275, 3500, or \"very disappointed\"), with the nation's adjective. From 60 days and for a national team, the closing: with an ultimatum less than 120 days ago and the confidence under 6500, the sentence becomes the one of the arrogance towards the board (above 4275 \"somewhat unsatisfied\", above 3500 \"disappointed\", with the fans' support when their mood is above 1500); otherwise a second paragraph by the fans' mood (under -750 the resignation calls, by the confidence under 3750 or 5500; under -500 the waning support; above 1500 the rapport with the fans). Defect of the original: a national team without a nation gets an adjective never written")])
