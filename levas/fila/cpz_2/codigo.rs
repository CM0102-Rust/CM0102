// ---------- %%LEVA%%a leva: as divisões inglesas no fim da temporada: os registros pela situação na tabela e no mata-mata, e a situação do clube na First e na Third (eng_second.cpp, eng_first.cpp, eng_third.cpp) ----------

unsafe fn cpz2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cpz2_registros() -> usize {
    cpz2_d(0xdd7f00)
}

/// { 0x576c50 } (thiscall, ret 0x18, eng_second.cpp): `EngSecond::update_staff_records_for_status(clube, situação,
/// fase, &saiu, _, _)`, a virtual +0x44 da First, da Second e da Third (anunciada pela 0x686670). Devolve 0.
/// Na liga (fase 0xff), pela situação: 5, o título (Staff_Records::add_league_champions_record 0x868c50,
/// thiscall(registros, clube, comp +4), ret 8); 0, o acesso (0x869480, thiscall(registros, clube, comp, 0x64), ret
/// 0xc); 1, a vaga no mata-mata (0x86bdd0, thiscall(registros, clube, comp, 0xa0, 0x82, 0x1e), ret 0x14); 3, o
/// rebaixamento (0x86a1c0, thiscall(registros, clube, comp), ret 8). No mata-mata (outra fase), na linha do clube da
/// tabela (+0xb1, 0x3b bytes, +0x3e linhas): 1 (venceu a final), o acesso com peso 0x32, +0x37 = 0 e saiu = 1; 0
/// (passou da rodada), a classificação (0xa0, 0x96, 0xf) e segue procurando; 0xff ou 2 (caiu), +0x37 = 0xfe.
unsafe extern "thiscall" fn registros_pela_situacao_inglesa(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, _a: u32, _b: u32) -> u32 {
    let s = situacao as u8;
    let comp = cpz2_d(this + 4);
    if fase as u8 == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
                titulo(cpz2_registros(), clube, comp);
            }
            0 => {
                crate::comparacao::ramo("cpz_2 acesso na tabela");
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                acesso(cpz2_registros(), clube, comp, 0x64); // cpz2: o acesso pela tabela pesa 0x64
            }
            1 => {
                crate::comparacao::ramo("cpz_2 vaga no mata-mata");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                vaga(cpz2_registros(), clube, comp, 0xa0, 0x82, 0x1e); // cpz2: a vaga no mata-mata pesa 0x1e
            }
            3 => {
                crate::comparacao::ramo("cpz_2 rebaixamento na tabela");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                cair(cpz2_registros(), clube, comp); // cpz2: o rebaixamento pela tabela
            }
            _ => {}
        }
        return 0;
    }
    let mut i: i32 = 0;
    while i < *((this + 0x3e) as *const i16) as i32 {
        let linha = cpz2_d(this + 0xb1) + (i as usize) * 0x3b;
        if cpz2_d(linha) == clube {
            match s {
                1 => {
                    crate::comparacao::ramo("cpz_2 venceu o mata-mata");
                    let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                    acesso(cpz2_registros(), clube, comp, 0x32);
                    *((cpz2_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                    *(saiu as *mut u8) = 1;
                    return 0;
                }
                0 => {
                    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                    vaga(cpz2_registros(), clube, comp, 0xa0, 0x96, 0xf);
                }
                0xff | 2 => {
                    *((cpz2_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                    return 0;
                }
                _ => {}
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x576c50: os registros gravam o histórico no arquivo, sorteiam o moral e podem renovar contratos
/// (não rodam duas vezes): só a original. O plano, feito antes, no mata-mata: a primeira linha do clube decide o que a
/// rotina grava (+0x37 = 0 e saiu = 1 com 1; +0x37 = 0xfe com 0xff ou 2; nada com 0, que segue procurando outra linha
/// do mesmo clube); depois a linha e `saiu` têm de estar assim. Na liga a rotina não grava nada: a devolução.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_inglesa(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, a: u32, b: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(%%I0%%);
    let s = situacao as u8;
    let mut plano: Option<(usize, u8, bool)> = None;
    if fase as u8 != 0xff {
        let tab = cpz2_d(this + 0xb1);
        let n = *((this + 0x3e) as *const i16) as i32;
        let mut i: i32 = 0;
        while i < n {
            let linha = tab + (i as usize) * 0x3b;
            if cpz2_d(linha) == clube {
                match s {
                    1 => {
                        plano = Some((linha, 0, true));
                        break;
                    }
                    0xff | 2 => {
                        plano = Some((linha, 0xfe, false));
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
    let r = o(this, clube, situacao, fase, saiu, a, b);
    let igual = r == 0
        && match plano {
            Some((linha, v, marca)) => cpz2_b(linha + 0x37) == v && (!marca || cpz2_b(saiu) == 1),
            None => true,
        };
    PLACAR.registrar(igual, || format!("registros_pela_situacao_inglesa({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, fase as u8, r, plano));
    r
}

/// A entrada do clube nas estatísticas da competição (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4;
/// põe a tabela na ordem dos clubes se precisa e procura).
unsafe fn cpz2_entrada(this: usize, clube: usize) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    achar(cpz2_d(this + 8), clube)
}

/// A situação do clube nas estatísticas (+0x73 a fase, +0x74 a melhor, +0x75 a pior), comum à First e à Third: na liga
/// (fase 0xff) a melhor e a pior de `baixo` a `alto` (as vagas do mata-mata) viram `baixo`, e com a pior nessa faixa a
/// pior vira `alto`; no mata-mata (fase 0), 1, 2 e 3 viram `baixo`, `baixo` + 1 e `baixo` + 2 (a melhor, a pior e a
/// fase); noutra fase, como vieram.
unsafe fn cpz2_situacao(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8, baixo: u8, alto: u8) {
    let e = cpz2_entrada(this, clube);
    let (mut m, mut p, mut a) = (melhor, pior, atual);
    if fase == 0xff {
        if m >= baixo && m <= alto {
            m = baixo;
        }
        if p >= baixo && p <= alto {
            p = alto;
        }
    } else if fase == 0 {
        let mapa = |v: u8| match v {
            1 => baixo,
            2 => baixo + 1,
            3 => baixo + 2,
            _ => v,
        };
        m = mapa(m);
        p = mapa(p);
        a = mapa(a);
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// { 0x573260 } (thiscall, ret 0x14, eng_first.cpp): `EngFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da First e da Second (da 0x684cd0 a cada linha da tabela): as vagas do mata-mata são 3 a 6.
unsafe extern "thiscall" fn situacao_na_first(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cpz2_situacao(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8, 3, 6)
}

/// { 0x578660 } (thiscall, ret 0x14, eng_third.cpp): `EngThird::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da Third: as vagas do mata-mata são 4 a 7.
unsafe extern "thiscall" fn situacao_na_third(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cpz2_situacao(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8, 4, 7)
}

/// O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma; a ordem dos clubes já fica
/// pronta) e as duas rodam lado a lado nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe fn cpz2_comparar(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let e = cpz2_entrada(this, clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}

unsafe extern "thiscall" fn comparar_situacao_na_first(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    cpz2_comparar("situacao_na_first", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_first(this, clube, fase, atual, melhor, pior))
}

unsafe extern "thiscall" fn comparar_situacao_na_third(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    cpz2_comparar("situacao_na_third", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_third(this, clube, fase, atual, melhor, pior))
}
