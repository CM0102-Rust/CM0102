// ---------- %%LEVA%%a leva: pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpp6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpp6_clube(id: usize) -> usize {
    cpp6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0 no CONTRACT_MANAGER [0xae1050]).
unsafe fn get_contract(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    contract_of(cpp6_d(0xae1050), pessoa)
}

/// O contrato da pessoa, guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn shortlist_manager_get_contract_and_store(pessoa: usize) -> usize {
    let c = get_contract(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// { 0x842c00 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_player_to_club_shortlists(jogador,
/// forçar)`: com ficha de posições (+0x61; sem forçar, nota +0xb ≥ 0x157c), fora da marca 0x10 dos bits 0x30 do registro
/// de 79 bytes (+0xb), jogador (+0x3d = 0xb), não em fim de carreira (+0x24 com +0x28 = 5), fora da nação [0x9cf458]
/// (+0x1a) e sem ser intocável (0x8cca10 = 0): se chegou ao clube há menos de 200 dias (hoje − 200, 0x54c8e0, antes da
/// data +0x3e/+0x40) e o contrato não está à venda (+0x4f nibble alto ≠ 7) nem tem +0x45, sai. Senão, com o clube do
/// contrato (guardado em [0xae1080]), o contador (nibble baixo do +0xb do registro de 79 bytes) e a máscara de
/// posições (0x5b7d20 com os vizinhos: 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400), percorre os clubes do vetor
/// [0xae2d1c] ([0xae235c] relido a cada volta) até o contador chegar a 5: fora seleção (0x53b900), o clube do contrato e
/// o do jogador (+0x39), tenta a 0x842e10 e conta cada clube que o pôs na lista.
unsafe extern "thiscall" fn add_player_to_club_shortlists(this: usize, jogador: usize, forcar: u32) {
    let fichas = cpp6_d(jogador + 0x61);
    if fichas == 0 {
        return;
    }
    if forcar == 0 && core::ptr::read_unaligned((fichas + 0xb) as *const i16) < 0x157c {
        return;
    }
    let registro = |j: usize| cpp6_d(0xae2d0c).wrapping_add(cpp6_d(j).wrapping_mul(0x4f));
    if cpp6_b(registro(jogador) + 0xb) & 0x30 == 0x10 {
        return;
    }
    if cpp6_b(jogador + 0x3d) != 0xb {
        return;
    }
    if cpp6_d(jogador + 0x24) != 0 && cpp6_b(jogador + 0x28) == 5 {
        return;
    }
    let nacao = cpp6_d(jogador + 0x1a);
    if nacao != 0 && cpp6_d(nacao) == cpp6_d(0x9cf458) {
        return;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    if cpp6_d(jogador + 0x39) != 0 {
        let menos: unsafe extern "thiscall" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let mut t = [0u32; 2];
        let p = menos(0xae2c90, t.as_mut_ptr(), 200);
        let lo = cpp6_d(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let ano_j = core::ptr::read_unaligned((jogador + 0x40) as *const u16);
        let dia_j = core::ptr::read_unaligned((jogador + 0x3e) as *const i16);
        let recente = if ano != ano_j { ano < ano_j } else { dia < dia_j };
        if recente {
            let c = get_contract(jogador);
            if c != 0 && cpp6_b(c + 0x4f) & 0xf0 != 0x70 && cpp6_d(c + 0x45) == 0 {
                return;
            }
        }
    }
    let c = shortlist_manager_get_contract_and_store(jogador);
    let transfer_manager_contract_club = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let mut conta = (cpp6_b(registro(jogador) + 0xb) & 0xf) as i8;
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    // ponytail: a 0x5b7d20 devolve só ax e a 0x842e10 usa 16 bits da máscara: os 16 de cima vão zerados
    let mut m = mascara(cpp6_d(jogador + 0x61)) as u32;
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 {
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x80 != 0 {
        m |= 0x100;
    }
    if m & 0x800 != 0 {
        m |= 0x400;
    }
    if *(0xae235c as *const i32) <= 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x842e10usize);
    let clubes = cpp6_d(0xae2d1c);
    let mut i = 0i32;
    loop {
        let clube = cpp6_clube(cpp6_d(clubes + i as usize * 4));
        if conta >= 5 {
            return;
        }
        if selecao(clube) == 0 && transfer_manager_contract_club != clube && cpp6_d(jogador + 0x39) != clube && acrescentar(this, jogador, clube, m) != 0 {
            crate::comparison::branch("cpp_6 clube acrescentou");
            conta = conta.wrapping_add(1); // mais um clube com ele na lista
        }
        i += 1;
        if i >= *(0xae235c as *const i32) {
            return;
        }
    }
}

/// { 0x842320 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::transfer_list_player(jogador, notícia)`:
/// se o clube ainda não o listou (0x5444a0 com 0) e há contrato: marca o contrato à venda (+0x4e bit 1, +0x4f nibble
/// alto 7); se não é intocável (0x8cca10): com o jogador gostando do clube (0x547d60) ou com a lealdade (+0x44 do
/// contrato, com sinal) acima da diferença de dois sorteios(100), o jogador passa a desgostar do técnico do clube
/// (0x547af0 com o +0xcf do clube, 1); com `notícia`, põe o jogador nas listas dos clubes (0x842c00 com 1) e dá a
/// notícia (0x4d71b0 com o clube +0x39).
unsafe extern "thiscall" fn transfer_list_player(this: usize, jogador: usize, noticia: u32) {
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if listada(jogador, 0) != 0 {
        return;
    }
    let c = get_contract(jogador);
    if c == 0 {
        return;
    }
    let a = cpp6_b(c + 0x4f);
    let b = cpp6_b(c + 0x4e);
    *((c + 0x4e) as *mut u8) = b | 1;
    crate::comparison::branch("cpp_6 por a venda");
    *((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // à venda
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let desgostar = if gosta(jogador, cpp6_d(jogador + 0x39)) != 0 {
        true
    } else {
        let lealdade = cpp6_b(c + 0x44) as i8 as i32;
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let s1 = sorteio(100);
        let s2 = sorteio(100);
        lealdade > s1.wrapping_sub(s2)
    };
    if desgostar {
        let antipatia: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        antipatia(jogador, cpp6_d(cpp6_d(jogador + 0x39) + 0xcf), 1);
    }
    if noticia != 0 {
        let nas_listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
        let dar_noticia: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d71b0usize);
        nas_listas(this, jogador, 1);
        dar_noticia(cpp6_d(0xae1050), jogador, cpp6_d(jogador + 0x39));
    }
}

/// O registro de prêmio por jogos (0xd bytes: +0 id da pessoa, +4 valor, +8 jogos, +9 id do clube antigo) da pessoa,
/// pela busca binária (0x946464 com 0x8b4e00) no vetor das seleções (tipo 5: +0x898, conta +0x8a0) ou do clube
/// (+0x88c, conta +0x894) do TRANSFER_MANAGER.
unsafe fn appearance_bonus_record(this: usize, jogador: usize, tipo: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpp6_d(jogador) as u32];
    let (vetor, conta) = if tipo as u8 == 5 { (cpp6_d(this + 0x898), cpp6_d(this + 0x8a0)) } else { (cpp6_d(this + 0x88c), cpp6_d(this + 0x894)) };
    bsearch(chave.as_ptr(), vetor, conta, 0xd, 0x8b4e00)
}

/// { 0x8cb340 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::pay_appearance_money(jogador, jogos,
/// tipo, arg)`: com contrato (guardado em [0xae1080]) e o registro do prêmio da pessoa (o `tipo` vai inteiro, como a
/// original o empurra) cujos jogos (+8, com sinal) são os `jogos` (> 0): dá a notícia (0x8db8a0), soma o valor (+4)
/// aos totais de transferência (0x448ad0 em [0xad9dac], com o clube atual e o antigo), o clube antigo (+9) recebe
/// (0x599800) e o atual paga (0x599890), em 64 bits com sinal. Devolve 1 se pagou.
unsafe extern "thiscall" fn pay_appearance_money(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let c = shortlist_manager_get_contract_and_store(jogador);
    if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        return 0;
    }
    let r = appearance_bonus_record(this, jogador, tipo);
    if r == 0 {
        return 0;
    }
    // a original: jogos diferentes só seguem com −1, que cai no teste seguinte (≤ 0)
    if cpp6_b(r + 8) as i8 as i32 != jogos || jogos <= 0 {
        return 0;
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(0x8db8a0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let receber: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599800usize);
    let pagar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599890usize);
    noticia(this, r, jogos, tipo, arg);
    let f_antigo = financas(cpp6_d(0xae2a38), cpp6_clube(cpp6_d(r + 9)));
    let c = shortlist_manager_get_contract_and_store(jogador);
    let f_atual = financas(cpp6_d(0xae2a38), if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 });
    let c = shortlist_manager_get_contract_and_store(jogador);
    let atual = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let valor = core::ptr::read_unaligned((r + 4) as *const i32);
    somar(cpp6_d(0xad9dac), valor, atual, cpp6_clube(cpp6_d(r + 9)));
    crate::comparison::branch("cpp_6 premio por jogos");
    receber(f_antigo, valor as u32, (valor >> 31) as u32); // o clube antigo recebe
    pagar(f_atual, valor as u32, (valor >> 31) as u32);
    1
}

/// Comparador: a 0x842e10 mexe nas vagas de muitos clubes (e as listas não cabem numa cópia barata): roda só a
/// original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_add_player_to_club_shortlists(this: usize, jogador: usize, forcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, jogador, forcar)
}

/// Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
unsafe extern "thiscall" fn compare_transfer_list_player(this: usize, jogador: usize, noticia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let marca = listada(jogador, 0) == 0 && get_contract(jogador) != 0;
    o(this, jogador, noticia);
    if marca {
        let c = get_contract(jogador);
        let ok = c != 0 && cpp6_b(c + 0x4e) & 1 != 0 && cpp6_b(c + 0x4f) & 0xf0 == 0x70;
        SCOREBOARD.record(ok, || format!("transfer_list_player(jogador {:x}) contrato {:x} sem a marca de à venda", cpp6_d(jogador), c));
    }
}

/// Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
unsafe extern "thiscall" fn compare_pay_appearance_money(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(%%I2%%);
    let c = get_contract(jogador);
    let esperado = if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        0
    } else {
        let r = appearance_bonus_record(this, jogador, tipo);
        (r != 0 && cpp6_b(r + 8) as i8 as i32 == jogos && jogos > 0) as u32
    };
    let r = o(this, jogador, jogos, tipo, arg);
    SCOREBOARD.record(r == esperado, || format!("pay_appearance_money(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", cpp6_d(jogador), jogos, tipo as u8, r, esperado));
    r
}
