# Mutacoes pela prova do save (prova_save.py --leva N; mutacoes.py --prova-save N):
# - rx08_1_empate_troca: no empate de partidas internacionais o ultimo da tabela de pessoas vira o recordista (>= em vez
#   de >): o .qst de uma carreira nova leva outro nome no recorde de varias selecoes ("club records.dat" com outros
#   bytes).
# - rx08_1_carga_perdida: a carga responde 0, o construtor joga fora os recordes lidos e monta tudo vazio: a carga do
#   save de ferias fica sem os recordes da temporada (acusada se a impressao da carga cobrir os recordes; senao pela
#   tela Clube -> Recordes do jogo carregado).
MUTACOES = [
    ('rx08_1_empate_troca',
     'if held == -1 || (caps as i16) > held { // rx08_1: the most capped player holds the record',
     'if held == -1 || (caps as i16) >= held { // rx08_1: mutation, the last one on a tie',
     'rx08_1 new club records'),
    ('rx08_1_carga_perdida',
     "    club_records_read_array(this, z, (f::NEWS_QUEUE, f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT), c::NEWS_SIZE, [0x15b9, 0x15c1, 0x15cd, 0x15d8]);\n    exe::zclose(&mut z);\n    1\n",
     "    club_records_read_array(this, z, (f::NEWS_QUEUE, f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT), c::NEWS_SIZE, [0x15b9, 0x15c1, 0x15cd, 0x15d8]);\n    exe::zclose(&mut z);\n    0 // rx08_1 mutation\n",
     'rx08_1 load club records'),
]
