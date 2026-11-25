# A forma do artigo do nome de uma região (index.cpp; leitor rx13, rodada 6, 25/11/2026). Telas: a confirmação de
# mandar o olheiro a uma região, na busca de jogadores (search_screens.rs, ramo 5, e os outros dois pontos do
# player_search 0x80b2a5..0x81110b). Caminho no jogo para o Lucas: Busca de jogadores > olheiro > mandar observar uma
# região (a caixa "Assign scout <...> to <região> ?"); a comparação só tem valor diferente de 0 na língua 9.
# - 0x617780 get_region_article_form cdecl(região) -> al: langlib_get_language 0x66f4d0; língua 9: movsx da região,
#   menos 1, acima de 0xd (sem sinal) 1, senão a tabela de saltos 0x6177b0 (4, 1, 3, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0);
#   outra língua: chama a língua de novo e devolve 0. O prólogo começa com call rel32 (call 0x66f4d0): entra em
#   RELATIVAS com (0x617780, 0) (INTEGRAR_ANTES.txt).
# Igual no cm0102-1080.exe. Comparador: o byte lado a lado (a função não tem efeito).
TELAS = True
SAVE = False
ROTINAS = [('get_region_article_form', 0x617780, [0xe8, 0x4b, 0x7d, 0x05, 0x00])]
DOCS = ('a forma do artigo do nome de uma região',
        "the article form of a region's name",
        [('get_region_article_form', 'get_region_article_form', '0x617780', 'index.cpp', 'cdecl',
          'regra: a forma do nome de uma região que escolhe o artigo nos textos do olheiro: só a língua 9 as tem (região 1, 4; região 3, 3; regiões 7 e 14, 0; as outras, 1); nas outras línguas, 0',
          "rule: the form of a region's name that picks its article in the scouting texts: only language 9 has them (region 1, 4; region 3, 3; regions 7 and 14, 0; any other, 1); every other language, 0")])
CAMPOS = []
INTERESSA = []
