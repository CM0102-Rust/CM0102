# Mutacoes (sx20_9), item de SAVE: a prova do save grava o .qst com e sem a leva (o free_game_data do .qst roda o
# free_data depois; liberar memoria nao deixa rastro, sem mutacao dele; o ShortlistOldSlot so roda carregando um save
# de versao antiga, que a prova nao tem: o comparador lado a lado o confere se rodar):
# - as linhas de tatica dos clubes gravadas com 0x90 bytes (o tactics.dat muda).
MUTACOES = [
    ('sx20_9_linhas_curtas',
     '(read_usize(this + f::ROWS), TACTICS_ROW_SIZE, read_u32(this + f::ROW_COUNT), 0x9a3), // sx20_9: the club rows',
     '(read_usize(this + f::ROWS), TACTICS_ROW_SIZE - 1, read_u32(this + f::ROW_COUNT), 0x9a3), // mutation: short rows',
     'sx20_9 save tactics'),
]
