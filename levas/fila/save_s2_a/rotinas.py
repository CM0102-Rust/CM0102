SAVE = True
ROTINAS = [('filtro_do_jogo_salvo', 0x838e10, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('filtro_do_inicio_rapido', 0x839290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('versao_do_jogo_salvo_valida', 0x838eb0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o save: os filtros da caixa de arquivos (o jogo salvo, o início rápido e a versão)',
        'the saved game: the file box filters (saved game, quick start and version)',
        [('filtro_do_jogo_salvo', 'check_saved_game_file', '0x838e10', 'Setup.cpp', 'cdecl',
          'filtro da caixa de arquivos dos jogos salvos: 1 se o contêiner tem o id 0x16 (.sav), senão 2 (só ax); caminho nulo dá a caixa de erro',
          'file box filter for saved games: 1 if the container has id 0x16 (.sav), else 2 (ax only); a null path gives the error box'),
         ('filtro_do_inicio_rapido', 'check_quick_start_file', '0x839290', 'Setup.cpp', 'cdecl',
          'filtro da caixa de arquivos do início rápido: 1 se o contêiner tem o id 0x17 (.qst) e a versão gravada é a do exe, senão 2 (só ax)',
          'file box filter for quick starts: 1 if the container has id 0x17 (.qst) and the saved version is the exe one, else 2 (ax only)'),
         ('versao_do_jogo_salvo_valida', 'valid_saved_game_version', '0x838eb0', 'Setup.cpp', 'cdecl',
          'abre o general.dat do contêiner, lê a versão e o resumo (0xf70 ou 0xf74 bytes) e devolve 1 se o texto da versão do resumo é o do exe; quando dá certo não fecha o item (como a original)',
          'opens the container general.dat, reads the version and the summary (0xf70 or 0xf74 bytes) and returns 1 if the summary version text is the exe one; on success it does not close the item (as the original)')])
CAMPOS = [('contêiner (arquivo) +4: id do arquivo (0x16 .sav, 0x17 .qst); valor; lê, pelo 0x9331c0 (filtro_do_jogo_salvo, filtro_do_inicio_rapido)', 'container (file) +4: archive id (0x16 .sav, 0x17 .qst); value; reads, through 0x9331c0 (filtro_do_jogo_salvo, filtro_do_inicio_rapido)'),
          ('general.dat +0: versão do save; valor (|v| < 0x3e027 = resumo antigo de 0xf70 bytes); lê (versao_do_jogo_salvo_valida)', 'general.dat +0: save version; value (|v| < 0x3e027 = old 0xf70-byte summary); reads (versao_do_jogo_salvo_valida)'),
          ('general.dat +4: resumo do save (0xf70 ou 0xf74 bytes; o de build_save_game_summary); bytes; lê (versao_do_jogo_salvo_valida)', 'general.dat +4: save summary (0xf70 or 0xf74 bytes; the build_save_game_summary one); bytes; reads (versao_do_jogo_salvo_valida)'),
          ('resumo do save +0x7d0: texto da versão do exe que gravou; texto; lê e compara com 0x90d130 (versao_do_jogo_salvo_valida)', 'save summary +0x7d0: version text of the exe that saved; text; reads and compares with 0x90d130 (versao_do_jogo_salvo_valida)'),
          ('[0xae24d0], [0xae28bc], [0xae26c4] textos da caixa de erro; texto; grava (versao_do_jogo_salvo_valida, nos erros)', '[0xae24d0], [0xae28bc], [0xae26c4] error box texts; text; writes (versao_do_jogo_salvo_valida, on errors)')]
INTERESSA = ['SGE']
