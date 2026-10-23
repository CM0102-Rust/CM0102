MOTOR = True
# Fora: 0x72aa70 (match_stats_static_date_atexit, 1 byte: so o ret, sem 5 bytes para o desvio).
ROTINAS = [('montar_estatisticas_da_partida', 0x729000, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('desmontar_estatisticas_da_partida', 0x7293c0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor',
        'the end of the match statistics: the constructor (with the CD check) and the destructor',
        [('montar_estatisticas_da_partida', 'Match_Stats::Match_Stats', '0x729000', 'match_stats.cpp', 'thiscall',
          'confere o CD do CM01/02 e monta as estatísticas: carrega do save ou cria o índice, os baldes e o arquivo temporário zerado',
          'checks for the CM01/02 CD and builds the statistics: loads from the save or creates the index, the buckets and the zeroed temporary file'),
         ('desmontar_estatisticas_da_partida', 'Match_Stats::~Match_Stats', '0x7293c0', 'match_stats.cpp', 'thiscall',
          'libera o índice dos registros e zera o ponteiro e a quantidade',
          'frees the record index and clears its pointer and count')])
CAMPOS = [('Match_Stats +0: índice dos registros (n × 12 bytes); ponteiro; grava (montar_estatisticas_da_partida); lê, libera e grava 0 (desmontar_estatisticas_da_partida)', 'Match_Stats +0: record index (n × 12 bytes); pointer; writes (montar_estatisticas_da_partida); reads, frees and writes 0 (desmontar_estatisticas_da_partida)'),
          ('Match_Stats +4: capacidade do índice (n); valor; grava (montar_estatisticas_da_partida); grava 0 (desmontar_estatisticas_da_partida)', 'Match_Stats +4: index capacity (n); value; writes (montar_estatisticas_da_partida); writes 0 (desmontar_estatisticas_da_partida)'),
          ('Match_Stats +8..+0x14: cursores dos quatro baldes (k × n / 4); valor; grava (montar_estatisticas_da_partida)', 'Match_Stats +8..+0x14: cursors of the four buckets (k × n / 4); value; writes (montar_estatisticas_da_partida)'),
          ('Match_Stats +0x18: sem nome (zerado depois de gravar o arquivo temporário); valor; grava 0 (montar_estatisticas_da_partida)', 'Match_Stats +0x18: unnamed (cleared after writing the temporary file); value; writes 0 (montar_estatisticas_da_partida)'),
          ('Match_Stats +0x1c: quantidade de registros (n); valor; grava (montar_estatisticas_da_partida)', 'Match_Stats +0x1c: record count (n); value; writes (montar_estatisticas_da_partida)'),
          ('match_stats_matches.tmp (CM3_TEMP): n registros de estatística de 0xd7a bytes zerados; arquivo; grava (montar_estatisticas_da_partida)', 'match_stats_matches.tmp (CM3_TEMP): n zeroed 0xd7a-byte statistics records; file; writes (montar_estatisticas_da_partida)'),
          ('[0x729089]/[0x7290cb] bytes do código da conferência do CD (nop/xor no exe de trabalho); valor; lê (montar_estatisticas_da_partida)', '[0x729089]/[0x7290cb] code bytes of the CD check (nop/xor in the working exe); value; reads (montar_estatisticas_da_partida)')]
INTERESSA = ['SGE']
