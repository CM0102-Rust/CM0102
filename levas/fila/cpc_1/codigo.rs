// ---------- %%LEVA%%a leva: a próxima partida pelo filtro, o estádio em uso no dia e a data do amistoso internacional (fix_man.cpp, friendly.cpp) ----------

/// A lista das partidas de (ano, dia, fatia, grupo) no gerenciador `this` (0x5abf10, thiscall ret 0x10, original:
/// carrega o ano se preciso), com o cursor voltado ao começo (+0xa = +2), como as duas originais fazem; nula: 0.
unsafe fn cpc1_lista(this: usize, ano: u32, dia: u32, fatia: u32, grupo: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let l = lista(this, ano, dia, fatia, grupo);
    if l != 0 {
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    }
    l
}

/// O próximo item da lista `l` (0x68d1b0, proximo_da_lista), ou 0.
unsafe fn cpc1_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// A partida `f` passa pelo filtro da find_next_fixture? Cada filtro em -1 deixa passar: a competição (+0), a fase
/// (+0x34, curto com sinal) e os dois clubes (cada um o mandante +0xc ou o visitante +0x10).
unsafe fn cpc1_passa(f: usize, comp: i32, fase: i32, clube_a: i32, clube_b: i32) -> bool {
    let d = |o: usize| core::ptr::read_unaligned((f + o) as *const i32);
    let clube = |c: i32| c == -1 || c == d(0xc) || c == d(0x10);
    (comp == -1 || comp == d(0)) // a competição
        && (fase == -1 || fase == core::ptr::read_unaligned((f + 0x34) as *const i16) as i32)
        && clube(clube_a)
        && clube(clube_b)
}

/// { 0x5ac920 } (thiscall, ret 0x2c, fix_man.cpp): `FIXTURE_MANAGER::find_next_fixture(ano*, dia*, fatia*, adiante,
/// comp, fase, clube_a, clube_b, ...)`: a partida seguinte (ou anterior, `adiante` 0) ao cursor (ano, dia, fatia),
/// o próprio cursor incluído, que passa pelo filtro (cpc1_passa). O grupo das listas vem da 0x48dd70 (a competição
/// da tabela [0xae23d0] + 107 × comp, 0). Adiante: fatias até 2, dias até 365 (0x16e exclusivo), anos seguintes no
/// dia 0 (a fatia só volta a 0 na virada do dia); atrás: fatias até 0, depois 2, dias até 0, anos anteriores no dia
/// 0x16d. A cinco anos do ano do cursor (adiante: com sinal menos sem sinal; atrás: com sinal), 0. Achou: grava o
/// cursor (dia, ano e a fatia em byte) e devolve a partida. Os 3 últimos argumentos não são lidos. A original passa
/// à 0x5abf10 ano, dia e fatia com a metade alta de locais nunca escritas (lixo), mas ela e o que chama (0x5aa2d0,
/// 0x5a9cf0, 0x54cfe0) só usam a palavra/o byte: a cópia passa zero. Nos sorteios de copa (Cup.cpp 0x519d90,
/// 0x51a150, 0x522c70). Sem SEH, sem caixa (a da 0x5abf10 só sem memória).
unsafe extern "thiscall" fn proxima_partida(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, fase: i32, clube_a: i32, clube_b: i32, _a9: u32, _a10: u32, _a11: u32) -> usize {
    let grupo_de: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x48dd70usize);
    let tabela = *(0xae23d0 as *const usize);
    let grupo = grupo_de(tabela.wrapping_add((comp as usize).wrapping_mul(107)), 0) as u8 as u32;
    let (mut ano, mut dia, mut fatia) = (*pano, *pdia, *pfatia as u16);
    let achar = |ano: u16, dia: u16, fatia: u16| -> usize {
        let l = cpc1_lista(this, ano as u32, dia as u32, fatia as u32, grupo);
        if l == 0 {
            return 0;
        }
        loop {
            let f = cpc1_proximo(l);
            if f == 0 || cpc1_passa(f, comp, fase, clube_a, clube_b) {
                return f;
            }
        }
    };
    let f = 'busca: {
        if adiante != 0 {
            loop {
                while (dia as i16) < 0x16e {
                    while (fatia as i16) < 3 {
                        let f = achar(ano, dia, fatia);
                        if f != 0 {
                            break 'busca f;
                        }
                        fatia = fatia.wrapping_add(1);
                    }
                    dia = dia.wrapping_add(1);
                    fatia = 0; // o dia seguinte
                }
                ano = ano.wrapping_add(1);
                dia = 0;
                if (ano as i16 as i32) - (*pano as i32) >= 5 {
                    return 0;
                }
            }
        } else {
            loop {
                while (dia as i16) >= 0 {
                    while (fatia as i16) >= 0 {
                        let f = achar(ano, dia, fatia);
                        if f != 0 {
                            break 'busca f;
                        }
                        fatia = fatia.wrapping_sub(1);
                    }
                    fatia = 2; // o dia anterior
                    dia = dia.wrapping_sub(1);
                }
                ano = ano.wrapping_sub(1);
                dia = 0x16d;
                if (*pano as i16 as i32) - (ano as i16 as i32) >= 5 {
                    return 0;
                }
            }
        }
    };
    *pdia = dia;
    *pano = ano;
    *pfatia = fatia as u8;
    f
}

unsafe extern "thiscall" fn comparar_proxima_partida(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, fase: i32, clube_a: i32, clube_b: i32, a9: u32, a10: u32, a11: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize>(%%I0%%);
    let antes = (*pano, *pdia, *pfatia);
    let reg = [(pano as usize, 2), (pdia as usize, 2), (pfatia as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg,
        || o(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11),
        || proxima_partida(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11));
    PLACAR.registrar(igual && r == n, || format!("proxima_partida(cursor {:?}, adiante {}, comp {}, fase {}, clubes {:x} {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        antes, adiante, comp, fase, clube_a, clube_b, r, n, mo, mr));
    r
}

/// { 0x5ae870 } (thiscall, ret 0x10, fix_man.cpp): `FIXTURE_MANAGER::is_stadium_in_use(estadio, dia, ano, fatia)`:
/// 1 se alguma partida (+0x18 = o estádio, não nulo) das listas do dia usa o estádio. Os grupos: 0 a 7 e o do estádio
/// (0x48df40, byte), este fora se repete um dos oito. Fatia -1 (byte): as fatias 0 a 2, senão só ela. A fatia vai à
/// 0x5abf10 no byte baixo do argumento do ano (a original guarda o laço lá: os 3 bytes altos são os do ano). Estádio
/// nulo: 0. Na marcação dos amistosos (friendly.cpp 0x5c6f00, 0x5c75a0) e em 0x7e6380, 0x7e6720. Sem SEH, sem locais
/// sem valor.
unsafe extern "thiscall" fn estadio_em_uso(this: usize, estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32 {
    if estadio == 0 {
        return 0;
    }
    let grupo_do_estadio: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x48df40usize);
    let mut grupos = [0u8, 1, 2, 3, 4, 5, 6, 7, grupo_do_estadio(estadio) as u8];
    for i in 0..8 {
        for j in i + 1..9 {
            if grupos[i] as i8 >= 0 && grupos[i] == grupos[j] {
                grupos[j] = 0xff;
            }
        }
    }
    let (ini, fim) = if fatia as u8 == 0xff { (0i8, 2i8) } else { (fatia as i8, fatia as i8) };
    for &g in &grupos {
        if g == 0xff {
            continue;
        }
        let mut s = ini;
        while s <= fim {
            let l = cpc1_lista(this, ano, dia, (ano & 0xffff_ff00) | s as u8 as u32, g as u32);
            if l != 0 {
                loop {
                    let f = cpc1_proximo(l);
                    if f == 0 {
                        break;
                    }
                    let e = core::ptr::read_unaligned((f + 0x18) as *const usize);
                    if e != 0 && e == estadio { // o estádio da partida
                        return 1;
                    }
                }
            }
            s = s.wrapping_add(1);
        }
    }
    0
}

comparar_valor!(comparar_estadio_em_uso, estadio_em_uso, %%I1%%, (estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32);

/// { 0x5cbdd0 } (thiscall, ret 4, friendly.cpp): `friendly_international_date_resolve(ano)`: a data (+3) da entrada
/// de data internacional `this` pela 0x54a130 (Date.cpp: dia, mês, ano, dia da semana) com o dia (+1) e o dia da
/// semana (+0) em byte com sinal e o mês (+2). A original monta os argumentos sobre o eax e o edx de entrada: o edx é o
/// ano (metade alta do dia), o eax é o ano também no único chamador (0x5cbc18, `push eax; call`; metade alta do mês e
/// do dia da semana). O eax devolvido é lixo: o endereço de pilha do temporário que a 0x417c10 copia no fim da 0x54a2d0
/// (muda com a profundidade da pilha), e o chamador o descarta (0x5cbc1d `mov eax, [esp+14h]`); só a data (+3..+0xb) conta.
unsafe extern "thiscall" fn data_do_amistoso_internacional(this: usize, ano: u32) -> u32 {
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x54a130usize);
    let b = |o: usize| *((this + o) as *const i8) as i16 as u16 as u32;
    let alto = ano & 0xffff_0000;
    let semana = alto | b(0);
    let mes = alto | (b(0) & 0xff00) | *((this + 2) as *const u8) as u32;
    acertar(this + 3, alto | b(1), mes, ano, semana) // a data com o ano dado
}

/// A original (ret 4) com eax = ano, como no único chamador.
unsafe fn cpc1_data_original(o: usize, this: usize, ano: u32) -> u32 {
    let r: u32;
    core::arch::asm!(
        "push {a}",
        "call {f}",
        a = in(reg) ano,
        f = in(reg) o,
        inout("eax") ano => r,
        inout("ecx") this => _,
        clobber_abi("C"),
    );
    r
}

unsafe extern "thiscall" fn comparar_data_do_amistoso_internacional(this: usize, ano: u32) -> u32 {
    let o = orig::<usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 11)], || cpc1_data_original(o, this, ano),
        || data_do_amistoso_internacional(this, ano));
    PLACAR.registrar(igual /* o eax é lixo de pilha: só a memória */, || format!("data_do_amistoso_internacional({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        this, ano, r, n, mo, mr));
    r
}
