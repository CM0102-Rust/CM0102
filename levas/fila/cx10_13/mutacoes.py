# Mutacoes (cx10_13) pela prova do save (mutacoes.py --prova-save N; CARGA), com um save que tenha a Irlanda e a
# Holanda. Nenhuma abre caixa.
# - a Irlanda grava 9 vagas: o .tmp da liga no .qst muda (e a carga seguinte le fora do lugar);
# - a Holanda carregada le a marca do primeiro periodo por cima da segunda: a liga carregada muda (o comparador acusa).
MUTACOES = [
    ('cx10_13_irlanda_grava_nove',
     '    exe::crt_fwrite(this + 0xee, 5, 10, file) as u32 // the fixed draw slot table',
     '    exe::crt_fwrite(this + 0xee, 5, 9, file) as u32 // mutation'),
    ('cx10_13_holanda_le_por_cima',
     '        exe::crt_fread(this + 0xee + k, 1, 1, file); // the period winner flag of period k',
     '        exe::crt_fread(this + 0xee + k / 2, 1, 1, file); // mutation'),
]
