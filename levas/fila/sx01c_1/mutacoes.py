# Mutacoes (sx01c_1). A rotina roda em toda resposta a uma historia de time (0x73e870, 44 chamadas na carreira ENG ate
# 2002-06-01, marcas 4 e 8 com a mudanca da confianca) e nas respostas as historias de jogador que chegam aos efeitos
# (apply_player_story_response, marcas 0x500/0x404/0x480/0x100/0x80):
# - o sorteio de quem do elenco reage: felicidade + 50 em vez de + 150 (muda quem reage e os sorteios seguintes);
# - a marca 4 (historias de time) com sorteio(4) + sorteio(4) em vez da diferenca (humor do elenco);
# - a confianca da comissao subtrai a mudanca (acusada pelo comparador da rotina, que confere a +0xa deixada pela
#   original contra este pedaco, sempre que a mudanca nao e zero);
# - a confianca da torcida e da diretoria (marca 0x400) com x 1000 em vez de x 225.
MUTACOES = [
    ('sx01c_quem_reage',
     'exe::utils_random(mc.happiness() as i32 + 150) < exe::utils_random(100)',
     'exe::utils_random(mc.happiness() as i32 + 50) < exe::utils_random(100) /* mutation: happiness + 50 */',
     'sx01c_1 story effects'),
    ('sx01c_marca_4',
     'exe::staff_adjust_morale(m, (a as u8).wrapping_sub(b as u8) as u32, 1);',
     'exe::staff_adjust_morale(m, (a as u8).wrapping_add(b as u8) as u32, 1); // mutation: the sum',
     'sx01c_1 story effects'),
    ('sx01c_confianca_da_comissao',
     'read_i16(manager_manager_row(club) + board_record::STAFF_CONFIDENCE).wrapping_add(change).clamp(1, 10000)',
     'read_i16(manager_manager_row(club) + board_record::STAFF_CONFIDENCE).wrapping_sub(change).clamp(1, 10000) /* mutation */',
     'sx01c_1 story effects'),
    ('sx01c_confianca_x225',
     'ftol((s.form as f64 - s.expected as f64) * read_f32(rdata::F32_225) as f64)',
     'ftol((s.form as f64 - s.expected as f64) * read_f64(rdata::F64_1000)) /* mutation: x 1000 */',
     'sx01c_1 story effects confidence'),
]
