# Mutacoes (cx06_6), com as 8 ligas e ATE 2003-08-15 (a Copa das Confederacoes de 2003):
# - a vaga continental fica com o vice no lugar do campeao: outra selecao joga a copa (o comparador lado a lado acusa; a
#   carreira muda em jul/2003);
# - o ultimo campeao da copa nao entra (o vice no lugar dele): o comparador lado a lado acusa e a carreira muda;
# - a lista nao e embaralhada: o gerador sorteia menos (a carreira muda dali em diante) e o comparador acusa a lista.
MUTACOES = [
    ('cx06_6_vice_continental',
     '    let mut team = exe::comp_util_get_latest_placed_club(record, 0);',
     '    let mut team = exe::comp_util_get_latest_placed_club(record, 1); // mutation',
     'cx06_6 continental champion'),
    ('cx06_6_sem_o_campeao',
     '    let mut holders = exe::comp_util_get_latest_placed_club(comp, 0);',
     '    let mut holders = exe::comp_util_get_latest_placed_club(comp, 1); // mutation',
     'cx06_6 confederations cup teams'),
    ('cx06_6_sem_embaralhar',
     '    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), read_u16(this + 0xf2) as u32);',
     '    // mutation: no shuffle',
     'cx06_6 confederations cup predicted'),
]
