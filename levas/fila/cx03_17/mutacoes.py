# Mutacoes (cx03_17), carreira de 8 ligas. Nenhuma abre caixa. A noticia so a comparacao acusa (modo CM_TEXTO_NOTICIAS);
# os auxiliares, o comparador (os dois rodam).
# - a eliminacao nos grupos sempre com o nome curto da competicao;
# - a vaga da nacao procurada so ate n - 1;
# - o clube so conta pela tabela da copa (a da fase 0 ignorada).
MUTACOES = [
    ('cx03_17_eliminacao_sempre_nome_curto',
     '        if news != 0 {\n            european_cup_fate_text(text, club_id, 0x987ae0',
     '        if false { // mutation\n            european_cup_fate_text(text, club_id, 0x987ae0'),
    ('cx03_17_vaga_ate_n_menos_um',
     '    let count = count as u8 as i8;\n    let mut k: i8 = 0;\n    while k < count {\n        if read_u32(list + k as usize * 4) == nation {',
     '    let count = (count as u8 as i8).wrapping_sub(1); // mutation\n    let mut k: i8 = 0;\n    while k < count {\n        if read_u32(list + k as usize * 4) == nation {'),
    ('cx03_17_so_a_tabela_da_copa',
     '        if read_usize(cup.team_table() + slot) == club || read_usize(first.team_table() + slot) == club {',
     '        if read_usize(cup.team_table() + slot) == club || (false && read_usize(first.team_table() + slot) == club) { // mutation'),
]
