// ---------- %%LEVA%%a leva: as quartas da Mercosul (os cinco primeiros e os três melhores segundos) e as posições previstas da Libertadores e da Mercosul (conmebol_merc.cpp, conmebol_liber.cpp) ----------

unsafe fn cqv11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqv11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A linha `i` (59 bytes) da tabela [t+0xb1] (0x6826d0: push i; mov ecx, t -> thiscall, ret 4).
unsafe fn cqv11_linha(t: usize, i: i32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    f(t, i)
}

/// O grupo `g` da Mercosul: −1 é a própria copa (o grupo A), os outros as fases [+0xc][g].
unsafe fn cqv11_grupo(this: usize, g: i8) -> usize {
    if g == -1 {
        this
    } else {
        cqv11_d(cqv11_d(this + 0xc) + g as isize as usize * 4)
    }
}

/// { 0x4c53d0 } (thiscall, ret 0, conmebol_merc.cpp; quadro SEH só para os new das fases, sem destrutor no caminho
/// normal): `ConmebolMerc::create_quarter_final_stage()`, as quartas da Mercosul, no fim dos cinco grupos. Uma fase de
/// liga dos segundos (0x944cf1(0xee) e o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa,
/// 5, sem times, 0, +4, sem tabela, 0, +0xc2, +0xc3, 0, &+0xc5, &+0xbe, o ano, −2, 0, 0, 1, 0, 2, −1, 0, 2; words e
/// bytes limpos), com a linha 1 (o segundo) de cada grupo copiada (0x3b bytes) para a sua linha e o dword +9 zerado
/// (os pontos ficam, [inferência] +9 é outra conta), e a tabela ordenada (0x6827d0(fase, 0)). Um vetor de 8 clubes
/// (0x944e46(0x20); sem memória, a caixa da linha 0x248 e volta sem destruir a fase dos segundos): os +0xbe primeiros
/// do grupo A (+0xb1) e de cada um dos grupos B..E (as fases 0..3, 0x6826d0), e os três primeiros da tabela dos
/// segundos. Cada um desses três ganha uma vaga a mais no seu grupo (+0xbe do grupo + 1), a linha dele marcada (+0x37
/// = 6) e o estado (a virtual +0x40(clube, 6, grupo, 0, 0): 5 pushes; mov ecx -> thiscall). Os segundos 4 e 5 ficam
/// fora (a virtual +0x68(clube, −1, 3, 3, 3)). Quem jogou os grupos e não está entre os 8 ganha o registro de não
/// classificado (Staff_Records 0x86c1d0([0xdd7f00], clube, +4, 0x41c, 0x28): 4 pushes; mov ecx -> thiscall, ret 0x10).
/// A fase dos segundos é destruída (a virtual 0 com 1). As quartas: a tabela das rodadas da fase 4 (a virtual +0x3c),
/// a fase (0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, ret 0x3c: a copa, 8, o vetor, as rodadas, +4, a tabela,
/// o ano, 4, 2, o extra, 0x14, 0, 0, 0, 0) em [+0xc][4]; o vetor e a tabela liberados; sem fase, a caixa da linha 0x2ab;
/// com ela, o sorteio da primeira rodada (Cup::draw_round 0x51c410(fase, 0): push 0; mov ecx -> thiscall, ret 4).
unsafe extern "thiscall" fn cqv11_mercosul_quartas(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let nao_classificado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
    #[allow(clippy::type_complexity)]
    let liga: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    #[allow(clippy::type_complexity)]
    let copa: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x522e00usize);
    let b = |d: usize| *((this + d) as *const u8) as u32;
    let p = novo(0xee);
    let segundos = if p != 0 {
        liga(p, this, 5, 0, 0, cqv11_d(this + 4), 0, 0, b(0xc2), b(0xc3), 0, this + 0xc5, this + 0xbe, cqv11_w(this + 0x40) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for g in 0..5usize {
        let origem = cqv11_d(cqv11_grupo(this, g as i8 - 1) + 0xb1) + 0x3b; // cqv11: a linha do segundo do grupo
        let destino = cqv11_d(segundos + 0xb1) + g * 0x3b;
        core::ptr::copy(origem as *const u8, destino as *mut u8, 0x3b);
        cqv11_pd(cqv11_d(segundos + 0xb1) + g * 0x3b + 9, 0);
    }
    ordenar(segundos, 0);
    let v = alocar(0x20);
    if v == 0 {
        return aviso_em(0x9a7e5c, 0x248);
    }
    let mut k = 0usize;
    for i in 0..(*((this + 0xbe) as *const i8)).max(0) as usize {
        cqv11_pd(v + k * 4, cqv11_d(cqv11_d(this + 0xb1) + i * 0x3b));
        k += 1;
    }
    for g in 0..4usize {
        let fase = cqv11_d(cqv11_d(this + 0xc) + g * 4);
        for i in 0..(*((this + 0xbe) as *const i8)).max(0) as i32 {
            cqv11_pd(v + k * 4, cqv11_d(cqv11_linha(fase, i)));
            k += 1;
        }
    }
    for s in 0..3i32 {
        crate::comparacao::ramo("cqv_11 melhores segundos");
        cqv11_pd(v + (k + s as usize) * 4, cqv11_d(cqv11_linha(segundos, s))); // cqv11: o segundo s das quartas
        for g in -1i8..4 {
            let grupo = cqv11_grupo(this, g);
            let clube = cqv11_d(cqv11_linha(segundos, s));
            if cqv11_d(cqv11_linha(grupo, 1)) == clube {
                let vagas = (grupo + 0xbe) as *mut u8;
                *vagas = (*vagas).wrapping_add(1);
                *((cqv11_linha(grupo, 1) + 0x37) as *mut u8) = 6;
                let estado: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cqv11_d(cqv11_d(this) + 0x40));
                // ponytail: a original empurra o dword cujo byte baixo é o grupo (os altos são os da conta dos 8, < 256: 0)
                estado(this, cqv11_d(cqv11_linha(segundos, s)), 6, g as u8 as u32, 0, 0);
                break;
            }
        }
    }
    for s in 3..5i32 {
        let fora: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(cqv11_d(cqv11_d(this) + 0x68));
        fora(this, cqv11_d(cqv11_linha(segundos, s)), -1, 3, 3, 3);
    }
    for g in -1i8..4 {
        let grupo = cqv11_grupo(this, g);
        for r in 0..cqv11_w(grupo + 0x3e).max(0) as usize {
            let clube = cqv11_d(cqv11_d(grupo + 0xb1) + r * 0x3b);
            if !(0..8).any(|j| cqv11_d(v + j * 4) == clube) {
                nao_classificado(cqv11_d(0xdd7f00), clube, cqv11_d(this + 4), 0x41c, 0x28);
            }
        }
    }
    if segundos != 0 {
        let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqv11_d(cqv11_d(segundos)));
        destruir(segundos, 1);
    }
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqv11_d(cqv11_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, 4, &mut rodadas, &mut extra, 0);
    let oito = v; // cqv11: os 8 das quartas na ordem
    let p = novo(0xb2);
    let quartas = if p != 0 {
        copa(p, this, 8, oito, rodadas as u16 as u32, cqv11_d(this + 4), tabela, cqv11_w(this + 0x40) as u16 as u32, 4, 2, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    cqv11_pd(cqv11_d(this + 0xc) + 0x10, quartas);
    soltar(v);
    soltar(tabela);
    let f = cqv11_d(cqv11_d(this + 0xc) + 0x10);
    if f == 0 {
        return aviso_em(0x9a7e5c, 0x2ab);
    }
    sortear(f, 0);
}

// ponytail: as fases são alocadas, os registros e o sorteio da rodada não rodam duas vezes. Só a original; o plano: os
// 8 das quartas são os +0xbe primeiros de cada grupo e três segundos, e os três segundos que entraram são os de
// melhor tabela entre os cinco segundos de antes (os de maior pontuação: [inferência] a ordem da tabela é a de
// 0x6827d0, então só confere que cada um é segundo de um grupo e que nenhum clube se repete); a fase em [+0xc][4] tem
// os 8 na sua tabela de times (+0xa7, linhas de 6 bytes).
unsafe extern "thiscall" fn comparar_cqv11_mercosul_quartas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let q = *((this + 0xbe) as *const i8);
    let fases = cqv11_d(this + 0xc);
    if !(1..=2).contains(&q) || fases == 0 || (0..4).any(|g| cqv11_d(fases + g * 4) == 0) {
        return o(this);
    }
    let mut primeiros: Vec<usize> = Vec::new();
    let mut segundos: Vec<usize> = Vec::new();
    for g in -1i8..4 {
        let grupo = cqv11_grupo(this, g);
        for i in 0..q as i32 {
            primeiros.push(cqv11_d(cqv11_linha(grupo, i)));
        }
        segundos.push(cqv11_d(cqv11_linha(grupo, 1)));
    }
    o(this);
    let f = cqv11_d(cqv11_d(this + 0xc) + 0x10);
    if f == 0 {
        return;
    }
    let t = cqv11_d(f + 0xa7);
    let mut lidos: Vec<usize> = if t != 0 { (0..8).map(|i| cqv11_d(t + i * 6)).collect() } else { Vec::new() };
    let entraram: Vec<usize> = lidos.iter().copied().filter(|c| !primeiros.contains(c)).collect();
    let ok = lidos.len() == 8
        && primeiros.iter().all(|c| lidos.contains(c))
        && entraram.len() + primeiros.len() == 8
        && entraram.iter().all(|c| segundos.contains(c))
        && {
            lidos.sort_unstable();
            lidos.windows(2).all(|w| w[0] != w[1])
        };
    PLACAR.registrar(ok, || format!("cqv11_mercosul_quartas({:#x}): quartas {:x?} primeiros {:x?} segundos {:x?}", this, lidos, primeiros, segundos));
}

/// { 0x4c4210 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x5c): `ConmebolLiber::set_predicted_positions()`: as
/// posições previstas da Libertadores, pelas rodadas da fase 7 (o mata-mata) e em blocos de 8 para os que ficam nos
/// grupos (caixa na linha 0x30e).
unsafe extern "thiscall" fn cqv11_libertadores_previstas(this: usize) {
    cqv11_previstas(this, 7, 8, 0x9a7de4, 0x30e)
}

unsafe extern "thiscall" fn comparar_cqv11_libertadores_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqv11_comparar_previstas("cqv11_libertadores_previstas", this, 7, 8, || o(this))
}

/// { 0x4c5a90 } (thiscall, ret 0, conmebol_merc.cpp; a virtual +0x5c): `ConmebolMerc::set_predicted_positions()`: as
/// posições previstas da Mercosul, pelas rodadas da fase 4 (as quartas em diante) e em blocos de 5 para os que ficam
/// nos grupos (caixa na linha 0x338).
unsafe extern "thiscall" fn cqv11_mercosul_previstas(this: usize) {
    cqv11_previstas(this, 4, 5, 0x9a7e5c, 0x338)
}

unsafe extern "thiscall" fn comparar_cqv11_mercosul_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqv11_comparar_previstas("cqv11_mercosul_previstas", this, 4, 5, || o(this))
}

/// As chamadas das posições previstas das copas da CONMEBOL (0x4c2a90, 0x4c4210 e 0x4c5a90, o mesmo corpo), na ordem:
/// a lista dos convidados (+0xee, +0xf2 clubes) já ordenada; as rodadas da tabela `tabela` (linhas de 0x68 bytes; o
/// word +0x1a de cada uma, os que saem nela), da última para a primeira: a última dá a posição 1 aos que saem nela e,
/// como todas, a posição corrente (que sobe a cada clube) a mais tantos; os que restam ficam em blocos de `grupo` com a
/// posição do começo do bloco. Devolve (clube, posição).
unsafe fn cqv11_plano_previstas(this: usize, tabela: usize, rodadas: i16, grupo: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let lista = cqv11_d(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut atual: u8 = 1;
    let mut r = rodadas as i32 - 1;
    while r >= 0 {
        let p = tabela + r as usize * 0x68 + 0x1a;
        if r == rodadas as i32 - 1 {
            let mut j: i16 = 0;
            while j < cqv11_w(p) {
                v.push((cqv11_d(lista + i as isize as usize * 4), atual));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        atual = bl;
        let mut j: i16 = 0;
        while j < cqv11_w(p) {
            v.push((cqv11_d(lista + i as isize as usize * 4), atual));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < cqv11_d(this + 0xf2) as i32 {
        v.push((cqv11_d(lista + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + grupo {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// O corpo das posições previstas: sem estatísticas (+8), a caixa `linha`; a lista +0xee de +0xf2 convidados ordenada
/// pela reputação (qsort 0x945333(lista, n, 4, 0x4b07d0): 4 pushes; add esp -> cdecl); a tabela das rodadas da fase
/// `fase` pela virtual +0x3c (push 0; push &extra; push &rodadas; push fase; mov ecx, this -> thiscall, ret 0x10); cada
/// clube do plano com a sua posição (Comp_Stats::set_club_final_position 0x4a2540: push posição; push clube; mov ecx,
/// [+8] -> thiscall, ret 8; a original empurra o byte com sobra nos bytes altos, a 0x4a2540 lê o byte); a tabela
/// liberada (0x9452ca).
unsafe fn cqv11_previstas(this: usize, fase: u32, grupo: u32, arquivo: usize, linha: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqv11_d(this + 8) == 0 {
        return aviso_em(arquivo, linha);
    }
    ordenar(cqv11_d(this + 0xee), cqv11_d(this + 0xf2) as u32, 4, 0x4b07d0);
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqv11_d(cqv11_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, fase, &mut rodadas, &mut extra, 0);
    for (c, p) in cqv11_plano_previstas(this, tabela, rodadas, grupo) {
        gravar(cqv11_d(this + 8), c, p as u32);
    }
    soltar(tabela);
}

// ponytail: Comp_Stats::set_club_final_position acrescenta o registro do clube (realoca): não roda duas vezes. Só a
// original; o plano, com a lista já ordenada por ela: cada convidado com registro e as posições (+0x72, 0x4a4850)
// crescentes na ordem da lista, a primeira 1. (Pedir de novo a tabela das rodadas da fase mexe no calendário da copa:
// a comparação de 07/11 deu a caixa da linha 1278 do Cup.cpp; o plano não chama mais nada além da busca.)
unsafe fn cqv11_comparar_previstas(nome: &str, this: usize, _fase: u32, _grupo: u32, o: impl FnOnce()) {
    o();
    let est = cqv11_d(this + 8);
    let lista = cqv11_d(this + 0xee);
    let n = cqv11_d(this + 0xf2) as i32;
    if est == 0 || lista == 0 || !(1..=256).contains(&n) {
        return;
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut lidas: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = cqv11_d(lista + k * 4);
        let e = if c != 0 { buscar(est, c) } else { 0 };
        lidas.push(if e != 0 { *((e + 0x72) as *const u8) as i32 } else { -1 });
    }
    let ok = lidas[0] == 1 && lidas.iter().all(|&x| x >= 1) && lidas.windows(2).all(|w| w[0] <= w[1]);
    PLACAR.registrar(ok, || format!("{}({:#x}): posições {:?}", nome, this, lidas));
}

