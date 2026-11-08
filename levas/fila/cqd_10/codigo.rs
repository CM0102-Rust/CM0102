// ---------- %%LEVA%%a leva: as aposentadorias do dia: as anunciadas e as que vencem, a notícia da aposentadoria e a do jovem promovido (player_regen.cpp) ----------

unsafe fn cqd10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd10_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqd10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O byte das marcas da aposentadoria da pessoa `id` (registros de 79 bytes em [0xae2d0c], +0xb).
unsafe fn retirement_flags_byte(id: i32) -> *mut u8 {
    cqd10_d(0xae2d0c).wrapping_add((id as isize * 0x4f + 0xb) as usize) as *mut u8
}

/// O contrato da pessoa (0x4cf5a0 no gerente [0xae1050]), sem guardar em [0xae1080] (a 0x7a59e0 não guarda).
unsafe fn contract_of_uncached(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cqd10_d(0xae1050), p)
}

/// PLAYER_REGEN::remove_retirement (0x7a8690): tira a aposentadoria pendente (as seguintes sobem um lugar).
unsafe fn player_regen_remove_retirement(this: usize, item: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7a8690usize);
    f(this, item);
}

/// Uma rotina thiscall(gerente [g], pessoa).
unsafe fn call_on_global_manager(rotina: usize, g: usize, p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(cqd10_d(g), p);
}

/// O atributo do técnico que vira o jogador aposentado: min(max(P, X) × k, teto), truncado (as contas x87 da original
/// em f64; P e X inteiros).
fn coach_attribute_capped(pv: f64, x: f64, k: f64, teto: f64) -> u16 {
    let m = if pv > x { pv } else { x };
    let v = m * k;
    match_pl_ftol(if v < teto { v } else { teto }) as u16
}

/// Os três atributos do técnico (+0xa, +8, +0xc do registro de não jogador `nj`) pelo jogador `p` (ficha `f`): a
/// os jogos pela seleção (+0x22, byte) ao quadrado (pow 0x945640/0x945ff0) contra +0xb, +9 e +0xd da ficha (o último com o quadrado ×
/// 0,75), × `k` e até os tetos.
unsafe fn set_coach_attributes(p: usize, f: usize, nj: usize, k: f64, tetos: [f64; 3]) {
    let r = *((p + 0x22) as *const u8) as f64;
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + 0xa) as *mut u16, coach_attribute_capped(q, cqd10_w(f + 0xb) as f64, k, tetos[0]));
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + 8) as *mut u16, coach_attribute_capped(q, cqd10_w(f + 9) as f64, k, tetos[1]));
    let q = game_pow(r, 2.0) * read_f64_at(0x969040);
    core::ptr::write_unaligned((nj + 0xc) as *mut u16, coach_attribute_capped(q, cqd10_w(f + 0xd) as f64, k, tetos[2]));
}

/// O jogador aposentado sem registro de não jogador (+0x69) pode virar técnico: sorteio(+0x5b) acima de sorteio(100);
/// sorteio(ftol(+0xb da ficha + reputação³)) acima de sorteio(50000), ou (+0x8e da nação +0x1a) × reputação acima de
/// sorteio(200000) + 200000; com mais de 2 fichas reservadas (this +0x20): a primeira (this +0x1c; registros de 0x44
/// bytes em [0xae23c0]) vira o +0x69 e sai da reserva (0x7a82b0); goleiro (+0xf da ficha 20): +0x10/+0x11 trocados
/// quando +0x10 é maior e, com sorteio(4) nulo, +0x11 = sorteio(50) + 50; a normalização (0x539490) e os atributos do
/// técnico (0x9/7500 com +0xb da ficha acima de 7500; senão, com sorteio(+0x57) ≠ 0, 0,75/6000/3750; senão
/// 0,5/4500/1).
unsafe fn maybe_become_coach(this: usize, p: usize) {
    let r = *((p + 0x22) as *const u8);
    let ra = cqd10_sortear(cqd10_b(p + 0x5b) as i32);
    if ra <= cqd10_sortear(100) {
        return;
    }
    let f = cqd10_d(p + 0x61);
    let t = match_pl_ftol(cqd10_w(f + 0xb) as f64 + game_pow(r as f64, 3.0));
    let r1 = cqd10_sortear(t);
    let passa = r1 > cqd10_sortear(50000) || { // cqd10: o sorteio de virar técnico
        let r3 = cqd10_sortear(0x30d40).wrapping_add(0x30d40);
        let v = (cqd10_w(cqd10_d(p + 0x1a) + 0x8e) as i32).wrapping_mul(*((p + 0x22) as *const u8) as i32);
        v > r3
    };
    if !passa || cqd10_i(this + 0x20) <= 2 {
        return;
    }
    crate::comparison::branch("cqd_10 vira tecnico");
    let id = cqd10_i(cqd10_d(this + 0x1c));
    core::ptr::write_unaligned((p + 0x69) as *mut u32, cqd10_d(0xae23c0).wrapping_add((id as isize * 0x44) as usize) as u32);
    let reserva: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a82b0usize);
    reserva(this, cqd10_i(cqd10_d(this + 0x1c)));
    if cqd10_b(cqd10_d(p + 0x61) + 0xf) == 0x14 {
        let nj = cqd10_d(p + 0x69);
        let (a, b) = (cqd10_b(nj + 0x10), cqd10_b(nj + 0x11));
        if a > b {
            *((nj + 0x11) as *mut i8) = a;
            *((cqd10_d(p + 0x69) + 0x10) as *mut i8) = b;
            if cqd10_sortear(4) == 0 {
                let v = (cqd10_sortear(0x32) as u8).wrapping_add(0x32);
                *((cqd10_d(p + 0x69) + 0x11) as *mut u8) = v;
            }
        }
    }
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(0x539490usize);
    normalizar(p);
    let f = cqd10_d(p + 0x61);
    let nj = cqd10_d(p + 0x69);
    if cqd10_w(f + 0xb) > 0x1d4c {
        set_coach_attributes(p, f, nj, read_f64_at(0x9689d8), [read_f64_at(0x9690b8), read_f64_at(0x9690b8), read_f64_at(0x96e298)]);
    } else if cqd10_sortear(cqd10_b(p + 0x57) as i32) != 0 {
        set_coach_attributes(p, f, nj, read_f64_at(0x969040), [read_f64_at(0x96d300), read_f64_at(0x96d300), read_f64_at(0x96e290)]);
    } else {
        set_coach_attributes(p, f, nj, read_f64_at(0x968968), [read_f64_at(0x96e288), read_f64_at(0x96e288), read_f64_at(0x967890)]);
    }
}

/// A aposentadoria que venceu (o item `item` da pessoa `p`).
unsafe fn process_due_retirement(this: usize, item: usize, p: usize, vaga: &mut [u32; 3]) {
    let id = cqd10_i(item);
    if cqd10_b(p + 0x18) < 0x1e && cqd10_b(item + 0x18) != 8 // cqd10: o jovem não se aposenta
        && (cqd10_d(p + 0x39) != 0 || *retirement_flags_byte(cqd10_i(p)) & 0xf != 0) {
        crate::comparison::branch("cqd_10 aposentadoria desfeita");
        player_regen_remove_retirement(this, item);
        return;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    if ficha(cqd10_d(0xae1050), p) != 0 || oferta(cqd10_d(0xde1f54), p, 0) != 0 {
        *retirement_flags_byte(id) &= 0xcf;
        player_regen_remove_retirement(this, item);
        return;
    }
    let antigo = cqd10_i(item + 4);
    let clube = cqd10_d(p + 0x39);
    let mudou = if antigo != -1 { clube != 0 && cqd10_i(clube) != antigo } else { clube != 0 };
    let mut acertar = true;
    if mudou {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cqd10_d(p + 0x3e) as u32, cqd10_d(p + 0x42) as u32) < 0xd2 && cqd10_b(p + 0x18) < 0x26 {
            acertar = false;
            let c = contract_of_uncached(p);
            let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
            if c != 0 && listado(p, 0) == 0 && cqd10_d(c + 0x45) == 0 {
                crate::comparison::branch("cqd_10 mudou de clube e fica");
                *retirement_flags_byte(id) &= 0xcf;
                player_regen_remove_retirement(this, item);
                return;
            }
        }
    }
    if acertar && cqd10_d(p + 0x39) == 0 && cqd10_i(item + 4) != -1 {
        core::ptr::write_unaligned((item + 4) as *mut i32, -1);
    }
    // ponytail: a original empurra o motivo e a marca de anunciada em registradores com bytes de cima que sobram; a
    // 0x7a92e0 lê os bytes
    let noticia: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(0x7a92e0usize);
    noticia(p, *((item + 0x18) as *const u8) as u32, *((item + 0x19) as *const u8) as u32);
    let ofertas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ccbf0usize);
    ofertas(cqd10_d(0xae1050), p);
    if cqd10_d(p + 0x61) != 0 && cqd10_d(p + 0x69) != 0 {
        if contract_of_uncached(p) != 0 {
            let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x85f800usize);
            limpar(contract_of_uncached(p));
        }
    } else {
        crate::comparison::branch("cqd_10 tira o contrato");
        call_on_global_manager(0x4d0b50, 0xae1050, p); // cqd10: tira o contrato
    }
    call_on_global_manager(0x775ba0, 0xb74318, p);
    let sm = cqd10_d(cqd10_d(0xde1f54) + 0x856);
    if sm != 0 {
        let c = cqd10_d(p + 0x39);
        if c != 0 {
            let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
            situacao(sm, p, c, 4, 2);
        }
        let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
        suspender(sm, p, 0);
        let c = cqd10_d(p + 0x39);
        if c != 0 {
            *(cqd10_d(sm + 0xdc).wrapping_add((cqd10_i(c) as isize * 9 + 8) as usize) as *mut u8) = 100;
        }
    }
    let mut banco = false;
    if cqd10_d(p + 0x61) != 0 {
        let elenco: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x7aace0usize);
        crate::comparison::branch("cqd_10 sai do elenco");
        elenco(p); // cqd10: sai do elenco
        if cqd10_d(p + 0x39) == 0 {
            call_on_global_manager(0x4d0b50, 0xae1050, p);
        }
        call_on_global_manager(0x4196a0, 0xad9c6c, p);
        call_on_global_manager(0x4197c0, 0xad9c6c, p);
        call_on_global_manager(0x56a090, 0xae2a30, p);
        if cqd10_d(p + 0x61) != 0 {
            if cqd10_d(p + 0x69) != 0 {
                banco = true;
            } else {
                maybe_become_coach(this, p);
            }
        }
    }
    if !banco {
        banco = cqd10_d(p + 0x69) != 0 && cqd10_d(p + 0x61) != 0;
    }
    if banco {
        // o jogador com registro de não jogador: a ficha vai para o banco dos regens e ele segue como não jogador
        crate::comparison::branch("cqd_10 jogador para o banco");
        call_on_global_manager(0x7c6650, 0xb74350, p);
        call_on_global_manager(0x55e1d0, 0xae28f0, p);
        call_on_global_manager(0x631f90, 0xb63d28, p);
        let guardar: unsafe extern "thiscall" fn(usize, i32, u32, u32) = core::mem::transmute(0x7a8350usize);
        let pais = cqd10_d(p + 0x1a);
        if pais == 0 {
            let n = cqd10_sortear(cqd10_i(0xae2350));
            guardar(this, cqd10_i(p), n as u32, u32::MAX);
        } else if cqd10_d(p + 0x1e) == 0 {
            guardar(this, cqd10_i(p), cqd10_d(pais) as u32, u32::MAX);
        } else {
            guardar(this, cqd10_i(p), cqd10_d(pais) as u32, cqd10_d(cqd10_d(p + 0x1e)) as u32);
        }
        core::ptr::write_unaligned((p + 0x61) as *mut u32, 0);
        *retirement_flags_byte(cqd10_i(item)) &= 0xcf;
        *((p + 0x5f) as *mut u8) = 1;
        if cqd10_d(p + 0x39) != 0 {
            let c = contract_of_uncached(p);
            if c != 0 {
                let inicial: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85e960usize);
                inicial(c, p, 0);
            }
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    // a vaga para o regen: hoje (até 29 anos) ou de 1 a 65 dias depois (se passar do dia 365, sorteio(35) do ano
    // seguinte)
    vaga[0] = cqd10_i(item) as u32;
    let hoje = cqd10_d(0xae2c90) as u32;
    let (dia, ano) = if cqd10_b(p + 0x18) < 0x1e {
        (hoje as u16, cqd10_w(0xae2c92) as u16)
    } else {
        let v = (cqd10_sortear(0x41) as u32).wrapping_add(hoje).wrapping_add(1) as u16; // cqd10: o dia do regen
        crate::comparison::branch("cqd_10 dia do regen sorteado");
        if v as i16 <= 0x16d {
            (v, cqd10_w(0xae2c92) as u16)
        } else {
            (cqd10_sortear(0x23) as u16, (cqd10_w(0xae2c92) as u16).wrapping_add(1))
        }
    };
    vaga[1] = (vaga[1] & 0xffff_0000) | dia as u32;
    vaga[1] = (vaga[1] & 0xffff) | (ano as u32) << 16;
    let m = retirement_flags_byte(cqd10_i(item));
    *m = (*m & 0xef) | 0x20;
    let guardar_vaga: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(0x7a87d0usize);
    let nj = cqd10_d(p + 0x69);
    if nj != 0 {
        let limpar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86fc20usize);
        limpar(cqd10_d(0xdd7f00), nj);
        let c = cqd10_d(p + 0x39);
        let cargo = |p: usize| cqd10_b(p + 0x3d);
        let sair = if c == 0 || cargo(p) == 5 {
            true
        } else {
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(c, 1, 1, 0, 0) != 0 && !matches!(cargo(p), 1 | 3 | 2 | 4) {
                true
            } else if cqd10_sortear(4) != 0 {
                false
            } else {
                !matches!(cargo(p), 1 | 3 | 2 | 4 | 0xa | 9)
            }
        };
        if sair {
            let cargos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x7aaeb0usize);
            cargos(p);
            guardar_vaga(this, vaga.as_ptr());
        } else {
            crate::comparison::branch("cqd_10 refeito como nao jogador");
            let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7ab340usize);
            refazer(this, p);
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    guardar_vaga(this, vaga.as_ptr());
    player_regen_remove_retirement(this, item);
}

/// { 0x7a59e0 } (thiscall, ret 0, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `PLAYER_REGEN::process_retirements()`, todo dia pelo laço do dia. A vaga local para o regen (id e a data padrão
/// 0x54a110, que deixa o bissexto 1 nela); ordena as aposentadorias pendentes ([this+4], [this+8] de 26 bytes: +0 a
/// pessoa, +4 o clube ou −1, +8/+0xa o dia e o ano do anúncio, +0x10/+0x12 os da aposentadoria, +0x18 o motivo, +0x19
/// anunciada) com 0x7ae2a0 e anda por elas (o ponteiro só avança no anúncio: a 0x7a8690 puxa as seguintes). Vencida
/// (a data passou): o jovem (até 29) com clube ou marca, fora do motivo 8, desiste; com ficha de contrato (0x4cf6d0)
/// ou sob oferta (0x8cde70), desiste (marcas & 0xcf); quem mudou de clube há menos de 210 dias, até 37 anos, com
/// contrato, fora da lista e sem +0x45, fica; senão a notícia (0x7a92e0), as ofertas (0x4ccbf0), o contrato, a seleção
/// (0x775ba0), as listas de observação, o elenco (0x7aace0), as faltas (0x4196a0, 0x4197c0) e a disputa (0x56a090); o
/// jogador pode virar técnico; com ficha e registro de não jogador, a ficha vai para o banco (0x7a8350) e o contrato
/// novo (0x85e960); senão a vaga para o regen (0x7a87d0) e, com registro de não jogador, sai dos cargos (0x7aaeb0) ou
/// é refeito como não jogador (0x7ab340). Não vencida: com a data do anúncio passada e sem anúncio, a notícia do
/// anúncio (0x7aa340, se as marcas & 0x30 não forem 0x10), a marca de anunciada e as marcas 0x10; senão para. No fim
/// ordena as vagas para o regen ([this+0x10], [this+0x14] de 12 bytes) com 0x7ae240.
unsafe extern "thiscall" fn process_retirements(this: usize) {
    let mut vaga = [0u32; 3]; // a pessoa e a data (dia, ano, bissexto)
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(vaga.as_mut_ptr() as usize + 4);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqd10_d(this + 4), cqd10_d(this + 8) as u32, 0x1a, 0x7ae2a0);
    let n = cqd10_i(this + 8);
    let mut item = cqd10_d(this + 4);
    let mut i = 0i32;
    while i < n {
        let hoje = cqd10_d(0xae2c90) as u32;
        let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
        let (vdia, vano) = (cqd10_w(item + 0x10), cqd10_w(item + 0x12) as u16);
        if ano > vano || (ano == vano && dia > vdia) {
            let p = cqd10_d(0xae23bc).wrapping_add((cqd10_i(item) as isize * 0x6e) as usize);
            process_due_retirement(this, item, p, &mut vaga);
        } else {
            if *((item + 0x19) as *const u8) != 0 {
                break;
            }
            let (adia, aano) = (cqd10_w(item + 8), cqd10_w(item + 0xa) as u16);
            if !(ano > aano || (ano == aano && dia > adia)) {
                break;
            }
            crate::comparison::branch("cqd_10 aposentadoria anunciada");
            let id = cqd10_i(item);
            if *retirement_flags_byte(id) & 0x30 != 0x10 {
                let anunciar: unsafe extern "stdcall" fn(usize, u32) = core::mem::transmute(0x7aa340usize);
                // ponytail: a original empurra o motivo no edx com bytes de cima que sobram; a 0x7aa340 lê o byte
                anunciar(cqd10_d(0xae23bc).wrapping_add((id as isize * 0x6e) as usize), *((item + 0x18) as *const u8) as u32);
            }
            let id = cqd10_i(item);
            *((item + 0x19) as *mut u8) = 1;
            let m = retirement_flags_byte(id);
            *m = (*m & 0xdf) | 0x10;
            item += 0x1a;
        }
        i += 1;
    }
    qsort(cqd10_d(this + 0x10), cqd10_d(this + 0x14) as u32, 0xc, 0x7ae240);
}

// ponytail: chama as notícias (alocam e entregam), o regen como não jogador, os contratos novos e as listas: o caminho
// só se sabe rodando e não pode rodar duas vezes. Só a original; a cópia leva os ramos e a carreira a prova.
unsafe extern "thiscall" fn compare_process_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x7a92e0 } (stdcall, ret 0xc, player_regen.cpp; a 0x7a59e0 põe o this no ecx, sem uso; quadro SEH sem destrutor
/// no caminho normal): `player_regen_post_retirement_news(pessoa, motivo, anunciada)`. Sem pessoa, jogador com cargo 8
/// ou 5, sem clube ou clube sem técnico humano (0x7875e0): nada. A notícia 0xbba (local): 0..3 a pessoa (id e os +0x33
/// de +4/+8/+0xc), 4 o motivo, 5 anunciada, 6 o clube (−1 sem), 7 a nação (+0x1a) com os jogos pela seleção (+0x22) ou −1, 8 o
/// cargo (0xf vira 8, 0xd vira 6), 9 a reputação, 0xa +0x28, 0xb ficha e registro de não jogador, 0xc a idade, 0xd o
/// registro de não jogador, 0xe a ficha. Os interessados (0x8d1da0 em [0xde1f54] com 0xa, até 30, só para jogador).
/// Nação de peso (+0x85 acima de 8) e estrela (+0xd da ficha acima de 8500, ou técnico de cargo 5 com +0xc acima de
/// 8750): a todos (0x786780); estrela menor (+0xd acima de 7500, ou cargo 5 com +0xa acima de 7250): aos técnicos da
/// nação e da do clube (0x7863f0); técnico de cargo 5 de clube com divisão: aos da divisão (0x7862d0); não jogador: ao
/// clube (0x7861e0); jogador: aos interessados. Motivo 8 com +0xb da ficha acima de 4250 e clube de contrato
/// (0x4cf5a0, guardado em [0xae1080]): o seguro (0x58a980 com +0x52).
unsafe extern "stdcall" fn player_regen_post_retirement_news(p: usize, motivo: u32, anunciada: u32) {
    if p == 0 {
        return;
    }
    if cqd10_d(p + 0x61) != 0 && matches!(cqd10_b(p + 0x3d), 8 | 5) {
        return;
    }
    let clube = cqd10_d(p + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(clube) == 0 {
        return;
    }
    crate::comparison::branch("cqd_10 noticia da aposentadoria");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0xbba, 0);
    gravar(nw, 0, cqd10_d(p) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, cqd10_d(cqd10_d(p + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, motivo as u8 as i8 as i32 as u32);
    gravar(nw, 5, anunciada as u8 as i8 as i32 as u32);
    let c = cqd10_d(p + 0x39);
    gravar(nw, 6, if c != 0 { cqd10_d(c) as u32 } else { u32::MAX });
    gravar(nw, 7, if *((p + 0x22) as *const u8) != 0 { cqd10_d(cqd10_d(p + 0x1a)) as u32 } else { u32::MAX });
    let cargo = match cqd10_b(p + 0x3d) {
        0xf => 8,
        0xd => 6,
        v => v,
    };
    gravar(nw, 8, cargo as i32 as u32);
    gravar(nw, 9, *((p + 0x22) as *const u8) as u32);
    gravar(nw, 0xa, cqd10_b(p + 0x28) as i32 as u32);
    gravar(nw, 0xb, (cqd10_d(p + 0x61) != 0 && cqd10_d(p + 0x69) != 0) as u32);
    gravar(nw, 0xc, cqd10_b(p + 0x18) as i32 as u32);
    gravar(nw, 0xd, (cqd10_d(p + 0x69) != 0) as u32);
    gravar(nw, 0xe, (cqd10_d(p + 0x61) != 0) as u32);
    let mut clubes = [0usize; 30];
    let tm = cqd10_d(0xde1f54);
    let n = if tm != 0 && cqd10_d(p + 0x61) != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
        interessados(tm, p, 0xa, clubes.as_mut_ptr(), 0x1e) as i8
    } else {
        0
    };
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let da_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mn = cqd10_d(0xb7431c);
    let pais = cqd10_d(p + 0x1a);
    let f = cqd10_d(p + 0x61);
    let nj = cqd10_d(p + 0x69);
    let cargo5 = || cqd10_b(p + 0x3d) == 5;
    if pais != 0 && cqd10_b(pais + 0x85) > 8
        && ((f != 0 && cqd10_w(f + 0xd) > 0x2134) || (nj != 0 && cqd10_w(nj + 0xc) > 0x222e && cargo5())) {
        todos(mn, nw);
    } else if (f != 0 && cqd10_w(f + 0xd) > 0x1d4c) || (nj != 0 && cqd10_w(nj + 0xa) > 0x1c52 && cargo5()) {
        do_pais(mn, nw, pais);
        let c = cqd10_d(p + 0x39);
        if c != 0 && cqd10_d(c + 0x53) != cqd10_d(p + 0x1a) {
            do_pais(cqd10_d(0xb7431c), nw, cqd10_d(c + 0x53));
        }
    } else {
        let c = cqd10_d(p + 0x39);
        if c != 0 && cqd10_d(c + 0x57) != 0 && cargo5() {
            da_divisao(mn, nw, cqd10_d(c + 0x57));
        } else if f == 0 {
            ao_clube(mn, nw, c);
        } else if n > 0 {
            for k in 0..n as usize {
                ao_clube(cqd10_d(0xb7431c), nw, clubes[k]);
            }
        }
    }
    if motivo as u8 == 8 {
        let f = cqd10_d(p + 0x61);
        if f != 0 && cqd10_w(f + 0xb) > 0x109a {
            let c = contract_of_uncached(p);
            *(0xae1080 as *mut usize) = c;
            if c != 0 {
                let clube = cqd10_d(0xae23b4).wrapping_add((cqd10_i(c + 4) as isize * 0x245) as usize);
                if clube != 0 {
                    crate::comparison::branch("cqd_10 seguro da aposentadoria");
                    let seguro: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x58a980usize);
                    seguro(clube, p, cqd10_d(p + 0x52) as u32);
                }
            }
        }
    }
}

// ponytail: aloca e entrega a notícia (0x77bbe0, 0x786780...). Só a original; a cópia leva o ramo.
unsafe extern "stdcall" fn compare_player_regen_post_retirement_news(p: usize, motivo: u32, anunciada: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(%%I1%%)(p, motivo, anunciada)
}

/// { 0x7a8ba0 } (stdcall, ret 8, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `player_regen_post_youth_promotion_news(jogador, clube)`, pelo regen jovem (0x7abac0). Sem jogador ou clube: nada.
/// A notícia 0xbce (local): 0..3 o jogador (id e os +0x33 de +4/+8/+0xc), 4 o id do clube, 5 a idade, 6 a força do
/// clube (+0x7f) acima de 15. O auxiliar (+0xd3) e os cinco treinadores (+0x19f) opinam até o primeiro que der
/// veredito: a reputação vista = sorteio(2500) + reputação do clube (+0x80) / 2 + força × 12,5 (truncada); as duas
/// habilidades vistas (0x6e86d0 com 2 e com 1) contra reputação / 50 + 75 (veredito 2) e reputação / 100 + 50 (1).
/// Com veredito: 7 auxiliar?, 8..0xb quem (id e nomes), 0xc e 0xd os vereditos; senão −1. Ao alcance do clube
/// (0x786090).
unsafe extern "stdcall" fn player_regen_post_youth_promotion_news(j: usize, c: usize) {
    if j == 0 || c == 0 {
        return;
    }
    crate::comparison::branch("cqd_10 noticia do jovem promovido");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let vista: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    let (mut v1, mut v2) = (0i8, 0i8);
    let mut quem = 0usize;
    construir(nw, 0xbce, 0);
    gravar(nw, 0, cqd10_d(j) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, cqd10_d(cqd10_d(j + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, cqd10_d(c) as u32);
    gravar(nw, 5, cqd10_b(j + 0x18) as i32 as u32);
    gravar(nw, 6, (cqd10_b(c + 0x7f) > 0xf) as u32);
    let mut atual = 0usize;
    let mut k = -1i8;
    loop {
        if atual != 0 {
            break;
        }
        let t = if k == -1 { cqd10_d(c + 0xd3) } else { cqd10_d(c + 0x19f + 4 * k as usize) };
        if t != 0 {
            let r = cqd10_sortear(0x9c4);
            let v = r.wrapping_add(cqd10_w(c + 0x80) as i32 / 2);
            let x = match_pl_ftol(v as f64 + cqd10_b(c + 0x7f) as f64 * read_f64_at(0x96afc8));
            // ponytail: a original empurra o eax inteiro do _ftol e o da primeira vista; a 0x6e86d0 lê a palavra
            let a = vista(t, j, c, 2, x as i16);
            let b = vista(t, j, c, 1, a);
            let rep = cqd10_w(c + 0x80) as i32;
            let alto = rep / 50 + 0x4b;
            if a as i32 > alto {
                v1 = 2;
                quem = t;
            } else if a as i32 > rep / 100 + 0x32 {
                v1 = 1;
                quem = t;
            }
            if b as i32 > alto {
                v2 = 2;
                quem = t;
            } else if b as i32 > rep / 100 + 0x32 {
                v2 = 1;
                quem = t;
            }
            atual = quem;
        }
        k += 1;
        if k >= 5 {
            break;
        }
    }
    if atual != 0 {
        gravar(nw, 7, (atual == cqd10_d(c + 0xd3)) as u32);
        gravar(nw, 8, cqd10_d(atual) as u32);
        for k in 1..4usize {
            gravar(nw, 8 + k as i8, cqd10_d(cqd10_d(atual + 4 * k) + 0x33) as u32);
        }
        gravar(nw, 0xc, v1 as i32 as u32);
        gravar(nw, 0xd, v2 as i32 as u32);
    } else {
        for k in 7..14i8 {
            gravar(nw, k, u32::MAX);
        }
    }
    let alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    alcance(cqd10_d(0xb7431c), nw, c, 0, 0, 0, 0);
}

// ponytail: aloca e entrega a notícia e sorteia a opinião: só a original; a cópia leva o ramo.
unsafe extern "stdcall" fn compare_player_regen_post_youth_promotion_news(j: usize, c: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize)>(%%I2%%)(j, c)
}
