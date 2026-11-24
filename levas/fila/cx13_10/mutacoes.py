# Mutacoes (cx13_10) pela prova do save (mutacoes.py --prova-save N; CARGA), com um save que tenha a Espanha e a
# Russia. Nenhuma abre caixa.
# - a Russia grava o byte seguinte: o .qst muda;
# - a Espanha nao grava a divisao inferior: o .qst muda (e a carga seguinte le fora do lugar);
# (A carga sem a divisao inferior derrubaria o jogo na virada: fica sem mutacao; o comparador confere o plano.)
MUTACOES = [
    ('cx13_10_russia_grava_o_seguinte',
     '    exe::crt_fwrite(this + 0xee, 1, 1, file) // cx13_10: the play-off pair count',
     '    exe::crt_fwrite(this + 0xef, 1, 1, file) // mutation'),
    ('cx13_10_espanha_sem_gravar_a_inferior',
     '    save(lower, 1) // cx13_10: the lower division\'s temporary file',
     '    { let _ = save; 1 } // mutation'),
]
