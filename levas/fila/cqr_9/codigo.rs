// ---------- %%LEVA%%a leva: a Copa da Itália e a Copa da Itália da Série C: o mata-mata depois dos grupos (os campeões dos grupos, os melhores segundos e os cabeças de chave que entram direto) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A tabela (+0xb1, linhas de 0x3b bytes) do grupo `i` de uma copa italiana: o grupo 0 é a própria copa; o grupo i ≥ 1 é
/// a fase [+0xc][i − 1].
unsafe fn group_table(this: usize, i: usize) -> usize {
    if i == 0 {
        cqr9_d(this + 0xb1)
    } else {
        cqr9_d(cqr9_d(cqr9_d(this + 0xc) + (i - 1) * 4) + 0xb1)
    }
}

/// Os que entram direto no mata-mata: a partir da vaga `k` do vetor `times`, até `fim`, os inscritos ainda na lista (+0xee,
/// até +0xf2 vagas: os não nulos, na ordem), cada um zerado ao sair.
unsafe fn take_direct_entrants(this: usize, times: usize, mut k: usize, fim: usize) {
    let mut j = 0i32;
    if (cqr9_d(this + 0xf2) as i32) <= 0 {
        return;
    }
    loop {
        if k >= fim {
            break;
        }
        let c = cqr9_d(cqr9_d(this + 0xee) + j as usize * 4);
        if c != 0 {
            cqr9_pd(times + k * 4, c);
            cqr9_pd(cqr9_d(this + 0xee) + j as usize * 4, 0);
            k += 1;
        }
        j += 1;
        if j >= cqr9_d(this + 0xf2) as i32 {
            break;
        }
    }
}

/// O mata-mata (Cup_Stage) com os `n` times de `times`: as rodadas pelo método +0x3c da copa (push 0; push &extra; push
/// &rodadas; push fase; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2), cdecl; Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, n, o vetor, as rodadas (word), +4, a tabela das rodadas, o ano +0x40 (word), a fase, 1, o
/// extra (word), 0x14, 0, 0, 0 e `sementes`; a original empurra os words com lixo nos 16 bits altos, que o construtor não
/// lê) em [+0xc][fase]; o vetor e a tabela das rodadas liberados (0x9452ca, cdecl; nesta ordem). Sem fase, a caixa da
/// `linha`; com ela, o sorteio da primeira rodada (Cup::draw_round 0x51c410: push 0; mov ecx, fase -> thiscall, ret 4).
unsafe fn create_knockout_stage(this: usize, n: u32, times: usize, fase: u32, sementes: *const u8, arquivo: usize, linha: u32) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr9_d(cqr9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, fase, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n, times, rodadas as u16 as u32, cqr9_d(this + 4), tabela, cqr9_w(this + 0x40) as u16 as u32, fase, 1, extra as u16 as u32, 0x14, 0, 0, 0, sementes)
    } else {
        0
    };
    cqr9_pd(cqr9_d(this + 0xc) + fase as usize * 4, f);
    soltar(times);
    soltar(tabela);
    let f = cqr9_d(cqr9_d(this + 0xc) + fase as usize * 4);
    if f == 0 {
        return show_error_at(arquivo, linha);
    }
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    sortear(f, 0);
}

/// { 0x63f870 } (thiscall, ret 0, ita_cup.cpp; da fase seguinte, depois da fase 6; quadro SEH só para o new, sem
/// destrutor): `ItaCup::create_knockout_stage()`: o mata-mata da Copa da Itália com 24 times: um vetor de 24 ponteiros
/// (0x944e46 de 0x60; sem memória, a caixa da linha 0x29c) com os 8 campeões dos grupos (a primeira linha da tabela de
/// cada grupo, `group_table`) e, nas vagas 8 a 23, os 16 cabeças de chave que entram direto (`take_direct_entrants`); a fase 7
/// (`create_knockout_stage`) com as sementes {3 × 8, 2 × 8, 1 × 8} (os campeões dos grupos, depois os cabeças de chave
/// [inferência: o pote de cada vaga no sorteio]) e a caixa da linha 0x2c0.
unsafe extern "thiscall" fn ita_cup_create_knockout_stage(this: usize) {
    let mut sementes = [0u8; 24];
    for (k, s) in sementes.iter_mut().enumerate() {
        *s = 3 - (k / 8) as u8;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let times = alocar(0x60);
    if times == 0 {
        return show_error_at(0x9ea23c, 0x29c);
    }
    crate::comparison::branch("cqr_9 italy cup knockout");
    for i in 0..8usize {
        cqr9_pd(times + i * 4, cqr9_d(group_table(this, i))); // cqr9: o campeão do grupo i
    }
    take_direct_entrants(this, times, 8, 0x18);
    create_knockout_stage(this, 0x18, times, 7, sementes.as_ptr(), 0x9ea23c, 0x2c0);
}

/// O plano dos dois mata-matas (feito antes): os campeões dos `grupos` grupos e os primeiros inscritos não nulos de +0xee
/// que cabem; o comparador confere, depois da original, a fase em [+0xc][fase] e esses inscritos zerados em +0xee.
unsafe fn ita_cup_compare_knockout_stage(nome: &str, this: usize, fase: usize, cabem: usize, original: impl FnOnce()) {
    let mut vagas = Vec::new();
    let mut j = 0usize;
    while vagas.len() < cabem && (j as i32) < cqr9_d(this + 0xf2) as i32 {
        if cqr9_d(cqr9_d(this + 0xee) + j * 4) != 0 {
            vagas.push(j);
        }
        j += 1;
    }
    original();
    let f = cqr9_d(cqr9_d(this + 0xc) + fase * 4);
    let ok = f != 0 && vagas.iter().all(|&j| cqr9_d(cqr9_d(this + 0xee) + j * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}) fase {:#x}, vagas tiradas {:?}", nome, this, f, vagas));
}

unsafe extern "thiscall" fn compare_ita_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    // ponytail: aloca a fase e sorteia a primeira rodada; não pode rodar duas vezes: só a original, com o plano
    ita_cup_compare_knockout_stage("ita_cup_create_knockout_stage", this, 7, 16, || o(this))
}

/// { 0x63dc10 } (thiscall, ret 0, ita_c_cup.cpp; da fase seguinte, depois da fase 14; quadro SEH só para os dois new, sem
/// destrutor no caminho normal): `ItaCCup::create_knockout_stage()`: o mata-mata da Copa da Itália da Série C com 32
/// times. 1) A classificação dos segundos colocados: uma liga provisória (0x944cf1(0xee); League_Stage 0x68b100,
/// thiscall, ret 0x58: a copa, 16, 0, 0, +4, 0, 0, +0xc2, +0xc3, 0, &{1, 2, 3, 0} (os critérios de desempate), &+0xbe, o
/// ano +0x40 (word), −2, 0, 0, 1, 0, 2, −1, 0, 2) cuja tabela recebe, na linha i, a segunda linha (+0x3b) da tabela do
/// grupo i (os 0x3b bytes copiados) com +5 e +9 zerados, ordenada (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall,
/// ret 4). 2) Um vetor de 32 ponteiros (0x944e46 de 0x80; sem memória, a caixa da linha 0x2ba) com os 16 campeões dos
/// grupos (a primeira linha de cada tabela) e, nas vagas 16 a 21, os 6 melhores segundos (as 6 primeiras linhas da
/// classificação): cada um, achado na tabela do seu grupo (o primeiro grupo em que aparece), marca o grupo (+0xbe = 2) e a
/// linha dele (+0x37 = 6). 3) A liga provisória destruída (a virtual 0 com 1). 4) Nas vagas 22 a 31, os 10 cabeças de chave
/// que entram direto (`take_direct_entrants`). 5) A fase 15 (`create_knockout_stage`), sem sementes, e a caixa da linha 0x304.
unsafe extern "thiscall" fn ita_cup_create_knockout_stage_63dc10(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let desempate: [u8; 4] = [1, 2, 3, 0];
    let p = novo(0xee);
    let liga = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(0x68b100usize);
        construir(p, this, 0x10, 0, 0, cqr9_d(this + 4), 0, 0, *((this + 0xc2) as *const u8) as u32, *((this + 0xc3) as *const u8) as u32, 0,
            desempate.as_ptr() as usize, this + 0xbe, cqr9_w(this + 0x40) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for i in 0..16usize {
        let de = group_table(this, i) + 0x3b;
        let para = cqr9_d(liga + 0xb1) + i * 0x3b;
        core::ptr::copy_nonoverlapping(de as *const u8, para as *mut u8, 0x3b); // cqr9: o segundo do grupo i
        cqr9_pd(cqr9_d(liga + 0xb1) + i * 0x3b + 9, 0);
        cqr9_pd(cqr9_d(liga + 0xb1) + i * 0x3b + 5, 0);
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    ordenar(liga, 0); // cqr9: a classificação dos segundos
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let times = alocar(0x80);
    if times == 0 {
        return show_error_at(0x9ea204, 0x2ba);
    }
    crate::comparison::branch("cqr_9 serie c cup knockout");
    for i in 0..16usize {
        cqr9_pd(times + i * 4, cqr9_d(group_table(this, i)));
    }
    let mut l = 0usize;
    while l < 0x162 {
        let c = cqr9_d(cqr9_d(liga + 0xb1) + l);
        cqr9_pd(times + 0x40 + (l / 0x3b) * 4, c); // cqr9: os 6 melhores segundos
        let mut g = 0usize;
        while g < 0x10 {
            let grupo = if g == 0 { this } else { cqr9_d(cqr9_d(this + 0xc) + (g - 1) * 4) };
            let n = cqr9_w(grupo + 0x3e) as i32;
            let mut r = 0i32;
            while r < n {
                if cqr9_d(cqr9_d(grupo + 0xb1) + r as usize * 0x3b) == cqr9_d(cqr9_d(liga + 0xb1) + l) {
                    *((grupo + 0xbe) as *mut u8) = 2;
                    *((cqr9_d(grupo + 0xb1) + r as usize * 0x3b + 0x37) as *mut u8) = 6;
                    g = 0x10;
                    break;
                }
                r += 1;
            }
            g += 1;
        }
        l += 0x3b;
    }
    if liga != 0 {
        let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqr9_d(cqr9_d(liga)));
        destruir(liga, 1);
    }
    take_direct_entrants(this, times, 0x16, 0x20);
    create_knockout_stage(this, 0x20, times, 0xf, core::ptr::null(), 0x9ea204, 0x304);
}

unsafe extern "thiscall" fn compare_ita_cup_create_knockout_stage_63dc10(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    // ponytail: monta e destrói a liga provisória, aloca a fase e sorteia; não pode rodar duas vezes: só a original
    ita_cup_compare_knockout_stage("ita_cup_create_knockout_stage_63dc10", this, 0xf, 10, || o(this))
}
