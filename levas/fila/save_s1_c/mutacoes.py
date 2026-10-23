# Mutacoes do RLE, nos caminhos da gravacao de um .qst tipo 4 (prova do save). Todas mudam o conteudo
# descomprimido: so mudar a codificacao (pares mais curtos, literal virando par) decodifica igual e
# passa como IGUAL quando a prova ignora os campos que variam sozinhos.
MUTACOES = [
    ('sv_rle_teto', "                if n.wrapping_add(0x80) < 0xff {", "                if n.wrapping_add(0x80) < 0x100 {"),
    ('sv_rle_dois', "            } else if n <= 1 && c <= 0x80 {", "            } else if n <= 2 && c <= 0x80 {"),
    ('sv_rle_fecho', "                let marca = (n as u8).wrapping_add(0x80);", "                let marca = (n as u8).wrapping_add(0x81);"),
]
