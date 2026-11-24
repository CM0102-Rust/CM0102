# Item de TELAS (cx07_12): sem mutacoes na cadeia; a prova e o Lucas jogando comparando. Para provar os comparadores
# lado a lado numa sessao dele:
# - o 565 com o verde de 5 bits (acusada pelo comparador de graphics_rgb_to_pixel em toda cor);
# - a escala sem o teto de 255 (acusada ao clarear uma cor viva);
# - o espaco entre letras sem a meia altura (acusada em todo texto de fonte de mapa de bits).
MUTACOES = [
    ('cx07_12_verde_de_5_bits',
     '        ((r & 0xf8) << 8) | ((g & 0xfc) << 3) | (b >> 3)',
     '        ((r & 0xf8) << 8) | ((g & 0xf8) << 3) | (b >> 3) // mutation'),
    ('cx07_12_escala_sem_teto',
     '    let scale = |c: u8| ((c as i32 * (percent & 0xff) as i32) / 100).min(0xff) as u8;',
     '    let scale = |c: u8| ((c as i32 * (percent & 0xff) as i32) / 100) as u8; // mutation'),
    ('cx07_12_espaco_sem_tres_quartos',
     '    let gap = read_i32(base + 0x284).wrapping_mul(3) / 4',
     '    let gap = read_i32(base + 0x284).wrapping_mul(3) / 2 // mutation'),
]
