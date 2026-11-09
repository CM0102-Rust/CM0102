// ---------- %%LEVA%%a leva: a cópia do contrato de pessoal, a moral depois da boa partida e tirar as conversas de contrato (contract_manager.cpp) ----------

unsafe fn cph3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cph3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
unsafe extern "thiscall" fn contract_manager_operator(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, cph3_d(origem + o));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = cph3_b(origem + o);
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // os cinco bytes
    }
    d(0x21);
    cph3_data(this + 0x25, origem + 0x25);
    cph3_data(this + 0x2d, origem + 0x2d);
    let m = (this + 0x35) as *mut u8;
    let s = cph3_b(origem + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // os 6 bits baixos
    *m = cl;
    let al = ((((cph3_b(origem + 0x35) << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = cph3_b(origem + 0x35);
    *m = ((al ^ s) & 0x7f) ^ s; // o 8º bit
    d(0x36);
    b(0x3a);
    w(0x3b);
    b(0x3d);
    b(0x3e);
    b(0x3f);
    d(0x40);
    b(0x44);
    d(0x45);
    b(0x49);
    d(0x4a);
    b(0x4e);
    let m = (this + 0x4f) as *mut u8;
    let c = *m;
    let al = ((cph3_b(origem + 0x4f) ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = cph3_b(origem + 0x4f);
    *m = ((al ^ s) & 0xf) ^ s;
    this
}

/// { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
unsafe extern "stdcall" fn contract_manager_raise_match_morale(pessoa: usize, clube: usize, contrato: usize) {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return;
    }
    let jogador = cph3_d(pessoa + 0x61) as usize;
    if jogador == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((jogador + 0xb) as *const i16) as i32 - core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let x = (a as f64 + cph3_b(pessoa + 0x18) as i8 as f64 * read_f64(0x968d90)) - read_f64(0x968d88);
    let mut y = (if x < read_f64(0x9689a0) { x * read_f64(0x968d80) } else { x * read_f64(0x968d78) }) + read_f64(0x967890);
    if y < read_f64(0x967898) {
        y = read_f64(0x967898);
    } else if y > read_f64(0x968950) {
        y = read_f64(0x968950);
    }
    let felicidade = (cph3_b(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut ganho = match_pl_ftol(felicidade / y) as u8 as i8;
    if ganho > 10 {
        ganho = 10;
    }
    let queixas = cph3_d(contrato + 0x45);
    let mut teto: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        teto = teto.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if teto < -50 {
            break;
        }
        if queixas & q != 0 {
            teto = teto.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let moral = (contrato + 0x44) as *mut i8;
    let cl = *moral;
    let s = cl as i32 + ganho as i32;
    *moral = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(ganho) };
    if *moral > teto {
        *moral = teto;
    }
    if cph3_b(contrato + 0x49) == 0 {
        let apaga = if ganho > 5 && *moral as i32 + ganho as i32 > 0 {
            true
        } else {
            cph3_b(contrato + 0x3d) as i8 as i32 >= 75 - ganho as i32
        };
        if apaga {
            let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
            if queixas & 4 != 0 {
                apagar(contrato, 4);
            }
            if cph3_b(contrato + 0x45) & 0x10 != 0 {
                apagar(contrato, 0x10);
            }
        }
    }
    let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
    retirada(contrato, 0);
}

/// { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
unsafe extern "thiscall" fn remove_talks_records(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let n = (this + 0x2c) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = cph3_d(this + 0x28) as usize;
            let r = base.wrapping_add(off);
            let casa = cph3_d(r) == cph3_d(pessoa)
                && (clube == 0 || cph3_d(r + 4) == cph3_d(clube)) // o clube
                && (tecnico == 0 || cph3_d(r + 8) == cph3_d(tecnico));
            if casa {
                let ultimo = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(ultimo as *const u8, r as *mut u8, 21);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(21);
            }
            i += 1;
            off = off.wrapping_add(21);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cph3_d(this + 0x28) as usize, *n, 0x15, 0x4dc970);
    }
}

unsafe extern "thiscall" fn compare_contract_manager_operator(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x50)], || o(this, origem), || contract_manager_operator(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("contract_manager_operator {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (153.222 chamadas); comparar pedia separar a moral das chamadas do fim.
unsafe extern "stdcall" fn compare_contract_manager_raise_match_morale(pessoa: usize, clube: usize, contrato: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, usize)>(%%I1%%)(pessoa, clube, contrato)
}

unsafe extern "thiscall" fn compare_remove_talks_records(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I2%%);
    let n = (*((this + 0x2c) as *const i32)).max(0) as usize;
    let reg = [(this + 0x28, 8), (cph3_d(this + 0x28) as usize, n * 21)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, pessoa, tecnico, clube), || remove_talks_records(this, pessoa, tecnico, clube));
    SCOREBOARD.record(igual, || format!("remove_talks_records({:x}, {:x}, {:x}) {:x?} x {:x?}", pessoa, tecnico, clube, mo, mr));
}
