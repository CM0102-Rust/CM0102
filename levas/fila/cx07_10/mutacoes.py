# Mutacoes (cx07_10), pela prova do save (mutacoes.py --prova-save N): o .tmp de cada liga entra no .qst; um byte fora do
# lugar muda o save gravado pela Rust (e a carga dele).
# - a capacidade e os assentos exigidos trocados de ordem no arquivo;
# - o historico de posicoes sem o ultimo jogo de cada clube.
MUTACOES = [
    ('cx07_10_capacidade_e_assentos_trocados',
     '    for (field, size) in [(league::REQUIRED_CAPACITY, 2), (league::REQUIRED_SEATS, 2), (league::PRIZE_MODE, 1), (league::PRIZE_FUND, 4),',
     '    for (field, size) in [(league::REQUIRED_SEATS, 2), (league::REQUIRED_CAPACITY, 2), (league::PRIZE_MODE, 1), (league::PRIZE_FUND, 4), // mutation'),
    ('cx07_10_historico_sem_o_ultimo_jogo',
     'temp_write(f, read_usize(row(i) + league_table_row::POSITION_HISTORY), 1, read_i16(this.addr() + league::GAMES_PER_CLUB) as i32 as u32);',
     'temp_write(f, read_usize(row(i) + league_table_row::POSITION_HISTORY), 1, read_i16(this.addr() + league::GAMES_PER_CLUB) as i32 as u32 - 1); // mutation'),
]
