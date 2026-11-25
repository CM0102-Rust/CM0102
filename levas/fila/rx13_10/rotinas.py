# O texto da notícia do destino na Terceira Divisão inglesa (eng_third.cpp; leitor rx13, rodada 6, 25/11/2026).
# Simulação ENG: a repescagem da Terceira Divisão em maio de 2002; o texto só sai com o modo de textos das notícias
# (CM_TEXTO_NOTICIAS, --textos) e um técnico humano como destinatário. Usa o fate_base_text, o fate_headline e o
# compare_fate_text do rx13_8: integrar depois dele (INTEGRAR_ANTES.txt).
# - 0x5785b0 EngThird::build_fate_news_text thiscall(clube, destino, fase, a4, a5, a6, a7, a8, notícia, texto) ret 0x28:
#   destino (al) 1, fase (dl) 0 e sem notícia: langlib_sprintf 0x66f4e0(0xde1f64, "{} win promotion", gênero x2, nome
#   curto), Text::set_string 0x4ae660 e o vínculo do clube (0x7d5); senão Comp::build_fate_news_text 0x48c6d0 com os
#   mesmos argumentos, a fase como veio (o dword) e a7 = 0 (cada push conferido). Prólogo mov eax, [esp+8];
#   mov edx, [esp+0xc] (8 bytes).
# Igual no cm0102-1080.exe. Comparador: a manchete lado a lado (compare_text_writer); o texto base só a original.
SAVE = False
ROTINAS = [('eng_third_build_fate_news_text', 0x5785b0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x0c])]
DOCS = ('o texto da notícia do destino na Terceira Divisão inglesa',
        'the fate news text of the English Third Division',
        [('eng_third_build_fate_news_text', 'EngThird::build_fate_news_text', '0x5785b0', 'eng_third.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na Terceira Divisão inglesa: o vencedor da repescagem, sem a notícia completa, recebe "conquista o acesso"; os outros destinos, o texto base da competição',
          'the fate news text in the English Third Division: the play-off winner, without the full news, gets "win promotion"; every other fate, the base competition text')])
CAMPOS = []
INTERESSA = []
