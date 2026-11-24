# Mutacoes (cx16_11), ENG ate 2005-12-31 (a campanha europeia de 2004); os comparadores rodam so a original, entao
# sao acusadas pela carreira Rust:
# - o grupo do proprio objeto com as vagas deslocadas uma (quem joga no grupo muda);
# - a repescagem montada so na fase da repescagem, nao ao chegar a dos segundos (a ordem das fases muda).
MUTACOES = [
    ('cx16_11_grupo_deslocado',
     'exe::league_init_table_row(league, i as u8 as u32, read_usize(slots + i as isize as usize * 4), 0, 0); // cx16_11: slot i in row i',
     'exe::league_init_table_row(league, i as u8 as u32, read_usize(slots + (i as isize as usize ^ 1) * 4), 0, 0); // mutation: neighbour slots swapped',
     'cx16_11 european own group'),
    ('cx16_11_repescagem_so_na_propria_fase',
     'if next == read_u8(this + 0x100) as i8 as i32 || next == read_u8(this + 0x101) as i8 as i32 { // cx16_11: the play-off when the runners-up stage comes',
     'if next == read_u8(this + 0x101) as i8 as i32 { // mutation: only at the play-off stage',
     'cx16_11 european next stage'),
]
