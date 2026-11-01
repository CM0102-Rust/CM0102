# Mutacoes (cpy_7) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao contra
# a base (a mesma DLL sem as rotinas da leva). Sem o 4o campo (contador): a base da prova do save roda com as rotinas
# desligadas, entao os ramos nao contariam; a prova e o item do .qst. As tres ficam na 0x5af040, porque o
# club_dates.tmp sai em todo save (5.040 bytes no .qst da prova: 2.520 clubes); o check_fixture_legs.tmp nao sai no
# .qst da prova (a lista esta vazia logo depois do inicio rapido), e as mutacoes da carga so apareceriam se a
# impressao ate 2001-08-28 dependesse das datas dos clubes (so as telas do clube as leem, 0x45f4a0/0x45f540).
# - um clube a menos: o club_dates.tmp sai 2 bytes menor (sem caixa: a conferencia usa a mesma conta);
# - um byte por clube em vez de dois: o arquivo sai com a metade (2.520 bytes) e so o comeco da tabela;
# - sem gravar: o arquivo sai vazio (aberto e fechado sem fwrite).
MUTACOES = [
    ('cpy7_datas_um_clube_a_menos',
     '    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: as datas dos clubes',
     '    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes() - 1, f) != cpy7_clubes() - 1 { // mutacao: um clube a menos'),
    ('cpy7_datas_um_byte_por_clube',
     '    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: as datas dos clubes',
     '    if escrever(cpy7_d(0xae2a5c), 1, cpy7_clubes(), f) != cpy7_clubes() { // mutacao: um byte por clube'),
    ('cpy7_datas_sem_gravar',
     '    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: as datas dos clubes',
     '    if { let _ = escrever; false } { // mutacao: o arquivo sai vazio'),
]
