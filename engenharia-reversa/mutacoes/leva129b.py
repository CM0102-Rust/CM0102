# Mutacoes de reforco da leva129 (as duas que escaparam na leva129.py eram fracas ou fora do alcance).
# Uso: python mutacoes.py mutacoes\leva129b.py --jogo 2001-10-01
MUTACOES = [
    ('j_falta_forte2', "let falta_forte = forca as i32 > sortear(0x96) + 0xc8;", "let falta_forte = forca as i32 > sortear(0x96) + 0x64;", "play falta punida"),
    ('j_estilo2', "narrar7(chamar, 0x1f66, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 5);", "narrar7(chamar, 0x1f66, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 6);", "play venceu"),
]
