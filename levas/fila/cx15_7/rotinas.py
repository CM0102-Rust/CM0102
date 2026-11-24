# O texto da noticia da inscricao europeia (squad_manager.cpp; leitor cx15, rodada 5, 24/11/2026). Carreira ENG (sem
# LIGAS). Escritor de texto: roda quando a noticia 0x19 (send_euro_registration_news, cx15_6) e aberta e, com
# CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando e enviada (0x77ffef no despachante dos textos); so
# a comparacao o prova, e so com clube humano numa copa europeia.
# - 0x85b190 SQUAD_MANAGER::write_euro_registration_news_text thiscall(noticia, corpo, texto) ret 0xc (o ecx nao e
#   lido). Os slots lidos em ordem 0 a 6 (NewsItem::get_slot 0x785830 thiscall(k), ret 4): o clube pelo indice, a
#   competicao pelo id (registro de 0x6b bytes em [0xae23d0]), o passo, o slot 3 (dword: inscricao feita), as vagas, os
#   inscritos e o slot 6 (quantos foram inscritos). A original guarda as vagas e os inscritos sobre os bytes baixos dos
#   seus argumentos noticia e corpo (e, no passo 1, o ponteiro do nome do clube sobre o corpo): so locais.
#   Manchete: passo 1 0xa82d3c, 2 0xa82cf4, 3 0xa82cac (o clube, 0x7d5); outro passo nao escreve. Corpo: passo 1 0xa834dc
#   (o maximo = as vagas) e, com o slot 3 e o slot 6 >= 0, 0xa834a8; passo 2, Liga dos Campeoes (chave 0x9cf6e8): vagas -
#   inscritos = 1 0xa8340c, inscritos 0xa83360 com a diferenca, senao 0xa832b8 com as vagas; outras copas 0xa8321c,
#   0xa8316c, 0xa830c4; com o slot 3: slot 6 = 0 0xa8309c, 1 0xa83074, negativo o texto vazio (strcpy de 0xad9c64),
#   senao 0xa8303c (com inscritos) ou 0xa8300c; passo 3, 25 - inscritos >= 2 0xa82dc4, 0 0xa82f34, 1 0xa82e7c (o byte
#   baixo; negativo cai no primeiro) e, com o slot 3, 0xa82d88; outro passo nao escreve. langlib_sprintf 0x66f4e0 (cdecl)
#   no buffer 0xde1f64 com os generos (+0x52) do clube e da competicao duas vezes cada, Text::set_string 0x4ae660,
#   Text::add_link 0x4ae8a0 (o clube 0x7d5, a competicao 0x7d0), Text::append_string 0x4ae770.
#   Prologo mov eax, [esp+8]; sub esp, 8 (7 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('write_euro_registration_news_text', 0x85b190, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x08])]
DOCS = ('o texto da notícia da inscrição europeia',
        'the text of the European registration news',
        [('write_euro_registration_news_text', 'SQUAD_MANAGER::write_euro_registration_news_text', '0x85b190', 'squad_manager.cpp', 'thiscall, ret 0xc',
          'escreve o texto da notícia da inscrição europeia: a manchete pela fase (preliminar, seguinte, finais); o corpo com o máximo de inscritos da fase preliminar, os que ainda cabem na fase de grupos da Liga dos Campeões ou nas três primeiras rodadas das outras copas, ou os acréscimos e substituições das fases finais, e, quando a inscrição foi feita, quantos foram inscritos',
          'writes the text of the European registration news: the headline by phase (qualifying, next, later stages); the body with the maximum to register for the qualifying phase, how many still fit in the Champions League group stage or the other cups\' first three rounds, or the additions and replacements of the later stages, and, when the registration was made, how many were registered')])
CAMPOS = []
INTERESSA = []
