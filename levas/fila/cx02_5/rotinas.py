# A noticia do destino de um clube no Trofeu das Divisoes Inferiores (Auto Windscreens Shield) (eng_auto_cup.cpp;
# leitor cx02, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS); escritor de texto: roda quando a noticia e montada e
# aberta (o modo de textos da cadeia).
# - 0x56b2b0 EngAutoCup::build_fate_news_text thiscall(clube_id, desfecho, fase, a4, a5, a6, a7, a8, noticia, texto)
#   ret 0x28 (a virtual 0x50): com noticia, 0x4b0b80 comp_util_write_cup_tie_result_news (6 pushes; add esp, 0x18 ->
#   cdecl) com (clube, a5, a8, desfecho, a6, texto), em qualquer fase; sem noticia, o campeao de uma secao (fase -1 ou 0,
#   desfecho 1): langlib_sprintf 0x66f4e0 (8 pushes; add esp, 0x20 -> cdecl) no buffer 0xde1f64 com 0x9c41bc ("{} through
#   to {} final"), Text::set_string 0x4ae660 (push; mov ecx -> thiscall, ret 4) e dois Text::add_link 0x4ae8a0 (3 pushes
#   -> thiscall, ret 0xc): o clube (0x7d5) e a competicao (0x7d0); os outros, Comp::build_fate_news_text 0x48c6d0
#   (10 pushes; mov ecx, this -> thiscall, ret 0x28) com a7 e a noticia 0.
#   Prologo mov eax, [esp+0x24]; push ebx (5 bytes).
# Igual no cm0102-1080.exe.
# Comparador: a manchete lado a lado com compare_text_writer; os caminhos que passam a noticia adiante, so a original.
SAVE = False
ROTINAS = [('eng_auto_cup_build_fate_news_text', 0x56b2b0, [0x8b, 0x44, 0x24, 0x24, 0x53])]
DOCS = ('a notícia do destino de um clube no Troféu das Divisões Inferiores', 'the Auto Windscreens Shield fate news',
        [('eng_auto_cup_build_fate_news_text', 'EngAutoCup::build_fate_news_text', '0x56b2b0', 'eng_auto_cup.cpp', 'thiscall, ret 0x28',
          'regra: no Troféu das Divisões Inferiores, o campeão de uma seção (norte ou sul) ganha a manchete "{clube} na final do {competição}", com os links do clube e da competição; com uma notícia, o resultado do confronto; os outros destinos, a notícia genérica da competição',
          'rule: in the Auto Windscreens Shield a section winner (north or south) gets the headline "{club} through to {competition} final", with links to the club and the competition; with a news item, the tie result; other fates, the competition\'s generic news')])
CAMPOS = [('competição +0x52: gênero do nome (byte usado no texto)', 'competition +0x52: the name gender (byte used in the text)')]
INTERESSA = []
