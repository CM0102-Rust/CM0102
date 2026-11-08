use super::*;






/// type 1 on Sunday becomes 7; local types (2, 4): the club's pundits; the others: of the type and nation (−1 any); up
/// to 20, draw(n) and the id, or −1
/// Original: `PunditList::pick_random_pundit` (0x7dab90, printouts.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7dab90 } (thiscall, ret 0xc, printouts.cpp, quadro de 0x1094 bytes pela 0x945610): `PunditList::
/// pick_random_pundit(clube, tipo, nação)`, pelas histórias da imprensa (0x72c1f0, 0x733a60) e pelos textos das
/// notícias. Tipo 1 no domingo (0x54a3f0 = 6) vira 7. Tipos 2 e 4 (locais): os da lista [this+4] cujo +0xcb é o id de
/// [[clube+0x69]+0x38] (sem eles: −1); os outros: os da lista [this] do tipo (+0xca) e da nação (+0xcf; −1 aceita
/// qualquer). Até 20; nenhum: −1; senão o id (palavra +0, com sinal) de um deles por sorteio(n).
pub(crate) unsafe extern "thiscall" fn pick_random_pundit(this: usize, clube: usize, kind: u32, nacao: i32) -> i32 {
    let mut t = kind as i8;
    if t == 1 {
        let week: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
        if week(G_CURRENT_DATE) as i16 == 6 {
            t = 7;
        }
    }
    let mut ids = [0i16; 20];
    let mut n = 0usize;
    if t == 2 || t == 4 {
        let est = read_usize(clube + club::STADIUM);
        if est == 0 || read_usize(est + 0x38) == 0 {
            return -1;
        }
        let l = read_usize(this + 4);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if read_u32(p + 0xcb) == read_u32(read_usize(read_usize(clube + club::STADIUM) + 0x38)) { // cpz10: o comentarista local
                crate::comparacao::ramo("cpz_10 comentarista local");
                ids[n] = read_i16_as_i32(p) as i16;
                n += 1;
                if n >= 20 {
                    break;
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this + 4)) {
                break;
            }
        }
    } else {
        let l = read_usize(this);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if *((p + 0xca) as *const i8) == t {
                let accepted = if nacao == -1 {
                    crate::comparacao::ramo("cpz_10 comentarista de qualquer nacao");
                    true // cpz10: qualquer nação
                } else {
                    read_u32(p + 0xcf) as i32 == nacao
                };
                if accepted {
                    ids[n] = read_i16_as_i32(p) as i16;
                    n += 1;
                    if n >= 20 {
                        break;
                    }
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this)) {
                break;
            }
        }
    }
    if n == 0 {
        return -1;
    }
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    crate::comparacao::ramo("cpz_10 sorteio do comentarista");
    let r = sortear(n as i32); // cpz10: o sorteio do comentarista
    ids[r as usize] as i32
}


/// Default media outlet name by type: 2 local paper, 3 radio, 4 local radio, 5 magazine, else national paper.
///
/// [pt] O nome padrão do veículo pelo tipo (as três tabelas da original são iguais): 2 jornal local, 3 rádio, 4 rádio
/// local, 5 revista; os outros, jornal nacional.
pub(crate) fn default_media_name(kind: i8) -> usize {
    match kind {
        2 => 0xa24284,
        3 => 0xa794cc,
        4 => 0xa241f4,
        5 => 0xa794b8,
        _ => 0xa242cc,
    }
}

/// the name (+2) and flag (+0xd3) of the pundit with the id, or the default outlet type name and flag 0
/// Original: `PunditList::get_pundit_name` (0x7dad20, printouts.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7dad20 } (thiscall, ret 0x10, printouts.cpp): `PunditList::get_pundit_name(tipo, id, nome, marca)`, pelos textos
/// das notícias. Sem `nome`: marca 0. Id −1: o nome padrão do tipo e marca 0. Senão o comentarista do id (palavra +0)
/// na lista [this+4] (tipos 2 e 4) ou [this]: o nome (+2, sprintf "%s" 0x944c9f) e a marca (+0xd3); sem ele, o nome
/// padrão e marca 0.
pub(crate) unsafe extern "thiscall" fn get_pundit_name(this: usize, kind: u32, id: u32, name: usize, mark: *mut u8) {
    if name == 0 {
        *mark = 0;
        return;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let t = kind as i8;
    let id = id as i16;
    if id != -1 {
        let l = read_usize(this + if t == 2 || t == 4 { 4 } else { 0 });
        list_rewind(l);
        if read_i16_as_i32(l) > 0 {
            let mut i = 0;
            loop {
                let p = contract_manager_list_next(l);
                if p == 0 {
                    break;
                }
                if read_i16_as_i32(p) as i16 == id {
                    sprintf(name, 0x991944, p + 2);
                    *mark = *((p + 0xd3) as *const u8);
                    return;
                }
                i += 1;
                if i >= read_i16_as_i32(l) {
                    break;
                }
            }
        }
    }
    sprintf(name, 0x991944, default_media_name(t));
    *mark = 0;
}



// simplification: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo. Sem notícia as duas não
// fazem nada.

