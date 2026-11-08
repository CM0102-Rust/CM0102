// ---------- %%LEVA%%a leva: o calendário dos jogos: achar os jogos de um clube ou competição entre duas datas, os jogos da competição num dia e se a competição tem jogo na data (fix_man.cpp) ----------

unsafe fn cpr12_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A lista dos jogos de um ano, dia, fatia e nação-chave no calendário `fm` (0x5abf10; cria o bloco do ano se a
/// corrente não o tem). A original compara o ano como palavra e mascara o dia, a fatia e a nação.
unsafe fn fix_man_get_fixture_list(fm: usize, ano: u32, dia: u32, fatia: u32, nacao: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    lista(fm, ano, dia, fatia, nacao)
}

/// O próximo item da lista (0x68d1b0: o cursor +0xa anda).
unsafe fn fix_man_list_next(l: usize) -> usize {
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    prox(l)
}

/// Põe o cursor da lista no começo (+0xa = +2) e devolve o primeiro item.
unsafe fn list_first(l: usize) -> usize {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpr12_d(l + 2));
    fix_man_list_next(l)
}

/// Acrescenta à lista `l` uma cópia dos 0x4f bytes do jogo `f` (0x68d0d0); 0 se falhou.
unsafe fn list_append_fixture(l: usize, f: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(l, f, 0x4f)
}

/// A nação-chave ativa da competição `comp` (0x48dd70 sobre o registro [0xae23d0] + 0x6b × comp, sem jogo).
unsafe fn comp_key_nation(comp: u32) -> u8 {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x48dd70usize);
    nacao((cpr12_d(0xae23d0).wrapping_add(comp.wrapping_mul(0x6b))) as usize, 0) as u8
}

/// O jogo `f` passa nos filtros da 0x5acbe0: tipo (+0x42; 0xfe qualquer), clube em casa (+0xc) ou fora (+0x10; −1
/// qualquer) com o lado dele (+0x40 em casa, +0x41 fora; 0xff qualquer) e a competição (+0; −1 qualquer).
unsafe fn fixture_matches_filter(f: usize, clube: u32, comp: u32, tipo: u8, lado: u8) -> bool {
    if tipo != 0xfe && tipo != *((f + 0x42) as *const u8) {
        return false;
    }
    if clube != u32::MAX {
        let casa = cpr12_d(f + 0xc);
        if clube != casa && clube != cpr12_d(f + 0x10) {
            return false;
        }
        if lado != 0xff {
            let o_dele = if clube == casa { *((f + 0x40) as *const u8) } else { *((f + 0x41) as *const u8) };
            if o_dele != lado {
                return false;
            }
        }
    }
    comp == u32::MAX || comp == cpr12_d(f)
}

/// { 0x5acbe0 } (thiscall, ret 0x24, fix_man.cpp): `FIXTURE_MANAGER::find_fixtures(dia0, ano0, dia1, ano1, clube, lista,
/// comp, tipo, lado)`: conta (e, com `lista`, copia para ela) os jogos do calendário entre (dia0, ano0) e (dia1, ano1)
/// inclusive, dia a dia (366 por ano), nas três fatias de cada dia e nas listas das nações-chave: do clube (índice em
/// [0xae23b4], 0x245 bytes; seleção 0x53b900: as nações 0 e 1; senão 2..7, a da 0x48dee0 e a parceira 0x5af5b0, sem
/// repetir) ou só a da competição (0x48dd70). Filtros: tipo (0xfe qualquer), lado do clube (0xff qualquer) e
/// competição (−1 qualquer). Devolve a conta (palavra); a cópia que falha mostra a caixa (linha 0x93b) e para ali.
/// Quente (288 mil chamadas até 2002-06-01): sem contador.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                      lista: usize, comp: u32, tipo: u32, lado: u32) -> u32 {
    let (tipo, lado) = (tipo as u8, lado as u8);
    let mut nacoes = [0xffu8; 8];
    let mut n = 0usize;
    if clube != u32::MAX {
        let reg = cpr12_d(0xae23b4).wrapping_add(clube.wrapping_mul(0x245)) as usize;
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(reg) != 0 {
            nacoes[0] = 0;
            nacoes[1] = 1;
            n = 2;
        } else {
            let do_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x48dee0usize);
            let parceira: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5af5b0usize);
            nacoes[..6].copy_from_slice(&[2, 3, 4, 5, 6, 7]);
            nacoes[6] = do_clube(reg) as u8;
            nacoes[7] = parceira(reg) as u8;
            n = 8;
            for i in 0..7 {
                if (i + 1..8).any(|j| nacoes[j] == nacoes[i]) {
                    nacoes[i] = 0xff; // repetida adiante
                }
            }
        }
    } else if (comp as i32) > -1 {
        nacoes[0] = comp_key_nation(comp);
        n = 1;
    }
    let (mut dia, mut ano, mut total) = (dia0, ano0, 0u32);
    loop {
        if dia as u16 >= 0x16e {
            ano = ano.wrapping_add(1); // o ano seguinte, do dia 0
            dia = 0;
            continue;
        }
        if ano as u16 > ano1 as u16 || (ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16) {
            return total & 0xffff; // passou do último dia
        }
        for fatia in 0..3u32 {
            for &nacao in &nacoes[..n] {
                if nacao == 0xff {
                    continue;
                }
                let l = fix_man_get_fixture_list(this, ano, dia, fatia, nacao as u32);
                if l == 0 {
                    continue;
                }
                let mut f = list_first(l);
                while f != 0 {
                    if fixture_matches_filter(f, clube, comp, tipo, lado) {
                        if lista != 0 && list_append_fixture(lista, f) == 0 {
                            show_error_at(0x9cc208, 0x93b);
                            return total & 0xffff;
                        }
                        total = total.wrapping_add(1);
                    }
                    f = fix_man_list_next(l);
                }
            }
        }
        dia = dia.wrapping_add(1);
    }
}

/// { 0x5abd90 } (thiscall, ret 0x1c, fix_man.cpp): `FIXTURE_MANAGER::collect_comp_fixtures_on_day(ano, dia, fatia,
/// comp, rodada, tipo, lista)` (do adiamento pela TV das ligas e da mudança das rodadas das copas para as datas fixas):
/// copia para `lista` os jogos do dia e fatia, na lista da nação-chave da competição (0x48dd70; −1: a nação 0), da
/// competição (+0; −1 qualquer), da rodada (+0x34, palavra; −1 qualquer) e do tipo (+0x42; 0xfe qualquer). Devolve
/// quantos (palavra); a cópia que falha mostra a caixa (linha 0x634) e para ali.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn collect_comp_fixtures_on_day(this: usize, ano: u32, dia: u32, fatia: u32, comp: u32, rodada: u32,
                                                       tipo: u32, lista: usize) -> u32 {
    let tipo = tipo as u8;
    // ponytail: a original empurra o local da nação com bytes altos de sobra de pilha; a 0x5abf10 só lê o byte
    let nacao = if comp != u32::MAX { comp_key_nation(comp) } else { 0 };
    let mut total = 0u32;
    let l = fix_man_get_fixture_list(this, ano, dia, fatia, nacao as u32);
    if l == 0 {
        return 0;
    }
    let mut f = list_first(l);
    while f != 0 {
        let mut ok = true;
        if comp != u32::MAX && comp != cpr12_d(f) {
            ok = false;
        }
        if rodada != u32::MAX && rodada as i32 != *((f + 0x34) as *const i16) as i32 {
            ok = false; // outra rodada
        }
        if (tipo == 0xfe || *((f + 0x42) as *const u8) == tipo) && ok {
            if list_append_fixture(lista, f) == 0 {
                show_error_at(0x9cc208, 0x634);
                return total & 0xffff;
            }
            crate::comparison::branch("cpr_12 round match");
            total = total.wrapping_add(1);
        }
        f = fix_man_list_next(l);
    }
    total & 0xffff
}

/// { 0x5ae6d0 } (stdcall, ret 8, fix_man.cpp; o `ecx` = [0xae2a58] que o chamador põe não é usado):
/// `fix_man_comp_has_fixture_on_date(data, comp)` (das eliminatórias da Eurocopa 0x57fd60): 1 se a competição tem
/// jogo (+0 = comp) na data em alguma das três fatias do calendário [0xae2a58], na lista da nação-chave dela
/// (0x48dd70). Competição −1 ou abaixo: 0; data nula: a caixa (linha 0xe89) e 0.
unsafe extern "stdcall" fn fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    if (comp as i32) <= -1 {
        return 0;
    }
    let nacao = comp_key_nation(comp);
    if data == 0 {
        show_error_at(0x9cc208, 0xe89);
        return 0;
    }
    let fm = cpr12_d(0xae2a58) as usize;
    // ponytail: a original empurra o ano e o dia (palavras) com bytes altos de sobra nos registradores; a 0x5abf10
    // compara o ano como palavra e mascara o dia (o dword do ano só segue se o bloco do ano faltar, e os dois anos da
    // carreira estão na corrente desde a criação do jogo)
    let (ano, dia) = (*((data + 2) as *const u16) as u32, *(data as *const u16) as u32);
    for fatia in 0..3u32 {
        let l = fix_man_get_fixture_list(fm, ano, dia, fatia, nacao as u32);
        if l == 0 {
            continue;
        }
        let mut f = list_first(l);
        while f != 0 {
            if cpr12_d(f) == comp {
                return 1;
            }
            f = fix_man_list_next(l);
        }
    }
    0
}

/// Os itens que a original acrescentou à `lista` (depois da cauda de antes) são os da lista provisória `nova`, na
/// mesma ordem, 0x4f bytes cada?
unsafe fn appended_items_match(lista: usize, conta_antes: u16, cauda_antes: u32, nova: usize) -> bool {
    let acrescentados = (*(lista as *const u16)).wrapping_sub(conta_antes);
    if acrescentados != *(nova as *const u16) {
        return false;
    }
    let mut a = if conta_antes == 0 { cpr12_d(lista + 2) } else { cpr12_d(cauda_antes as usize + 6) } as usize;
    let mut b = cpr12_d(nova + 2) as usize;
    for _ in 0..acrescentados {
        if a == 0 || b == 0 {
            return false;
        }
        let (da, db) = (cpr12_d(a) as usize, cpr12_d(b) as usize);
        if core::slice::from_raw_parts(da as *const u8, 0x4f) != core::slice::from_raw_parts(db as *const u8, 0x4f) {
            return false;
        }
        a = cpr12_d(a + 6) as usize;
        b = cpr12_d(b + 6) as usize;
    }
    true
}

/// Roda a cópia com uma lista provisória (0x68d020; destruída pela 0x68d040) e confere os itens contra os que a
/// original acrescentou à lista real.
unsafe fn run_with_temp_list(lista: usize, conta_antes: u16, cauda_antes: u32, copia: impl FnOnce(usize) -> u32) -> (u32, bool) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut nova = [0u8; 0x10];
    let p = nova.as_mut_ptr() as usize;
    criar(p);
    let n = copia(p);
    let iguais = appended_items_match(lista, conta_antes, cauda_antes, p);
    destruir(p);
    (n, iguais)
}

/// Comparador: sem lista, pela devolução (a original e a cópia só andam os cursores das listas do calendário). Com
/// lista, a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                               lista: usize, comp: u32, tipo: u32, lado: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32>(%%I0%%);
    if lista == 0 {
        let r = o(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        let n = find_fixtures(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        SCOREBOARD.record(r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}) original={} rust={}",
            dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16));
        return r;
    }
    let (conta, cauda) = (*(lista as *const u16), cpr12_d(lista + 6));
    let r = o(this, dia0, ano0, dia1, ano1, clube, lista, comp, tipo, lado);
    let (n, iguais) = run_with_temp_list(lista, conta, cauda,
        |p| find_fixtures(this, dia0, ano0, dia1, ano1, clube, p, comp, tipo, lado));
    SCOREBOARD.record(iguais && r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}, com lista) original={} rust={} itens iguais {}",
        dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16, iguais));
    r
}

/// Comparador: a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_collect_comp_fixtures_on_day(this: usize, ano: u32, dia: u32, fatia: u32, comp: u32, rodada: u32,
                                                                tipo: u32, lista: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32>(%%I1%%);
    let (conta, cauda) = (*(lista as *const u16), cpr12_d(lista + 6));
    let r = o(this, ano, dia, fatia, comp, rodada, tipo, lista);
    let (n, iguais) = run_with_temp_list(lista, conta, cauda,
        |p| collect_comp_fixtures_on_day(this, ano, dia, fatia, comp, rodada, tipo, p));
    SCOREBOARD.record(iguais && r as u16 == n as u16, || format!("collect_comp_fixtures_on_day({}/{} fatia {}, comp {}, rodada {}, tipo {:x}) original={} rust={} itens iguais {}",
        dia as u16, ano as u16, fatia as u8, comp as i32, rodada as i32, tipo as u8, r as u16, n as u16, iguais));
    r
}

/// Comparador: pela devolução (só anda os cursores das listas do calendário).
unsafe extern "stdcall" fn compare_fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(%%I2%%)(data, comp);
    let n = fix_man_comp_has_fixture_on_date(data, comp);
    SCOREBOARD.record(r == n, || format!("fix_man_comp_has_fixture_on_date(data {:x}, comp {}) original={} rust={}", data, comp as i32, r, n));
    r
}
