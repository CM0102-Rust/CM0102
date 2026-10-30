// ---------- %%LEVA%%a leva: a diretoria e o técnico: a diretoria do técnico novo, o motivo da saída do técnico e o registro do técnico (manager_manager.cpp, staff_records.cpp) ----------

unsafe fn cpp3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp3_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpp3_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A linha do clube de id `id` na tabela da diretoria do MANAGER_MANAGER ([this] + 0x49 × id; relida a cada uso, como
/// na original): +1 a faixa de riqueza, +2/+3 marcas, +6 confiança, +8/+0xa/+0xc os humores, +0x10..+0x16 as mudanças,
/// +0x28, +0x30/+0x38 doubles, +0x40 marcas.
unsafe fn cpp3_linha(this: usize, id: u32) -> usize {
    (cpp3_d(this) as usize).wrapping_add((id as usize).wrapping_mul(0x49))
}

/// O humor novo (`mov word`): ftol do float `f32(sorteio1 + q)` menos `sorteio2` (os dois de 0..499), em dupla precisão.
unsafe fn cpp3_humor(q: f32) -> i16 {
    let s1 = sortear(0x1f4);
    let base = (s1 as f64 + q as f64) as f32; // fstp dword
    let s2 = sortear(0x1f4);
    ftol(base as f64 - s2 as f64) as i16
}

/// { 0x694c80 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::reset_board_for_new_manager(clube)`: marca o
/// bit 1 de +2 da linha da diretoria, recalcula as expectativas da temporada (0x6a3770 com a notícia) e marca de novo;
/// a lista dos clubes do Shortlist_Manager (+0xdc, 9 bytes por clube, +4) fica com 2. A reputação do técnico (+0xcf,
/// ficha +0x69): na mesma nação (+0x53 = +0x1a do técnico) a atual (+0xa) e a doméstica (+8) andam 10% para a do clube
/// (+0x80); senão a atual anda 10% a partir da de outra nação (+0xc). A base: a reputação atual (zero: a metade da do
/// clube, ou 5000 sem; abaixo de 500: 500 + sorteio de 1000) sobre a do clube (zero: ela mesma; abaixo de 500: 250 +
/// sorteio de 750) × 7500, em float; acima do double de 0x96d1b0 (lido como double: um denormal positivo) vira 10000,
/// abaixo de 6500, 6500. A confiança (+6) e os humores +8/+0xa/+0xc saem da base com dois sorteios de 500 cada (limites
/// 6250..9500, 6250..10000, 5250..10000, 5750..10000, com sorteios nos de baixo); zera as mudanças +0x10..+0x16, os
/// doubles +0x38, +0x28, +4, +5 e marcas (+2 &~0x2e, +3 &0xe0, +0x40 &7). Registra o técnico novo (0x869aa0 com 1). Com o
/// técnico humano (id ≥ [0xae2364] − 16): cancela as ofertas do clube (0x8c4950), refaz a lista do clube (0x84abe0) e
/// as táticas das 50 vagas (0x896c60). +1 = a faixa de riqueza das finanças (0x5986b0), 0 nas seleções ou sem finanças.
unsafe extern "thiscall" fn diretoria_do_tecnico_novo(this: usize, clube: usize) {
    let id = cpp3_d(clube);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    expectativas(this, clube, 1);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let lista = cpp3_d(cpp3_d(cpp3_d(0xde1f54) as usize + 0x856) as usize + 0xdc) as usize;
    *(lista.wrapping_add((id as usize).wrapping_mul(9)) as *mut u8).add(4) = 2;
    let k = f64_em(0x967880); // 0,1
    let rep = cpp3_w(clube + 0x80) as i32;
    let tecnico = cpp3_d(clube + 0xcf) as usize;
    let ficha = cpp3_d(tecnico + 0x69) as usize;
    if cpp3_d(clube + 0x53) == cpp3_d(tecnico + 0x1a) {
        let a = cpp3_w(ficha + 0xa) as i32;
        cpp3_por_w(ficha + 0xa, ftol((rep - a) as f64 * k + a as f64) as i16);
        let ficha = cpp3_d(cpp3_d(clube + 0xcf) as usize + 0x69) as usize;
        let b = cpp3_w(ficha + 8) as i32;
        cpp3_por_w(ficha + 8, ftol((rep - b) as f64 * k + b as f64) as i16);
    } else {
        let c = cpp3_w(ficha + 0xc) as i32;
        let a = cpp3_w(ficha + 0xa) as i32;
        cpp3_por_w(ficha + 0xa, ftol((rep - c) as f64 * k + a as f64) as i16);
    }
    let ficha = cpp3_d(cpp3_d(clube + 0xcf) as usize + 0x69) as usize;
    let v = cpp3_w(ficha + 0xa) as i32;
    let mut atual = v as f32; // fst dword
    let zero = f32_em(0x968970) as f64;
    if v as f64 == zero {
        let r = cpp3_w(clube + 0x80) as i32;
        atual = if r != 0 { (r / 2) as f32 } else { 5000.0 };
    } else if (atual as f64) < f32_em(0x96d1a8) as f64 {
        atual = (sortear(0x3e8) + 0x1f4) as f32;
    }
    let d = cpp3_w(clube + 0x80) as i32;
    let divisor = if d as f64 == zero {
        atual as f64
    } else if (d as f64) < f32_em(0x96d1a8) as f64 {
        (sortear(0x2ee) + 0xfa) as f64
    } else {
        d as f64
    };
    let mut q = ((atual as f64 / divisor) * f32_em(0x96d1a4) as f64) as f32; // fstp dword
    if q as f64 > f64_em(0x96d1b0) {
        crate::comparacao::ramo("cpp_3 base no teto");
        q = 10000.0;
    } else if !(q as f64 >= f32_em(0x96d1ac) as f64) {
        q = 6500.0;
    }
    // a confiança e os humores, cada um com os seus limites
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 6, h);
    let c = cpp3_w(cpp3_linha(this, id) + 6);
    if c < 0x186a {
        cpp3_por_w(cpp3_linha(this, id) + 6, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x251c {
        cpp3_por_w(cpp3_linha(this, id) + 6, (0x2710 - sortear(0x1f4)) as i16);
    }
    let s1 = sortear(0x1f4);
    let parcial = cpp3_w(cpp3_linha(this, id) + 6).wrapping_sub(s1 as i16);
    let s2 = sortear(0x1f4);
    cpp3_por_w(cpp3_linha(this, id) + 8, parcial.wrapping_add(s2 as i16));
    let c = cpp3_w(cpp3_linha(this, id) + 8);
    if c < 0x186a {
        cpp3_por_w(cpp3_linha(this, id) + 8, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 8, 0x2710);
    }
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 0xa, h);
    let c = cpp3_w(cpp3_linha(this, id) + 0xa);
    if c < 0x1482 {
        cpp3_por_w(cpp3_linha(this, id) + 0xa, (sortear(0xfa) + 0x1482) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 0xa, 0x2710);
    }
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 0xc, h);
    let c = cpp3_w(cpp3_linha(this, id) + 0xc);
    if c < 0x1676 {
        cpp3_por_w(cpp3_linha(this, id) + 0xc, (sortear(0xfa) + 0x1676) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 0xc, 0x2710);
    }
    let e = cpp3_linha(this, id);
    for o in [0x10, 0x12, 0x14, 0x16] {
        cpp3_por_w(e + o, 0); // as mudanças
    }
    let m = |o: usize, mascara: u8| *((cpp3_linha(this, id) + o) as *mut u8) &= mascara;
    m(2, 0xf7);
    m(2, 0xfb);
    m(2, 0xfd);
    cpp3_por(e + 0x38, 0);
    cpp3_por(e + 0x3c, 0);
    *((e + 4) as *mut u8) = 0;
    m(3, 0xe1);
    m(2, 0xdf);
    cpp3_por(e + 0x28, 0);
    *((e + 5) as *mut u8) = 0;
    m(3, 0xfe);
    m(0x40, 7);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869aa0usize);
    registrar(cpp3_d(0xdd7f00) as usize, cpp3_d(clube + 0xcf) as usize, clube, 1);
    if cpp3_d(cpp3_d(clube + 0xcf) as usize) as i32 >= (cpp3_d(0xae2364) as i32).wrapping_sub(0x10) {
        let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4950usize);
        cancelar(cpp3_d(0xde1f54) as usize, clube);
        let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x84abe0usize);
        refazer(cpp3_d(cpp3_d(0xde1f54) as usize + 0x856) as usize, clube);
        let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
        for k in 0..0x32 {
            let p = cpp3_d(clube + 0xd7 + 4 * k) as usize;
            if p != 0 {
                tatica(cpp3_d(0xde1d08) as usize, clube, p, 1, 1);
            }
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut faixa = 0u8;
    if selecao(clube) == 0 {
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let fin = financas(cpp3_d(0xae2a38) as usize, clube);
        if fin != 0 {
            let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
            faixa = riqueza(fin);
        }
    }
    *((cpp3_linha(this, cpp3_d(clube)) + 1) as *mut u8) = faixa;
}

unsafe extern "thiscall" fn comparar_diretoria_do_tecnico_novo(this: usize, clube: usize) {
    // ponytail: as expectativas (0x6a3770) postam notícia e o registro (0x869aa0) grava o histórico em arquivo: não
    // rodam duas vezes; só a original, a cópia prova pela carreira no modo Rust (o ramo)
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, clube)
}

/// { 0x69b350 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_departure_reason(id)`: o motivo (0..11) da
/// saída do técnico do clube `id`. Seleção (0x53b900): 0. Mais de 2 jogadores do elenco com a queixa 0x10000000 no
/// contrato (+0x45): 9. Na temporada (0x547770): na liga com a nação marcada (+0x11c), linha da tabela com mais de 25
/// (+4) e mais de 1 jogo (+0x55), pela posição 0x9c da competição: progresso (+0x73) dentro e a posição projetada
/// (0x4ae300) fora das (+0x3e − +0xc1) vagas: 11; dívida (double +0x30 < +0x38) com a mudança +0x12 abaixo de −300: 10.
/// Depois: +0x12 abaixo de −750 fora de uma sequência do tipo 1 (0x690c30) com sorteio de 3 = 0: 2; na temporada com o
/// bit 8 de +2: 7; +0x10 < −350: 3; +0x14 < −750: 5; humor +0xa abaixo da confiança +6 e sorteio(+0xa) < sorteio(+6):
/// 8; +0x16 < −750 com sorteio de 3 ≠ 0: 4; o técnico há mais dias (0x54cdb0 desde +0x3e) que 1000 + sorteio de 2000:
/// 6. Senão 0.
unsafe extern "thiscall" fn motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let clube = (cpp3_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245));
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return 0;
    }
    let em_temporada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547770usize);
    crate::comparacao::ramo("cpp_3 motivo da saida");
    let temporada = em_temporada(clube, 0xae2c90);
    let (mut tipo, mut tamanho) = (0u8, 0u8);
    let sequencia: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x690c30usize);
    sequencia(this, clube, &mut tipo, &mut tamanho);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut queixosos = 0u8;
    for k in 0..0x32 {
        let p = cpp3_d(clube + 0xd7 + 4 * k) as usize;
        if p != 0 && cpp3_d(p + 0x61) != 0 {
            let c = contrato(cpp3_d(0xae1050) as usize, p, clube);
            if c != 0 && cpp3_d(c + 0x45) & 0x1000_0000 != 0 {
                queixosos = queixosos.wrapping_add(1);
            }
        }
    }
    if queixosos as i8 > 2 {
        return 9;
    }
    if temporada != 0 {
        let liga = cpp3_d(clube + 0x57) as usize;
        let nacao = cpp3_d(clube + 0x53) as usize;
        if liga != 0 && nacao != 0 && cpp3_b(nacao + 0x11c) != 0 {
            let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(liga) as usize).wrapping_mul(4))) as usize;
            if comp != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let linha = busca(cpp3_d(comp + 8) as usize, clube);
                let (mut pa, mut pb, mut pc) = (0u8, 0u8, 0u8); // a 0x4ae300 grava as três (com a linha achada)
                let projetar: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32 = core::mem::transmute(0x4ae300usize);
                let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                projetar(cpp3_d(comp + 8) as usize, clube, &mut pa, &mut pb, &mut pc);
                if cpp3_b(linha + 4) > 0x19 && cpp3_d(linha + 0x55) as i32 > 1 {
                    let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpp3_d(cpp3_d(comp) as usize + 0x9c) as usize);
                    if f(comp) != 0 {
                        let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                        let vagas = cpp3_w(comp + 0x3e) as i32 - cpp3_b(comp + 0xc1) as i8 as i32;
                        if cpp3_b(linha + 0x73) as i32 >= vagas && (pa as i32) < vagas {
                            return 0xb;
                        }
                    }
                }
            }
        }
        let e = cpp3_linha(this, id);
        let (gasto, teto) = (core::ptr::read_unaligned((e + 0x30) as *const f64), core::ptr::read_unaligned((e + 0x38) as *const f64));
        if !(gasto >= teto) && cpp3_w(e + 0x12) < -300 {
            return 0xa;
        }
    }
    let e = cpp3_linha(this, id);
    if cpp3_w(e + 0x12) < -750 && tipo != 1 && sortear(3) == 0 {
        crate::comparacao::ramo("cpp_3 motivo 2");
        return 2;
    }
    if temporada != 0 && cpp3_b(e + 2) & 8 != 0 {
        return 7;
    }
    if cpp3_w(e + 0x10) < -350 {
        return 3;
    }
    if cpp3_w(e + 0x14) < -750 {
        return 5;
    }
    let humor = cpp3_w(e + 0xa);
    if humor < cpp3_w(e + 6) {
        let a = sortear(cpp3_w(cpp3_linha(this, id) + 6) as i32);
        let b = sortear(humor as i32);
        if b < a {
            return 8;
        }
    }
    if cpp3_w(cpp3_linha(this, id) + 0x16) < -750 && sortear(3) != 0 {
        return 4;
    }
    let tecnico = cpp3_d(clube + 0xcf) as usize;
    if tecnico != 0 {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        let d = dias(0xae2c90, cpp3_d(tecnico + 0x3e), cpp3_d(tecnico + 0x42));
        if d > sortear(0x7d0) + 0x3e8 {
            return 6;
        }
    }
    0
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador (a rotina sorteia em vários caminhos), pela devolução;
/// a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, id),
        || crate::aleatorio::sem_contar(|| motivo_da_saida_do_tecnico(this, id)));
    PLACAR.registrar(igual && r == n, || format!("motivo_da_saida_do_tecnico(clube {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo, mr));
    r
}

/// O registro do técnico (210 bytes) que a 0x86f5b0 olha: humano (id em [0xae2364] − 16 .. [0xae2364]) pela vaga dele
/// depois dos [0xae2368] registros; senão pelo id da ficha de não jogador (+0x69). None: sem ficha, ou humano sem
/// situação (+0x5f 0 ou 5).
unsafe fn cpp3_registro(this: usize, pessoa: usize) -> Option<usize> {
    let n = cpp3_d(0xae2364) as i32;
    let id = cpp3_d(pessoa) as i32;
    let k = if id >= n.wrapping_sub(0x10) && id < n {
        let s = cpp3_b(pessoa + 0x5f);
        if s == 0 || s == 5 {
            return None;
        }
        id.wrapping_sub(n).wrapping_add(cpp3_d(0xae2368) as i32).wrapping_add(0x10)
    } else {
        let ficha = cpp3_d(pessoa + 0x69) as usize;
        if ficha == 0 {
            return None;
        }
        cpp3_d(ficha) as i32
    };
    Some((cpp3_d(this + 8) as usize).wrapping_add((k as usize).wrapping_mul(0xd2)))
}

unsafe fn cpp3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// { 0x86f5b0 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::get_manager_record(pessoa, criar)`: o registro de
/// 210 bytes do técnico (vetor +8). Pessoa nula: caixa da linha 0xf6e e 0. Vaga livre (+0x86 = −1): sem `criar`, 0;
/// com ele, a vaga fica com o id. Vaga de outro: zera o registro da ficha dele (−1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e,
/// +0x72) e olha de novo. Atualiza as nações do clube (+0x39) e da seleção (+0x24) em +0xc2/+0xc6, zerando os pontos
/// (+0xba/+0xbe) quando mudam. Com a data de [0xdd6ec8] até hoje: sem passagem (+0x4a = −1), abre a do clube atual
/// (desde +0x3e da pessoa; +0x56 = a data 13/11/1975 da 0x549f70); a mais longa (+0x5e, +0x62..+0x6a) e a mais curta
/// (+0x72, +0x76..+0x7e) pela 0x54cdb0, a de hoje em +0x6a / +0x7e. Devolve o registro.
unsafe extern "thiscall" fn registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    if pessoa == 0 {
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
        let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        separar(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        caixa(0x9870e0, 0xae24d0, 0);
        cpp3_por(0xb67a34, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = cpp3_registro(this, pessoa) else {
            return 0;
        };
        let dono = cpp3_d(r + 0x86) as i32;
        if dono == -1 {
            if criar == 0 {
                return 0;
            }
            crate::comparacao::ramo("cpp_3 criar registro");
            cpp3_por(r + 0x86, cpp3_d(pessoa)); // a vaga fica com ele
            break r;
        }
        if dono == cpp3_d(pessoa) as i32 {
            break r;
        }
        // a vaga é de outro: zera o registro da ficha dele e olha de novo
        let z = (cpp3_d(this + 8) as usize).wrapping_add((cpp3_d(cpp3_d(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            cpp3_por(z + o, u32::MAX);
        }
    };
    let clube = cpp3_d(pessoa + 0x39) as usize;
    if clube != 0 {
        let nacao = cpp3_d(clube + 0x53) as usize;
        if nacao != 0 && cpp3_d(nacao) != cpp3_d(r + 0xc2) {
            cpp3_por(r + 0xc2, cpp3_d(nacao));
            cpp3_por(r + 0xba, 0);
        }
    }
    let selecao = cpp3_d(pessoa + 0x24) as usize;
    if selecao != 0 {
        let nacao = cpp3_d(selecao + 0x53) as usize;
        if nacao != 0 && cpp3_d(nacao) != cpp3_d(r + 0xc6) {
            cpp3_por(r + 0xc6, cpp3_d(nacao));
            cpp3_por(r + 0xbe, 0);
        }
    }
    let (dia, ano) = (cpp3_w(0xdd6ec8), core::ptr::read_unaligned(0xdd6eca as *const u16));
    let hoje_ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > cpp3_w(0xae2c90)) {
        return r; // a data de [0xdd6ec8] ainda não chegou
    }
    if cpp3_d(r + 0x4a) as i32 == -1 {
        let clube = cpp3_d(pessoa + 0x39) as usize;
        if clube != 0 {
            crate::comparacao::ramo("cpp_3 abrir passagem");
            cpp3_por(r + 0x4a, cpp3_d(clube));
            cpp3_data(r + 0x4e, pessoa + 0x3e);
            let mut local = [0u8; 8];
            let construir: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
            let p = construir(&mut local, 0xd, 0xb, 0x7b7, -1);
            cpp3_data(r + 0x56, p);
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if cpp3_d(r + 0x5e) as i32 == -1 {
        cpp3_por(r + 0x5e, cpp3_d(r + 0x4a));
        cpp3_data(r + 0x62, r + 0x4e);
        cpp3_data(r + 0x6a, 0xae2c90);
    } else if cpp3_d(r + 0x4a) as i32 != -1 {
        let a = dias(r + 0x6a, cpp3_d(r + 0x62), cpp3_d(r + 0x66));
        let b = dias(0xae2c90, cpp3_d(r + 0x4e), cpp3_d(r + 0x52));
        if b > a {
            cpp3_por(r + 0x5e, cpp3_d(r + 0x4a)); // a mais longa
            cpp3_data(r + 0x62, r + 0x4e);
            cpp3_data(r + 0x6a, 0xae2c90);
        }
    }
    if cpp3_d(r + 0x72) as i32 == -1 {
        cpp3_por(r + 0x72, cpp3_d(r + 0x4a));
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
        return r;
    }
    let atual = cpp3_d(r + 0x4a);
    if cpp3_d(r + 0x72) == atual && cpp3_w(r + 0x50) == cpp3_w(r + 0x78) && cpp3_w(r + 0x4e) == cpp3_w(r + 0x76) {
        cpp3_data(r + 0x7e, 0xae2c90); // a mais curta é a atual: vai até hoje
        return r;
    }
    if atual as i32 == -1 {
        return r;
    }
    let a = dias(r + 0x7e, cpp3_d(r + 0x76), cpp3_d(r + 0x7a));
    let b = dias(0xae2c90, cpp3_d(r + 0x4e), cpp3_d(r + 0x52));
    if b < a {
        cpp3_por(r + 0x72, cpp3_d(r + 0x4a)); // a mais curta
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
    }
    r
}

/// Comparador: lado a lado pelo registro do técnico (e, quando a vaga é de outro, pelo registro que a original zera),
/// pela devolução. Pessoa nula (a caixa) e o humano sem situação: só a original.
unsafe extern "thiscall" fn comparar_registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I2%%);
    if pessoa == 0 {
        return o(this, pessoa, criar);
    }
    let Some(r) = cpp3_registro(this, pessoa) else {
        return o(this, pessoa, criar);
    };
    let mut reg = vec![(r, 0xd2)];
    let dono = cpp3_d(r + 0x86) as i32;
    if dono != -1 && dono != cpp3_d(pessoa) as i32 {
        let z = (cpp3_d(this + 8) as usize).wrapping_add((cpp3_d(cpp3_d(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        if z != r {
            reg.push((z, 0xd2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, criar), || registro_do_tecnico(this, pessoa, criar));
    PLACAR.registrar(igual && a == b, || format!("registro_do_tecnico(pessoa {:x}, criar {}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, criar, a, b, mo, mr));
    a
}
