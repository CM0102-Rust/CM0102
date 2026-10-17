# Mutacoes refeitas da leva148 (as quatro que escaparam eram fracas). Uso: python mutacoes.py mutacoes\leva148b.py --carreira 2002-06-01
MUTACOES = [
    ('i_ceu', "*b(0xf638) = if sortear(2) != 0 { (sortear(0x19) as u8).wrapping_add(0xf) } else { (sortear(0x1e) as u8).wrapping_add(0x32) };", "*b(0xf638) = if sortear(2) != 0 { 1 } else { 1 };"),
    ('i_pontos', "                    if p > 5 && (p as i32) > sortear(0x1e) {", "                    if p > -100 && (p as i32) > sortear(0x1e) {", "prejogo insatisfeito pela posicao"),
    ('i_contrato', "                let r = sortear(200) + 0x32;", "                let r = sortear(200);", "prejogo fim de contrato"),
    ('i_insatisfeito', "                evento_de_prejogo(m, f, clube, 0, 0, k as u8, 0x13);", "                evento_de_prejogo(m, f, clube, 0, 0, k as u8, 0x1e);", "prejogo insatisfeito"),
]
