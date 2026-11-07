// ---------- 262a leva: os veículos da imprensa: o comentarista sorteado, o nome do veículo e a notícia da queixa do técnico à federação (printouts.cpp, media.cpp) ----------


unsafe fn cpz10_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz10_sw(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpz10_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpz10_u(l + 2));
}

unsafe fn cpz10_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

/// { 0x7dab90 } (thiscall, ret 0xc, printouts.cpp, quadro de 0x1094 bytes pela 0x945610): `PunditList::
/// pick_random_pundit(clube, tipo, nação)`, pelas histórias da imprensa (0x72c1f0, 0x733a60) e pelos textos das
/// notícias. Tipo 1 no domingo (0x54a3f0 = 6) vira 7. Tipos 2 e 4 (locais): os da lista [this+4] cujo +0xcb é o id de
/// [[clube+0x69]+0x38] (sem eles: −1); os outros: os da lista [this] do tipo (+0xca) e da nação (+0xcf; −1 aceita
/// qualquer). Até 20; nenhum: −1; senão o id (palavra +0, com sinal) de um deles por sorteio(n).
unsafe extern "thiscall" fn sortear_comentarista(this: usize, clube: usize, tipo: u32, nacao: i32) -> i32 {
    let mut t = tipo as i8;
    if t == 1 {
        let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
        if semana(0xae2c90) as i16 == 6 {
            t = 7;
        }
    }
    let mut ids = [0i16; 20];
    let mut n = 0usize;
    if t == 2 || t == 4 {
        let est = read_usize(clube + 0x69);
        if est == 0 || read_usize(est + 0x38) == 0 {
            return -1;
        }
        let l = read_usize(this + 4);
        cpz10_inicio(l);
        if cpz10_sw(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = cpz10_proximo(l);
            if p == 0 {
                break;
            }
            if cpz10_u(p + 0xcb) == cpz10_u(read_usize(read_usize(clube + 0x69) + 0x38)) { // cpz10: o comentarista local
                crate::comparacao::ramo("cpz_10 comentarista local");
                ids[n] = cpz10_sw(p) as i16;
                n += 1;
                if n >= 20 {
                    break;
                }
            }
            i += 1;
            if i >= cpz10_sw(read_usize(this + 4)) {
                break;
            }
        }
    } else {
        let l = read_usize(this);
        cpz10_inicio(l);
        if cpz10_sw(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = cpz10_proximo(l);
            if p == 0 {
                break;
            }
            if *((p + 0xca) as *const i8) == t {
                let aceita = if nacao == -1 {
                    crate::comparacao::ramo("cpz_10 comentarista de qualquer nacao");
                    true // cpz10: qualquer nação
                } else {
                    cpz10_u(p + 0xcf) as i32 == nacao
                };
                if aceita {
                    ids[n] = cpz10_sw(p) as i16;
                    n += 1;
                    if n >= 20 {
                        break;
                    }
                }
            }
            i += 1;
            if i >= cpz10_sw(read_usize(this)) {
                break;
            }
        }
    }
    if n == 0 {
        return -1;
    }
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    crate::comparacao::ramo("cpz_10 sorteio do comentarista");
    let r = sortear(n as i32); // cpz10: o sorteio do comentarista
    ids[r as usize] as i32
}

/// O comparador: lado a lado nos cabeçalhos das duas listas (o cursor) e no sorteio, e a devolução.
unsafe extern "thiscall" fn comparar_sortear_comentarista(this: usize, clube: usize, tipo: u32, nacao: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32>(1407);
    let mut regioes = Vec::new();
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, clube, tipo, nacao),
        || crate::aleatorio::sem_contar(|| sortear_comentarista(this, clube, tipo, nacao)),
    );
    PLACAR.registrar(igual && r == n, || format!("sortear_comentarista(clube {:#x}, tipo {}, nacao {}) original={} {:x?} rust={} {:x?}", clube, tipo as i8, nacao, r, mo, n, mr));
    r
}

/// O nome padrão do veículo pelo tipo (as três tabelas da original são iguais): 2 jornal local, 3 rádio, 4 rádio
/// local, 5 revista; os outros, jornal nacional.
fn cpz10_veiculo_padrao(tipo: i8) -> usize {
    match tipo {
        2 => 0xa24284,
        3 => 0xa794cc,
        4 => 0xa241f4,
        5 => 0xa794b8,
        _ => 0xa242cc,
    }
}

/// { 0x7dad20 } (thiscall, ret 0x10, printouts.cpp): `PunditList::get_pundit_name(tipo, id, nome, marca)`, pelos textos
/// das notícias. Sem `nome`: marca 0. Id −1: o nome padrão do tipo e marca 0. Senão o comentarista do id (palavra +0)
/// na lista [this+4] (tipos 2 e 4) ou [this]: o nome (+2, sprintf "%s" 0x944c9f) e a marca (+0xd3); sem ele, o nome
/// padrão e marca 0.
unsafe extern "thiscall" fn nome_do_veiculo(this: usize, tipo: u32, id: u32, nome: usize, marca: *mut u8) {
    if nome == 0 {
        *marca = 0;
        return;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x944c9fusize);
    let t = tipo as i8;
    let id = id as i16;
    if id != -1 {
        let l = read_usize(this + if t == 2 || t == 4 { 4 } else { 0 });
        cpz10_inicio(l);
        if cpz10_sw(l) > 0 {
            let mut i = 0;
            loop {
                let p = cpz10_proximo(l);
                if p == 0 {
                    break;
                }
                if cpz10_sw(p) as i16 == id {
                    sprintf(nome, 0x991944, p + 2);
                    *marca = *((p + 0xd3) as *const u8);
                    return;
                }
                i += 1;
                if i >= cpz10_sw(l) {
                    break;
                }
            }
        }
    }
    sprintf(nome, 0x991944, cpz10_veiculo_padrao(t));
    *marca = 0;
}

/// O comparador: lado a lado no nome (até 0xd4 bytes: o nome do comentarista tem 0xd1 e o padrão menos), na marca e
/// nos cabeçalhos das listas (o cursor).
unsafe extern "thiscall" fn comparar_nome_do_veiculo(this: usize, tipo: u32, id: u32, nome: usize, marca: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8)>(1408);
    let mut regioes = vec![(marca as usize, 1)];
    if nome != 0 {
        regioes.push((nome, 0xd4));
    }
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tipo, id, nome, marca), || nome_do_veiculo(this, tipo, id, nome, marca));
    PLACAR.registrar(igual, || format!("nome_do_veiculo(tipo {}, id {}) original={:x?} rust={:x?}", tipo as i8, id as i16, mo, mr));
}

/// { 0x758de0 } (thiscall, ret 0x18, media.cpp, SEH da notícia local, sem destrutor): `Media::
/// queue_manager_fa_complaint_story(clube, pessoa, tipo, a, b, c)`, pela disciplina (0x55e300) e pelo MANAGER_MANAGER
/// (0x6a9e60, 0x6ab800). Clube com nação (+0x53) e técnico (+0xcf), e pessoa: a notícia 0x178e (o clube; o técnico, id
/// e chaves, ou −1; o tipo; a pessoa, id e chaves; a; b; a nação ou −1; c) aos técnicos da nação (0x7863f0) se a
/// reputação (+0x80) passar de 4750, senão ao clube (0x7861e0).
unsafe extern "thiscall" fn noticia_queixa_a_federacao(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    let _ = this;
    if clube == 0 || read_usize(clube + 0x53) == 0 || read_usize(clube + 0xcf) == 0 || pessoa == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_10 noticia da queixa a federacao");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178e, 0);
    gravar(nt, 0, cpz10_u(clube));
    let tecnico = read_usize(clube + 0xcf);
    if tecnico != 0 {
        gravar(nt, 1, cpz10_u(tecnico));
        gravar(nt, 2, cpz10_u(read_usize(tecnico + 4) + 0x33));
        gravar(nt, 3, cpz10_u(read_usize(tecnico + 8) + 0x33));
        gravar(nt, 4, cpz10_u(read_usize(tecnico + 0xc) + 0x33));
    } else {
        for k in 1..5 {
            gravar(nt, k, u32::MAX);
        }
    }
    gravar(nt, 5, tipo as i8 as u32);
    gravar(nt, 6, cpz10_u(pessoa));
    gravar(nt, 7, cpz10_u(read_usize(pessoa + 4) + 0x33));
    gravar(nt, 8, cpz10_u(read_usize(pessoa + 8) + 0x33));
    gravar(nt, 9, cpz10_u(read_usize(pessoa + 0xc) + 0x33));
    gravar(nt, 0xa, a as i8 as u32);
    gravar(nt, 0xb, b as i16 as u32);
    let nacao = read_usize(clube + 0x53);
    gravar(nt, 0xc, if nacao != 0 { cpz10_u(nacao) } else { u32::MAX });
    gravar(nt, 0xd, c as i8 as u32);
    let nacao = read_usize(clube + 0x53);
    if nacao != 0 && cpz10_sw(clube + 0x80) > 0x128e {
        let aos_da_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        aos_da_nacao(read_usize(0xb7431c), nt, nacao);
    } else {
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        entregar(read_usize(0xb7431c), nt, clube);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo. Sem notícia as duas não
// fazem nada.
unsafe extern "thiscall" fn comparar_noticia_queixa_a_federacao(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(1409)(this, clube, pessoa, tipo, a, b, c)
}

