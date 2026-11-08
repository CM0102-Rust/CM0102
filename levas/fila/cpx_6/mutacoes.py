# Mutacoes (cpx_6) pela prova do save (mutacoes.py --prova-save N): cada uma encurta um .tmp que o zadd_matching_files
# leva ao .qst (o fwrite devolve a quantidade de registros, entao nenhuma cai na caixa de erro). Sem o 4o campo
# (contador): a base da prova do save roda com as rotinas desligadas.
# - os 50 registros fixos da UEFA com 0xb bytes: uefa_seeding.tmp 50 bytes menor;
# - os paises da CONMEBOL com 9 bytes: conmebol_seeding.tmp 11 bytes menor e deslocado;
# - os anfitrioes com 0x21 bytes: HostCountry.tmp 57 bytes menor e deslocado.
MUTACOES = [
    ('cpx6_uefa_registro_curto', '    if crt_fwrite(this + 0x440, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos',
     '    if crt_fwrite(this + 0x440, 0xb, 0x32, f) != 0x32 { // mutacao: registro curto'),
    ('cpx6_conmebol_registro_curto', '    if crt_fwrite(this, 0xa, signed_byte_count(this + 0x6e), f) != signed_byte_count(this + 0x6e) { // cpx6: os registros dos países',
     '    if crt_fwrite(this, 9, signed_byte_count(this + 0x6e), f) != signed_byte_count(this + 0x6e) { // mutacao: registro curto'),
    ('cpx6_anfitriao_curto', '    crt_fwrite(cpx6_d(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões',
     '    crt_fwrite(cpx6_d(this), 0x21, *((this + 4) as *const i16) as i32 as u32, f); // mutacao: registro curto'),
]
