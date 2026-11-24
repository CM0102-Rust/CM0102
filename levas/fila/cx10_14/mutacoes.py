# Mutacoes (cx10_14) pela prova do save (mutacoes.py --prova-save N), com um save que tenha a Holanda. Nenhuma abre
# caixa.
# - a Holanda grava a marca do primeiro periodo no lugar das outras: o .tmp da liga no .qst muda.
MUTACOES = [
    ('cx10_14_holanda_grava_a_primeira',
     '        r = exe::crt_fwrite(this + 0xee + k, 1, 1, file) as u32; // the period winner flag of period k',
     '        r = exe::crt_fwrite(this + 0xee, 1, 1, file) as u32; // mutation'),
]
