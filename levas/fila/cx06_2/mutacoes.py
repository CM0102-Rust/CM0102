# Mutacoes (cx06_2) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa (as leituras seguem
# alinhadas: so muda o valor que fica no objeto ou o que vai para o arquivo):
# - a gravacao escreve o byte +0xff (7, o indice da ultima fase de grupo) no lugar do numero de sedes (+0xf6, 1): o .tmp
#   das eliminatorias no .qst muda;
# - a carga deixa o numero de sedes com o bit 0 invertido: as eliminatorias carregadas mudam (a impressao da carga acusa).
MUTACOES = [
    ('cx06_2_grava_byte_errado',
     '    if exe::crt_fwrite(this + euro_champ_qual::GROUP_FORMAT, 1, 1, file) != 1 {\n        return fail(0x6d5);',
     '    if exe::crt_fwrite(this + 0xff, 1, 1, file) != 1 { // mutation\n        return fail(0x6d5);',
     'cx06_2 qualifiers data saved'),
    ('cx06_2_carga_sedes_trocada',
     '    if exe::crt_fread(this + 0xfb, 4, 1, file) != 1 { // the qualified flag\n        show_error_at(0x9c4674, 0x68a);',
     '    write_u8(this + euro_champ_qual::GROUP_FORMAT, read_u8(this + euro_champ_qual::GROUP_FORMAT) ^ 1); // mutation\n    if exe::crt_fread(this + 0xfb, 4, 1, file) != 1 { // the qualified flag\n        show_error_at(0x9c4674, 0x68a);',
     'cx06_2 qualifiers data loaded'),
]
