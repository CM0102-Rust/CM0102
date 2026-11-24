# Os arquivos proprios da busca de jogadores (player_search.cpp; leitor sx17, rodada 5, 24/11/2026): load_from_file
# (0x7b2bf0) e save_to_file (0x7b3660), os arquivos de busca e de lista curta que a tela grava e abre (nao o save do
# jogo). Rotinas inteiras (0x7b2bf0..0x7b365c ret 8; 0x7b3660..0x7b3d4b ret 8; o resto e enchimento).
# Quem chama: a sessao de edicao da busca (0x804d60 abre, 0x804e7c / 0x804ec5 grava). A carreira automatica nao passa.
# TELAS: a prova e o Lucas jogando comparando: na tela de busca, gravar uma busca (ou lista curta, ou a lista de um
# olheiro com clube observado) e abri-la de novo; abrir uma busca de outro jogo (outra semente: o clube observado cai
# e os resultados sao religados).
# Convencoes conferidas na desmontagem:
# - thiscall(busca, caminho, quieto) ret 8 -> 1/0; com quieto != 0 nenhuma caixa;
# - 0x945b00 fopen ("rb" 0x98f928, "wb" 0x9a7f28), 0x945bfe fread, 0x945ce6 fwrite, 0x945a09 fclose cdecl; 0x90c140
#   utils_read_file_format_marker cdecl(arquivo); 0x90c1a0 utils_append_u32_to_file cdecl(arquivo, 0xf2fffff2);
#   0x944cff _splitpath cdecl(caminho, 0, 0, +0xb, 0) (o titulo vira o nome do arquivo); 0x806ff0 / 0x8071d0
#   SearchFilters::set_nationality_nation / set_based_nation thiscall(filtros, nacao) ret 4; 0x7f8500
#   is_club_in_session_searches thiscall([0xb74448], clube) ret 4; 0x7b1520 purge_invalid_results thiscall ret 4;
#   0x5e8290 a caixa ("Error", texto, 0) sem zerar [0xb67a34] na mensagem do arquivo de Mac.
# Defeitos do original: save_to_file grava o registro do clube observado sem conferir a gravacao; load_from_file com
# leitura curta do registro do clube fica com o registro meio lido (fora da lista global) e sem caixa.
TELAS = True
ROTINAS = [('player_search_load_from_file', 0x7b2bf0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('player_search_save_to_file', 0x7b3660, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os arquivos proprios da busca de jogadores',
        'the player search own files',
        [('player_search_load_from_file', 'PLAYER_SEARCH::load_from_file', '0x7b2bf0', 'player_search.cpp', 'thiscall, ret 8',
          'abre uma busca gravada: arquivo de Mac, a caixa propria e 0; o cabecalho (numero magico, versao 1 e o tipo desta lista) substitui +5..+0x16f e a semente gravada e comparada com a do jogo; fora das buscas rapida e de comissao, capacidade, contagem e resultados; os filtros, as nacoes de nacionalidade e de onde joga pelo nome, o registro de filtro e o clube observado (abaixo de −1, ja observado por outra lista da sessao, ou de outro jogo: nenhum; com clube, um registro novo lido do arquivo); fecha, o titulo vira o nome do arquivo, regiao e competicao limpas, os invalidos tirados (religados se for de outro jogo) e as marcas postas; devolve 1. Sem caminho, arquivo que nao abre, cabecalho errado ou leitura curta: a caixa (se nao quieto), fecha e 0',
          'opens a saved search: a Mac file, its own box and 0; the header (magic, version 1 and this list type) replaces +5..+0x16f and the stored seed is compared with the game one; outside quick and staff searches, capacity, count and results; the filters, the nationality and based-in nations by name, the filter record and the watched club (below −1, already watched by another session list, or from another game: none; with a club, a new record read from the file); closes, the title becomes the file name, region and competition cleared, the invalid results dropped (relinked when from another game) and the flags set; returns 1. No path, a file that does not open, a bad header or a short read: the box (unless quiet), closes and 0'),
         ('player_search_save_to_file', 'PLAYER_SEARCH::save_to_file', '0x7b3660', 'player_search.cpp', 'thiscall, ret 8',
          'grava a busca num arquivo: o cabecalho (+5, 0x16b bytes); fora das buscas rapida e de comissao, capacidade, contagem e resultados; os filtros, as duas nacoes dos filtros pelo nome (comprimento e nome; 0 sem nacao), o registro de filtro, o clube observado e, com ele, o registro do clube; a marca de fim do Windows; fecha, o titulo vira o nome do arquivo e as marcas sao postas; devolve 1. Sem caminho, arquivo que nao abre ou gravacao curta: a caixa (se nao quieto), fecha e 0',
          'writes the search to a file: the header (+5, 0x16b bytes); outside quick and staff searches, capacity, count and results; the filters, both filter nations by name (length and name; 0 without a nation), the filter record, the watched club and, with one, the club record; the Windows end marker; closes, the title becomes the file name and the flags are set; returns 1. No path, a file that does not open or a short write: the box (unless quiet), closes and 0')])
