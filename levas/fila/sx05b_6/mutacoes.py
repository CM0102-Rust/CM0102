# Mutacoes pela prova do save com a carga (CARGA: o FeriasTesteSave carregado com e sem as rotinas; a impressao do
# jogo carregado tem de bater). Uma leitura a menos desalinha o resto do arquivo do save.
# - o premio de equipe sem ler a conta de reservas;
# - o premio do ano sem ler o ultimo deslocamento;
# - as notas do ano com uma entrada a menos.
MUTACOES = [
    ('sx05b_6_sem_reservas',
     '        (ta::SUBSTITUTES, 1, 0x77, 0xa8c7d8),',
     '        // mutation: the substitutes not read'),
    ('sx05b_6_sem_ultimo',
     "        || !award_read(this + ya::HISTORY_LAST, 4, file, YEAR_AWARD_CPP, 0x77, 0xad5940) {",
     "        || false { // mutation: the last offset not read"),
    ('sx05b_6_entrada_a_menos',
     '    if exe::zread(entries, yr::ENTRY_SIZE as u32, count, file) != read_u32(this + 4) {',
     '    if exe::zread(entries, yr::ENTRY_SIZE as u32, count - 1, file) != read_u32(this + 4) - 1 { // mutation'),
]
