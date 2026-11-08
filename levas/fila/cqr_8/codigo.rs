// ---------- %%LEVA%%a leva: a Copa da Itália e a Copa da Itália da Série C: a fase de cada grupo (os times tirados da lista dos inscritos, as rodadas e a liga do grupo) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqr8_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A fase do grupo `g` de uma copa italiana (`create_group_stage(g)`, quadro SEH só para o new: sem destrutor no caminho
/// normal; o molde da Copa da Itália e da Série C, conferido instrução a instrução: só mudam o fonte e as linhas das
/// caixas). A tabela das rodadas pelo método +0x3c da copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall,
/// ret 0x10); um vetor de +0x3e ponteiros (0x944e46, cdecl; sem memória, a caixa da `linhas[0]` e 0) com os primeiros
/// +0x3e inscritos ainda na lista (+0xee, vetor de ponteiros: os não nulos, na ordem, cada um zerado ao sair); a fase
/// (0x944cf1(0xee), cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (word
/// +0x3e), o vetor, 1, +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40
/// (word), g, o id (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da `linhas[1]` e 0. Com ela: a notícia do sorteio do grupo (0x684230
/// na fase), +0x30 = g e 1.
unsafe fn ita_cup_create_group_stage_cqr8_fase_do_grupo(this: usize, g: u32, arquivo: usize, linhas: [u32; 2], ramo: &'static str) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr8_d(cqr8_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let times = alocar(((cqr8_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        show_error_at(arquivo, linhas[0]);
        return 0;
    }
    if cqr8_w(this + 0x3e) > 0 {
        let mut k = 0i32;
        let mut o = 0usize;
        loop {
            let c = cqr8_d(cqr8_d(this + 0xee) + o);
            if c != 0 {
                crate::comparison::branch(ramo);
                cqr8_pd(times + k as usize * 4, c); // cqr8: o time na vaga k do grupo
                cqr8_pd(cqr8_d(this + 0xee) + o, 0);
                k += 1;
            }
            o += 4;
            if k >= cqr8_w(this + 0x3e) as i32 {
                break;
            }
        }
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            cqr8_w(this + 0x3e) as u16 as u32,
            times,
            1,
            cqr8_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            cqr8_b(this + 0xc2) as u32,
            cqr8_b(this + 0xc3) as u32,
            cqr8_b(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            cqr8_w(this + 0x40) as u16 as u32,
            g,
            id as u16 as u32,
            cqr8_b(this + 0x51) as u32,
            1,
            0,
            cqr8_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr8_pd(cqr8_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr8_d(cqr8_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        show_error_at(arquivo, linhas[1]);
        return 0;
    }
    noticia(f);
    cqr8_pd(this + 0x30, g as usize);
    1
}

/// Só a original (aloca a fase, que aloca a tabela dela, e manda a notícia; não pode rodar duas vezes), conferindo o plano
/// feito antes: devolve 1 com a fase em [+0xc][g] e +0x30 = g; a fase com os times (+0x3e) e o ano (+0x40) da copa; os
/// primeiros +0x3e inscritos não nulos de +0xee zerados e, na tabela da fase (+0xb1, linhas de 0x3b bytes), esses clubes
/// (em qualquer ordem: a fase ordena a tabela dela [inferência, como na fase dos grupos da Copa do Mundo]).
unsafe fn compare_group_stage(nome: &str, this: usize, g: u32, original: impl FnOnce() -> u32) -> u32 {
    let n = cqr8_w(this + 0x3e).max(0) as usize;
    let mut plano = Vec::new();
    let mut vagas = Vec::new();
    let mut o = 0usize;
    while plano.len() < n && o < 0x400 {
        let c = cqr8_d(cqr8_d(this + 0xee) + o);
        if c != 0 {
            plano.push(c);
            vagas.push(o);
        }
        o += 4;
    }
    let r = original();
    let mut ok = r == 1;
    if ok {
        let f = cqr8_d(cqr8_d(this + 0xc) + g as usize * 4);
        let t = cqr8_d(f + 0xb1);
        ok = f != 0 && cqr8_d(this + 0x30) == g as usize && cqr8_w(f + 0x3e) == cqr8_w(this + 0x3e) && cqr8_w(f + 0x40) == cqr8_w(this + 0x40)
            && vagas.iter().all(|&o| cqr8_d(cqr8_d(this + 0xee) + o) == 0)
            && (t == 0 || {
                let mut a: Vec<usize> = (0..plano.len()).map(|k| cqr8_d(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}) devolveu {}; plano {:x?}", nome, this, g, r, plano));
    r
}

/// { 0x63f640 } (thiscall, ret 4, ita_cup.cpp): `ItaCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_cup.cpp nas linhas 0x263 e 0x27c).
unsafe extern "thiscall" fn ita_cup_create_group_stage(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea23c, [0x263, 0x27c], "cqr_8 time do grupo da copa da italia")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    compare_group_stage("ita_cup_create_group_stage", this, g, || o(this, g))
}

/// { 0x63d9e0 } (thiscall, ret 4, ita_c_cup.cpp): `ItaCCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// da Série C (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_c_cup.cpp nas linhas 0x265 e 0x27e).
unsafe extern "thiscall" fn ita_cup_create_group_stage_63d9e0(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea204, [0x265, 0x27e], "cqr_8 time do grupo da copa da serie c")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_group_stage_63d9e0(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    compare_group_stage("ita_cup_create_group_stage_63d9e0", this, g, || o(this, g))
}
