# O genero gramatical do nome de uma fase, rodada ou grupo (comp_util.cpp; leitor cx01, rodada 5, 24/11/2026). Carreira
# ENG (sem LIGAS). Rotina grande (20.320 bytes ate a fronteira, quase tudo tabela de salto): sozinha no item.
# - 0x4ba8f0 comp_util_get_stage_name_gender cdecl(id, estilo) -> al: so le a palavra do id, o byte do estilo e o idioma
#   (dword [0xae3138]+8). Os idiomas 2, 3, 4, 5 e 9 tem tabelas (para estilo 3 e para os outros); as tabelas foram
#   lidas rodando a original no unicorn para cada idioma 0..11, estilo 0 e 3 e id -3..0x485 (mais 0x7fff, -0x8000 e ids
#   com a palavra alta suja): 0 divergencias da copia contra essa leitura. Fora da tabela: sprintf("Cannot find index %d",
#   id) no buffer 0xde1f64 e 0 (cdecl, add esp), menos no idioma 2 com estilo 3 para id > 0x483 (0 sem o texto).
#   Chamada pelos escritores de rotulos e noticias das competicoes (comp_util 0x4b0d53..0x4c0037, 0x857f12).
# Igual no cm0102-1080.exe (inteira).
# Comparador: lado a lado nos 32 primeiros bytes do buffer 0xde1f64 e o valor. Na carreira inglesa (idioma ingles) a
# original responde 0 logo de cara: so o comparador ve a mutacao.
SAVE = False
ROTINAS = [('comp_util_get_stage_name_gender', 0x4ba8f0, [0xa1, 0x38, 0x31, 0xae, 0x00])]
DOCS = ('o gênero gramatical do nome de uma fase, rodada ou grupo',
        'the grammatical gender of a stage, round or group name',
        [('comp_util_get_stage_name_gender', 'comp_util_get_stage_name_gender', '0x4ba8f0', 'comp_util.cpp', 'cdecl',
          'devolve o gênero do nome de uma fase, rodada ou grupo no idioma do jogo, com uma tabela à parte para o estilo 3 (o plural): só os idiomas 2, 3, 4, 5 e 9 têm gêneros, os outros respondem 0; um id fora da tabela escreve "Cannot find index" no buffer de texto e responde 0 (no idioma 2, estilo 3, um id acima de 1155 responde 0 sem o texto)',
          'returns the gender of a stage, round or group name in the game\'s language, with a separate table for style 3 (the plural): only languages 2, 3, 4, 5 and 9 have genders, the others answer 0; an id outside the table writes "Cannot find index" into the text buffer and answers 0 (in language 2, style 3, an id above 1155 answers 0 without the text)')])
CAMPOS = [('configuração do jogo [0xae3138] +8: o código do idioma (dword)', 'game config [0xae3138] +8: the language code (dword)')]
INTERESSA = []
