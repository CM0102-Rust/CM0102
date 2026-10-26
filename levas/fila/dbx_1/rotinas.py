# nation_get_nationality_adjective (0x6181d0, index.cpp; ~76 chamadores: regras por nação, notícias, olheiro). A rotina
# vai de 0x6181d0 a 0x62a67a (74.922 bytes; o mapa de funções corta em 26.014) e é igual no cm0102-1080.exe (o
# resolucao.py não usa espaço dela). Quase toda tabela: 937 nações em cadeias por idioma (9, 4, 5, 10, 6), geradas do
# exe por engenharia-reversa/adjetivos_nacao.py (execução simbólica por todos os caminhos, conferência exaustiva
# grafo × tabelas; os textos vão pelo endereço). Sem SEH, sem locais (só ebx/esi/edi/ebp salvos), sem sorteio, sem
# caixa. O jogo dos testes está em inglês (idioma 0): ali só roda o começo (zero, nulos, tamanho < 50) e o nome da
# nação (+0x57); as cadeias só rodam em português (9), italiano (4), espanhol (5) e nos idiomas 10 e 6.
SAVE = False
ROTINAS = [('adjetivo_da_nacao', 0x6181d0, [0x8b, 0x44, 0x24, 0x04, 0x55])]
DOCS = ('o adjetivo de nacionalidade pelo idioma e pela forma',
        'the nationality adjective by language and form',
        [('adjetivo_da_nacao', 'nation_get_nationality_adjective', '0x6181d0', 'index.cpp', 'cdecl',
          'o adjetivo da nação no texto (tamanho >= 50): com gênero 0 ou fora dos idiomas 9, 4, 5, 10 e 6 o nome da nação; nesses, a forma da 0x545a60 e as cadeias de 937 nações (geradas do exe por adjetivos_nacao.py, textos pelo endereço), com o nome se o texto ficar vazio (menos nos idiomas 4 e 5); devolve 1 (0 com nação ou texto nulo ou tamanho curto)',
          "the nation's adjective into the text (size >= 50): with gender 0 or outside languages 9, 4, 5, 10 and 6 the nation's name; in those, the form from 0x545a60 and the chains of 937 nations (generated from the exe by adjetivos_nacao.py, texts by address), with the name if the text stays empty (except in languages 4 and 5); returns 1 (0 with a null nation or text or a short size)")])
CAMPOS = [('TNation +0 id, +0x57 nome; [0x9cf178..0x9d0527] chaves das nações; [0x9eb2dc] idioma; lê (adjetivo_da_nacao)',
           'TNation +0 id, +0x57 name; [0x9cf178..0x9d0527] nation keys; [0x9eb2dc] language; reads (adjetivo_da_nacao)')]
INTERESSA = ['SGE', 'CMRTE', 'CMScout']
