# Terceiro reforco da leva129: a falta forte sempre ligada (a registrada vira punida e a punicao muda).
# Uso: python mutacoes.py mutacoes\leva129c.py --jogo 2001-10-01
MUTACOES = [
    ('j_falta_forte3', "let falta_forte = forca as i32 > sortear(0x96) + 0xc8;", "let falta_forte = forca as i32 > sortear(0x96) + 0xc8 || true;", "play falta punida"),
]
