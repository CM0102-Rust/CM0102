// ---------- %%LEVA%%a leva: a lista de observação do clube: o estatuto no elenco que o clube dá à pessoa, a estimativa do clube humano e soltar os alvos que não são do clube (shortlist_manager.cpp) ----------

unsafe fn cpn6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do contrato `c` (id em +4; o registro de clube tem 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
unsafe fn shortlist_manager_club_of_contract(c: usize) -> usize {
    if c == 0 {
        return 0;
    }
    cpn6_d(0xae23b4).wrapping_add(cpn6_d(c + 4).wrapping_mul(0x245))
}

/// { 0x849f00 } (thiscall, ret 0xc, shortlist_manager.cpp): `Shortlist_Manager::get_squad_status_for_club(pessoa,
/// clube, cargo)`: o estatuto no elenco que o `clube` dá à `pessoa`. Clube nulo: 7. A entrada da pessoa na lista do
/// clube (0x849ce0: a de tipo ≠ 11 para jogador, +0x61; tendo +0x69, a de tipo 11 também): o cargo (+0 da entrada) e
/// os bits 1..4 do +0x24. Senão o cargo é o +0x3d da pessoa e vale o contrato (0x4cf5a0, guardado em [0xae1080]): do
/// próprio clube, o nibble alto do +0x4f do contrato com o clube (0x4cf5f0; sem ele, 7); clube humano (0x600a50) com
/// cargo 11 a 15 (0x53b620): a estimativa 0x849d60; senão, o clube da pessoa (+0x39) sendo o clube ou o parceiro dele
/// (0x540a50) leva ao contrato com o clube; nenhum: 7.
unsafe extern "thiscall" fn get_squad_status_for_club(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    if clube == 0 {
        return 7;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let mut e = entrada(this, clube, pessoa, (cpn6_d(pessoa + 0x61) == 0) as u32, 0);
    if e == 0 && cpn6_d(pessoa + 0x69) != 0 {
        e = entrada(this, clube, pessoa, 1, 0);
    }
    if e != 0 {
        if !cargo.is_null() {
            *cargo = cpn6_b(e);
        }
        crate::comparacao::ramo("cpn_6 estatuto pela lista");
        return (cpn6_b(e + 0x24) >> 1) & 0xf; // os bits 1..4 da entrada
    }
    if !cargo.is_null() {
        *cargo = cpn6_b(pessoa + 0x3d);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpn6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let pelo_contrato = if shortlist_manager_club_of_contract(c) == clube {
        true
    } else {
        // ponytail: o 3º argumento é empurrado como dword 1; o cargo vai num byte (a 0x53b620 só lê o byte)
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        let type_11_to_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
        if humano(clube, 1, 1, 0, 0) != 0 && type_11_to_15(cpn6_b(pessoa + 0x3d) as i8) != 0 {
            let estimar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(0x849d60usize);
            return estimar(pessoa, clube) as u8;
        }
        cpn6_d(pessoa + 0x39) == clube || {
            let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
            cpn6_d(pessoa + 0x39) == parceiro(clube, 0, 1)
        }
    };
    if !pelo_contrato {
        return 7;
    }
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = com_clube(cpn6_d(0xae1050), pessoa, clube);
    if k == 0 {
        return 7;
    }
    crate::comparacao::ramo("cpn_6 estatuto pelo contrato");
    cpn6_b(k + 0x4f) >> 4 // o nibble alto do contrato
}

/// { 0x849d60 } (stdcall, ret 8, shortlist_manager.cpp): `shortlist_manager_estimate_squad_status(pessoa, clube)`: o
/// estatuto que o clube humano daria. A reputação da pessoa vista pelo técnico do clube (0x53ca90, 0x5407e0; até 1:
/// 1, senão as duas chamadas de novo) e a do clube (+0x80, no mínimo 1); pessoa na lista do humano (0x7f7490 do
/// [0xb74448]): a reputação passa a ser o maior de 1,5 × ela e 0,75 × a do clube (truncado, 16 bits). A razão (limitada
/// a 0,1..2): acima de 1,5: 1; o float dela acima de 1,2 com o clube abaixo de 8500: 2; até 1,2 com a reputação até
/// 6500: 5 (acima de 0,6 e idade +0x18 abaixo de 23), 4 (24 ou mais) ou 6; senão 3. Só roda para o clube humano.
unsafe extern "stdcall" fn shortlist_manager_estimate_squad_status(pessoa: usize, clube: usize) -> u32 {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let injury_word_0c: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(0x5407e0usize);
    let mut rep = if injury_word_0c(pessoa, tecnico(clube), 0) as i16 > 1 { injury_word_0c(pessoa, tecnico(clube), 0) as i16 } else { 1 };
    let mut rc = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    if rc <= 1 {
        rc = 1;
    }
    let t = tecnico(clube);
    let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
    if in_list(cpn6_d(0xb74448), pessoa, t) != 0 {
        let a = rep as f64 * read_f64_at(0x968940);
        let b = rc as f64 * read_f64_at(0x969040);
        rep = match_pl_ftol(if a > b { a } else { b }) as i16;
    }
    let mut q = rep as f64 / rc as f64;
    if q < read_f64_at(0x967880) {
        q = read_f64_at(0x967880);
    } else if !(q <= read_f64_at(0x968950)) {
        q = read_f64_at(0x968950);
    }
    let f = q as f32 as f64; // a original guarda a razão num float e compara o resto por ele
    if q > read_f64_at(0x968940) {
        return 1;
    }
    if f > read_f64_at(0x968978) && rc < 0x2134 {
        return 2;
    }
    let idade = cpn6_b(pessoa + 0x18) as i8;
    if !(f > read_f64_at(0x968978)) && rep <= 0x1964 {
        if f > read_f64_at(0x968e48) && idade < 0x17 {
            return 5;
        }
        return if idade >= 0x18 { 4 } else { 6 };
    }
    3
}

/// { 0x844660 } (thiscall, ret 8, shortlist_manager.cpp): `shortlist_manager_release_club_slot_targets(clube,
/// entrada)`: nas 40 vagas de 0x25 bytes de [entrada], o alvo (+0xf) acima do último id de pessoa ([0xae2364] − 1)
/// vira −1; o alvo válido cujo contrato (0x4cf5a0, guardado em [0xae1080]; o de tipo 5 em +0x35 não conta) não é do
/// clube, nem o clube dele (+0x39) é o clube, e com o +0x17 da vaga em −1: o evento 3 da lista (0x843e10) com o tipo
/// "não é 11" do +0 da vaga (no byte baixo do endereço da vaga, como a original).
unsafe extern "thiscall" fn shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entrada: usize) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x843e10usize);
    for k in 0..40usize {
        let o = k * 0x25;
        let v = cpn6_d(entrada).wrapping_add(o + 0xf);
        if cpn6_d(v) as i32 > (*(0xae2364 as *const i32)).wrapping_sub(1) {
            core::ptr::write_unaligned(v as *mut i32, -1);
        }
        let id = cpn6_d(cpn6_d(entrada).wrapping_add(o + 0xf)) as i32;
        if id <= -1 {
            continue;
        }
        let p = cpn6_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let c = contract_of(cpn6_d(0xae1050), p);
        *(0xae1080 as *mut usize) = c;
        let do_contrato = if c != 0 && cpn6_b(c + 0x35) & 0x3f != 5 { shortlist_manager_club_of_contract(c) } else { 0 };
        if do_contrato == clube || cpn6_d(p + 0x39) == clube {
            continue;
        }
        let s = cpn6_d(entrada).wrapping_add(o);
        if cpn6_d(s + 0x17) as i32 != -1 {
            continue;
        }
        // setne al sobre o endereço da vaga: os 24 bits altos são os dele
        let tipo = ((s & !0xff) | (cpn6_b(s) != 0xb) as usize) as u32;
        crate::comparacao::ramo("cpn_6 alvo fora do clube");
        atualizar(this, p, clube, 3, tipo);
    }
}

/// Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
unsafe extern "thiscall" fn comparar_get_squad_status_for_club(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(%%I0%%);
    let saida = if cargo.is_null() { 0xae1080 } else { cargo as usize };
    let reg = [(0xae1080usize, 4usize), (saida, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, cargo), || get_squad_status_for_club(this, pessoa, clube, cargo));
    PLACAR.registrar(igual && r == n, || format!("get_squad_status_for_club({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
unsafe extern "stdcall" fn comparar_shortlist_manager_estimate_squad_status(pessoa: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(%%I1%%)(pessoa, clube);
    let n = shortlist_manager_estimate_squad_status(pessoa, clube);
    PLACAR.registrar(r as u8 == n as u8, || format!("shortlist_manager_estimate_squad_status({:#x}, {:#x}) original={:#x} rust={}", pessoa, clube, r, n));
    r
}

/// Comparador: só a original.
// ponytail: a 0x843e10 muda a lista do clube (e outras) a cada evento e não pode rodar duas vezes; a cópia tem o ramo
// e a prova é pela carreira (modo Rust contra a original).
unsafe extern "thiscall" fn comparar_shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entrada: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, clube, entrada)
}
