# A gravacao dos recordes dos clubes e das estatisticas dos jogadores no save (club_records.cpp, player_stats.cpp; leitor
# rx08, rodada 6, 25/11/2026). Inteiras (0x451ff0..0x452702, ret 4; 0x7c55a0..0x7c5d73, ret 4; sem tabela de saltos).
# Chamadas pela gravacao do jogo (SaveStep 0xc e 0xa do setup.rs).
# - 0x451ff0 Club_Records::save thiscall(this, save) ret 4 -> 0/1: zopen 0x9326e0(save, "club records.dat", 1, 0x16, 4)
#   (= zip_open_item), zwrite 0x932e10 cdecl(dados, tamanho, n, z), zclose 0x932b00(&z); sem save a caixa simples 0x15fa
#   e 0; o item que nao abre e as escritas curtas param o jogo (fatal_error_at). Os vetores de recordes e a fila so com
#   contagem positiva; as entradas dos clubes sempre (com contagem 0 a escrita de 0 itens passa).
# - 0x7c55a0 PlayerStats::save_stats thiscall(this, save) ret 4 -> 0/1: zopen(save, "player stats.dat", 1, 0x16, 4);
#   sem save a caixa 0xe09 e 0; o item e as escritas ate +0x38 param o jogo; as tres escritas das linguas (capacidade,
#   contagem e CAPACIDADE registros de 6 bytes) so mostram a caixa (0xe68, 0xe6e, 0xe74), fecham o item e devolvem 0.
#   As categorias e as entradas sao gravadas pela contagem sem conferir > 0.
# Prologos `mov eax, [esp+4]; sub esp, 0x204` (10 bytes cada, sem relativo); iguais no cm0102-1080.exe.
# Defeito do original [inferencia]: as linguas vao pela capacidade, nao pela contagem: o fim nao usado do bloco (o que o
# malloc/realloc deixou) vai para o save; a carga le o mesmo, entao nao se perde nada, so bytes de lixo no .qst.
# Depende do rx08_1 (club_records_consts): integrar depois dele (INTEGRAR_ANTES.txt).
SAVE = True
ROTINAS = [('club_records_save', 0x451ff0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('player_stats_save', 0x7c55a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a gravação dos recordes dos clubes e das estatísticas dos jogadores no save', 'the club records and the player statistics written to the save',
        [('club_records_save', 'Club_Records::save', '0x451ff0', 'club_records.cpp', 'thiscall, ret 4',
          'grava no item "club records.dat" do save os recordes de sempre e os da temporada (capacidade, contagem e os registros de 0x38b bytes), as entradas de todos os clubes (0x6a bytes cada) e a fila das notícias de recorde (6 bytes cada); falha de escrita para o jogo',
          'writes into the save\'s "club records.dat" item the all-time and the season records (capacity, count and the 0x38b-byte records), every club\'s entry (0x6a bytes each) and the record news queue (6 bytes each); a failed write stops the game'),
         ('player_stats_save', 'PlayerStats::save_stats', '0x7c55a0', 'player_stats.cpp', 'thiscall, ret 4',
          'grava no item "player stats.dat" do save os registros de estatística por categoria (0x20 bytes), as entradas dos jogadores (0x27 bytes), as quatro somas das notas, a contagem de registros livres e as línguas (6 bytes cada, tantas quanto a capacidade); falha na gravação das línguas só avisa e devolve 0, as outras param o jogo',
          'writes into the save\'s "player stats.dat" item the per-category statistics records (0x20 bytes), the player entries (0x27 bytes), the four rating sums, the free record count and the languages (6 bytes each, as many as the capacity); a failed write of the languages only warns and answers 0, the others stop the game')])
CAMPOS = []
INTERESSA = ['SGE', 'CMRTE']
