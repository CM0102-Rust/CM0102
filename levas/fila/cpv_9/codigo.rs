// ---------- %%LEVA%%a leva: o regen no clube: a vaga no elenco, o valor e o contrato do jovem; a aposentadoria pela lesão e a notícia da aposentadoria anunciada (player_regen.cpp) ----------

unsafe fn cpv9_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv9_por(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// A primeira vaga livre (dword 0) das 50 do elenco do clube (+0xd7); 0x32 sem vaga.
unsafe fn cpv9_vaga_livre(clube: usize) -> usize {
    (0..0x32usize).find(|&k| cpv9_d(clube + 0xd7 + 4 * k) == 0).unwrap_or(0x32)
}

/// { 0x7a6f10 } (stdcall, ret 8, player_regen.cpp, quadro SEH só das datas locais): `player_regen_sign_for_club(pessoa,
/// clube)`, pela PLAYER_REGEN::regenerate_as_youth_player (0x7abac0) em todo regen jovem: na primeira vaga livre das 50
/// do elenco (+0xd7; sem vaga, 0) a pessoa, o clube dela (+0x39) e +0x4e = 0; o valor (+0x52) pela 0x4d0c60 (clube, 1,
/// −1, 0), o elenco (0x540980 com 1), uma vez em quatro (sorteio(4) = 0) a 0x547d90 (pessoa, clube, 0); clube com +0x64
/// = 3: valor até 5000. O fim do contrato (+0x46): abaixo de 19 anos (+0x18), o ano de hoje + 2; senão + sorteio(6) +
/// 1; no mês do fim da temporada do clube (0x4d45f0) e no dia sorteio(28) + 1 (0x549f70). O contrato (0x4c7950 com o
/// valor); clube humano (0x600a50 com 1, 1): para os reservas (0x4cc0f0) e os bits +0x4f (0x60 em cima) e +0x35 (1 em
/// baixo) do contrato. Com contrato: o salário padrão (0x8626e0), +0x3a = 0 e, fora do humano, o fim (+0x2f) até o ano de
/// hoje + 5 (com o dia +0x2d = 0). No fim os números do elenco ([0xdd7ef0], 0x85add0 e 0x85a470). Devolve 1.
unsafe extern "stdcall" fn player_regen_sign_for_club(pessoa: usize, clube: usize) -> u8 {
    let k = cpv9_vaga_livre(clube);
    if k == 0x32 {
        return 0;
    }
    let gerente = cpv9_d(0xae1050) as usize;
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let f547d90: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547d90usize);
    let mes_do_fim: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x4d45f0usize);
    let data_dma: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> *mut u8 = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(0x417c10usize);
    let contrato_novo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4c7950usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cc0f0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let salario: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8626e0usize);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85add0usize);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    crate::comparacao::ramo("cpv_9 regen contratado");
    cpv9_por(clube + 0xd7 + 4 * k, pessoa as i32);
    cpv9_por(pessoa + 0x39, clube as i32);
    cpv9_por(pessoa + 0x4e, 0);
    let v = valor(gerente, pessoa, clube, 1, -1, 0);
    cpv9_por(pessoa + 0x52, v); // cpv9: o valor do regen
    elenco(pessoa, 1);
    if sorteio(4) == 0 { // cpv9: uma vez em quatro
        crate::comparacao::ramo("cpv_9 uma em quatro");
        f547d90(pessoa, clube, 0);
    }
    if *((clube + 0x64) as *const u8) == 3 && cpv9_d(pessoa + 0x52) > 0x1388 {
        cpv9_por(pessoa + 0x52, 0x1388);
    }
    // ponytail: a original grava o ano na palavra de baixo do argumento `clube` e empurra o dword; a 0x549f70 lê 16 bits
    let ano = *(0xae2c92 as *const u16) as u32;
    let (mut d, mut ret) = ([0u8; 8], [0u8; 8]);
    let dia_e_ano = if *((pessoa + 0x18) as *const i8) < 0x13 {
        crate::comparacao::ramo("cpv_9 contrato do jovem");
        let a = ano.wrapping_add(2); // cpv9: o jovem por dois anos
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    } else {
        let a = (sorteio(6) as u32).wrapping_add(ano).wrapping_add(1);
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    };
    let dia = (dia_e_ano.0 as u8).wrapping_add(1) as i8 as i16 as u16 as u32;
    let p = data_dma(d.as_mut_ptr(), dia, dia_e_ano.1, dia_e_ano.2, 0xffff_ffff);
    copia(pessoa + 0x46, ret.as_mut_ptr(), p);
    contrato_novo(gerente, pessoa, cpv9_d(pessoa + 0x52) as u32);
    if humano(clube, 1, 1, 0, 0) != 0 {
        reservas(cpv9_d(0xae1050) as usize, pessoa, clube);
        let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
        *((c + 0x4f) as *mut u8) = *((c + 0x4f) as *const u8) & 0xf | 0x60;
        let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
        *((c + 0x35) as *mut u8) = *((c + 0x35) as *const u8) & 0xc1 | 1;
    }
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    if c != 0 {
        salario(c, pessoa);
        *((c + 0x3a) as *mut u8) = 0;
        if humano(clube, 1, 1, 0, 0) == 0 {
            let a = (*(0xae2c92 as *const u16)).wrapping_add(5);
            if core::ptr::read_unaligned((c + 0x2f) as *const u16) > a {
                core::ptr::write_unaligned((c + 0x2f) as *mut u16, a);
                core::ptr::write_unaligned((c + 0x2d) as *mut u16, 0);
            }
        }
    }
    if usa_numeros(cpv9_d(0xdd7ef0) as usize, clube, 0) != 0 {
        dar_numero(cpv9_d(0xdd7ef0) as usize, clube, 0);
    }
    1
}

/// Comparador: o contrato novo aloca e o valor sorteia: só a original, conferindo o plano (a vaga, o clube da pessoa, a
/// devolução e o +0x3a do contrato).
unsafe extern "stdcall" fn comparar_player_regen_sign_for_club(pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(%%I0%%);
    let k = cpv9_vaga_livre(clube);
    let r = o(pessoa, clube);
    if k == 0x32 {
        PLACAR.registrar(r == 0, || format!("player_regen_sign_for_club({:x}, {:x}) sem vaga original={}", pessoa, clube, r));
        return r;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    let ok = r == 1 && cpv9_d(clube + 0xd7 + 4 * k) == pessoa as i32 && cpv9_d(pessoa + 0x39) == clube as i32 && (c == 0 || *((c + 0x3a) as *const u8) == 0);
    PLACAR.registrar(ok, || format!("player_regen_sign_for_club({:x}, {:x}) vaga {} original={}", pessoa, clube, k, r));
    r
}

/// { 0x7a7f20 } (thiscall, ret 8, player_regen.cpp, quadro SEH só das datas locais):
/// `PLAYER_REGEN::schedule_injury_retirement(pessoa, dias)`, pela lesão nova (0x62d250) que encerra a carreira: tira a
/// aposentadoria pendente da pessoa (a primeira com o id dela na tabela de 26 bytes [this+4], conta +8; 0x7a8690) e
/// acrescenta (0x7a8510) a nova: id, id do clube (+0x39, ou −1), a data de hoje + sorteio(dias / 2) + 1 (0x54c5b0) em
/// +8 e de novo em +0x10, o motivo 8 (+0x18) e 0 (+0x19).
unsafe extern "thiscall" fn schedule_injury_retirement(this: usize, pessoa: usize, dias: i16) {
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7a8690usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *mut u8 = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(0x417c10usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let n = cpv9_d(this + 8);
    if pessoa != 0 && n > 0 {
        let base = cpv9_d(this + 4) as usize;
        let id = cpv9_d(pessoa);
        if let Some(i) = (0..n as usize).find(|&i| cpv9_d(base + 0x1a * i) == id) {
            crate::comparacao::ramo("cpv_9 tira a aposentadoria pendente");
            tirar(this, base + 0x1a * i);
        }
    }
    let mut r = [0u8; 0x1a];
    let clube = cpv9_d(pessoa + 0x39) as usize;
    let rp = r.as_mut_ptr();
    core::ptr::write_unaligned(rp as *mut i32, cpv9_d(pessoa));
    core::ptr::write_unaligned(rp.add(4) as *mut i32, if clube == 0 { -1 } else { cpv9_d(clube) });
    let q = sorteio(dias as i32 / 2);
    let (mut t, mut t2) = ([0u8; 8], [0u8; 8]);
    let d = mais(0xae2c90, t.as_mut_ptr(), q.wrapping_add(1) as u32);
    copia(rp.add(8), t2.as_mut_ptr(), d);
    copia(rp.add(0x10), t2.as_mut_ptr(), rp.add(8));
    r[0x18] = 8;
    r[0x19] = 0;
    acrescentar(this, r.as_ptr());
}

/// Comparador: lado a lado no estado do sorteio, na tabela (ponteiro, conta e capacidade, e as entradas até a
/// capacidade). Com a tabela cheia (conta = capacidade − 1, a 0x7a8510 realoca): só a original.
unsafe extern "thiscall" fn comparar_schedule_injury_retirement(this: usize, pessoa: usize, dias: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i16)>(%%I1%%);
    let (n, cap) = (cpv9_d(this + 8), cpv9_d(this + 0xc));
    if pessoa == 0 || n >= cap.wrapping_sub(1) || cap <= 0 {
        return o(this, pessoa, dias);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 4, 12));
    reg.push((cpv9_d(this + 4) as usize, cap as usize * 0x1a));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, dias), || crate::aleatorio::sem_contar(|| schedule_injury_retirement(this, pessoa, dias)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("schedule_injury_retirement({:x}, {}) diferencas em {:x?}", pessoa, dias, dif)
    });
}

/// { 0x7aa340 } (stdcall, ret 8, player_regen.cpp, quadro SEH sem destrutor): `player_regen_post_retirement_announcement_news(
/// pessoa, motivo)`, pela PLAYER_REGEN::process_retirements (0x7a59e0): fora do jogador (+0x61) com motivo +0x3d 8 ou
/// 5, a notícia 0xbbb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os nomes (+0x33 de +4, +8 e +0xc), 4 =
/// motivo, 5 = o id do clube do contrato (0x4cf5a0, guardado em [0xae1080]; clube do registro de 0x245 bytes em
/// [0xae23b4] pelo +4; contrato de tipo +0x35 & 0x3f = 5 ou sem clube: −1) e 6 = a idade (+0x18); vai para os técnicos
/// dos até 30 clubes interessados ([0xde1f54], 0x8d1da0 com 10).
unsafe extern "stdcall" fn player_regen_post_retirement_announcement_news(pessoa: usize, motivo: i8) {
    if pessoa == 0 {
        return;
    }
    if cpv9_d(pessoa + 0x61) != 0 && matches!(*((pessoa + 0x3d) as *const u8), 8 | 5) {
        return;
    }
    crate::comparacao::ramo("player_regen_post_retirement_announcement_news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbb, 0);
    gravar(n, 0, cpv9_d(pessoa) as u32);
    for (k, o) in [(1i8, 4usize), (2, 8), (3, 0xc)] {
        gravar(n, k, cpv9_d(cpv9_d(pessoa + o) as usize + 0x33) as u32);
    }
    gravar(n, 4, motivo as i32 as u32);
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    cpv9_por(0xae1080, c as i32);
    let mut clube = -1i32;
    if c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5 {
        let idx = cpv9_d(c + 4) as u32;
        let registro = (cpv9_d(0xae23b4) as u32).wrapping_add(idx.wrapping_mul(0x245)) as usize;
        if registro != 0 {
            clube = cpv9_d(registro);
        }
    }
    gravar(n, 5, clube as u32);
    gravar(n, 6, *((pessoa + 0x18) as *const i8) as i32 as u32);
    let mut lista = [0usize; 0x1e];
    let q = if cpv9_d(0xde1f54) != 0 { interessados(cpv9_d(0xde1f54) as usize, pessoa, 0xa, lista.as_mut_ptr(), 0x1e) } else { 0 };
    for i in 0..(q as i8).max(0) as usize {
        ao_tecnico(cpv9_d(0xb7431c) as usize, n, lista[i]);
    }
}

/// Comparador: a notícia vai para as caixas dos técnicos: só a original (a prova é a carreira e o modo Rust).
unsafe extern "stdcall" fn comparar_player_regen_post_retirement_announcement_news(pessoa: usize, motivo: i8) {
    orig::<unsafe extern "stdcall" fn(usize, i8)>(%%I2%%)(pessoa, motivo)
}
