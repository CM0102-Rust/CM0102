// ---------- %%LEVA%%a leva: a Copa do Mundo: a tabela do grupo 1, as fases dos grupos 2 a 8 e as posições previstas (world_cup.cpp) ----------

unsafe fn cqf5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// O clube da entrada `i` da lista do sorteio (5 bytes: +0 o id): 0xae23b4 + id × 0x245.
unsafe fn cqf5_clube(lista: usize, i: usize) -> usize {
    cqf5_d(0xae23b4).wrapping_add(cqf5_d(lista + i * 5).wrapping_mul(0x245))
}

/// { 0x92d0f0 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::create_group_table(lista)`, do sorteio
/// (0x92e7f0). A própria Copa vira o grupo 1: +0x3e (times) = 4, a tabela [+0xb1] alocada com 0xec bytes (0x944e46 push;
/// add esp -> cdecl; sem memória, a caixa da linha 0x2b1 e devolve 0); as entradas da lista do grupo 1 (+4 = 1), na
/// ordem, entram nas linhas 0..3 (0x687430 push 0; push 0; push clube; push linha; mov ecx, this -> thiscall, ret 0x10;
/// a linha vai no cl e a original deixa lixo nos bytes altos do ecx, que a 0x687430 não lê: a cópia passa o byte
/// limpo), até 4 linhas ou o fim das 32 entradas; depois a notícia dos grupos (0x684230 mov ecx, this -> thiscall).
/// Devolve 1.
unsafe extern "thiscall" fn mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha_da_tabela: unsafe extern "thiscall" fn(usize, u32, usize, u32, usize) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    cqf5_pw(this + 0x3e, 4);
    let t = alocar(0xec);
    cqf5_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xad5888, 0x2b1);
        return 0;
    }
    let mut linha = 0i8;
    let mut i = 0i8;
    while linha < 4 {
        let e = i as usize; // cqf5: a entrada da lista
        if cqf5_b(lista + e * 5 + 4) == 1 {
            crate::comparacao::ramo("cqf_5 time do grupo um");
            let clube = cqf5_clube(lista, e);
            linha_da_tabela(this, linha as u8 as u32, clube, 0, 0);
            linha += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    noticia(this);
    1
}

/// Os clubes do grupo `g` (+4 = g) da lista, na ordem, até 4 (o que a 0x92d0f0 e a 0x92d540 põem nas linhas).
unsafe fn cqf5_do_grupo(lista: usize, g: i32) -> Vec<usize> {
    (0..32usize).filter(|&i| cqf5_b(lista + i * 5 + 4) as i8 as i32 == g).map(|i| cqf5_clube(lista, i)).take(4).collect()
}

/// Só a original (aloca a tabela e manda a notícia dos grupos), conferindo o plano feito antes: devolve 1, +0x3e = 4 e
/// as linhas da tabela (0x3b bytes, +0 o clube) com os clubes do grupo 1 da lista, na ordem.
unsafe extern "thiscall" fn comparar_mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let plano = cqf5_do_grupo(lista, 1);
    let r = o(this, lista);
    if r == 1 {
        let t = cqf5_d(this + 0xb1);
        let ok = cqf5_w(this + 0x3e) == 4 && plano.iter().enumerate().all(|(k, &c)| cqf5_d(t + k * 0x3b) == c);
        PLACAR.registrar(ok, || format!("mundial_tabela_do_grupo_um: as linhas não têm os clubes do grupo 1 ({:x?})", plano));
    }
    r
}

/// { 0x92d540 } (thiscall, ret 8, world_cup.cpp, quadro SEH só para o new: sem destrutor no caminho normal):
/// `WorldCup::create_group_stage(g, lista)`, do sorteio (0x92e7f0, g de 0 a 6: os grupos 2 a 8). Os times do grupo g+2
/// (até 4, na ordem da lista) num vetor de +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x366 e 0); a tabela
/// das rodadas pelo método +0x3c da Copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall, ret 0x10: a
/// 0x92c290 grava as duas saídas para as fases 0..6); a fase (0x944cf1(0xee) push; add esp -> cdecl; o construtor
/// League_Stage 0x68b100, thiscall, ret 0x58: 22 argumentos, conferidos na leitura dele: a Copa, os times (word),
/// o vetor, 1, +4, a tabela das rodadas, as rodadas (word: +0xa9 da fase, a conta das datas), +0xc2, +0xc3, +0xc4,
/// &+0xc5, &+0xbe, o ano +0x40 (word), g (byte), o id (word: +0x3a da fase), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x380 e 0. Com ela: +0x30 = g e a notícia do grupo
/// (0x684230 na fase). Devolve 1.
unsafe extern "thiscall" fn mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((cqf5_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0xad5888, 0x366);
        return 0;
    }
    let mut n = 0i16;
    let mut i = 0i16;
    while n < 4 {
        let e = lista + i as usize * 5;
        if cqf5_b(e + 4) as i8 as i32 == g + 2 {
            crate::comparacao::ramo("cqf_5 time do grupo");
            cqf5_pd(times + n as usize * 4, cqf5_clube(lista, i as usize)); // cqf5: o time na vaga do grupo
            n += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqf5_d(cqf5_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            cqf5_w(this + 0x3e) as u16 as u32,
            times,
            1,
            cqf5_d(this + 4),
            tabela,
            rodadas as u16 as u32, // cqf5: as rodadas (as datas, +0xa9 da fase)
            cqf5_b(this + 0xc2) as u32,
            cqf5_b(this + 0xc3) as u32,
            cqf5_b(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            cqf5_w(this + 0x40) as u16 as u32,
            g as u8 as u32,
            id as u16 as u32,
            cqf5_b(this + 0x51) as u32,
            4,
            0,
            cqf5_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqf5_pd(cqf5_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    if cqf5_d(cqf5_d(this + 0xc) + g as usize * 4) == 0 {
        aviso_em(0xad5888, 0x380);
        return 0;
    }
    cqf5_pd(this + 0x30, g as usize);
    noticia(cqf5_d(cqf5_d(this + 0xc) + g as usize * 4));
    1
}

/// Só a original (aloca a fase e manda a notícia), conferindo o plano: devolve 1 com a fase em [+0xc][g], +0x30 = g, a
/// fase com os times (+0x3e) da Copa, o ano dela e, na tabela da fase (+0xb1, linhas de 0x3b bytes), os clubes do
/// grupo g+2 da lista (em qualquer ordem: na comparação de 02/11 as linhas da fase não
/// saíram na ordem da lista [inferência: a fase ordena ou sorteia a tabela dela]).
unsafe extern "thiscall" fn comparar_mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I1%%);
    let plano = cqf5_do_grupo(lista, g + 2);
    let r = o(this, g, lista);
    if r == 1 {
        let f = cqf5_d(cqf5_d(this + 0xc) + g as usize * 4);
        let t = cqf5_d(f + 0xb1);
        // as rodadas (3) na conta das datas +0xa9 e o id (0x420 + g) em +0x3a: a ordem dos argumentos do construtor
        let ok = f != 0 && cqf5_d(this + 0x30) as i32 == g && cqf5_w(f + 0x3e) == cqf5_w(this + 0x3e) && cqf5_w(f + 0x40) == cqf5_w(this + 0x40)
            && cqf5_w(f + 0xa9) == 3 && cqf5_w(f + 0x3a) as i32 == 0x420 + g
            && (t == 0 || {
                // as linhas da fase não seguem a ordem da lista: confere o conjunto dos clubes
                let mut a: Vec<usize> = (0..plano.len()).map(|k| cqf5_d(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
        PLACAR.registrar(ok, || format!("mundial_fase_do_grupo({}): a fase não bate com o plano ({:x?})", g, plano));
    }
    r
}

/// { 0x92f680 } (thiscall, ret 0, world_cup.cpp, método +0x5c da Copa do Mundo): `WorldCup::set_predicted_positions()`,
/// do sorteio (0x92e7f0). Sem estatística (+8): a caixa da linha 0x671 e volta. Menos de 32 nações (+0x36): a caixa da
/// linha 0x678 e +0x36 = 32. Os clubes da tabela das nações ([+0x14], 6 bytes) num vetor (0x944e46; sem memória, a
/// caixa da linha 0x680 e volta), ordenado pela reputação (qsort 0x945333 com 0x4b07d0: push; add esp, 0x10 -> cdecl),
/// e a posição prevista de cada um na estatística (0x4a2540 push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 4, quatro 5, oito 9, oito 17 e oito 25 (as faixas das fases finais); depois a competição atual
/// da nação de cada clube (+0x53) vira a Copa (0x775220 push [+4]; push nação; mov ecx, [0xb74318] -> thiscall, ret 8)
/// e o vetor é liberado (0x9452ca).
unsafe extern "thiscall" fn mundial_posicoes_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let competicao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x775220usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqf5_d(this + 8) == 0 {
        aviso_em(0xad5888, 0x671);
        return;
    }
    if cqf5_w(this + 0x36) < 0x20 {
        aviso_em(0xad5888, 0x678);
        cqf5_pw(this + 0x36, 0x20);
    }
    let v = alocar(((cqf5_w(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        aviso_em(0xad5888, 0x680);
        return;
    }
    let mut i = 0i16;
    while i < cqf5_w(this + 0x36) {
        cqf5_pd(v + i as usize * 4, cqf5_d(cqf5_d(this + 0x14) + i as usize * 6));
        i += 1;
    }
    ordenar(v, cqf5_w(this + 0x36) as i32 as u32, 4, 0x4b07d0);
    const FAIXAS: [u32; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    for (k, &p) in FAIXAS.iter().enumerate() {
        posicao(cqf5_d(this + 8), cqf5_d(v + k * 4), p);
    }
    let mut i = 0i16;
    while i < cqf5_w(this + 0x36) {
        crate::comparacao::ramo("cqf_5 competição da nação");
        competicao(cqf5_d(0xb74318), cqf5_d(cqf5_d(v + i as usize * 4) + 0x53), cqf5_d(this + 4));
        i += 1; // cqf5: a próxima nação
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística e avisa o gerenciador dos técnicos), conferindo o plano feito
/// antes: o vetor ordenado como ela ordena (a mesma qsort, numa cópia) e, depois, a posição prevista (+0x72 do registro
/// achado por 0x4a4850) de cada um dos 32 primeiros.
unsafe extern "thiscall" fn comparar_mundial_posicoes_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    if cqf5_d(this + 8) == 0 || cqf5_w(this + 0x36) < 0x20 || cqf5_d(this + 0x14) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let n = cqf5_w(this + 0x36) as usize;
    let mut v: Vec<usize> = (0..n).map(|i| cqf5_d(cqf5_d(this + 0x14) + i * 6)).collect();
    ordenar(v.as_mut_ptr(), n as u32, 4, 0x4b07d0);
    o(this);
    const FAIXAS: [u8; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    let errado = (0..32).find(|&k| {
        let e = if v[k] != 0 { achar(cqf5_d(this + 8), v[k]) } else { 0 };
        e == 0 || cqf5_b(e + 0x72) != FAIXAS[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("mundial_posicoes_previstas: a posição prevista da vaga {:?} não bate", errado));
}
