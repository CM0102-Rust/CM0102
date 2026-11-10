# Mutacoes de reforco da leva129 (as duas que escaparam na leva129.py eram fracas ou fora do alcance).
# Uso: python mutacoes.py mutacoes\leva129b.py --jogo 2001-10-01
MUTACOES = [
    ('j_falta_forte2', "let foul_strong = strength as i32 > random(0x96) + 0xc8;", "let foul_strong = strength as i32 > random(0x96) + 0x64;", "play foul punished"),
    ('j_estilo2', "narrar7(call, event::BLOCK_SHOT_HEADER, j.commentary_id() as u32 , (j.side() as u8) as u32 , u32::MAX, 5);", "narrar7(call, event::BLOCK_SHOT_HEADER, j.commentary_id() as u32 , (j.side() as u8) as u32 , u32::MAX, 6);", "play won"),
]
