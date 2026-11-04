// ---------- %%LEVA%%a leva: a Libertadores: os 32 classificados, o destino de cada clube (o campeão vai ao Mundial Interclubes) e a fase seguinte (conmebol_liber.cpp) ----------

unsafe fn cqk11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os clubes da competição da Libertadores na ordem do banco ([0xae23b4], 0x245 bytes, [0xae235c] clubes): os que têm
/// a competição europeia/continental da temporada (+0x1db) igual ao id da competição ([+4]), até `max`.
unsafe fn cqk11_inscritos(this: usize, max: i32, saida: &mut [usize; 32]) -> i32 {
    let mut n = 0i32;
    let mut i = 0i16;
    let mut clube = cqk11_d(0xae23b4);
    if (cqk11_d(0xae235c) as i32) > 0 {
        loop {
            if cqk11_d(clube + 0x1db) == cqk11_d(cqk11_d(this + 4)) && n < max {
                if (n as usize) < saida.len() {
                    saida[n as usize] = clube;
                }
                n += 1;
            }
            i = i.wrapping_add(1);
            clube = clube.wrapping_add(0x245);
            if i as i32 >= cqk11_d(0xae235c) as i32 {
                break;
            }
        }
    }
    n
}

/// { 0x4c3f40 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::select_qualified_clubs()`: +0xf2 = 32 e a lista
/// +0xee (0x944e46(0x80): push 0x80; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x285 e devolve 0) com os
/// clubes, na ordem do banco, cuja competição da temporada (+0x1db) é a Libertadores, até 32; menos de 32, a caixa da
/// linha 0x297 e devolve 0; senão a lista embaralhada (0x4afb40(lista, 32): push n; push lista; add esp, 8 -> cdecl; 3 ×
/// 32 trocas sorteadas) e devolve 1.
unsafe extern "thiscall" fn cqk11_libertadores_classificados(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    let lista = novo(0x80);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, lista as u32);
    if lista == 0 {
        aviso_em(0x9a7de4, 0x285);
        return 0;
    }
    let mut v = [0usize; 32];
    let n = cqk11_inscritos(this, cqk11_d(this + 0xf2) as i32, &mut v);
    for (k, &c) in v.iter().enumerate().take(n.min(32) as usize) {
        core::ptr::write_unaligned((lista + k * 4) as *mut u32, c as u32); // cqk11: o clube na ordem do banco
    }
    if n != cqk11_d(this + 0xf2) as i32 {
        aviso_em(0x9a7de4, 0x297);
        return 0;
    }
    crate::comparacao::ramo("cqk_11 classificados da libertadores");
    embaralhar(lista, cqk11_w(this + 0xf2) as u16 as u32); // cqk11: os 32 da Libertadores embaralhados
    1
}

/// Comparador da 0x4c3f40: lado a lado com as duas listas (a original aloca a dela, a cópia a sua): a original roda e a
/// lista dela fica; o estado do sorteador volta, a cópia roda sem contar e a lista dela é comparada com a da original e
/// liberada (0x9452ca: push; add esp, 4 -> cdecl). Com menos de 32 inscritos (as caixas): só a original.
unsafe extern "thiscall" fn comparar_cqk11_libertadores_classificados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let mut v = [0usize; 32];
    if cqk11_inscritos(this, 0x20, &mut v) != 0x20 {
        return o(this); // ponytail: as caixas de erro: só a original
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (ee, f2) = (cqk11_d(this + 0xee), cqk11_d(this + 0xf2));
    let r = o(this);
    let (la, depois) = (cqk11_d(this + 0xee), crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO));
    let a: [u32; 32] = if la != 0 { core::ptr::read_unaligned(la as *const [u32; 32]) } else { [0; 32] };
    estado.escrever();
    core::ptr::write_unaligned((this + 0xee) as *mut u32, ee as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, f2 as u32);
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cqk11_libertadores_classificados(this)));
    let lb = cqk11_d(this + 0xee);
    let b: [u32; 32] = if lb != 0 { core::ptr::read_unaligned(lb as *const [u32; 32]) } else { [0; 32] };
    let igual_estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois;
    if lb != 0 && lb != la {
        soltar(lb);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut u32, la as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    depois.escrever();
    PLACAR.registrar(r == rr && a == b && igual_estado, || format!("cqk11_libertadores_classificados({:#x}) devolveu {} x {} estado igual {} original {:x?} rust {:x?}", this, r, rr, igual_estado, a, b));
    r
}

/// { 0x4c4080 } (thiscall, ret 0x18, conmebol_liber.cpp; a virtual +0x44): `ConmebolLiber::
/// update_staff_records_for_status(clube, desfecho, fase, _, jogo, a6)`; devolve 0. Fase (byte com sinal) abaixo de 7,
/// os grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x14, peso 0x28); 0xfe, a não classificação (0x86c1d0:
/// 0x41c, 0x28). Fase 7, o mata-mata: peso 0x4bf850(0x32, 0x64, rodada do jogo +0x34, 4); 0xff, a eliminação (0x86c000:
/// +0x32, +0x30 do jogo, peso); 0, a classificação com +0x32 e o +0x6f da rodada do jogo na tabela de rodadas da fase 7
/// ([+0xc] + 0x1c, +0xa3, 0x68 bytes; os 16 bits altos que a original empurra são os do ponteiro da fase); 1, o registro
/// de campeão (0x86a800) e o clube vai ao Mundial Interclubes (o objeto [[0xadadfc] + 4 × [0x9cf6e4]]): na vaga 0 da
/// tabela +0xa7 dele se o clube que lá está é do continente [0x9cfa1c] (a nação +0x53, o continente +0x71 da nação, +0
/// o id), senão na vaga 1; depois o arquivo da temporada das estatísticas dele (0x4a1c50 em +8: push 1 -> thiscall, ret
/// 4) e as posições previstas (virtual +0x5c); 2, o registro de vice (0x86b0b0).
unsafe extern "thiscall" fn cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqk11_d(this + 4);
    let final_ = |g: usize| {
        let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
        h(cpz4_registros(), clube, jogo, a6);
    };
    if f < 7 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x14, 0x28);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x28);
        }
    } else if f == 7 {
        match d {
            0xff => {
                let peso = cpz4_escala(0x32, 0x64, cqk11_w(jogo + 0x34), 4);
                // ponytail: +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
                cpz4_eliminado(clube, comp, cqk11_w(jogo + 0x32) as u16 as u32, cqk11_w(jogo + 0x30) as u16 as u32, peso);
            }
            0 => {
                let peso = cpz4_escala(0x32, 0x64, cqk11_w(jogo + 0x34), 4);
                let mata = cqk11_d(cqk11_d(this + 0xc) + 0x1c);
                let r = cqk11_w(jogo + 0x34) as isize;
                let rodada = (mata as u32 & 0xffff_0000) | core::ptr::read_unaligned((cqk11_d(mata + 0xa3).wrapping_add((r * 0x68) as usize) + 0x6f) as *const u16) as u32;
                cpz4_classificado(clube, comp, core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
            }
            1 => {
                final_(0x86a800);
                let mundial = cqk11_d(cqk11_d(0xadadfc).wrapping_add(cqk11_d(0x9cf6e4).wrapping_mul(4)));
                let t = cqk11_d(mundial + 0xa7);
                let nacao = cqk11_d(cqk11_d(t) + 0x53);
                crate::comparacao::ramo("cqk_11 campeao da libertadores");
                if nacao != 0 && cqk11_d(cqk11_d(nacao + 0x71)) == cqk11_d(0x9cfa1c) {
                    core::ptr::write_unaligned(t as *mut u32, clube as u32); // cqk11: o campeão na vaga sul-americana
                } else {
                    core::ptr::write_unaligned((t + 6) as *mut u32, clube as u32);
                }
                let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
                arquivar(cqk11_d(mundial + 8), 1);
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqk11_d(cqk11_d(mundial) + 0x5c));
                prever(mundial);
            }
            2 => final_(0x86b0b0),
            _ => {}
        }
    }
    0
}

// ponytail: os registros (histórico, convocações), o arquivo das estatísticas e as posições previstas do Mundial não
// rodam duas vezes. Só a original; o plano: a devolução 0 e, no título (fase 7, desfecho 1), o campeão na vaga do
// Mundial Interclubes que a regra escolhe (0 se lá está um clube do continente [0x9cfa1c], senão 1).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%);
    let titulo = fase as u8 == 7 && desfecho as u8 == 1;
    let mut vaga = 0usize;
    if titulo {
        let mundial = cqk11_d(cqk11_d(0xadadfc).wrapping_add(cqk11_d(0x9cf6e4).wrapping_mul(4)));
        let t = cqk11_d(mundial + 0xa7);
        let nacao = cqk11_d(cqk11_d(t) + 0x53);
        vaga = if nacao != 0 && cqk11_d(cqk11_d(nacao + 0x71)) == cqk11_d(0x9cfa1c) { t } else { t + 6 };
    }
    let r = o(this, clube, desfecho, fase, a4, jogo, a6);
    let igual = r == 0 && (!titulo || cqk11_d(vaga) == clube);
    PLACAR.registrar(igual, || format!("cqk11_libertadores_registros({:#x}, {}, {}) devolveu {} titulo={} vaga {:#x}", clube, desfecho as u8, fase as u8, r, titulo, vaga));
    r
}

/// { 0x4c3a80 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x28): `ConmebolLiber::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x1f6; senão +0x30 sobe e, chegando a 7 (o fim dos sete grupos), o
/// mata-mata (0x4c3d10: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqk11_libertadores_fase_seguinte(this: usize) {
    let f = cqk11_d(this + 0x30) as i32;
    if f >= (cqk11_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9a7de4, 0x1f6);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 7 { // cqk11: o mata-mata depois dos sete grupos
        crate::comparacao::ramo("cqk_11 mata-mata da libertadores");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c3d10usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x4c3d10) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let f = cqk11_d(this + 0x30) as i32;
    if f >= (cqk11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = cqk11_d(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("cqk11_libertadores_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}
