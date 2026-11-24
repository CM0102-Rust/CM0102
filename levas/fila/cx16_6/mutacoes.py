# Mutacoes (cx16_6) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa (a leitura segue
# alinhada).
# - a CONCACAF grava a contagem de isentas caribenhas no lugar da de isentas: o .tmp da secao no .qst muda;
# - os indices carregados viram o clube seguinte (as duas cargas: as listas carregadas mudam; a impressao da carga e os
#   comparadores acusam).
MUTACOES = [
    ('cx16_6_grava_isentas_trocadas',
     'for k in [0xba, 0xbb, 0xbc] {\n        exe::crt_fwrite(this + k, 1, 1, file); // cx16_6: the three exempt counts',
     'for k in [0xbb, 0xbb, 0xbc] {\n        exe::crt_fwrite(this + k, 1, 1, file); // mutation: the Caribbean count twice',
     'cx16_6 concacaf qualifying saved'),
    ('cx16_6_clube_seguinte',
     "write_usize(list + k * 4, if v != -1 { club_at(v as isize).addr() } else { 0 }); // cx16_6: the saved index back to the club",
     "write_usize(list + k * 4, if v != -1 { club_at(v as isize + 1).addr() } else { 0 }); // mutation: the next club",
     'cx16_6 concacaf qualifying loaded'),
]
