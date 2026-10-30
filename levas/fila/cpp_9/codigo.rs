// ---------- %%LEVA%%a leva: a criação do jogo: a forma inicial dos elencos, os registros de lesão com as lesões iniciais e as listas de clubes por nação dos jovens (injury.cpp, player_regen.cpp) ----------

unsafe fn cpp9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp9_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cpp9_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    e_selecao(clube) != 0
}

/// Os clubes sem as seleções do fim do vetor: [0xae235c] − 2 × [0xae2350].
unsafe fn cpp9_clubes_sem_selecoes() -> i32 {
    cpp9_i(0xae235c).wrapping_sub(cpp9_i(0xae2350).wrapping_mul(2))
}

/// Os dias de pré-temporada que contam na forma inicial: dentro da temporada da nação do clube (0x669bd0), 0; fora,
/// os dias do começo dela até hoje (0x54cdb0 sobre a primeira data): acima de 80, 40; de 41 a 80, 80 − dias (no byte,
/// com sinal); senão os dias.
unsafe fn cpp9_dias_de_preparo(clube: usize, inicio: &mut [u32; 2], fim: &mut [u32; 2]) -> i32 {
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(0x669bd0usize);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let d = dias(inicio.as_mut_ptr(), cpp9_d(0xae2c90) as u32, cpp9_d(0xae2c94) as u32);
    if d > 0x50 {
        0x28
    } else if d > 0x28 {
        (0x50u8.wrapping_sub(d as u8)) as i8 as i32
    } else {
        d
    }
}

/// { 0x62a920 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::init_squad_fitness()`: para cada
/// clube (sem as seleções), os dias de preparo (`cpp9_dias_de_preparo`); cada uma das 50 vagas (+0xd7) com pessoa de
/// id abaixo de [0xae2364]: jogador (+0x61): v = dias − (+0x34 do jogador)/2 − (+0x5b da pessoa)/4 (com sinal,
/// truncado), entre 1 e 40, e a forma (+8 do registro de 31 bytes em [this]) = 10000 − 75 × v; quem não é jogador sai
/// da vaga (0), e se o clube dele (+0x39) é este: cargo (+0x3d) 0xb fica sem clube e 8, 0xf vira 8, 0xd vira 6.
unsafe extern "thiscall" fn forma_inicial_dos_elencos(this: usize) {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    let base = cpp9_d(0xae23b4);
    let mut i = 0i32;
    while i < cpp9_clubes_sem_selecoes() {
        let clube = base.wrapping_add((i as usize).wrapping_mul(0x245));
        let dias = cpp9_dias_de_preparo(clube, &mut inicio, &mut fim);
        for s in 0..50usize {
            let vaga = clube + 0xd7 + 4 * s;
            let p = cpp9_d(vaga);
            if p == 0 || cpp9_i(p) >= cpp9_i(0xae2364) {
                continue;
            }
            let jogador = cpp9_d(p + 0x61);
            if jogador != 0 {
                crate::comparacao::ramo("cpp_9 forma inicial");
                let v = dias - (cpp9_b(jogador + 0x34) as i8 as i32) / 2 - (cpp9_b(p + 0x5b) as i8 as i32) / 4;
                let v = v.clamp(1, 0x28);
                let r = cpp9_d(this).wrapping_add((cpp9_i(p) as usize).wrapping_mul(31));
                core::ptr::write_unaligned((r + 8) as *mut u16, (10000 - v * 75) as u16); // a forma inicial
            } else {
                if cpp9_d(p + 0x39) == clube {
                    match cpp9_b(p + 0x3d) {
                        0xb => {
                            core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
                            *((p + 0x3d) as *mut u8) = 8;
                        }
                        0xf => *((p + 0x3d) as *mut u8) = 8,
                        0xd => *((p + 0x3d) as *mut u8) = 6,
                        _ => {}
                    }
                }
                core::ptr::write_unaligned(vaga as *mut usize, 0);
            }
        }
        i += 1;
    }
}

/// { 0x62cf70 } (thiscall, ret 0, injury.cpp): `Injury::init_records()`: [this] = 31 bytes por pessoa de [0xae2364]
/// (0x944e46; sem memória, a caixa "Error" da linha 0x7d0 e nada). Cada registro: a data de hoje (0x417c10), forma e
/// condição 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1
/// (+0x14, +0x18), +0x1d 0xff, +0x1e 50. Jogador (+0x61): o tipo sorteado de 82 com peso (+8 da tabela de 13 bytes em
/// 0x9e7ad8) e 1 em 10 fica recorrente; 1 em 100 começa lesionado: categoria sorteada de 12, tipo pela 0x62d140
/// (gravidade −1) e a lesão nova 0x62d250. No fim os dois totais em 0xde1f64 ("%d players with ...").
unsafe extern "thiscall" fn registros_iniciais_de_lesao(this: usize) {
    let n = cpp9_i(0xae2364);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar((n as u32).wrapping_mul(31));
    core::ptr::write_unaligned(this as *mut usize, v);
    if v == 0 {
        aviso_em(0x9e7f28, 0x7d0);
        return;
    }
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let escolher: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x62d140usize);
    let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
    let (mut recorrentes, mut iniciais) = (0i32, 0i32);
    let mut r = v;
    let mut pessoa = cpp9_d(0xae23bc);
    let mut i = 0i32;
    while i < cpp9_i(0xae2364) {
        let mut rascunho = [0u8; 8];
        copiar(r, rascunho.as_mut_ptr(), 0xae2c90);
        core::ptr::write_unaligned((r + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((r + 8) as *mut u16, 10000);
        core::ptr::write_unaligned((r + 0xa) as *mut u16, 10000);
        *((r + 0x12) as *mut u8) = 0xff;
        core::ptr::write_unaligned((r + 0xe) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x14) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x18) as *mut i32, -1);
        *((r + 0x1d) as *mut u8) = 0xff;
        *((r + 0x1e) as *mut u8) = 0x32;
        if cpp9_d(pessoa + 0x61) != 0 {
            let t = cpp9_sorteio(0x52) as u8;
            if cpp9_d((0x9e7ae0usize).wrapping_add((t as i8 as isize * 13) as usize)) != 0 && cpp9_sorteio(10) == 0 {
                recorrentes += 1;
                *((r + 0x12) as *mut u8) = t;
            }
            if cpp9_sorteio(100) == 0 { // 1 em 100 começa lesionado
                crate::comparacao::ramo("cpp_9 lesao inicial");
                // ponytail: a original empurra dwords da pilha com só o byte baixo gravado (o resto é lixo); a 0x62d140
                // e a 0x62d250 leem só o byte: aqui vão limpos
                let categoria = cpp9_sorteio(0xc) as u8;
                let tipo = escolher(this, categoria as u32, 0xffff_ffff, pessoa) as u8;
                lesionar(this, tipo as u32, 1, pessoa, 0, 1, -1, 0, 0);
                iniciais += 1;
            }
        }
        i += 1;
        pessoa = pessoa.wrapping_add(0x6e);
        r = r.wrapping_add(31);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    formatar(0xde1f64, 0x9e7f98, recorrentes);
    formatar(0xde1f64, 0x9e7f74, iniciais);
}

/// A nação em cuja lista dos jovens o clube entra (0x7acfc0): nenhuma para seleção (0x53b900), sem nação (+0x53),
/// nação sem +0x71 ou os dois clubes de 0x9d00a4/0x9d00a8. Nação com +0x11c: só com +0x80 até 2 e divisão (+0x57);
/// sem: com divisão.
unsafe fn cpp9_nacao_da_lista(clube: usize) -> usize {
    if cpp9_selecao(clube) {
        return 0;
    }
    let nacao = cpp9_d(clube + 0x53);
    if nacao == 0 || cpp9_d(nacao + 0x71) == 0 {
        return 0;
    }
    if cpp9_b(nacao + 0x11c) != 0 {
        crate::comparacao::ramo("cpp_9 nacao chave");
        if !(cpp9_w(clube + 0x80) <= 2 && cpp9_d(clube + 0x57) != 0) { // nação com +0x11c
            return 0;
        }
    } else if cpp9_d(clube + 0x57) == 0 {
        return 0;
    }
    let id = cpp9_d(clube);
    if id == cpp9_d(0x9d00a4) || id == cpp9_d(0x9d00a8) {
        return 0;
    }
    nacao
}

/// { 0x7acfc0 } (thiscall, ret 0, player_regen.cpp, quadro SEH do `new`): `PLAYER_REGEN::build_nation_club_lists()`:
/// [this] = 14 bytes por nação de [0xae2350] (0x944e46, sem teste), com uma lista nova (0x944cf1 de 0xe e 0x68d020;
/// sem memória, 0) por nação; cada clube de [0xae235c] entra (0x68d0d0, os 4 bytes do id) na lista da nação dele
/// (`cpp9_nacao_da_lista`).
unsafe extern "thiscall" fn listas_de_clubes_por_nacao(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let tabela = alocar((cpp9_i(0xae2350) as u32).wrapping_mul(14));
    core::ptr::write_unaligned(this as *mut usize, tabela);
    let mut i = 0i32;
    while i < cpp9_i(0xae2350) {
        let obj = novo(0xe);
        let l = if obj != 0 { lista_nova(obj) } else { 0 };
        core::ptr::write_unaligned(cpp9_d(this).wrapping_add(i as usize * 4) as *mut usize, l);
        i += 1;
    }
    let base = cpp9_d(0xae23b4);
    let mut j = 0i32;
    while j < cpp9_i(0xae235c) {
        let clube = base.wrapping_add((j as usize).wrapping_mul(0x245));
        let nacao = cpp9_nacao_da_lista(clube);
        if nacao != 0 {
            crate::comparacao::ramo("cpp_9 clube na lista");
            let l = cpp9_d(cpp9_d(this).wrapping_add((cpp9_i(nacao) as usize).wrapping_mul(4)));
            acrescentar(l, clube, 4);
        }
        j += 1;
    }
}

/// Comparador: lado a lado pelos registros de lesão (31 bytes × [0xae2364]), pelos clubes (as vagas) e pelo clube e
/// cargo (+0x39..+0x3d) de quem não é jogador nas vagas.
unsafe extern "thiscall" fn comparar_forma_inicial_dos_elencos(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let base = cpp9_d(0xae23b4);
    let mut regioes = vec![(cpp9_d(this), (cpp9_i(0xae2364).max(0) as usize) * 31), (base, (cpp9_i(0xae235c).max(0) as usize) * 0x245)];
    for i in 0..cpp9_clubes_sem_selecoes().max(0) as usize {
        for s in 0..50usize {
            let p = cpp9_d(base + i * 0x245 + 0xd7 + 4 * s);
            if p != 0 && cpp9_i(p) < cpp9_i(0xae2364) && cpp9_d(p + 0x61) == 0 {
                regioes.push((p + 0x39, 5));
            }
        }
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this), || forma_inicial_dos_elencos(this));
    PLACAR.registrar(igual, || format!("forma_inicial_dos_elencos: memória difere ({} regiões)", regioes.len()));
}

/// Comparador: a original aloca, sorteia e lesiona (0x62d250): roda só ela e confere o que a 0x62d250 não muda: o
/// vetor alocado e o +0x1e = 50 de todos os registros.
unsafe extern "thiscall" fn comparar_registros_iniciais_de_lesao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let v = cpp9_d(this);
    if v == 0 {
        return;
    }
    let errado = (0..cpp9_i(0xae2364).max(0) as usize).find(|&k| cpp9_b(v + 31 * k + 0x1e) != 0x32);
    PLACAR.registrar(errado.is_none(), || format!("registros_iniciais_de_lesao: registro {:?} sem o +0x1e = 50", errado));
}

/// Comparador: a original aloca as listas: o plano antes (os ids dos clubes de cada nação, na ordem); a original roda;
/// confere os itens de cada lista.
unsafe extern "thiscall" fn comparar_listas_de_clubes_por_nacao(this: usize) {
    let n = cpp9_i(0xae2350).max(0) as usize;
    let mut plano: Vec<Vec<usize>> = vec![Vec::new(); n];
    let base = cpp9_d(0xae23b4);
    for j in 0..cpp9_i(0xae235c).max(0) as usize {
        let clube = base + j * 0x245;
        let nacao = cpp9_nacao_da_lista(clube);
        if nacao != 0 && (cpp9_i(nacao) as usize) < n {
            plano[cpp9_i(nacao) as usize].push(cpp9_d(clube));
        }
    }
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let errada = (0..n).find(|&k| {
        let l = cpp9_d(cpp9_d(this) + 4 * k);
        let ids: Vec<usize> = if l != 0 { cpd1_itens(l).into_iter().map(|e| cpp9_d(e)).collect() } else { Vec::new() };
        l != 0 && ids != plano[k]
    });
    PLACAR.registrar(errada.is_none(), || format!("listas_de_clubes_por_nacao: a lista da nação {:?} difere do plano", errada));
}
