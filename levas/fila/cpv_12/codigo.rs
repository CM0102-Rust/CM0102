// ---------- %%LEVA%%a leva: a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso (national_teams.cpp, friendly.cpp) ----------

unsafe fn cpv12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv12_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O nome do time (0x542100: time, marca de segundo time, destino de 0x47 bytes, 2, 1) e o byte de gramática (+0x52).
unsafe fn cpv12_nome_do_time(time: usize, segundo: u32, buf: &mut [u8; 0x48]) -> i32 {
    let nome: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(0x542100usize);
    nome(time, segundo, buf.as_mut_ptr(), 0x47, 2, 1);
    cpv12_b(time + 0x52) as i8 as i32
}

/// { 0x76db00 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH com a lista local da busca de jogos):
/// `national_teams_can_change_squad(nacao, tipo, p, entrando, msg, tam)`: o jogador (se houver) aposentado da seleção
/// (0x547d30) ou de clube de outra nação não muda (0). O time do tipo (0 e 1 a principal, o 1 com a marca 8 e o segundo
/// time; senão o sub-21 0x53b4f0, marca 4). Entrando: elenco cheio (0x53c590 contra +0x42 do registro do tipo) ou
/// jogador machucado (0x62c400 = 2) não muda. Pode mudar (1): entrando com vagas abertas (+0x45), o tipo sem janela
/// (+0x44 = 0), a data do tipo (+0x3a) já passada ou, no próprio dia, um jogo do time hoje (0x5acee0 numa lista local);
/// nos tipos 1 e 2 com técnico humano que decide (0x600a50), também quando a lista [this] não tem o time com data
/// passada e ele não deixa (0x5fd6b0). Os motivos vão para `msg` (0x66f4e0). Sem nação, ou `msg` com menos de 0x32
/// bytes: a original (caixa e 1).
unsafe extern "thiscall" fn national_teams_can_change_squad(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(%%I0%%)(this, nacao, tipo, p, entrando, msg, tam);
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let aposentado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    if p != 0 {
        if aposentado(p, principal(cpv12_d(p + 0x1a))) != 0 { // cpv12: aposentado
            crate::comparacao::ramo("cpv_12 aposentado");
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                texto(msg, 0xa5b4c8, nome.as_ptr());
            }
            return 0;
        }
        let c = cpv12_d(p + 0x24);
        if c != 0 && cpv12_d(c + 0x53) != nacao {
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                let outra = cpv12_d(cpv12_d(p + 0x24) + 0x53);
                let g = cpv12_b(outra + 0x52) as i8 as i32;
                texto(msg, 0xa5b460, g, g, nome.as_ptr(), outra + 0x38);
            }
            return 0;
        }
    }
    let s = tipo as u8;
    // a marca vai na local [esp+0x1c] só no byte de baixo (o resto é sobra de pilha); a 0x53c590 lê só o byte
    let (time, segundo, marca) = match s {
        0 => (principal(nacao), 0u32, 4u32),
        1 => (principal(nacao), 1, 8),
        _ => (b_national_team(nacao), 0, 4),
    };
    let mut tecnico = cpv12_d(principal(nacao) + 0xcf);
    if humano(principal(nacao), 0, 1, 0, 0) == 0 {
        tecnico = 0;
    }
    let rec = cpv12_d(this + 4).wrapping_add((cpv12_d(nacao) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let teto = cpv12_b(e + 0x42) as i8;
        if (contar(time, marca, 0) as i8) >= teto {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = cpv12_nome_do_time(time, segundo, &mut buf);
                texto(msg, 0xa5b3e0, g, g, teto as i32, buf.as_ptr());
            }
            return 0;
        }
        if estado(cpv12_d(0xb63d28), p) == 2 {
            if msg != 0 {
                texto(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if cpv12_b(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if cpv12_b(e + 0x44) == 0 {
        crate::comparacao::ramo("cpv_12 sem janela");
        return 1; // cpv12: sem janela
    }
    let hoje = cpv12_d(0xae2c90) as u32;
    let (ano, dia) = ((hoje >> 16) as u16, hoje as u16 as i16);
    if s != 0 && tecnico != 0 {
        let mut passou = false;
        let lista = cpv12_d(this);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpv12_d(lista + 2) as u32);
            let mut it = proximo(cpv12_d(this));
            while it != 0 {
                if cpv12_d(it) == cpv12_d(time) {
                    let d = cpv12_d(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passou = true;
                        break;
                    }
                }
                it = proximo(cpv12_d(this));
            }
        }
        if !passou && t9a06(cpv12_d(0xb63c98), tecnico, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = cpv12_d(e + 0x3a) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        crate::comparacao::ramo("cpv_12 data passada");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        crate::comparacao::ramo("cpv_12 jogo hoje");
        let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(0x5acee0usize);
        // as vagas da data que a busca lê (a palavra do ano, o dword do dia, o byte da fatia), como a original monta
        let mut v_ano: u32 = ano as u32;
        let mut v_dia: u32 = hoje;
        let mut fatia: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        nova(lp);
        let achou = buscar(cpv12_d(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, -1, -1, -1, cpv12_d(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
            (segundo == 0) as u32, 0);
        destruir(lp);
        if achou != 0 {
            return 1;
        }
    }
    if msg != 0 {
        let mut buf = [0u8; 0x48];
        let g = cpv12_nome_do_time(time, segundo, &mut buf);
        texto(msg, if entrando != 0 { 0xa5b34c } else { 0xa5b2d8 }, g, g, buf.as_ptr());
    }
    0
}

/// { 0x76da80 } (stdcall, ret 0xc, national_teams.cpp): `national_teams_player_in_squad(p, nacao, tipo)`: 1 se o
/// clube da seleção do jogador (+0x24) é o time do tipo (0 e 1 a principal 0x53b3d0, 2 o sub-21 0x53b4f0) e ele tem a
/// marca do tipo (+0x5e: 4 nos tipos 0 e 2, 8 no 1). Sem clube ou outro tipo: 0.
unsafe extern "stdcall" fn national_teams_player_in_squad(p: usize, nacao: usize, tipo: u32) -> u32 {
    if cpv12_d(p + 0x24) == 0 {
        return 0;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let (time, marca) = match tipo as u8 {
        0 => (principal(nacao), 4u8),
        1 => (principal(nacao), 8),
        2 => (b_national_team(nacao), 4),
        _ => return 0,
    };
    (cpv12_d(p + 0x24) == time && cpv12_b(p + 0x5e) & marca != 0) as u32
}

/// { 0x5c57d0 } (thiscall, ret 8, friendly.cpp; `this` não é lido): `Friendly::schedule_assign_opponent(agenda,
/// adversario)`: na primeira das `[agenda+8]` (byte com sinal) vagas de 25 bytes (a partir de +0x19) sem adversário
/// (dword +0), grava o adversário e marca o pedido (+0x21 da vaga anterior, ou seja agenda + 25·i + 0x21).
unsafe extern "thiscall" fn schedule_assign_opponent(_this: usize, agenda: usize, adversario: usize) {
    let n = cpv12_b(agenda + 8) as i8;
    let mut i: i8 = 0;
    while i < n {
        let vaga = agenda + 25 * (i as usize + 1);
        if cpv12_d(vaga) == 0 {
            core::ptr::write_unaligned(vaga as *mut usize, adversario);
            *((agenda + 25 * i as usize + 0x21) as *mut u8) = 1;
            return;
        }
        i += 1;
    }
}

/// Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
unsafe extern "thiscall" fn comparar_national_teams_can_change_squad(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(%%I0%%);
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return o(this, nacao, tipo, p, entrando, msg, tam);
    }
    let r = o(this, nacao, tipo, p, entrando, msg, tam);
    let mut meu: Vec<u8> = if msg != 0 { vec![0u8; tam as usize] } else { Vec::new() };
    let n = national_teams_can_change_squad(this, nacao, tipo, p, entrando, if msg != 0 { meu.as_mut_ptr() as usize } else { 0 }, tam);
    let texto = |a: *const u8, m: usize| -> Vec<u8> { (0..m).map(|i| *a.add(i)).take_while(|&c| c != 0).collect() };
    let (to, tr) = if msg != 0 { (texto(msg as *const u8, tam as usize), texto(meu.as_ptr(), tam as usize)) } else { (Vec::new(), Vec::new()) };
    PLACAR.registrar(r == n && to == tr, || format!("national_teams_can_change_squad({:x}, {}, {:x}, {}) {}/{} {:?} x {:?}", nacao, tipo as u8, p, entrando, r, n,
        String::from_utf8_lossy(&to), String::from_utf8_lossy(&tr)));
    r
}

/// Comparador: pura.
unsafe extern "stdcall" fn comparar_national_teams_player_in_squad(p: usize, nacao: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(%%I1%%)(p, nacao, tipo);
    let n = national_teams_player_in_squad(p, nacao, tipo);
    PLACAR.registrar(r == n, || format!("national_teams_player_in_squad({:x}, {:x}, {}) original={} rust={}", p, nacao, tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
unsafe extern "thiscall" fn comparar_schedule_assign_opponent(this: usize, agenda: usize, adversario: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    let n = cpv12_b(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, adversario);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, agenda, adversario), || schedule_assign_opponent(this, agenda, adversario));
    PLACAR.registrar(igual, || format!("schedule_assign_opponent({:x}, {:x}) {:x?} x {:x?}", agenda, adversario, mo, mr));
}
