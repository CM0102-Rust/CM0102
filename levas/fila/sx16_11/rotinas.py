# player_regen.cpp (leitor sx16, rodada 5, 24/11/2026): o gerente das regeneracoes: construtor, destrutor e a carga do
# retire.dat. Usa game_cd_check do sx16_7 (vai depois dele).
# - 0x7a5380 PLAYER_REGEN::PLAYER_REGEN thiscall(regen, save) ret 4 -> regen: a verificacao do CD (no cm0102-1080 o jne
#   0x7a53c9 vira nop e o test 0x7a540d vira xor); malloc das quatro tabelas (0x1964, 0xbb8, 0xfa0, 0xfa0: 250
#   registros de 0x1a, 0xc, 0x10, 0x10), capacidades 250 e contagens 0; 0x7acfc0 build_nation_club_lists thiscall;
#   sem save 0x7a58f0 build_initial_retirement_list thiscall e 10 vezes 0x7a81a0 add_spare_non_player_record
#   thiscall(regen, [0xae2368] - k - 1, -1, -1) ret 0xc; com save 0x7adcd0 (pelo desvio; resposta nao usada); depois
#   os testes das quatro alocacoes (caixas fatais 0x7f, 0x83, 0x87, 0x8b com 0x5cc7d0, 0x5a8f60, exit(-1)). Todas ja
#   Rust e chamadas pelo exe. Chamado pela criacao do jogo (0x829b67) e pela carga (0x82c8ce).
# - 0x7a56f0 ~PLAYER_REGEN thiscall ret: free das quatro tabelas (zerando) e, com +0, as G_NATION_COUNT listas
#   (0x68d040 ~LLIST e 0x944c94 delete) e free(+0). Pela limpeza (0x830430).
# - 0x7adcd0 load_retire_dat thiscall(regen, save) ret 4 -> eax: zopen(save, "retire.dat", 0, 0x16, 4); por tabela a
#   contagem, capacidade = contagem + 250, 0x945667 realloc cdecl(tabela, tamanho), os registros (so com contagem);
#   caixas 0x1a9d, 0x1aa5 (sem fechar) e 0x1aac..0x1b0f (com zclose), 0.
# Defeito do original: o construtor testa as alocacoes so depois de usar as tabelas.
# Comparadores: so a original.
SAVE = True
CARGA = True
ROTINAS = [('new_player_regen', 0x7a5380, [0x81, 0xec, 0xe0, 0x04, 0x00, 0x00]),
           ('drop_player_regen', 0x7a56f0, [0x55, 0x56, 0x8b, 0xf1, 0x33, 0xed]),
           ('load_retire_dat', 0x7adcd0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o gerente das regenerações e a carga do retire.dat',
        'the regen manager and the loading of retire.dat',
        [('new_player_regen', 'PLAYER_REGEN::PLAYER_REGEN', '0x7a5380', 'player_regen.cpp', 'thiscall, ret 4',
          'pede o CD do jogo; monta as quatro tabelas vazias com espaço para 250 registros (aposentadorias pendentes, fila de regens, registros de não jogador de reserva, fichas de jogador livres) e as listas de clubes por nação; num jogo novo, a lista inicial de aposentadorias e as dez últimas linhas de não jogador como reserva; com um save, lê o retire.dat; uma tabela sem memória para o jogo',
          'asks for the game CD; builds the four empty tables with room for 250 records (pending retirements, regen queue, spare non-player records, free player data records) and the per-nation club lists; in a new game, the initial retirement list and the last ten non-player rows as spares; with a save, reads retire.dat; a table without memory stops the game'),
         ('drop_player_regen', 'PLAYER_REGEN::~PLAYER_REGEN', '0x7a56f0', 'player_regen.cpp', 'thiscall',
          'libera as quatro tabelas e as listas de clubes por nação, zerando os ponteiros',
          'frees the four tables and the per-nation club lists, zeroing the pointers'),
         ('load_retire_dat', 'PLAYER_REGEN::load_retire_dat', '0x7adcd0', 'player_regen.cpp', 'thiscall, ret 4',
          'lê o retire.dat do save: para cada uma das quatro tabelas, a contagem, a capacidade (contagem + 250, a tabela realocada) e os registros; sem save ou sem o item, a caixa de erro e 0; leitura curta ou realocação que falha, a caixa, o item fechado e 0',
          'reads retire.dat from the save: for each of the four tables, the count, the capacity (count + 250, the table reallocated) and the records; without a save or the item, the error box and 0; a short read or a failed reallocation, the box, the item closed and 0')])
