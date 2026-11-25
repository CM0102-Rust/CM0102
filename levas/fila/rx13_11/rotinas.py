# O texto da notícia do destino na Primeira Divisão finlandesa (fin_first.cpp; leitor rx13, rodada 6, 25/11/2026).
# Simulação com o grupo da Finlândia (ENG SWE FIN NOR POR, até 2002-08-01): a primeira fase e as repescagens de outubro
# de 2001; o texto só sai com o modo de textos das notícias (CM_TEXTO_NOTICIAS, --textos) e um técnico humano como
# destinatário (0 chamadas na cobertura do mundo). Usa o fate_base_text, o fate_headline e o compare_fate_text do
# rx13_8: integrar depois dele (INTEGRAR_ANTES.txt).
# - 0x594a10 FinFirst::build_fate_news_text thiscall(clube, destino, fase, a4, a5, a6, a7, a8, notícia, texto) ret 0x28
#   (cada push conferido): destino 6 e fase (byte com sinal) < 1: "have qualified for the promotion playoffs of the
#   {liga}." (com notícia) ou "qualify for playoffs", os dois vínculos. Senão: fase 1, base(1); fase 2: com notícia
#   comp_util_write_cup_tie_result_news 0x4b0b80 cdecl(clube, a5, a8, destino, a6, texto), sem: destino 1 "avoid
#   relegation", 2 ou 0xff "relegated" (só o vínculo do clube); fase 3: com notícia base(3), sem: destino 1, a7 igual a
#   [KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION] "avoid relegation", senão "win promotion". Outras: nada. Prólogo
#   mov eax, [esp+8]; push ebx (5).
# Igual no cm0102-1080.exe. Comparador: as manchetes lado a lado (compare_text_writer); o resto só a original.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('fin_first_build_fate_news_text', 0x594a10, [0x8b, 0x44, 0x24, 0x08, 0x53])]
DOCS = ('o texto da notícia do destino na Primeira Divisão finlandesa',
        'the fate news text of the Finnish First Division',
        [('fin_first_build_fate_news_text', 'FinFirst::build_fate_news_text', '0x594a10', 'fin_first.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na Primeira Divisão finlandesa: quem passa à repescagem de acesso (com a notícia completa ou a manchete curta); na repescagem de rebaixamento, "evita o rebaixamento" ou "rebaixado"; na de acesso contra o clube da Primeira, "evita o rebaixamento" para ele e "conquista o acesso" para o da Primeira Divisão; com a notícia completa, as repescagens vão pelo resultado do confronto e pelo texto base',
          "the fate news text in the Finnish First Division: a club through to the promotion play-offs (with the full news or the short headline); in the relegation play-off, \"avoid relegation\" or \"relegated\"; in the promotion play-off against the Premier Division's club, \"avoid relegation\" for that club and \"win promotion\" for the First Division's; with the full news, the play-offs go through the tie result and the base text")])
CAMPOS = []
INTERESSA = []
