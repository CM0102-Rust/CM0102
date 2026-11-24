# Mutacoes (cx02_17) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa nem para o jogo (a
# leitura e a gravacao seguem alinhadas: so muda o banco da fase).
# - a fase grava o numero de reservas no lugar das substituicoes: o .tmp da copa muda (o .qst) e a fase carregada tem
#   as substituicoes trocadas;
# - a fase carregada le as substituicoes por cima das reservas: a fase carregada muda (a impressao da carga e o
#   comparador).
MUTACOES = [
    ('cx02_17_grava_reservas_trocadas',
     '    for (field, size, _) in CUP_STAGE_FILE_FIELDS {\n        exe::crt_fwrite(this + field, size, 1, file);',
     '    for (field, size, _) in CUP_STAGE_FILE_FIELDS {\n        let field = if field == cup::SUBSTITUTIONS_ALLOWED { cup::SUBSTITUTES_NAMED } else { field }; // mutation\n        exe::crt_fwrite(this + field, size, 1, file);'),
    ('cx02_17_le_substituicoes_nas_reservas',
     '    for (field, size, line) in CUP_STAGE_FILE_FIELDS {\n        cup_stage_read(this + field, size, 1, file, 1, line);',
     '    for (field, size, line) in CUP_STAGE_FILE_FIELDS {\n        let field = if field == cup::SUBSTITUTIONS_ALLOWED { cup::SUBSTITUTES_NAMED } else { field }; // mutation\n        cup_stage_read(this + field, size, 1, file, 1, line);'),
]
