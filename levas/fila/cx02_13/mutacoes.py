# Mutacoes (cx02_13) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa (a leitura segue
# alinhada: so muda onde o byte cai).
# - a FA Cup grava a escolha da quinta rodada no lugar dos isentos: o .tmp da FA Cup no .qst muda;
# - a FA Cup carregada le a escolha da quinta rodada por cima dos isentos: a copa carregada muda (a impressao da carga e
#   o comparador acusam).
MUTACOES = [
    ('cx02_13_fa_cup_grava_trocado',
     '    exe::crt_fwrite(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file);\n    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);',
     '    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file); // mutation\n    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);'),
    ('cx02_13_fa_cup_le_por_cima',
     '    exe::crt_fread(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);\n    if read_i32(G_GAME_DATA_VERSION)',
     '    exe::crt_fread(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file); // mutation\n    if read_i32(G_GAME_DATA_VERSION)'),
]
