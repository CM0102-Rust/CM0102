// ---------- centesima trigesima oitava leva: o batedor guardado, as instrucoes guardadas e o codigo da competicao ----------

/// { 0x897990 } (ret 0x10, tactics.cpp): quem bate a bola parada `tipo` (0..=4) no lado `lado`
/// (0..=1) pelo clube: a celula (dword) na linha tatica do clube (0x8982b0 com `humano`) em +0x601 +
/// linha x 0x91 + 0x5e + (lado + tipo x 2) x 4. Acima de [0xae2364] + 0x52d a celula vira -1 (0);
/// negativa, 0. A pessoa (0x6e bytes em [0xae23bc]) sem +0x61 apaga a celula (0); do clube (+0x39),
/// numa selecao (0x53b900) ou com o vinculo das selecoes (0x775790 em [0xb74318]) e devolvida; senao
/// a celula vira -1 (0). Clube nulo, tipo ou lado fora: a original (caixa de erro).
unsafe extern "thiscall" fn batedor_guardado(this: usize, clube: usize, tipo: u32, lado: u32, humano: u32) -> usize {
    let (t, l) = (tipo as u8 as i8, lado as u8 as i8);
    if clube == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(304)(this, clube, tipo, lado, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, humano);
    if k == -1 {
        crate::comparacao::ramo("batedor sem linha");
        return 0;
    }
    let celula = (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;
    let v = *celula;
    if v > (*(0xae2364 as *const i32)).wrapping_add(0x52d) {
        crate::comparacao::ramo("batedor alem das pessoas");
        *celula = -1;
        return 0;
    }
    if v <= -1 {
        crate::comparacao::ramo("batedor vazio");
        return 0;
    }
    let pessoa = (*(0xae23bc as *const usize)).wrapping_add(v as usize * 0x6e);
    if *((pessoa + 0x61) as *const u32) == 0 {
        crate::comparacao::ramo("batedor sem ficha");
        *celula = -1;
        return 0;
    }
    if *((pessoa + 0x39) as *const usize) == clube {
        crate::comparacao::ramo("batedor do clube");
        return pessoa;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x775790usize);
    if selecao(clube) != 0 || vinculo(*(0xb74318 as *const usize), clube) != 0 {
        crate::comparacao::ramo("batedor de fora do clube");
        return pessoa;
    }
    crate::comparacao::ramo("batedor saiu do clube");
    *celula = -1;
    0
}

/// { 0x896fe0 } (ret 0x10, tactics.cpp): guarda (`volta` 0) ou devolve (`volta` != 0) as 20
/// instrucoes (dwords em +0x601 + linha x 0x91 + 2, linha da 0x8982b0 com `pessoa`) do clube num dos
/// 16 espacos de 0x5c bytes a partir de +0x30 (clube +0x30, pessoa +0x34, ocupado +0x38, instrucoes
/// +0x3c; ocupados em +0x5f0). O espaco e o do humano da vez ([0xb74c26 + [0xb77c7a] x 0x300]):
/// [0xae2364] - id dele - 1, em 0..16. Guardando (com `usar`), marca o espaco e copia; depois as 20
/// instrucoes viram -1. Devolvendo com `usar`, o espaco do mesmo clube e pessoa volta para a tabela
/// e e liberado; sem `usar`, libera todos os espacos do clube e pessoa. Clube nulo: a original (caixa
/// de erro); linha -1: 0; senao 1.
unsafe extern "thiscall" fn guardar_instrucoes(this: usize, clube: usize, pessoa: u32, usar: u32, volta: u32) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(305)(this, clube, pessoa, usar, volta);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, pessoa);
    if k == -1 {
        return 0;
    }
    let instrucao = |i: usize| (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 2 + 4 * i) as *mut u32;
    let ocupados = (this + 0x5f0) as *mut u8;
    // o espaco do humano da vez, se houver
    let espaco = || {
        let humano = *((0xb74c26 + *(0xb77c7a as *const i16) as isize as usize * 0x300) as *const usize);
        if humano == 0 {
            return None;
        }
        let e = (*(0xae2364 as *const i32)).wrapping_sub(*(humano as *const i32)).wrapping_sub(1);
        if (0..16).contains(&e) { Some(this + e as usize * 0x5c) } else { None }
    };
    let u = |p: usize, o: usize| (p + o) as *mut u32;
    if volta == 0 {
        if usar != 0 {
            if let Some(s) = espaco() {
                crate::comparacao::ramo("instrucoes guardadas");
                if *u(s, 0x38) == 0 {
                    *ocupados = (*ocupados).wrapping_add(1);
                }
                *u(s, 0x30) = clube as u32;
                *u(s, 0x34) = pessoa;
                *u(s, 0x38) = 1;
                for i in 0..20 {
                    *u(s, 0x3c + 4 * i) = *instrucao(i);
                }
            }
        }
        crate::comparacao::ramo("instrucoes apagadas");
        for i in 0..20 {
            *instrucao(i) = u32::MAX;
        }
        return 1;
    }
    if usar != 0 {
        if let Some(s) = espaco() {
            if *u(s, 0x38) != 0 && *u(s, 0x30) == clube as u32 && *u(s, 0x34) == pessoa {
                crate::comparacao::ramo("instrucoes devolvidas");
                for i in 0..20 {
                    *instrucao(i) = *u(s, 0x3c + 4 * i);
                }
            }
            if *u(s, 0x38) != 0 {
                *ocupados = (*ocupados).wrapping_sub(1);
            }
            *u(s, 0x38) = 0;
        }
        return 1;
    }
    if *ocupados != 0 {
        for e in 0..16 {
            let s = this + e * 0x5c;
            if *u(s, 0x38) != 0 && *u(s, 0x30) == clube as u32 && *u(s, 0x34) == pessoa {
                crate::comparacao::ramo("instrucoes liberadas");
                *u(s, 0x38) = 0;
                *ocupados = (*ocupados).wrapping_sub(1);
            }
        }
    }
    1
}

/// { 0x6b6570 } (cdecl, match_day.cpp): o numero da competicao pelo codigo (0: 5/4, 1: 11/10, 0x1ef:
/// 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; o primeiro com `segunda` 1). Outro codigo: a original
/// (caixa de erro).
unsafe extern "C" fn numero_da_competicao(codigo: u32, segunda: u32) -> u32 {
    let s = (segunda as u8 == 1) as u32;
    crate::comparacao::ramo("numero da competicao");
    match codigo as u16 as i16 {
        0 => 4 + s,
        1 => 10 + s,
        0x1ef => s,
        0x3de => 2 + s,
        0x483 => 6 + s,
        0x528 => 8 + s,
        _ => orig::<unsafe extern "C" fn(u32, u32) -> u32>(306)(codigo, segunda),
    }
}

unsafe extern "thiscall" fn comparar_batedor_guardado(this: usize, clube: usize, tipo: u32, lado: u32, humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(304);
    let (t, l) = (tipo as u8 as i8, lado as u8 as i8);
    if gravando_no_topo() || clube == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return o(this, clube, tipo, lado, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, humano);
    let mut reg = vec![(0xb67a34, 4)];
    if k != -1 {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4), 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, tipo, lado, humano), || batedor_guardado(this, clube, tipo, lado, humano));
    PLACAR.registrar(igual && r == n, || format!("batedor_guardado(tipo {} lado {}) original={:#x} rust={:#x} {:x?} x {:x?}", t, l, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_guardar_instrucoes(this: usize, clube: usize, pessoa: u32, usar: u32, volta: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(305);
    if gravando_no_topo() || clube == 0 {
        return o(this, clube, pessoa, usar, volta);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, pessoa);
    // os 16 espacos e o contador, e a linha de instrucoes do clube
    let mut reg = vec![(this + 0x30, 0x5c * 16), (this + 0x5f0, 1)];
    if k != -1 {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 2), 80));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, pessoa, usar, volta), || guardar_instrucoes(this, clube, pessoa, usar, volta));
    PLACAR.registrar(igual && r == n, || format!("guardar_instrucoes(usar {} volta {}) original={} rust={} {:x?} x {:x?}", usar, volta, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_numero_da_competicao(codigo: u32, segunda: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, u32) -> u32>(306);
    let r = o(codigo, segunda);
    if matches!(codigo as u16 as i16, 0 | 1 | 0x1ef | 0x3de | 0x483 | 0x528) {
        let n = numero_da_competicao(codigo, segunda);
        PLACAR.registrar(r as u8 == n as u8, || format!("numero_da_competicao({:#x}, {}) original={} rust={}", codigo as u16, segunda as u8, r as u8, n as u8));
    }
    r
}

// ---------- centesima quadragesima leva: a linha tatica (0x8983f0, 0x8984d0) e a simulacao rapida (0x6beeb0) ----------

/// { 0x8983f0 } (ret 0xc, tactics.cpp): copia a linha tatica do clube (0x91 bytes em +0x601 + linha
/// x 0x91, linha da 0x8982b0 com `humano`) para `destino`. Clube nulo: a original (caixa de erro);
/// linha -1: 0; senao 1.
unsafe extern "thiscall" fn ler_linha_tatica(this: usize, clube: usize, destino: *mut u8, humano: u32) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(310)(this, clube, destino, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, humano);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("linha tatica lida");
    core::ptr::copy((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91) as *const u8, destino, 0x91);
    1
}

/// { 0x8984d0 } (ret 0xc, tactics.cpp): o inverso da 0x8983f0, grava `origem` (0x91 bytes) na linha
/// tatica do clube. Linha -1 ou alem do total (+0x605): 0.
unsafe extern "thiscall" fn gravar_linha_tatica(this: usize, clube: usize, origem: *const u8, humano: u32) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(311)(this, clube, origem, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, humano);
    if k == -1 || k >= *((this + 0x605) as *const i32) {
        return 0;
    }
    crate::comparacao::ramo("linha tatica gravada");
    core::ptr::copy(origem, (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91) as *mut u8, 0x91);
    1
}

/// Regioes das duas: o destino/origem e a linha do clube (se houver).
unsafe fn regioes_da_linha_tatica(this: usize, clube: usize, p: usize, humano: u32) -> Vec<(usize, usize)> {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, humano);
    let mut reg = vec![(0xb67a34, 4), (p, 0x91)];
    if k != -1 && k < *((this + 0x605) as *const i32) {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91), 0x91));
    }
    reg
}

unsafe extern "thiscall" fn comparar_ler_linha_tatica(this: usize, clube: usize, destino: *mut u8, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(310);
    if gravando_no_topo() || clube == 0 {
        return o(this, clube, destino, humano);
    }
    let reg = regioes_da_linha_tatica(this, clube, destino as usize, humano);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, destino, humano), || ler_linha_tatica(this, clube, destino, humano));
    PLACAR.registrar(igual && r == n, || format!("ler_linha_tatica(humano {}) original={} rust={} {:x?} x {:x?}", humano, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_gravar_linha_tatica(this: usize, clube: usize, origem: *const u8, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(311);
    if gravando_no_topo() || clube == 0 {
        return o(this, clube, origem, humano);
    }
    let reg = regioes_da_linha_tatica(this, clube, origem as usize, humano);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, origem, humano), || gravar_linha_tatica(this, clube, origem, humano));
    PLACAR.registrar(igual && r == n, || format!("gravar_linha_tatica(humano {}) original={} rust={} {:x?} x {:x?}", humano, r, n, mo, mr));
    r
}

/// { 0x6beeb0 } (match_eng.cpp, chamada pelo passo do motor 0x6b98c0) com as chamadas que mexem fora
/// do motor injetadas (0x6d6f70 narra, 0x6d0980 lesiona; e as da saida do jogador 0x6cda20): a
/// partida simulada sem lances, em periodos de duas rodadas (contador +0x8ed0, codigo do periodo
/// +0x8ed4). Por periodo: a forca dos times (0x6c0c60) com quatro pares limitados entre metade e o
/// dobro do lado 1, e o lado que comeca (sorteio(2)). Por rodada, do lado da vez: tres ataques (o
/// terceiro so se o lado vencer o sorteio de forca) com duelo de um jogador ativo (+0xc5 + 20 contra
/// a forca do outro); o segundo duelo (0x6c2880, +0xdd + 15) e o terceiro (+0xad + 15); o desgaste
/// (0x6c2750, +0xb9 + 20); os cartoes do outro lado (expulsao 0x6c2310 modo 4, amarelo modo 2, o
/// segundo amarelo expulsa; mais de 4 expulsos e abandono, 3 x 0); o tipo do lance; o fator do placar
/// (diferenca, 1, 0,65 depois dos 60 ou 0,75), a disputa (0x6c06b0/0x6c07a0 ou 0x6c0a70) e, com
/// vantagem, o chute (0x6c23f0, 0x6c0890/0x6c0980) e o gol (0x6c2050; goleiros 0x717410; narra
/// 0x1fcc ou 0x1fcf, ou 0x1fce no erro do penalti). Nos minutos 45, 90, 105 e 120 grava os placares
/// (+0x45, +0x43, +0x47) e decide prorrogacao, gol de ouro (+0x38 & 0x80) e penaltis (+0x49/+0x4a,
/// sorteio(5)); o agregado (+0x4b/+0x4c). Depois: o gol do mando (sorteio(4) com placar igual), as
/// lesoes do ultimo lado da vez (0x6d0980), as notas (amistoso -2: ajuste pela reputacao do clube) e
/// os contadores +0xf5e2/+0xf618.
unsafe fn simulacao_rapida_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |o: usize| (m + o) as *mut i16;
    let b = |o: usize| (m + o) as *mut u8;
    let mais = |p: *mut u8| *p = (*p).wrapping_add(1);
    let mais16 = |p: *mut i16| *p = (*p).wrapping_add(1);
    let palavra = |o: usize, lado: usize| *((m + o + 2 * lado) as *const i16) as i32;
    let jog = |lado: usize, i: i8| (m as isize + 0x4796 + (i as isize + lado as isize * 20) * 0x1be) as usize;
    let somar = |p: usize, d: i16| {
        let q = (p + 0x35) as *mut i16;
        *q = (*q).wrapping_add(d);
    };
    let pt = || u(m, 0x4792);
    let pb = |o: usize| (u(m, 0x4792) + o) as *mut i8;
    let gols = |lado: usize| *b(0xf5bd + lado * 0x36) as i32;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, c: u8, lado: usize| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, c as u32, lado as u32, u32::MAX]);
    };
    // goleiros (tabela 1) ativos do lado sofrem o gol
    let goleiros = |lado: usize| {
        for i in 0..20 {
            let q = jog(lado, i);
            if byte(q, 0x19) >= 0 && tabela_do_jogador(q) == 1 {
                mais((q + 0xf) as *mut u8);
            }
        }
    };
    let mut fim = false;
    let mut decide = false;
    let mut local_28 = 0u8;
    let mut lado = 0usize;
    *w(0x8ed0) = 0;
    *w(0x8ed4) = 0x1ef;
    let penaltis = 'periodo: loop {
        forca_do_time(m, 0);
        forca_do_time(m, 1);
        for o in [0x1da, 0x1de, 0x1e2, 0x1e6] {
            let (a, x) = (*w(o) as f64, *w(o + 2) as f64);
            let meio = x * f64_em(0x968968);
            if !(a >= meio) {
                *w(o) = ftol(meio) as i16;
            } else if a > x + x {
                *w(o) = ftol(x + x) as i16;
            }
        }
        let c7 = sortear(2) as u8;
        let mut rodada = 0u8;
        'rodadas: while *b(0x1a8) == 0 {
            lado = if c7 != 0 { rodada as usize } else { (rodada == 0) as usize };
            let outro = (lado == 0) as usize;
            let forte = || sortear(palavra(0x1de, lado) + palavra(0x1da, lado)) > sortear(palavra(0x1da, outro) + palavra(0x1de, outro));
            // os tres ataques (o terceiro pela forca)
            for k in 0..3 {
                if k >= 2 && !forte() {
                    continue;
                }
                mais16(w(0xf5e2 + lado * 0x36));
                let mut t = 0u8;
                let i = loop {
                    let i = sortear(0x14) as i8;
                    t += 1;
                    if t >= 0x78 || byte(jog(lado, i), 0x19) >= 0 {
                        break i;
                    }
                };
                if t >= 0x78 {
                    fim = true;
                    break;
                }
                let p = jog(lado, i);
                let f = (f32_em(p + 0xc5) as f64 + f32_em(0x96afb0) as f64) as f32;
                let r = sortear((palavra(0x1da, outro) + 200) / 19);
                if !(r as f64 >= f as f64) {
                    mais16(w(0xf5e4 + lado * 0x36));
                    somar(p, 0x28);
                } else {
                    somar(p, -0x1e);
                }
            }
            if forte() {
                mais16(w(0xf5e6 + lado * 0x36));
                let c = escolher_por_papel_b(m, lado as u32);
                if c == 0xff {
                    fim = true;
                    break 'rodadas;
                }
                let p = jog(lado, c as i8);
                let f = (f32_em(p + 0xdd) as f64 + f32_em(0x96b020) as f64) as f32;
                let r = sortear(palavra(0x1de, outro) / 5);
                if !(r as f64 >= f as f64) {
                    somar(p, 0x28);
                    mais16(w(0xf5e8 + lado * 0x36));
                } else {
                    somar(p, -10);
                }
            }
            if forte() {
                mais16(w(0xf5ea + lado * 0x36));
                let mut t = 0u8;
                let i = loop {
                    let i = sortear(0x14) as i8;
                    t += 1;
                    if t >= 0x78 {
                        fim = true;
                        break 'rodadas;
                    }
                    if byte(jog(lado, i), 0x19) >= 0 {
                        break i;
                    }
                };
                let p = jog(lado, i);
                let f = (f32_em(p + 0xad) as f64 + f32_em(0x96b020) as f64) as f32;
                let r = sortear((palavra(0x1f2, outro) + 100) / 7);
                if !(r as f64 >= f as f64) {
                    mais16(w(0xf5ec + lado * 0x36));
                    somar(p, 0x14);
                } else {
                    somar(p, -8);
                }
            }
            if sortear(10) == 0 {
                mais(b(0xf5c8 + lado * 0x36));
            }
            let p = jog(lado, escolher_por_papel(m, lado as u32) as i8);
            let v = ftol(f32_em(p + 0xb9) as f64 + f32_em(0x96afb0) as f64);
            if sortear(v) < sortear(2) {
                somar(p, -0xf);
                mais(b(0xf5ca + lado * 0x36));
            }
            // os cartoes do outro lado, ou o tipo do lance
            local_28 = 0;
            let mut c20 = false;
            if sortear(palavra(0x1ea, outro)) > sortear(0x640) {
                let mo = m + outro * 0x36;
                mais((mo + 0xf5c9) as *mut u8);
                let expulsar = |c: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                    let p = jog(outro, c as i8);
                    mais((p + 0x23) as *mut u8);
                    mais((mo + 0xf5cb) as *mut u8);
                    jogador_saiu_com(m, p, 1, chamar);
                    forca_do_time(m, outro as i8);
                    narrar(chamar, 0x1ff8, c, outro);
                };
                if sortear(palavra(0x1ea, outro)) > sortear(0x61a8) {
                    crate::comparacao::ramo("simulacao rapida expulsao");
                    let c = escolher_pela_condicao(m, outro as i8, 4);
                    somar(jog(outro, c as i8), -0xfa);
                    expulsar(c, chamar);
                } else if sortear(palavra(0x1ea, outro)) > sortear(0x6a4) {
                    crate::comparacao::ramo("simulacao rapida amarelo");
                    let c = escolher_pela_condicao(m, outro as i8, 2);
                    let p = jog(outro, c as i8);
                    somar(p, -0x32);
                    mais((p + 0x24) as *mut u8);
                    mais((mo + 0xf5cc) as *mut u8);
                    if byte(p, 0x24) == 2 {
                        crate::comparacao::ramo("simulacao rapida segundo amarelo");
                        expulsar(c, chamar);
                    }
                    if *((mo + 0xf5cb) as *const u8) > 4 {
                        // abandono: 3 x 0 para o lado da vez
                        crate::comparacao::ramo("simulacao rapida abandono");
                        fim = true;
                        *b(0xf5bd + outro * 0x36) = 0;
                        let g = b(0xf5bd + lado * 0x36);
                        if *g <= 3 {
                            *g = 3;
                        }
                        let (x, y) = if lado != 0 { (3, 0) } else { (0, 3) };
                        *pb(0x43) = x;
                        *pb(0x44) = y;
                        if *pb(0x4b) != -1 && *pb(0x43) as i32 + *pb(0x4b) as i32 == *pb(0x4c) as i32 + *pb(0x44) as i32 {
                            *pb(0x43) = (*pb(0x43)).wrapping_add(1);
                            mais(b(0xf5bd));
                        }
                        break 'rodadas;
                    }
                }
                if sortear(0x1e) != 0 {
                    c20 = true;
                    mais((mo + 0xf5c6) as *mut u8);
                } else {
                    local_28 = 1;
                }
            } else if sortear(palavra(0x1de, lado)) > sortear(3000) {
                c20 = true;
                mais(b(0xf5c7 + lado * 0x36));
            }
            // o fator do placar
            let d = gols(lado) - gols(outro);
            let saldo = if lado == 0 { *pb(0x4b) as i32 - *pb(0x4c) as i32 } else { *pb(0x4c) as i32 - *pb(0x4b) as i32 };
            let x = if saldo + d > 3 {
                d as f64
            } else if gols(lado) + gols(outro) != 0 {
                f32_em(0x968e70) as f64
            } else if *w(0x8ed0) > 0x3c {
                f32_em(0x96d320) as f64
            } else {
                f32_em(0x96d248) as f64
            };
            let (v, repartir_outro): (f64, unsafe extern "thiscall" fn(usize, i8, i16)) = if c20 {
                (palavra(0x1f2, outro) as f64 * x * f64_em(0x96d310), repartir_c)
            } else {
                (palavra(0x1da, outro) as f64 * x * f64_em(0x96d318), repartir_b)
            };
            let r1 = sortear(ftol(v));
            let l24 = sortear(palavra(0x1de, lado)).wrapping_sub(r1) as i16;
            if l24 > 0 {
                repartir_a(m, lado as i8, 0x1c);
                repartir_outro(m, outro as i8, -0x14);
            } else {
                repartir_a(m, lado as i8, -6);
                repartir_outro(m, outro as i8, 0x28);
            }
            // o chute
            if l24 > 0 && sortear(gols(1) + gols(0)) <= 3 {
                let ml = m + lado * 0x36;
                if local_28 != 0 {
                    mais((ml + 0xf5c0) as *mut u8);
                }
                mais((ml + 0xf5be) as *mut u8);
                let c8 = escolher_por_papel_e(m, lado as u32, c20 as u32) as i8;
                let a = if c20 { sortear(palavra(0x1ee, lado)) } else { sortear(palavra(0x1e2, lado) * (local_28 as i32 + 1)) };
                let us = a as i16 as i32;
                if us + 0xa5 > sortear(0x172) + (0x168 - l24 as i32) {
                    mais((ml + 0xf5bf) as *mut u8);
                    if c20 {
                        repartir_d(m, lado as i8, a as i16);
                    } else {
                        repartir_e(m, lado as i8, a as i16);
                    }
                    if us + 0x2d > sortear((7 - local_28 as i32 * 3) * palavra(0x1e6, outro)) {
                        crate::comparacao::ramo("simulacao rapida gol");
                        let c9 = escolher_por_papel_c(m, lado as u32, local_28 as u32);
                        let p = jog(lado, c9 as i8);
                        mais((p + 0xc) as *mut u8);
                        somar(p, 0x1db);
                        goleiros(outro);
                        if local_28 != 0 {
                            crate::comparacao::ramo("simulacao rapida gol de penalti");
                            narrar(chamar, 0x1fcf, c9, lado);
                            mais((ml + 0xf5c1) as *mut u8);
                            mais((p + 0xd) as *mut u8);
                            mais((p + 0xe) as *mut u8);
                        } else {
                            narrar(chamar, 0x1fcc, c9, lado);
                            if c20 {
                                let o = match sortear(4) {
                                    0 | 1 => 0xf5c3,
                                    2 => 0xf5c5,
                                    _ => 0xf5c4,
                                };
                                mais((ml + o) as *mut u8);
                            }
                        }
                        if c8 > -1 {
                            let q = jog(lado, c8);
                            mais((q + 0x10) as *mut u8);
                            somar(q, 0x15e);
                        }
                        mais(b(0xf5c2 + outro * 0x36));
                        mais((ml + 0xf5bd) as *mut u8);
                        for (s, delta) in [(lado, 0x4b), (outro, -0x43)] {
                            for i in 0..20 {
                                let q = jog(s, i);
                                if byte(q, 0x19) >= 0 {
                                    somar(q, delta);
                                }
                            }
                        }
                        if decide {
                            crate::comparacao::ramo("simulacao rapida gol de ouro");
                            *b(0x1a8) = 1;
                            break 'rodadas;
                        }
                    } else {
                        repartir_f(m, outro as i8, 0xe1);
                        if local_28 != 0 {
                            crate::comparacao::ramo("simulacao rapida penalti perdido");
                            let c = escolher_por_papel_c(m, lado as u32, local_28 as u32);
                            narrar(chamar, 0x1fce, c, lado);
                            let p = jog(lado, c as i8);
                            mais((p + 0xd) as *mut u8);
                            somar(p, -500);
                        }
                    }
                }
            }
            rodada += 1;
            if rodada >= 2 {
                break;
            }
        }
        // o fim do periodo
        mais16(w(0x8ed0));
        let t = *w(0x8ed0);
        let w38 = *((pt() + 0x38) as *const u16);
        let igual_no_agregado = |a: usize, x: usize| *pb(a) as i32 + *pb(0x4b) as i32 == *pb(0x4c) as i32 + *pb(x) as i32;
        if fim {
            let (a, x) = if t <= 0x5a { (0x43, 0x44) } else { (0x47, 0x48) };
            *pb(a) = gols(0) as i8;
            *pb(x) = gols(1) as i8;
            let desempata = if *pb(0x4b) != -1 { igual_no_agregado(0x43, 0x44) } else { *pb(0x43) == *pb(0x44) && w38 & 0xa3 != 0 };
            if desempata {
                *pb(a) = (*pb(a)).wrapping_add(1);
            }
            break 'periodo false;
        }
        if *b(0x1a8) != 0 {
            *pb(0x47) = gols(0) as i8;
            *pb(0x48) = gols(1) as i8;
            break 'periodo false;
        }
        match t {
            0x2d => {
                *w(0x8ed4) = 0x3de;
                *pb(0x45) = gols(0) as i8;
                *pb(0x46) = gols(1) as i8;
            }
            0x5a => {
                *pb(0x43) = gols(0) as i8;
                *pb(0x44) = gols(1) as i8;
                *w(0x8ed4) = 0x483;
                let agregado = *pb(0x4b) != -1;
                if (agregado && !igual_no_agregado(0x43, 0x44)) || (!agregado && *pb(0x43) != *pb(0x44)) {
                    break 'periodo false;
                }
                if w38 & 0x80 != 0 {
                    crate::comparacao::ramo("simulacao rapida prorrogacao com gol de ouro");
                    decide = true;
                    continue 'periodo;
                }
                if w38 & 2 != 0 {
                    crate::comparacao::ramo("simulacao rapida prorrogacao");
                    continue 'periodo;
                }
                if agregado && w38 & 4 == 0 && *pb(0x44) != *pb(0x4b) && w38 & 0x4000 == 0 {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            0x69 => *w(0x8ed4) = 0x528,
            0x78 => {
                *pb(0x47) = gols(0) as i8;
                *pb(0x48) = gols(1) as i8;
                if *pb(0x4b) != -1 {
                    if !igual_no_agregado(0x47, 0x48) || (w38 & 0x4000 != 0 && *pb(0x48) != *pb(0x4b)) {
                        break 'periodo false;
                    }
                } else if *pb(0x47) != *pb(0x48) {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            _ => {}
        }
    };
    if penaltis {
        crate::comparacao::ramo("simulacao rapida penaltis");
        sortear(5);
        sortear(5);
        *pb(0x49) = sortear(5) as i8;
        sortear(5);
        sortear(5);
        *pb(0x4a) = sortear(5) as i8;
        if *pb(0x49) == *pb(0x4a) {
            let o = if sortear(2) != 0 { 0x49 } else { 0x4a };
            *pb(o) = (*pb(o)).wrapping_add(1);
        }
    }
    // o agregado
    if *pb(0x4b) != -1 {
        let (a, x) = if *pb(0x47) != -1 { (0x47, 0x48) } else { (0x43, 0x44) };
        *pb(0x4b) = (*pb(0x4b)).wrapping_add(*pb(a));
        *pb(0x4c) = (*pb(0x4c)).wrapping_add(*pb(x));
    }
    // o gol do mando
    let c1 = u(pt(), 0x1c);
    if c1 != 0 && u(pt(), 0x14) == u(c1, 0x57) && (*pb(0x43) as i32 + *pb(0x44) as i32) < 5 && *pb(0x43) == *pb(0x44) && sortear(4) == 0 {
        let (r0, r1) = (*((c1 + 0x80) as *const i16) as i32, *((u(pt(), 0x20) + 0x80) as *const i16) as i32);
        let s = if r0 > r1 { Some(0) } else if r0 + 0xfa < r1 { Some(1) } else { None };
        if let Some(s) = s {
            crate::comparacao::ramo("simulacao rapida gol do mando");
            mais(b(0xf5be + s * 0x36));
            let c = escolher_por_papel_c(m, s as u32, local_28 as u32);
            let p = jog(s, c as i8);
            mais((p + 0xc) as *mut u8);
            somar(p, 0x1db);
            goleiros(1 - s);
            narrar(chamar, 0x1fcc, c, s);
        }
    }
    // as lesoes (duas passadas pelo ultimo lado da vez)
    if *(pt() as *const i32) != -2 {
        for _ in 0..2 {
            for i in 0..20 {
                let p = jog(lado, i);
                let pessoa = u(p, 0x6d);
                if byte(p, 0x19) < 0 || pessoa == 0 {
                    continue;
                }
                let f = f32_em(p + 0x4d);
                let mut cl = (byte(pessoa, 0x2d) as u8).wrapping_mul(2).wrapping_add(5);
                if !(f >= f32_em(0x96d30c)) {
                    cl = cl.wrapping_add(0xf);
                } else if !(f >= f32_em(0x96d308)) {
                    cl = cl.wrapping_add(5);
                }
                if sortear(cl as i8 as i32) < sortear(400) {
                    continue;
                }
                let comp = u(pt(), 0x14);
                let liga = u(0xae2c74, 0);
                let teto = if comp == 0 || (liga != 0 && e_amistoso(liga, comp) != 0) { 0x5b } else { 0x66 };
                let g = sortear(teto) as u8;
                let tipo = match sortear(10) {
                    0 => 1,
                    1..=3 => 3,
                    4..=6 => 4,
                    7..=9 => 5,
                    _ => continue,
                };
                crate::comparacao::ramo("simulacao rapida lesao");
                chamar(0x6d0980, m, [tipo, g as u32, p as u32, 0, 0]);
            }
        }
    }
    // as notas
    for s in 0..2 {
        for i in 0..20i8 {
            let p = jog(s, i);
            let q = (p + 0x35) as *mut i16;
            if *(pt() as *const i32) == -2 && i < 11 {
                crate::comparacao::ramo("simulacao rapida nota do amistoso");
                let clube = if s != 0 { u(pt(), 0x20) } else { u(pt(), 0x1c) };
                let a = (*((p + 0x3b) as *const i16) as f64 - f64_em(0x968e30)) * f64_em(0x968ff8);
                let r = *((clube + 0x80) as *const i16) as f64 * f64_em(0x969018) - f64_em(0x96d300);
                let v = ftol(((a - r) * f64_em(0x96d2f8) + f64_em(0x967890)) * *q as f64) as i16;
                *q = v.clamp(0xdac, 0x2ee0);
            }
            let n = (ftol((*q as i32 + 0x1770) as f64 * f64_em(0x96a608) + f64_em(0x96d2f0)) as i8).clamp(1, 10);
            *((p + 0x1b) as *mut i8) = n;
            *((p + 0x51) as *mut u32) = *((p + 0x4d) as *const u32);
        }
    }
    for (a, x) in [(0xf5e2, 0xf5e4), (0xf618, 0xf61a)] {
        if *w(a) < 10 {
            crate::comparacao::ramo("simulacao rapida contador reiniciado");
            *w(a) = 0x14;
            *w(x) = (sortear(6) + 12) as i16;
        } else if *w(a) == *w(x) {
            crate::comparacao::ramo("simulacao rapida contador igual");
            let r = sortear(3);
            *w(x) = (*w(x)).wrapping_add((-1i32).wrapping_sub(r) as i16);
        }
    }
}

/// { 0x6beeb0 } (match_eng.cpp): ver `simulacao_rapida_com`.
unsafe extern "thiscall" fn simulacao_rapida(m: usize) {
    simulacao_rapida_com(m, &mut |r, t, a| despachar_dividida(r, t, a))
}

unsafe extern "thiscall" fn comparar_simulacao_rapida(m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(312);
    if gravando_no_topo() {
        return o(m);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080, a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao) e o registro da partida (placares e agregado)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    reg.push((t, 0x50));
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || {
        zerar_pilha();
        o(m)
    }, |chamar| {
        simulacao_rapida_com(m, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_716.store(gravar, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("simulacao_rapida {} chamadas: {}", k, detalhe_fita()));
}

// ---------- sexagesima oitava leva: vagas da tabela tatica ----------

/// Linha tatica do clube (0x8982b0) e o inicio dela na tabela em +0x601 (0x91 bytes por linha), ou
/// `None` na linha -1.
unsafe fn linha_na_tabela(this: usize, clube: usize, humano: u32) -> Option<usize> {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha(clube, humano);
    (l != -1).then(|| (*((this + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)))
}

/// { 0x897590 } (tactics.cpp): a vaga (0..20) cuja pessoa (indice em +2 + 4 x vaga, no vetor de
/// 0xae23bc) tem o id guardado em +0x52 da linha; sem nenhuma: -1 com `menos_um`, senao 0. Linha
/// -1: 0. Clube nulo: a original (caixa de erro).
unsafe extern "thiscall" fn vaga_marcada(this: usize, clube: usize, humano: u32, menos_um: u32) -> i8 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(186)(this, clube, humano, menos_um);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let pessoas = *(0xae23bc as *const usize);
    for vaga in 0..20usize {
        let i = *((linha + 2 + 4 * vaga) as *const i32);
        if i != -1 && *(pessoas.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == *((linha + 0x52) as *const i32) {
            return vaga as i8;
        }
    }
    -((menos_um != 0) as i8)
}

/// { 0x897760 } (tactics.cpp): guarda em +0x52 da linha o indice da pessoa da vaga (0..20); 1 se
/// havia alguem (-1 = vazia). Linha -1: 0. Clube nulo ou vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn marcar_vaga(this: usize, clube: usize, vaga: i8, humano: u32) -> u32 {
    if clube == 0 || !(0..20).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(187)(this, clube, vaga, humano);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let i = *((linha + 2 + 4 * vaga as usize) as *const i32);
    if i == -1 {
        return 0;
    }
    *((linha + 0x52) as *mut i32) = i;
    1
}

/// { 0x897dc0 } (tactics.cpp): a pessoa (vetor de 0xae23bc, 110 bytes cada) do indice em +0x56 +
/// 4 x `qual` (0 ou 1) da linha; indice negativo ou linha -1: 0. Clube nulo ou `qual` fora: a
/// original (caixa de erro).
unsafe extern "thiscall" fn pessoa_da_linha(this: usize, clube: usize, qual: i8, humano: u32) -> usize {
    if clube == 0 || !(0..2).contains(&qual) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(188)(this, clube, qual, humano);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let i = *((linha + 0x56 + 4 * qual as usize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(0xae23bc as *const usize)).wrapping_add(110 * i as usize)
}

// ---------- sexagesima quinta leva: tabela tatica do clube ----------

/// { 0x8982b0 } (tactics.cpp; o `this` nao e usado, ret 8): a linha do clube na tabela tatica.
/// Clube dirigido por humano (0x600a50 com 1, 1) e `humano` ligado: segunda selecao, -1; senao a
/// linha do tecnico, depois dos clubes (0xae235c): quantidade de pessoas (0xae2364) - id do tecnico
/// (+0xcf) - 1, ou + 15 numa selecao (0x53b900), fora da faixa: a original (caixa de erro). Nos
/// outros casos, o id do clube.
unsafe extern "stdcall" fn linha_tatica(clube: usize, humano: u32) -> i32 {
    let e_humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if e_humano(clube, 1, 1, 0, 0) != 0 && humano != 0 {
        let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
        if segunda(clube) != 0 {
            return -1;
        }
        let tecnico = *((clube + 0xcf) as *const usize);
        if tecnico == 0 {
            return orig::<unsafe extern "stdcall" fn(usize, u32) -> i32>(182)(clube, humano);
        }
        let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let clubes = *(0xae235c as *const i32);
        let base = (*(0xae2364 as *const i32)).wrapping_sub(*(tecnico as *const i32));
        let i = base.wrapping_add(if e_selecao(clube) != 0 { 0xf } else { -1 }).wrapping_add(clubes);
        if i < 0 || i >= clubes.wrapping_add(0x20) {
            return orig::<unsafe extern "stdcall" fn(usize, u32) -> i32>(182)(clube, humano);
        }
        return i;
    }
    *(clube as *const i32)
}

/// { 0x898040 } (tactics.cpp): byte `pos` (0..=10) da linha tatica do clube (0x8982b0) na tabela
/// em +0x601 (0x91 bytes por linha, a partir de +0x86); linha -1: 0. Clube nulo ou posicao fora:
/// a original (caixa de erro).
unsafe extern "thiscall" fn byte_tatico(this: usize, clube: usize, pos: i8, humano: u32) -> u8 {
    if clube == 0 || !(0..=10).contains(&pos) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8>(183)(this, clube, pos, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha(clube, humano);
    if l == -1 {
        return 0;
    }
    let tabela = *((this + 0x601) as *const usize);
    *((tabela.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x86 + pos as usize) as *const u8)
}

// ---------- 262a leva: as estatísticas das táticas: o resultado de cada partida na linha da tática do clube do computador, o capitão guardado e o capitão escolhido (tactics.cpp) ----------



/// A linha da tabela de estatísticas das táticas ([this], 0x28 bytes por tática) usada pelo clube: a palavra +0 do
/// registro de 0x91 bytes do clube (id em +0) na tabela tática (+0x601).
unsafe fn cpz5_linha(this: usize, clube: usize) -> usize {
    let id = read_usize(clube);
    let k = read_i16(read_usize(this + 0x601).wrapping_add(id.wrapping_mul(0x91))) as i32;
    read_usize(this).wrapping_add((k as isize * 0x28) as usize)
}

/// x < 1 vale 1 (palavra com sinal), como as três comparações antes de cada `fild`.
fn cpz5_pelo_menos_1(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// { 0x896290 } (thiscall, ret 4, tactics.cpp): `Tactics::record_tactic_result(jogo)`, do fim de cada partida no laço
/// do motor (0x6ba9a0, com o placar agregado; fixture +0 ≠ −2). Jogo nulo: a caixa da linha 0x1f9 e 0. Mandante (+0x1c)
/// ou visitante (+0x20) dirigido por humano (0x600a50 com 0, 1, 0, 0; cdecl 5): 0. Senão, para cada lado (clube nulo:
/// a caixa da linha 0x20a e segue), o registro da partida pelo clube (0x7e4750, num registro local que começa com a
/// data padrão 0x54a110) soma na linha da tática do clube (cpz5_linha): +2 jogos, +4 vitórias (+3 pontos em +0xe), +6
/// empates (+1 ponto), +8 derrotas, +0xa gols pró (+0x19 do registro, com sinal), +0xc contra (+0x1a); com p = jogos,
/// a = pró, b = contra e c = pontos (cada um pelo menos 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18
/// = r·g·q²·q, +0x20 = r·r·h (a ordem das contas do x87). A escolha da formação do computador (0x6e0230) lê +2 e as
/// três razões. Devolve 1.
unsafe extern "thiscall" fn registrar_resultado_da_tatica(this: usize, jogo: usize) -> u32 {
    if jogo == 0 {
        aviso_em(0xa8b908, 0x1f9);
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    for k in 0..2 {
        let c = read_usize(jogo + 0x1c + 4 * k);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 {
            return 0;
        }
    }
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
    let mut reg = [0u8; 0x20];
    let pr = reg.as_mut_ptr() as usize;
    data_padrao(pr);
    for k in 0..2 {
        let c = read_usize(jogo + 0x1c + 4 * k);
        if c == 0 {
            aviso_em(0xa8b908, 0x20a);
            continue;
        }
        let l = cpz5_linha(this, c);
        registro(jogo, c, pr);
        let m = |o: usize| (l + o) as *mut i16;
        crate::comparacao::ramo("cpz_5 linha da tatica");
        *m(2) = (*m(2)).wrapping_add(1);
        let (pro, contra) = (reg[0x19] as i8, reg[0x1a] as i8);
        if pro > contra {
            *m(4) = (*m(4)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(3); // cpz5: a vitória vale 3 pontos
        } else if pro == contra {
            *m(6) = (*m(6)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(1);
        } else {
            *m(8) = (*m(8)).wrapping_add(1);
        }
        *m(0xa) = (*m(0xa)).wrapping_add(pro as i16); // cpz5: os gols pró
        *m(0xc) = (*m(0xc)).wrapping_add(contra as i16);
        let a = cpz5_pelo_menos_1(*m(0xa));
        let b = cpz5_pelo_menos_1(*m(0xc));
        let c = cpz5_pelo_menos_1(*m(0xe));
        let p = cpz5_pelo_menos_1(*m(2));
        let q = c / p;
        let q2 = q * q;
        let g = a / p;
        let h = g * q2;
        let r = p / b;
        let f = |o: usize| (l + o) as *mut f64;
        core::ptr::write_unaligned(f(0x10), r * h * g);
        core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: a razão da postura normal
        core::ptr::write_unaligned(f(0x20), r * r * h);
    }
    1
}

/// Comparador da 0x896290: lado a lado nas linhas das táticas dos dois clubes (0x28 bytes cada) e na devolução; o
/// registro da partida é local. Jogo ou clube nulos: a caixa, só a original.
unsafe extern "thiscall" fn comparar_registrar_resultado_da_tatica(this: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1392);
    if jogo == 0 || read_usize(jogo + 0x1c) == 0 || read_usize(jogo + 0x20) == 0 {
        return o(this, jogo);
    }
    let reg = [(cpz5_linha(this, read_usize(jogo + 0x1c)), 0x28), (cpz5_linha(this, read_usize(jogo + 0x20)), 0x28)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || registrar_resultado_da_tatica(this, jogo));
    PLACAR.registrar(igual && r == n, || format!("registrar_resultado_da_tatica(jogo {:x}) original={} rust={} {:x?} x {:x?}", jogo, r, n, mo, mr));
    r
}

/// { 0x8976a0 } (thiscall, ret 8, tactics.cpp): `Tactics::get_captain(clube, time)`, das telas de tática (0x8a7440). Clube
/// nulo: a caixa da linha 0x527 e 0. A linha do clube (0x8982b0, thiscall(this, clube, time), ret 8; −1: 0); devolve o
/// dword +0x52 do registro de 0x91 bytes da linha na tabela tática (+0x601).
unsafe extern "thiscall" fn capitao_guardado(this: usize, clube: usize, time: u32) -> u32 {
    if clube == 0 {
        aviso_em(0xa8b908, 0x527);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    read_usize(read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52) as u32
}

/// Comparador da 0x8976a0: pela devolução (só lê; a linha nula ou fora da faixa mostra a caixa nas duas).
unsafe extern "thiscall" fn comparar_capitao_guardado(this: usize, clube: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1393);
    if clube == 0 {
        return o(this, clube, time);
    }
    let r = o(this, clube, time);
    let n = capitao_guardado(this, clube, time);
    PLACAR.registrar(r == n, || format!("capitao_guardado(clube {:x}, time {}) original={:x} rust={:x}", clube, time, r, n));
    r
}

/// { 0x897880 } (thiscall, ret 0xc, tactics.cpp): `Tactics::set_captain(clube, &pessoa, time)`, ao aplicar as mudanças da
/// tela de tática (0x8a80d0). Clube nulo: a caixa da linha 0x579; ponteiro nulo: a da 0x581; 0. A linha do clube
/// (0x8982b0; −1: 0); grava o dword apontado em +0x52 do registro da linha (+0x601) e devolve 1.
unsafe extern "thiscall" fn escolher_capitao(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    if clube == 0 {
        aviso_em(0xa8b908, 0x579);
        return 0;
    }
    if pessoa == 0 {
        aviso_em(0xa8b908, 0x581);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    let p = read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    core::ptr::write_unaligned(p as *mut u32, core::ptr::read_unaligned(pessoa as *const u32));
    1
}

/// Comparador da 0x897880: lado a lado no dword do capitão da linha (achada antes pela 0x8982b0, que só lê) e na
/// devolução. Clube ou ponteiro nulos, ou sem linha: só a original.
unsafe extern "thiscall" fn comparar_escolher_capitao(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1394);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time);
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return o(this, clube, pessoa, time);
    }
    let p = read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(p, 4)], || o(this, clube, pessoa, time), || escolher_capitao(this, clube, pessoa, time));
    PLACAR.registrar(igual && r == n, || format!("escolher_capitao(clube {:x}, time {}) original={} rust={} {:x?} x {:x?}", clube, time, r, n, mo, mr));
    r
}

// ---------- 260a leva: a escalação guardada do clube: tirar o jogador da escalação, trocar duas vagas e a cópia para desfazer (tactics.cpp) ----------


unsafe fn cpx9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}




/// A linha do clube na tabela tática (0x8982b0, stdcall(clube, humano); −1 sem linha).
unsafe fn cpx9_linha(clube: usize, humano: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    f(clube, humano)
}

/// O começo da linha `l` (0x91 bytes cada, a partir de [this+0x601]).
unsafe fn cpx9_inicio(this: usize, l: i32) -> usize {
    read_usize(this + 0x601).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// As 16 cópias para desfazer (0x5c bytes a partir de this+0x30: +0 clube, +4 time, +8 a cópia; conta em +0x5f0): a do
/// clube e do time some (+8 = 0) e a conta desce.
unsafe fn cpx9_soltar_copias(this: usize, clube: usize, time: u32) {
    if read_u8(this + 0x5f0) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + 0x30 + k * 0x5c;
        if read_usize(e + 8) != 0 && read_usize(e) == clube && read_usize(e + 4) as u32 == time {
            write_u32(e + 8, 0);
            *((this + 0x5f0) as *mut u8) = read_u8(this + 0x5f0).wrapping_sub(1);
        }
    }
}

/// As regiões da escalação do clube: a linha dele na tabela tática (se houver) e as cópias para desfazer com a conta.
unsafe fn cpx9_regioes(this: usize, clube: usize, time: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + 0x30, 0x5c1)];
    let l = cpx9_linha(clube, time);
    if l != -1 {
        reg.push((cpx9_inicio(this, l), 0x91));
    }
    reg
}

/// { 0x896c60 } (thiscall, ret 0x10, tactics.cpp): `Tactics::remove_player_from_selection(clube, pessoa, time,
/// papeis)`, pelos contratos (troca de lugar no elenco, empréstimo e o fim dele), pelas saídas do elenco, pela escalação
/// automática (0x6dbc50), pelas seleções, pela aposentadoria e pelas transferências. Sem linha tática (0x8982b0): 0. As
/// 20 vagas da escalação (+2 da linha, ids) com a pessoa (id em +0) viram −1. Com `papeis`: as duas vagas extras (+0x56,
/// +0x5a: 0x897dc0 e 0x897ee0) e os batedores das cinco bolas paradas nos dois lados (0x897990 e 0x897bb0) que são ela
/// ficam vazios. As cópias para desfazer do clube e do time somem. Devolve 1 se a tirou de uma vaga ou de bola parada.
/// Clube ou pessoa nulos: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(1329)(this, clube, pessoa, time, papeis); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let mut achou = 0;
    for k in 0..20usize { // cpx9: as vagas da escalação
        let vaga = cpx9_inicio(this, l) + 2 + 4 * k;
        if cpx9_i(vaga) == cpx9_i(pessoa) {
            crate::comparacao::ramo(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            write_u32(vaga, u32::MAX); // cpx9: a vaga fica vazia
            achou = 1;
        }
    }
    if papeis != 0 {
        let da_linha: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x897dc0usize);
        let extra: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x897ee0usize);
        // ponytail: a original empurra o tipo e o lado com sobra nos bytes altos (o deslocamento da linha e a pilha);
        // as chamadas leem só o byte baixo, e a cópia passa os valores limpos
        let batedor: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x897990usize);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x897bb0usize);
        for qual in 0..2u32 {
            if da_linha(this, clube, qual, time) == pessoa {
                extra(this, clube, qual, 0, time);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if batedor(this, clube, tipo, lado, time) == pessoa {
                    crate::comparacao::ramo("cpx_9 batedor tirado");
                    marcar(this, clube, tipo, lado, 0, time);
                    achou = 1;
                }
            }
        }
    }
    cpx9_soltar_copias(this, clube, time);
    achou
}

/// Comparador: lado a lado pela devolução, pela linha do clube na tabela tática (as vagas, as extras e os batedores) e
/// pelas cópias para desfazer com a conta. Clube ou pessoa nulos (caixa): só a original.
unsafe extern "thiscall" fn comparar_tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(1329);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time, papeis);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, pessoa, time, papeis), || tirar_da_escalacao(this, clube, pessoa, time, papeis));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_escalacao(clube {:#x}, pessoa {:#x}, time {}, papeis {}) original={} {:x?} rust={} {:x?}", clube, pessoa, time, papeis, r, mo, n, mr));
    r
}

/// { 0x8973e0 } (thiscall, ret 0x10, tactics.cpp): `Tactics::swap_selection_slots(clube, a, b, time)`, pela tela do
/// elenco (0x46835e). Vagas `a` e `b` (bytes com sinal, 0..19): troca os ids das duas vagas da escalação (+2 + 4 × vaga
/// da linha tática, 0x90ba90 com 4 bytes) e as cópias para desfazer do clube e do time somem; devolve 1. Sem linha
/// (0x8982b0): 0. Clube nulo ou vaga fora: a original (caixa "Error", 0).
unsafe extern "thiscall" fn trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(1330)(this, clube, a, b, time); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let trocar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x90ba90usize);
    let inicio = cpx9_inicio(this, l);
    trocar(inicio + 2 + 4 * va as usize, inicio + 2 + 4 * vb as usize, 4);
    cpx9_soltar_copias(this, clube, time);
    1
}

/// Comparador: lado a lado pela devolução, pela linha tática do clube e pelas cópias para desfazer. Clube nulo ou vaga
/// fora (caixa): só a original.
unsafe extern "thiscall" fn comparar_trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(1330);
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return o(this, clube, a, b, time);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, a, b, time), || trocar_vagas_da_escalacao(this, clube, a, b, time));
    PLACAR.registrar(igual && r == n, || format!("trocar_vagas_da_escalacao(clube {:#x}, {}, {}, time {}) original={} {:x?} rust={} {:x?}", clube, va, vb, time, r, mo, n, mr));
    r
}

/// { 0x897250 } (thiscall, ret 8, tactics.cpp): `Tactics::has_selection_undo(clube, time)`, pela tela do elenco
/// (0x458a58). Com cópias (+0x5f0) e o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]): a cópia dele (índice
/// [0xae2364] − id − 1, de 0 a 15) existe (+8) e é deste clube (+0) e time (+4): 1; senão 0.
unsafe extern "thiscall" fn tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    if read_u8(this + 0x5f0) == 0 {
        return 0;
    }
    let t = read_usize((read_i16(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize);
    if t == 0 {
        return 0;
    }
    let k = cpx9_i(0xae2364).wrapping_sub(cpx9_i(t)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        return 0;
    }
    let e = this + 0x30 + k as usize * 0x5c;
    (read_usize(e + 8) != 0 && read_usize(e) == clube && read_usize(e + 4) as u32 == time) as u32
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1331)(this, clube, time);
    let n = tem_copia_da_escalacao(this, clube, time);
    PLACAR.registrar(r == n, || format!("tem_copia_da_escalacao(clube {:#x}, time {}) original={} rust={}", clube, time, r, n));
    r
}

// ---------- 253a leva: a escalação guardada: limpar uma vaga e o batedor de bola parada (tactics.cpp) ----------


/// A linha do clube na tabela tática (0x8982b0, o `this` não é usado; −1 sem linha).
unsafe fn cpq13_linha(clube: usize, time: u32) -> i32 {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    linha(clube, time)
}

/// A célula `c` (dword) da linha `k` da tabela tática: [this + 0x601] + 0x91 × k + `c`.
unsafe fn cpq13_celula(this: usize, k: i32, c: usize) -> usize {
    (read_u32(this + 0x601) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// { 0x896e90 } (thiscall, ret 0xc, tactics.cpp): `Tactics::clear_selection_slot(clube, vaga, time)`: a vaga (0..=19)
/// da escalação do clube na tabela tática (linha da 0x8982b0, dword em +2 + 4 × vaga) fica −1 e, com espaços guardados
/// (+0x5f0), os do mesmo clube e time (16 espaços de 0x5c bytes a partir de +0x30: clube +0x30, time +0x34, ocupado
/// +0x38) são soltos (+0x38 = 0, +0x5f0 − 1). Devolve 1; sem linha, 0. Clube nulo ou vaga fora: a original (caixa).
unsafe extern "thiscall" fn limpar_vaga_da_escalacao(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1081)(this, clube, vaga, time);
    }
    let k = cpq13_linha(clube, time);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("cpq_13 limpar vaga");
    core::ptr::write_unaligned(cpq13_celula(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga
    if *((this + 0x5f0) as *const u8) != 0 {
        for e in 0..16 {
            let a = this + 0x30 + 0x5c * e;
            if read_u32(a + 8) != 0 && read_u32(a) as usize == clube && read_u32(a + 4) == time {
                core::ptr::write_unaligned((a + 8) as *mut u32, 0);
                *((this + 0x5f0) as *mut u8) = (*((this + 0x5f0) as *const u8)).wrapping_sub(1);
            }
        }
    }
    1
}

/// Comparador: só grava a vaga e os espaços guardados: lado a lado pela linha da escalação (20 dwords), pelos 16
/// espaços e pela conta (+0x5f0). Clube nulo ou vaga fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_limpar_vaga_da_escalacao(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1081);
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return o(this, clube, vaga, time);
    }
    let mut reg = vec![(this + 0x30, 16 * 0x5c), (this + 0x5f0, 1)];
    let k = cpq13_linha(clube, time);
    if k != -1 {
        reg.push((cpq13_celula(this, k, 2), 20 * 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, vaga, time), || limpar_vaga_da_escalacao(this, clube, vaga, time));
    PLACAR.registrar(igual && r == n, || format!("limpar_vaga_da_escalacao(vaga {}, time {}) original={} rust={} {:x?} x {:x?}", v, time, r, n, mo, mr));
    r
}

/// { 0x897bb0 } (thiscall, ret 0x14, tactics.cpp): `Tactics::set_set_piece_taker(clube, papel, escolha, pessoa, time)`:
/// o batedor `escolha` (0..=1) da bola parada `papel` (0..=4) na linha tática do clube (0x8982b0; dword em +0x5e +
/// 4 × (escolha + 2 × papel)). Com `pessoa` (ponteiro para o id), grava o id. Sem ela, a célula fica −1 e, sendo a
/// primeira escolha, a seguinte com batedor (> −1) sobe para ela e fica −1. Devolve 1; sem linha, 0. Clube nulo,
/// papel ou escolha fora: a original (caixa).
unsafe extern "thiscall" fn marcar_batedor_de_bola_parada(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1082)(this, clube, papel, escolha, pessoa, time);
    }
    let k = cpq13_linha(clube, time);
    if k == -1 {
        return 0;
    }
    let celula = |j: i8| cpq13_celula(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if pessoa != 0 {
        core::ptr::write_unaligned(celula(e), read_u32(pessoa) as i32);
        return 1;
    }
    crate::comparacao::ramo("cpq_13 batedor limpo");
    core::ptr::write_unaligned(celula(e), -1); // sem batedor
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(celula(j));
            if c > -1 {
                crate::comparacao::ramo("cpq_13 batedor sobe");
                core::ptr::write_unaligned(celula(e), c);
                core::ptr::write_unaligned(celula(j), -1);
                return 1;
            }
            j += 1;
        }
    }
    1
}

/// Comparador: só grava a linha dos batedores: lado a lado pelos 10 dwords (+0x5e) da linha do clube. Clube nulo,
/// papel ou escolha fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_marcar_batedor_de_bola_parada(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1082);
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return o(this, clube, papel, escolha, pessoa, time);
    }
    let k = cpq13_linha(clube, time);
    let reg = if k != -1 { vec![(cpq13_celula(this, k, 0x5e), 10 * 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, papel, escolha, pessoa, time),
        || marcar_batedor_de_bola_parada(this, clube, papel, escolha, pessoa, time));
    PLACAR.registrar(igual && r == n, || format!("marcar_batedor_de_bola_parada(papel {}, escolha {}, pessoa {:x}) original={} rust={} {:x?} x {:x?}", p, e, pessoa, r, n, mo, mr));
    r
}

// ---------- 175a leva: a formacao da tabela tatica, o peso das competicoes do jogador e o dado da noticia ----------

/// { 0x896570 } (thiscall, ret 8, tactics.cpp): o registro (0x18e3 bytes) da formação `f`: as
/// [+0x5fd] primeiras em [+0x5f9], as seguintes (até [+0x5f5] mais, só sem `humano`) em [+0x5f1].
/// Formação negativa ou fora: a original (caixa de erro).
unsafe extern "thiscall" fn registro_da_formacao(t: usize, f: i16, humano: u32) -> usize {
    let primeiras = core::ptr::read_unaligned((t + 0x5fd) as *const i32);
    let limite = if humano != 0 { primeiras } else { core::ptr::read_unaligned((t + 0x5f5) as *const i32).wrapping_add(primeiras) };
    let f = f as i32;
    if f < 0 || f >= limite {
        return orig::<unsafe extern "thiscall" fn(usize, i16, u32) -> usize>(397)(t, f as i16, humano);
    }
    if f < primeiras {
        core::ptr::read_unaligned((t + 0x5f9) as *const usize).wrapping_add(f as usize * 0x18e3)
    } else {
        core::ptr::read_unaligned((t + 0x5f1) as *const usize).wrapping_add((f - primeiras) as usize * 0x18e3)
    }
}

/// { 0x7c6930 } (stdcall, ret 8, player_stats.cpp): o peso da competição `cod` para o jogador `p`
/// pelas duas competições do clube (+0x1a e +0x1e): nas quatro competições especiais ([0x9cf4a8]
/// com 0x16/0x20, [0x9cf408] com 0x1e/0x20, [0x9cf2c0] com 0x16/0x1e, [0x9cf244] com 7) e nos códigos
/// +0x78/+0x79 da competição: 500/1000/500/300 pela primeira, 300/500/300/100 pela segunda; sem
/// competição, 0. Devolve só `ax`.
unsafe extern "stdcall" fn peso_da_competicao(p: usize, cod: i8) -> u16 {
    let k = |a: usize| *(a as *const i32);
    // como na original: a primeira constante igual ao id decide (sem o código certo, vai aos códigos)
    let especial = |id: i32| -> u8 {
        if id == k(0x9cf4a8) {
            (cod == 0x16 || cod == 0x20) as u8
        } else if id == k(0x9cf408) {
            (cod == 0x1e || cod == 0x20) as u8
        } else if id == k(0x9cf2c0) {
            (cod == 0x16 || cod == 0x1e) as u8
        } else if id == k(0x9cf244) && cod == 7 {
            2
        } else {
            0
        }
    };
    let c1 = core::ptr::read_unaligned((p + 0x1a) as *const usize);
    if c1 == 0 {
        return 0;
    }
    match especial(k(c1)) {
        1 => return 500,
        2 => return 1000,
        _ => {}
    }
    if *((c1 + 0x78) as *const i8) == cod {
        return 500;
    }
    if *((c1 + 0x79) as *const i8) == cod {
        return 300;
    }
    let c2 = core::ptr::read_unaligned((p + 0x1e) as *const usize);
    if c2 == 0 {
        return 0;
    }
    match especial(k(c2)) {
        1 => return 300,
        2 => return 500,
        _ => {}
    }
    if *((c2 + 0x78) as *const i8) == cod {
        return 300;
    }
    if *((c2 + 0x79) as *const i8) == cod { 100 } else { 0 }
}

/// { 0x785790 } (thiscall, ret 8, news.cpp): grava `valor` no dado `k` 0..49 da notícia (+5, 4 bytes
/// cada). Devolve 1. `k` fora: a original (caixa de erro).
unsafe extern "thiscall" fn gravar_dado_da_noticia(t: usize, k: i8, valor: u32) -> u32 {
    if !(0..0x32).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(399)(t, k, valor);
    }
    core::ptr::write_unaligned((t + 5 + 4 * k as usize) as *mut u32, valor);
    1
}

comparar_valor!(comparar_registro_da_formacao, registro_da_formacao, 397, (f: i16, humano: u32) -> usize);

unsafe extern "stdcall" fn comparar_peso_da_competicao(p: usize, cod: i8) -> u16 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8) -> u16>(398)(p, cod);
    let n = peso_da_competicao(p, cod);
    PLACAR.registrar(r == n, || format!("peso_da_competicao({:#x}, {}) original={} rust={}", p, cod, r, n));
    r
}

unsafe extern "thiscall" fn comparar_gravar_dado_da_noticia(t: usize, k: i8, valor: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(399);
    let reg = if (0..0x32).contains(&k) { vec![(t + 5 + 4 * k as usize, 4)] } else { vec![] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, k, valor), || gravar_dado_da_noticia(t, k, valor));
    PLACAR.registrar(igual && r == v, || format!("gravar_dado_da_noticia({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, k, valor, r, mo, v, mr));
    r
}

// ---------- centesima sexagesima quinta leva: as vagas extras da tatica, as idades da selecao e o dado do tecnico humano ----------

/// { 0x897ee0 } (thiscall, ret 0x10, tactics.cpp): as duas vagas extras (+0x56 e +0x5a) da linha do
/// `clube` na tabela tática (0x8982b0 com `humano`; sem linha, 0): com `pessoa`, `*pessoa` na vaga
/// `qual` 0/1; sem, a vaga vira −1 e, na 0, a 1 desce para a 0 e fica −1. Devolve 1. `clube` nulo ou
/// `qual` fora: a original (caixa de erro).
unsafe extern "thiscall" fn vaga_extra_tatica(t: usize, clube: usize, qual: i8, pessoa: *const u32, humano: u32) -> u32 {
    if clube == 0 || !(0..=1).contains(&qual) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(367)(t, clube, qual, pessoa, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    let linha = linha_da_tabela(t, l);
    let casa = (linha + 0x56 + 4 * qual as usize) as *mut u32;
    if !pessoa.is_null() {
        crate::comparacao::ramo("vaga extra gravada");
        core::ptr::write_unaligned(casa, *pessoa);
        return 1;
    }
    crate::comparacao::ramo("vaga extra apagada");
    core::ptr::write_unaligned(casa, u32::MAX);
    if qual == 0 {
        let segunda = core::ptr::read_unaligned((linha + 0x5a) as *const u32);
        core::ptr::write_unaligned((linha + 0x56) as *mut u32, segunda);
        core::ptr::write_unaligned((linha + 0x5a) as *mut u32, u32::MAX);
    }
    1
}

/// { 0x76d430 } (stdcall, ret 0x10, national_teams.cpp): as idades limite (`*minima`, `*maxima`) de
/// uma seleção na competição `comp`: 23/23 nas competições olímpicas e de base ([comp+0x14] com id
/// numa das 9 constantes 0x9cf78c…0x9cf964); 24/22 com `tipo` 1 ou acima de 2; senão 26/22.
/// Ponteiros nulos: a original (caixa de erro).
unsafe extern "stdcall" fn idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    if minima.is_null() || maxima.is_null() {
        return orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368)(comp, tipo, minima, maxima);
    }
    if comp != 0 {
        let c = *((comp + 0x14) as *const usize);
        if c != 0 {
            let id = *(c as *const i32);
            if [0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964usize]
                .iter()
                .any(|&a| *(a as *const i32) == id)
            {
                *minima = 0x17;
                *maxima = 0x17;
                return;
            }
        }
    }
    if tipo != 0 && tipo != 2 {
        *minima = 0x18;
        *maxima = 0x16;
    } else {
        *minima = 0x1a;
        *maxima = 0x16;
    }
}

/// { 0x5fdd30 } (thiscall, ret 8, human_manager.cpp): grava `valor` em +0x9a65 do registro do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]) entre os humanos `this`
/// ([this], 0xa6ef bytes cada; o índice é o id − [0xae2364] + 16, até [this+4] − 1). Devolve 1.
/// Sem técnico ou índice fora: a original (caixa de erro).
unsafe extern "thiscall" fn gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if p.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    let i = (*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    crate::comparacao::ramo("dado do humano gravado");
    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, valor);
    1
}

unsafe extern "thiscall" fn comparar_vaga_extra_tatica(t: usize, clube: usize, qual: i8, pessoa: *const u32, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(367);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, qual, pessoa, humano), || vaga_extra_tatica(t, clube, qual, pessoa, humano));
    PLACAR.registrar(igual && r == v, || format!("vaga_extra_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, qual, humano, r, mo, v, mr));
    r
}

unsafe extern "stdcall" fn comparar_idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    let o = orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368);
    let reg = if minima.is_null() || maxima.is_null() { vec![] } else { vec![(minima as usize, 1), (maxima as usize, 1)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(comp, tipo, minima, maxima), || idades_da_selecao(comp, tipo, minima, maxima));
    PLACAR.registrar(igual, || format!("idades_da_selecao({:#x}, {}) {:x?} x {:x?}", comp, tipo, mo, mr));
}

unsafe extern "thiscall" fn comparar_gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369);
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    let reg = if p.is_null() {
        vec![]
    } else {
        let i = (*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
        if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) { vec![] } else { vec![(*(t as *const usize) + i as usize * 0xa6ef + 0x9a65, 4)] }
    };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, valor, tecnico), || gravar_dado_do_humano(t, valor, tecnico));
    PLACAR.registrar(igual && r == v, || format!("gravar_dado_do_humano({:#x}, {:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, valor, tecnico, r, mo, v, mr));
    r
}

