// ---------- %%LEVA%%a leva: o fechamento do motor (7): a regra do elenco e as notas do jogador (match_man.cpp) ----------

/// { 0x6e8910 } (cdecl, match_man.cpp): cumpre a `regra` do elenco (byte 0: 1 estrangeiro de fora da
/// comunidade, 2 estrangeiro, 3 acima da idade em +1; +2 máximo em campo, +3 máximo no banco, +4 e +5
/// o permitido e o atual) na escalação do `clube` (nação +0x53; 0x53d980/0x53d740): tira os restritos
/// da lista de reserva (`reserva`, `nr`); se há mais restritos em campo que o máximo (+2), escolhe entre
/// os titulares restritos e os candidatos (banco não restrito + reserva) pela 0x6e2000 e troca, um a
/// um, o de menor perda na nota (0x6e2f20), refazendo o banco (0x6e2000) se um titular saiu dele;
/// depois faz o mesmo no banco (`saida` + `nt`) com o máximo +3 (ou +4 − +5).
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn apply_squad_rule(
    clube: usize, tecnico: usize, regra: usize, modo: u32, nt: u32, nb: u32, nr: u32, starters: usize, banco: usize, reserva: usize,
    saida: usize, codigos: usize, curtos: usize, marcas: usize, bytes: usize, lado: usize,
) {
    let r = |k: usize| *((regra + k) as *const i8);
    if r(0) == 0 || r(5) >= r(4) {
        return;
    }
    let pick: Escalar = core::mem::transmute(0x6e2000usize);
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(0x6e2f20usize);
    let restrito = |p: usize| -> bool {
        let estrangeiro: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let comunitario: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x53d740usize);
        let nacao = *((clube + 0x53) as *const u32);
        match r(0) {
            1 => p != 0 && estrangeiro(p, nacao, 1) != 0 && comunitario(p, nacao) == 0,
            2 => p != 0 && estrangeiro(p, nacao, 1) != 0,
            3 => p != 0 && r(1) < *((p + 0x18) as *const i8),
            _ => false,
        }
    };
    let (nt, nb) = (nt as u8 as i8, nb as u8 as i8);
    let mut nr = nr as u8 as i8;
    let res = |k: i8| (reserva as isize + 4 * k as isize) as *mut usize;
    let sai = |k: i32| (saida as isize + 4 * k as isize) as *mut usize;
    let w = |base: usize, k: i32| *((base as isize + 2 * k as isize) as *const u16);
    // tira `alvo` (ou a posição `k`) da reserva, pondo o último no lugar
    let tirar = |k: i8, nr: &mut i8| {
        if (k as i32) < *nr as i32 - 1 {
            *res(k) = *res(*nr - 1);
            *res(*nr - 1) = 0;
        } else {
            *res(k) = 0;
        }
        *nr -= 1;
    };
    let tirar_alvo = |alvo: usize, nr: &mut i8| {
        let mut k = 0i8;
        if 0 < *nr {
            loop {
                if *res(k) == alvo {
                    tirar(k, nr);
                    break;
                }
                k += 1;
                if k >= *nr {
                    break;
                }
            }
        }
    };
    let marca = if lado != 0 { *((*((lado + 0x2b) as *const usize) + 0x584) as *const u32) } else { 0 };
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserva == 0 {
                break;
            }
            if restrito(*res(k)) {
                let ultimo = (k as i32) < nr as i32 - 1;
                tirar(k, &mut nr);
                if ultimo {
                    k -= 1;
                }
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
    }
    let mut vaga = [0i8; 52];
    let mut cod = [0u16; 50];
    let mut cod2 = [0u16; 50];
    let mut m8 = [0u32; 100];
    let mut b1 = [0u8; 52];
    let mut cand = [0usize; 50];
    let mut escolha = [0usize; 50];
    let mut falta = r(5);
    if falta < r(2) {
        let mut trocas = r(2) - falta;
        falta = r(3);
        if trocas != 0 {
            let mut nv = 0i8;
            let mut k = 0i8;
            while k < nt {
                if restrito(*((starters + 4 * k as usize) as *const usize)) {
                    let i = nv as usize;
                    vaga[i] = k;
                    cod[i] = w(codigos, k as i32);
                    cod2[i] = if curtos == 0 { 0 } else { w(curtos, k as i32) };
                    (m8[2 * i], m8[2 * i + 1]) = if marcas == 0 { (0, 0) } else { (*((marcas + 8 * k as usize) as *const u32), *((marcas + 8 * k as usize + 4) as *const u32)) };
                    b1[i] = if bytes == 0 { 0 } else { *((bytes + k as usize) as *const u8) };
                    nv += 1;
                }
                k += 1;
            }
            let mut nc = 0i8;
            for k in 0..nb.max(0) as usize {
                let q = *((banco + 4 * k) as *const usize);
                if q != 0 && !restrito(q) {
                    cand[nc as usize] = q;
                    nc += 1;
                }
            }
            let mut k = 0i8;
            if 0 < nr {
                loop {
                    if reserva == 0 {
                        break;
                    }
                    cand[nc as usize] = *res(k);
                    nc += 1;
                    k += 1;
                    if k >= nr {
                        break;
                    }
                }
            }
            if nc != 0 {
                pick(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, escolha.as_mut_ptr() as usize, modo, clube, lado,
                    tecnico as u32, 0, m8.as_ptr() as usize, b1.as_ptr() as usize, marca, cod2.as_ptr() as usize);
                let mut melhor = 0i32;
                let mut saiu_do_banco = false;
                loop {
                    if nv == 0 || nv < 1 {
                        break;
                    }
                    let mut esc = -1i32;
                    for k in 0..nv as usize {
                        if vaga[k] == -1 {
                            continue;
                        }
                        let atual = *sai(vaga[k] as i32);
                        if atual == 0 {
                            continue;
                        }
                        let valer = |p: usize| nota(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, modo, clube, lado, tecnico as u32, u32::MAX,
                            m8[2 * k], m8[2 * k + 1], b1[k] as u32, marca, cod2[k] as u32, 0);
                        let a = valer(atual);
                        let b = if escolha[k] == 0 { -1000000 } else { valer(escolha[k]) };
                        if esc == -1 || a.wrapping_sub(b) < melhor {
                            melhor = a.wrapping_sub(b);
                            esc = k as i32;
                        }
                    }
                    if esc < 0 {
                        break;
                    }
                    let e = esc as usize;
                    *sai(vaga[e] as i32) = escolha[e];
                    tirar_alvo(escolha[e], &mut nr);
                    for k in 0..nb.max(0) as usize {
                        let q = (banco + 4 * k) as *mut usize;
                        if escolha[e] != 0 && *q == escolha[e] {
                            saiu_do_banco = true;
                            *q = 0;
                        }
                    }
                    vaga[e] = -1;
                    trocas -= 1;
                    if trocas == 0 {
                        break;
                    }
                }
                if saiu_do_banco {
                    let mut n = 0i8;
                    for k in 0..nb.max(0) as usize {
                        let q = *((banco + 4 * k) as *const usize);
                        if q != 0 {
                            escolha[n as usize] = q;
                            n += 1;
                        }
                    }
                    for k in 0..nr.max(0) {
                        if *res(k) != 0 {
                            escolha[n as usize] = *res(k);
                            n += 1;
                        }
                    }
                    if n != 0 {
                        pick(escolha.as_ptr() as usize, n as u8 as u32, codigos + 2 * nt as isize as usize, nb as u8 as u32, 0, sai(nt as i32) as usize, modo,
                            clube, lado, tecnico as u32, 0, 0, 0, 0, curtos.wrapping_add(2 * nt as isize as usize));
                    }
                }
            }
        }
    } else {
        falta = r(4) - falta;
    }
    if falta == 0 {
        return;
    }
    let mut nv = 0i8;
    let mut k = 0i8;
    while k < nb {
        let j = nt as i32 + k as i32;
        if restrito(*sai(j)) {
            let i = nv as usize;
            cod[i] = w(codigos, j);
            cod2[i] = w(curtos, j);
            vaga[i] = k;
            nv += 1;
        }
        k += 1;
    }
    let mut nc = 0i8;
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserva == 0 {
                break;
            }
            let q = *res(k);
            if !(0..nb.max(0) as i32).any(|j| q == *sai(j + nt as i32)) {
                cand[nc as usize] = q;
                nc += 1;
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
        if nc != 0 {
            pick(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, escolha.as_mut_ptr() as usize, modo, clube, lado,
                tecnico as u32, 0, 0, 0, 0, cod2.as_ptr() as usize);
            let mut melhor = 0i32;
            loop {
                if nv == 0 || nv < 1 {
                    return;
                }
                let mut esc = -1i32;
                for k in 0..nv as usize {
                    if vaga[k] == -1 {
                        continue;
                    }
                    let atual = *sai(vaga[k] as i32 + nt as i32);
                    if atual == 0 {
                        continue;
                    }
                    let valer = |p: usize| nota(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, modo, clube, lado, tecnico as u32, u32::MAX, 0, 0, 0, 0, cod2[k] as u32, 0);
                    let a = valer(atual);
                    let b = if escolha[k] == 0 { -1000000 } else { valer(escolha[k]) };
                    if esc == -1 || a.wrapping_sub(b) < melhor {
                        melhor = a.wrapping_sub(b);
                        esc = k as i32;
                    }
                }
                if esc < 0 {
                    return;
                }
                let e = esc as usize;
                *sai(vaga[e] as i32 + nt as i32) = escolha[e];
                tirar_alvo(escolha[e], &mut nr);
                vaga[e] = -1;
                falta -= 1;
                if falta == 0 {
                    break;
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_apply_squad_rule(
    clube: usize, tecnico: usize, regra: usize, modo: u32, nt: u32, nb: u32, nr: u32, starters: usize, banco: usize, reserva: usize,
    saida: usize, codigos: usize, curtos: usize, marcas: usize, bytes: usize, lado: usize,
) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, usize)>(%%I0%%);
    if is_recording_top_level() {
        return o(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado);
    }
    let n = |v: u32| (v as u8 as i8).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 4 * (n(nt) + n(nb))));
    if banco != 0 {
        reg.push((banco, 4 * n(nb)));
    }
    if reserva != 0 {
        reg.push((reserva, 4 * n(nr)));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || o(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado),
        || crate::aleatorio::sem_contar(|| apply_squad_rule(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado)),
    );
    PLACAR.registrar(igual, || format!("apply_squad_rule(regra {:x?}) {:x?} x {:x?}", core::slice::from_raw_parts(regra as *const u8, 6), mo, mr));
}
// ---- as notas do jogador do match_man.cpp (0x6e6340 fora da partida, 0x6e2f20 para uma vaga): partes comuns ----

/// A data (dia +0, ano +2) de `a` vem antes da de `b`? (comparação do ano sem sinal, do dia com sinal)
unsafe fn match_man_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (*((a + 2) as *const u16), *((b + 2) as *const u16));
    aa < ab || (aa == ab && *(a as *const i16) < *(b as *const i16))
}

/// Os dias desde a data de `p` (+0x3e) até hoje ([0xae2c90]), contando 365 por ano.
unsafe fn match_man_days_since_date(p: usize) -> i32 {
    let anos = *(0xae2c92 as *const u16) as i32 - *((p + 0x40) as *const u16) as i32;
    anos * 365 - *((p + 0x3e) as *const i16) as i32 + *(0xae2c90 as *const i16) as i32
}

/// O observador `o` (+0x3e) chegou antes da data do jogador `p` (+0x3e) e esta data é de mais de um
/// ano atrás (0x54c8e0 com 365)?
unsafe fn scouted_over_a_year_ago(o: usize, p: usize) -> bool {
    if !match_man_date_is_before(o + 0x3e, p + 0x3e) {
        return false;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> *const [u8; 8] = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    let r = datar(0xae2c90, &mut d, 0x16d);
    match_man_date_is_before(r as usize, p + 0x3e)
}

/// Byte com sinal em `p + k`.
unsafe fn signed_byte_at(p: usize, k: usize) -> i32 {
    *((p + k) as *const i8) as i32
}

/// A 0x7c1e30 das notas ([0xb74350]) da ficha `f`.
unsafe fn player_stat(f: usize, k: u32, c: u32, x: f32) -> f64 {
    let g: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    g(*(0xb74350 as *const usize), f, k, c, x)
}

/// A nota da posição (máscara `m`) pelos atributos da ficha `f` na tabela `tab` (0xa15d50 ou
/// 0xa15cfc). A posição principal tem os casos de `m` igual a 8 ou 0x10 e a exceção de lateral
/// (0x800/0x80) na faixa 0x10; a secundária da 0x6e2f20 não.
unsafe fn position_rating(f: usize, m: u32, masc: u32, tab: usize, principal: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let t = |k: i32| *((tab as isize + 4 * k as isize) as *const i32);
    let m16 = m as u16;
    let sem_ala = m & 0x800 == 0 && m & 0x80 == 0;
    if m & 1 != 0 {
        fb(0xf) * fb(0xf) - 300
    } else if masc & 1 != 0 {
        -200
    } else if m & 2 != 0 {
        t(fb(0x10))
    } else if m & 4 != 0 {
        t(fb(0x11))
    } else if principal && m16 == 8 {
        t(fb(0x16).max(fb(0x11)).max(fb(0x12)).max(fb(0x13)))
    } else if m & 8 != 0 {
        t(if sem_ala { fb(0x12) } else { fb(0x16) })
    } else if m & 0x10 != 0 {
        if !principal || sem_ala {
            if principal && m16 == 0x10 {
                t(fb(0x14).max(fb(0x12)).max(fb(0x13)))
            } else {
                let (a, c, d) = (fb(0x14) - 5, fb(0x13), fb(0x12) - 5);
                let al = if a.max(c) > d { if a > c { a } else { c } } else { d };
                t(al as i8 as i32)
            }
        } else {
            t(fb(0x14).max(fb(0x13)))
        }
    } else if m & 0x20 != 0 {
        if sem_ala {
            let a = fb(0x14) as f64;
            let meio = (fb(0x14) + fb(0x1a)) as f64 * read_f64_at(0x968968);
            t(match_pl_ftol(if a <= meio { meio } else { a }))
        } else {
            t(fb(0x14))
        }
    } else if m & 0x40 != 0 {
        t(fb(0x15))
    } else {
        100
    }
}

/// O ajuste da posição (máscara `m`) pelos atributos de jogo da ficha `f`; 100 sem nenhum.
unsafe fn position_attribute_adjust(f: usize, m: u32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    if m & 0x1000 != 0 {
        (fb(0x1a) - 10) * 10
    } else if m & 0x80 != 0 {
        (fb(0x18) - 10) * 10
    } else if m & 0x100 != 0 {
        fb(0x30) + ((fb(0x18) / 2).max(fb(0x19)) * 5 - 0x3c) * 2
    } else if m & 0x200 != 0 {
        (fb(0x19) - 10) * 10
    } else if m & 0x400 != 0 {
        fb(0x3b) + ((fb(0x17) / 2).max(fb(0x19)) * 5 - 0x3c) * 2
    } else if m & 0x800 != 0 {
        (fb(0x17) - 10) * 10
    } else {
        100
    }
}

/// O acréscimo da média das notas (0x7c1950 x 0,2; com o observador `quem` de mais de um ano no
/// clube do jogador, pesada pelos dias) à soma, por `cont`.
unsafe fn recent_form_bonus(p: usize, f: usize, clube: usize, quem: usize, cont: i8) -> i32 {
    let media: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(0x7c1950usize);
    let mut x = (media(*(0xb74350 as *const usize), f) as f64 * read_f64_at(0x968918)) as f32;
    if clube == *((p + 0x39) as *const usize) && quem != 0 && scouted_over_a_year_ago(quem, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64_at(0x969070) - d) * read_f64_at(0x968eb0) + d * x as f64) * read_f64_at(0x968e10)) as f32;
    }
    let x = x as f64;
    if x > read_f64_at(0x968998) {
        match_pl_ftol((x * read_f64_at(0x96d638) - read_f64_at(0x96d630)) * cont as f64) as i8 as i32
    } else if x > read_f64_at(0x9689a0) {
        match_pl_ftol((x * read_f64_at(0x96d628) - read_f64_at(0x96d620)) * cont as f64) as i8 as i32
    } else {
        0
    }
}

/// O acréscimo da forma (flag 2) à soma: a nota 0x7c1e30(6) de um teto pela média (ou por `l3c`),
/// limitado para os de mais de 33 anos quando `teto_veterano`, pesado pelos dias com o observador
/// `quem`, e as notas 0x7c1e30(1) e 0x7c1e30(6, 0x14, `ultimo`) no clube (ou mesma nação).
#[allow(clippy::too_many_arguments)]
unsafe fn form_bonus(p: usize, f: usize, clube: usize, quem: usize, idade: i8, cont: i8, soma: i32, l3c: i32, teto_veterano: bool, ultimo: u32) -> i32 {
    let notas = *(0xb74350 as *const usize);
    let teto = |v: i32| {
        let y = v as f64 * read_f32_at(0x96d61c) as f64 + read_f32_at(0x96d618) as f64;
        (if y > read_f64_at(0x96d610) { read_f64_at(0x96d610) } else { y }) as f32
    };
    let razao = || {
        let (m10, m18) = (*((notas + 0x10) as *const f64), *((notas + 0x18) as *const f64));
        if m18 == read_f64_at(0x9689a0) { read_f64_at(0x968998) } else { m10 / m18 }
    };
    let mut x = f32::from_bits(0x40d9999a);
    if cont != 0 && soma / cont as i32 > 0x96 {
        x = teto(soma / cont as i32 - 0x96);
    } else if l3c as i16 > 0x96 {
        x = teto(l3c as i16 as i32 - 0x96);
    }
    let mut x = player_stat(f, 6, 0x12, x) as f32;
    if teto_veterano && idade > 0x21 {
        let r = razao() + read_f64_at(0x968fd0);
        if (x as f64) > r {
            x = r as f32;
        }
    }
    if clube == *((p + 0x39) as *const usize) && quem != 0 && scouted_over_a_year_ago(quem, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64_at(0x969070) - d) * read_f64_at(0x968eb0) + x as f64 * d) * read_f64_at(0x968e10)) as f32;
        let fb_p = signed_byte_at(*((p + 0x61) as *const usize), 0xf);
        if idade as i32 > fb_p / 10 + 0x22 && x as f64 > razao() {
            x = ((razao() + x as f64) * read_f32_at(0x968f18) as f64) as f32;
        }
    }
    let xf = x as f64;
    let c = cont as f64;
    let mut soma = soma;
    if xf > read_f64_at(0x968f10) || (xf <= read_f64_at(0x968998) && xf > read_f64_at(0x9689a0)) {
        soma += match_pl_ftol((xf * read_f32_at(0x96af88) as f64 - read_f32_at(0x96d608) as f64) * c) as i8 as i32;
    } else if xf > read_f64_at(0x968998) {
        soma += match_pl_ftol((xf * read_f64_at(0x96afc8) - read_f64_at(0x96d510)) * c) as i8 as i32;
    }
    let dono = *((p + 0x39) as *const usize);
    if clube != 0 && (clube == dono || (dono != 0 && *((clube + 0x53) as *const usize) == *((dono + 0x53) as *const usize))) {
        let xk = (xf * read_f64_at(0x9689d8)) as f32;
        let r = player_stat(f, 1, 0x12, xk);
        if r > read_f64_at(0x968998) {
            soma += match_pl_ftol((r * read_f64_at(0x96d628) - read_f64_at(0x96d600)) * c) as i8 as i32;
        } else if r < xk as f64 {
            soma += match_pl_ftol((r * read_f32_at(0x96d5f8) as f64 / xk as f64 - read_f32_at(0x96d5f8) as f64) * c) as i8 as i32;
        }
        soma += match_pl_ftol((player_stat(f, 6, 0x14, f32::from_bits(ultimo)) * read_f64_at(0x968e30) - read_f64_at(0x96d5f0)) * c);
    }
    soma
}

/// O desconto do histórico do jogador (0x62ed70, +0x28).
unsafe fn injury_history_penalty(p: usize, l4c: i32) -> i32 {
    let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let h = historico(*(0xb63d28 as *const usize), p);
    if h == 0 {
        return l4c;
    }
    let w = core::ptr::read_unaligned((h + 0x28) as *const i16) as i32;
    l4c + if w <= 10 { -2 * w } else { -20 - w }
}

/// A reputação do jogador (0x62c3d0 pelo id).
unsafe fn injury_word_0c(p: usize) -> i16 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(0x62c3d0usize);
    g(*(0xb63d28 as *const usize), *(p as *const u32))
}

/// O fim da flag 0x4000 (comum às duas notas): a idade com +0x100, a soma das metades dos
/// atributos (sem goleiro), +0x2f e o fator do estrangeiro (0x7c6830 e a tabela por continente).
#[allow(clippy::too_many_arguments)]
unsafe fn age_and_foreigner_adjust(p: usize, f: usize, clube: usize, flags: u32, pos: u32, masc: u32, idade: i8, cont: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    if flags & 0x100 != 0 {
        let cl = if masc & 1 != 0 { 0x1d } else { 0x19 };
        let y = read_f64_at(0x968e38) - cont as f64 * read_f64_at(0x968f80);
        if idade as i32 > cl && y > read_f64_at(0x9689a0) {
            l4c -= match_pl_ftol((idade as i32 - cl) as f64 * y);
        }
    }
    if pos & 1 == 0 {
        let s: i32 = (0x10..=0x19).map(|k| fb(k) / 2).sum();
        if s > 0x23 {
            l4c += (s - 20) * (s - 20) / 250;
        }
    }
    l4c += match_pl_ftol((fb(0x2f) - 10) as f64 * read_f64_at(0x96a670));
    let nac = *((clube + 0x53) as *const usize);
    let pais = *((p + 0x1a) as *const usize);
    if nac == 0 || *((nac + 0x77) as *const u8) == *((pais + 0x77) as *const u8) {
        return l4c;
    }
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7c6830usize);
    let mut c = g(*(0xb74350 as *const usize), p, clube, (nac as u32 & !0xff) | *((nac + 0x77) as *const u8) as u32, 1) as u8 as i8;
    if flags & 0x100 != 0 {
        let k = (idade as u8).wrapping_sub((*((p + 0x56) as *const i8) / 2) as u8).wrapping_add(5) as i8;
        let m = read_f64_at(0x96d5d0) - k as f64 * read_f64_at(0x96d0e8);
        c = match_pl_ftol(if c as f64 > m { c as f64 } else { m }) as i8;
    } else {
        let y = l4c as f64 * read_f64_at(0x96d1e0) - read_f64_at(0x9688a0);
        if (c as f64) < y {
            c = match_pl_ftol(y) as i8;
        }
    }
    let b22 = *((p + 0x22) as *const u8) as i32;
    if b22 != 0 && b22 * b22 + core::ptr::read_unaligned((pais + 0x8e) as *const i16) as i32 > 5000 && c <= 5 {
        c = 5;
    }
    if (0..=9).contains(&c) {
        const IGUAL: [usize; 10] = [0x96a6a8, 0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8];
        const OUTRO: [usize; 10] = [0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8, 0x968e50];
        let t = if *((pais + 0x76) as *const u8) == 0xe { &IGUAL } else { &OUTRO };
        l4c = match_pl_ftol(l4c as f64 * read_f64_at(t[c as usize]));
    }
    l4c
}

/// O ajuste pelo contrato (flag 0x2000000): sem clube (+0x39) há mais de 60 dias, o salário (+0x80)
/// do clube contra o do dono e a satisfação (+0x3d do contrato), a idade e o goleiro, a lesão
/// (0x5444a0) e a idade com os jogos (+0x22).
unsafe fn contract_adjust(p: usize, f: usize, clube: usize, contrato: usize, idade: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let mul = |l: i32, k: f64| match_pl_ftol(l as f64 * k);
    let dono = *((p + 0x39) as *const usize);
    if dono == 0 {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) > 0x3c {
            l4c = mul(l4c, read_f64_at(0x969040));
        }
    } else {
        let lim = core::ptr::read_unaligned((dono + 0x80) as *const i16) as i32 - 0x4e2;
        let dx = core::ptr::read_unaligned((clube + 0x80) as *const i16);
        let ax = *((contrato + 0x3d) as *const i8) as i16;
        let cl = idade;
        let k = if dx as i32 > lim {
            let ax = ax.wrapping_add(0x64);
            if ax >= 0x7d {
                None
            } else {
                let mut st = read_f32_at(0x96d248) as f64;
                if dx != 0 {
                    st = lim as f64 / dx as f64 * ax as f64 * read_f32_at(0x96d5d8) as f64;
                    if st > read_f64_at(0x967890) {
                        st = read_f32_at(0x968e70) as f64;
                    } else if st < read_f64_at(0x969040) {
                        st = read_f32_at(0x96d248) as f64;
                    }
                }
                if cl > 0x22 {
                    st *= read_f64_at(0x969040);
                } else if cl > 0x1e {
                    st *= read_f64_at(0x969520);
                } else if cl > 0x1b {
                    st *= read_f64_at(0x9689d0);
                }
                if fb(0xf) == 0x14 {
                    st *= read_f64_at(if cl < 0x12 { 0x969040 } else if cl < 0x14 { 0x969520 } else { 0x9689d0 });
                }
                Some(st)
            }
        } else if ax >= 0x19 || cl <= 0x1e {
            None
        } else {
            let mut st = read_f32_at(if cl > 0x25 { 0x968f18 } else if cl > 0x22 { 0x96d248 } else { 0x96d364 }) as f64;
            if dx > 0x1482 && cl < 0x14 {
                if ax < -0x4b {
                    st = read_f32_at(0x968f18) as f64;
                } else if ax < -0x19 {
                    st = read_f32_at(0x96d320) as f64;
                } else if ax < 0 {
                    st = read_f32_at(0x96d364) as f64;
                }
            }
            if fb(0xf) == 0x14 {
                st *= read_f64_at(if cl < 0x12 { 0x969040 } else if cl < 0x15 { 0x969520 } else { 0x9689d0 });
            }
            Some(st)
        };
        if let Some(st) = k {
            l4c = mul(l4c, st);
        }
        let lesao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x5444a0usize);
        if lesao(p, 0) != 0 {
            l4c = mul(l4c, read_f64_at(0x9689d0));
        }
    }
    let (cl, al) = (idade, *((p + 0x22) as *const u8));
    // Some(0): a nota vira 5
    let fator: Option<usize> = if cl > 0x22 {
        if al == 0 {
            Some(0x9689e0)
        } else if al < 8 {
            Some(0x968968)
        } else if cl > 0x23 && al < 0x19 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x969040)
        } else if al < 0x23 {
            Some(0x969520)
        } else {
            None
        }
    } else if cl > 0x1b {
        if al == 0 {
            Some(0x968968)
        } else if al < 8 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x9689b0)
        } else {
            None
        }
    } else if cl > 0x18 {
        if al == 0 {
            Some(0x96a5d0)
        } else if al < 8 {
            Some(0x969040)
        } else if al < 0xf {
            Some(0x969520)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if cl > 0x15 {
        if al == 0 {
            Some(0x969520)
        } else if al < 6 {
            Some(0x9689d8)
        } else if al < 0xa {
            Some(0x9689d0)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if al > 0xa {
        Some(0)
    } else {
        None
    };
    match fator {
        Some(0) => 5,
        Some(k) => mul(l4c, read_f64_at(k)),
        None => l4c,
    }
}

/// O bônus do observador (flag 0x1000): o conhecimento `conhece` (+0x22 dos jovens, +0x14, +0x0e,
/// +0x1a, +0x1f, +0x13), o gosto do jogador pelo clube e por `quem`, e os atributos de cada faixa de
/// posição; `craque_meia` liga o bônus dos craques também na faixa 0x20 (só na 0x6e6340).
#[allow(clippy::too_many_arguments)]
unsafe fn scout_knowledge_bonus(p: usize, f: usize, clube: usize, conhece: usize, obs: usize, quem: usize, pos: u32, masc: u32, idade: i8, mut l4c: i32, craque_meia: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let kb = |k: usize| signed_byte_at(conhece, k);
    if idade <= 0x17 {
        l4c += match_pl_ftol((kb(0x22) - 10) as f64 * read_f64_at(0x96a670));
    }
    let g = |a: usize| -> unsafe extern "C" fn(usize, usize) -> u32 { core::mem::transmute(a) };
    if g(0x547d30)(p, clube) != 0 {
        let m = l4c as f64 * read_f64_at(0x969040);
        let x = (l4c - 10) as f64;
        l4c = match_pl_ftol(if x < m { x } else { m });
    } else if g(0x547d60)(p, clube) != 0 {
        let m = l4c as f64 * read_f64_at(0x9689c0);
        let x = (l4c + 10) as f64;
        l4c = match_pl_ftol(if x > m { x } else { m });
    }
    if g(0x547840)(p, quem) != 0 {
        l4c -= 50;
    } else if g(0x547810)(p, quem) != 0 {
        let x = l4c as f64 * read_f64_at(0x968928);
        let m = (l4c + 20) as f64;
        l4c = match_pl_ftol(if x > m { x } else { m });
    }
    let f44 = fb(0x44);
    let k14 = kb(0x14);
    let mut ecx = (k14 * (f44 - 10) / 3).min(10) + ((signed_byte_at(p, 0x5b) - 10) * k14 / 3).min(10) + ((signed_byte_at(p, 0x59) - 10) * signed_byte_at(obs, 0x59) / 3).min(10);
    if pos & 1 == 0 {
        let fora = pos & 0x80 == 0 && pos & 0x800 == 0;
        let e = kb(0xe);
        let k1a = kb(0x1a) * fb(0x32);
        let k1f = kb(0x1f) * (f44 - 10);
        // os craques: (+0x26 / 3) x +0x1b x +0x36 / 1500, com +0x26 > 30 e +0x36 > 15
        let craque = |ecx: i32| if fb(0x26) > 0x1e && fb(0x36) > 0xf { ecx + fb(0x26) / 3 * fb(0x1b) * fb(0x36) / 1500 } else { ecx };
        if pos & 6 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 10 + fb(0x26) * e / 500 + fb(0x27) * e / 350 + fb(0x43) * e / 250 + k1f / 8 - 10;
            if fora {
                ecx += fb(0x2e) - 15;
            }
        } else if pos & 8 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 8 + fb(0x26) * e / 325 + fb(0x43) * e / 150 + k1f / 6 + fb(0x27) / 15 - 10;
            if fora {
                ecx += fb(0x3e) / 15 + f44 - 15;
            }
        } else if pos & 0x10 != 0 {
            ecx += k1a / 60 + (fb(0x28) - 10) * e / 5 + fb(0x26) * e / 150 + fb(0x43) * e / 100 + k1f / 6 + fb(0x27) / 10 - 10;
            if fora {
                ecx += fb(0x3e) / 15 + fb(0x37) / 15 + f44 - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
        } else if pos & 0x20 != 0 {
            ecx += k1a / 150 + (fb(0x28) - 10) * e / 4 + k1f / 8 + fb(0x43) * e / 75 + fb(0x26) * e / 150 + fb(0x27) / 8 - 10;
            if fora {
                ecx += fb(0x43) / 15 + fb(0x1a) - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
            if craque_meia {
                ecx = craque(ecx);
            }
        } else if pos & 0x40 != 0 {
            ecx += k1a / 400 + (fb(0x28) - 10) * e / 3 + fb(0x43) * e / 200 + k1f / 10 + fb(0x26) * e / 175 + fb(0x27) / 5 - 10;
            if fora {
                ecx += fb(0x33) / 15 + fb(0x36) + fb(0x40) + fb(0x27) / 15 - 0x14;
                ecx = craque(ecx);
            }
        }
    }
    if masc & 0x40 != 0 && pos & 0x40 != 0 {
        ecx += (fb(0x36) * fb(0x2e) - 200) * kb(0x13) / 500;
    }
    if ecx != 0 {
        l4c += match_pl_ftol((read_f64_at(0x967890) - l4c as f64 * read_f64_at(0x96d580)) * ecx as f64).clamp(-0x50, 0x50);
    }
    l4c
}

/// Na lista de transferências ([0xde1f54], 0x8c7260/0x8c7560 com o clube do contrato, que fica em
/// [0xae1080]): a porcentagem +4 / (+4 + +6) em byte (0 sem nenhum); None fora da lista ou de 70 em
/// diante.
unsafe fn work_permit_appearance_pct(p: usize) -> Option<i8> {
    let lista = *(0xde1f54 as *const usize);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c7260usize);
    let x = busca(lista, p);
    if x == 0 {
        return None;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(*(0xae1050 as *const usize), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c == 0 { 0 } else { *(0xae23b4 as *const usize) + *((c + 4) as *const usize) * 0x245 };
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if pode(lista, p, dono, 1) == 0 {
        return None;
    }
    let (a, b) = (*((x + 4) as *const u16) as i32, *((x + 6) as *const u16) as i32);
    if a + b == 0 {
        return Some(0);
    }
    let bl = (a * 100 / (a + b)) as i8;
    if bl >= 0x46 { None } else { Some(bl) }
}

/// O extra do contrato (flag 0x10000): +0x44 pelas bandeiras +0x45, o tipo (+0x4f >> 4) e a lista
/// de transferências.
unsafe fn contract_extra(p: usize, contrato: usize, flags: u32, l28: i32) -> i32 {
    let e = *((contrato + 0x45) as *const u32);
    let s44 = *((contrato + 0x44) as *const i8) as f64;
    let mut extra = 0i32;
    if e & 0x2808 != 0 {
        let x = s44 * read_f64_at(0x96a670);
        extra = match_pl_ftol(if read_f64_at(0x96d578) < x { read_f64_at(0x96d578) } else { x });
    } else if e != 0 {
        let x = s44 * read_f64_at(0x968e00);
        extra = match_pl_ftol(if read_f64_at(0x96d410) < x { read_f64_at(0x96d410) } else { x });
    }
    let t = (*((contrato + 0x4f) as *const u8) >> 4) as i8 as i32 - 1;
    let lista = match t {
        0 | 1 => {
            extra += if t == 0 { 0x19 } else { 0xf };
            flags & 0x800000 == 0
        }
        2 => {
            if flags & 0x800000 != 0 {
                extra += 10;
                false
            } else {
                let d = (*((contrato + 0x3e) as *const i8) as i32 - *((contrato + 0x3f) as *const i8) as i32) * 5;
                extra += match_pl_ftol(d as f64 + (l28 as i16 as f64 * read_f64_at(0x968e18) - read_f64_at(0x96d570)));
                true
            }
        }
        6 => {
            extra -= 0x14;
            false
        }
        _ => false,
    };
    if lista {
        if let Some(bl) = work_permit_appearance_pct(p) {
            extra += 0x32 - bl as i32 / 2;
        }
    }
    extra
}

/// O vínculo (flag 0x20000) pelo tipo do contrato `c`: -25 a +25, ou pela lista de transferências.
unsafe fn contract_bond(p: usize, c: usize, flags: u32) -> i32 {
    let t = (*((c + 0x4f) as *const u8) >> 4) as i8 as i32 - 1;
    match t {
        0..=2 => {
            if flags & 0x800000 != 0 {
                -25
            } else {
                match work_permit_appearance_pct(p) {
                    Some(bl) => bl as i32 / 2 - 0x4b,
                    None => -25,
                }
            }
        }
        3 | 4 | 7 => 25,
        _ => 0,
    }
}

/// O ajuste pela reputação (0x62c3d0) abaixo de -1000 ou acima de 500.
unsafe fn injury_word_0c_adjust(p: usize, l4c: i32) -> i32 {
    if injury_word_0c(p) < -1000 {
        l4c - match_pl_ftol(injury_word_0c(p) as f64 * read_f64_at(0x96a740) + read_f64_at(0x96afc8))
    } else if injury_word_0c(p) > 0x1f4 {
        l4c + match_pl_ftol(read_f64_at(0x968890) - injury_word_0c(p) as f64 * read_f64_at(0x967888))
    } else {
        l4c
    }
}

/// Os jogos (+0x22, flag 0x100000) ou o clube (flag 4: o dono +0x39 e o +0x69 do treinador dele).
unsafe fn appearances_or_club_adjust(p: usize, f: usize, flags: u32, da_base: bool, l4c: i32) -> i32 {
    if flags & 0x100000 != 0 {
        return l4c - (*((p + 0x22) as *const u8) as i32 * 5).min(0x32);
    }
    if flags & 4 == 0 || !da_base {
        return l4c;
    }
    let dono = *((p + 0x39) as *const usize);
    if dono == 0 {
        return if core::ptr::read_unaligned((f + 9) as *const i16) < 0x1388 { l4c - 0x1e } else { l4c };
    }
    let s = *((dono + 0x57) as *const usize);
    if s == 0 {
        return l4c;
    }
    l4c + match_pl_ftol((core::ptr::read_unaligned((s + 0x69) as *const i16) as i32 - 0x10) as f64 * read_f64_at(0x9688a0))
}

/// No próprio clube (+0x39): a nota sobe 5 ou 5%.
unsafe fn own_club_bonus(p: usize, clube: usize, l4c: i32) -> i32 {
    if *((p + 0x39) as *const usize) != clube {
        return l4c;
    }
    let x = (l4c + 5) as f64;
    let m = l4c as f64 * read_f64_at(0x9689b8);
    match_pl_ftol(if x > m { x } else { m })
}

/// A combinação da nota da posição `l40` com o ajuste `l4c` (só `l40` com as flags 0x800 sozinhas).
fn combine_rating(flags: u32, l40: i32, l4c: i32) -> i32 {
    if flags == 0x800 {
        l40
    } else if l40 < 0 {
        (500 - l4c) * l40
    } else if l4c < 1 {
        l4c + l40
    } else {
        l40 * l4c
    }
}

/// { 0x6e6340 } (cdecl, match_man.cpp): a nota do jogador `p` para a posição `pos` fora da partida
/// (transferências, observação), pelas `flags`, para o `clube`, com o `limite` de corte e o
/// observador `o`: a nota da posição (tabela 0xa15d50) vezes o ajuste da reputação, forma,
/// habilidade vista, idade, contrato e conhecimento do observador. -10000 sem ficha (+0x61) ou
/// abaixo do limite.
unsafe extern "C" fn off_match_score(p: usize, pos: u32, flags: u32, clube: usize, limit: i32, o: usize) -> i32 {
    if p == 0 {
        return -10000;
    }
    let f = *((p + 0x61) as *const usize);
    if f == 0 {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f, k);
    let bd = *(0xb63d28 as *const usize);
    let da_base = *(p as *const i32) < *(0xae2364 as *const i32);
    let ativo = limit > -1;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    let sem_valor = flags & 0x800;
    if sem_valor == 0 || flags & 0x210000 != 0 {
        let valor: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x62c590usize);
        l24 = valor(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let idade = *((p + 0x18) as *const i8);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = if *(0xae1050 as *const usize) != 0 { contrato_com(*(0xae1050 as *const usize), p, clube) } else { 0 };
    let (conhece, obs) = if o != 0 {
        (*((o + 0x69) as *const usize), o)
    } else if clube != 0 && *((clube + 0xcf) as *const usize) != 0 {
        let t = *((clube + 0xcf) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5407e0usize);
    let mut l3c = relacao(p, obs, 0);
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5b7d20usize);
    let masc = mascara(f);
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    edi += if flags & 0x400 != 0 { 100 } else { position_rating(f, pos, masc, 0xa15d50, true) };
    let l40 = edi + if flags & 0x200 != 0 { 100 } else { position_attribute_adjust(f, pos) };
    let mut cont: i8 = 0;
    let mut soma: i32 = 0;
    let corte = |soma: i32, cont: i8| ativo && soma * 200 / (cont as i32) < limit;
    if flags & 0x40 != 0 {
        cont = 1;
        soma = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && o != 0 && scouted_over_a_year_ago(o, p) {
            soma = match_pl_ftol((read_f64_at(0x968fa8) - match_man_days_since_date(p) as f64 * read_f64_at(0x96a350)) * soma as f64);
        }
        if ativo && soma * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        soma += match_pl_ftol(a / ((read_f64_at(0x96d640) - b * read_f64_at(0x96a5c0)) * read_f64_at(0x968ff8)));
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        soma += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 4 != 0 && da_base {
        let v = match_pl_ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64_at(0x968ea8));
        if v != 0 {
            cont += 1;
            soma += v;
        }
    }
    let habilidade: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = soma * 50 / cont as i32;
        }
        l34 = habilidade(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        soma += l34 as i16 as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = potential_or_average_value(p, f, clube, soma, cont);
        }
        let mut r = habilidade(obs, p, clube, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        cont += 1;
        soma += r as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    // daqui em diante o original reaproveita o lugar do limite para contas: não corta mais
    if flags & 0x40000 != 0 && da_base {
        soma += recent_form_bonus(p, f, clube, o, cont);
    }
    if flags & 2 != 0 && da_base {
        soma = form_bonus(p, f, clube, o, idade, cont, soma, l3c, false, 0x3fb66666);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = soma / cont as i32;
        if l4c > 0x96 {
            l4c = match_pl_ftol((l4c - 0x96) as f64 * read_f64_at(0x968940) + read_f64_at(0x968f50));
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += match_pl_ftol(fb(0x45) as f64 * read_f64_at(0x968940) - read_f64_at(0x968e08));
    }
    if flags & 0x4000 != 0 {
        let q = (0x1b58 - core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) / 100;
        let k = (if q < 10 { 10 } else { q }) as i8 as i32;
        let lim = if fb(0xf) == 0x14 { 0x24 } else if fb(0x15) == 0x14 || fb(0x14) == 0x14 { 0x22 } else { 0x23 };
        if idade as i32 > lim {
            let y = (read_f64_at(0x967890) - (idade as i32 - lim) as f64 / k as f64) * l4c as f64;
            let m = (l4c - 5) as f64;
            l4c = match_pl_ftol(if y < m { y } else { m });
        }
        let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
        let h = historico(bd, p);
        if h != 0 {
            l4c -= core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32;
        }
        l4c = age_and_foreigner_adjust(p, f, clube, flags, pos, masc, idade, cont, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = contract_adjust(p, f, clube, contrato, idade, l4c);
    }
    if sem_valor == 0 {
        let mut v = l24 as i16 as i32;
        if flags & 0x200000 == 0 && injury_word_0c(p) > 1000 {
            v += 500 - injury_word_0c(p) as i32;
        }
        if v < 5000 {
            return v + (l40 * 5 - 10000) * 20;
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, clube, conhece, obs, o, pos, masc, idade, l4c, true);
    }
    l4c = outside_nation_penalty(p, clube, flags, l4c);
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            l4c += contract_extra(p, contrato, flags, l28);
        }
    } else if flags & 0x20000 != 0 {
        let c = contrato_com(*(0xae1050 as *const usize), p, clube);
        if c != 0 {
            l4c += contract_bond(p, c, flags);
        }
        if sem_valor == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, da_base, l4c);
    if !da_base {
        l4c -= 0x1e;
    }
    l4c = own_club_bonus(p, clube, l4c);
    combine_rating(flags, l40, l4c)
}

/// O valor para a habilidade potencial (flag 0x100): no clube do contrato (0x4cf5a0, que fica em
/// [0xae1080]) a média x 25 mais o potencial (+7) x 25 (em 16 bits); fora dele, a média x 50.
unsafe fn potential_or_average_value(p: usize, f: usize, clube: usize, soma: i32, cont: i8) -> i32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(*(0xae1050 as *const usize), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c == 0 { 0 } else { *(0xae23b4 as *const usize) + *((c + 4) as *const usize) * 0x245 };
    if dono == clube {
        let q = soma * 25 / cont as i32;
        let _ = f;
        let pot = core::ptr::read_unaligned((*((p + 0x61) as *const usize) + 7) as *const i16).wrapping_mul(0x19) as u16 as u32;
        q.wrapping_add(((q as u32 & 0xffff0000) | pot) as i32)
    } else {
        soma * 50 / cont as i32
    }
}

/// A fama (flag 0x200000): -200 para os famosos (valor/100 >= 99) de +0x3c alto, +0x2d baixo e
/// reputação abaixo de 500; senão 100 - valor/100.
unsafe fn fame_adjust(f: usize, p: usize, l28: i32, l4c: i32) -> i32 {
    let v = l28 as i16 as i32;
    if v >= 0x63 && signed_byte_at(f, 0x3c) > 0xe && signed_byte_at(f, 0x2d) < 8 && injury_word_0c(p) < 0x1f4 {
        l4c - 200
    } else {
        l4c + 100 - v
    }
}

/// Fora da nação do clube (flag 0x1000000): -50 sem nenhuma das duas nacionalidades (+0x1a, +0x1e).
unsafe fn outside_nation_penalty(p: usize, clube: usize, flags: u32, l4c: i32) -> i32 {
    if flags & 0x1000000 == 0 || clube == 0 {
        return l4c;
    }
    let nac = *((clube + 0x53) as *const usize);
    if *((p + 0x1a) as *const usize) != nac && *((p + 0x1e) as *const usize) != nac {
        l4c - 50
    } else {
        l4c
    }
}

/// { 0x6e2f20 } (cdecl, match_man.cpp): a nota do jogador `p` (ficha `f`) para uma vaga da escalação
/// (`pos`, e a segunda posição `pos2` com peso 1/3), pelas `flags`, no `clube`, com o lado `lado`
/// (+0x1b observador, +0x1f conhecimento), o técnico `tecnico`, o `limite`, as marcas da tática
/// (`marca` e `bytes`, `tatica`) e a lista das férias `ferias` (16 pares id, posição). Como a
/// 0x6e6340 (tabela 0xa15cfc), mais a condição na partida (`reg`, o jogador da partida), a tática,
/// a idade dos veteranos e o bônus das férias. -10000 sem pessoa ou ficha; -1000000 vetado.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, lado: usize, tecnico: usize, limit: i32, marca: u32, _marca_alta: u32,
    bytes: u32, tatica: u32, pos2: u32, ferias: usize,
) -> i32 {
    if p == 0 || f == 0 {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f, k);
    let bd = *(0xb63d28 as *const usize);
    let da_base = *(p as *const i32) < *(0xae2364 as *const i32);
    let idade = *((p + 0x18) as *const i8);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = if *(0xae1050 as *const usize) != 0 { contrato_com(*(0xae1050 as *const usize), p, clube) } else { 0 };
    let sem_valor = flags & 0x800;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    if sem_valor == 0 || flags & 0x210000 != 0 {
        let valor: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x62c590usize);
        l24 = valor(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let limit = if ferias != 0 { 0 } else { limit };
    let ativo = ferias != 0 || limit > -1;
    let (conhece, obs) = if lado != 0 {
        (*((lado + 0x1f) as *const usize), *((lado + 0x1b) as *const usize))
    } else if tecnico != 0 {
        (*((tecnico + 0x69) as *const usize), tecnico)
    } else if clube != 0 && *((clube + 0xcf) as *const usize) != 0 {
        let t = *((clube + 0xcf) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5407e0usize);
    let mut l3c = relacao(p, obs, 0);
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5b7d20usize);
    let masc = mascara(f);
    let segunda = pos2 as u16 != 0 && pos2 as u16 != pos as u16;
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    let mut ebp: i32 = 0;
    if flags & 0x400 != 0 {
        edi += 100;
    } else {
        edi += position_rating(f, pos, masc, 0xa15cfc, true);
        if segunda {
            ebp = position_rating(f, pos2, masc, 0xa15cfc, false);
        }
    }
    let l40 = if flags & 0x200 != 0 {
        edi + 100
    } else {
        edi += position_attribute_adjust(f, pos);
        if segunda { (ebp + position_attribute_adjust(f, pos2) + 2 * edi) / 3 } else { edi }
    };
    let mut cont: i8 = 0;
    let mut soma: i32 = 0;
    let corte = |soma: i32, cont: i8| ativo && soma * 200 / (cont as i32) < limit;
    if flags & 0x40 != 0 {
        cont = 1;
        soma = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && tecnico != 0 && scouted_over_a_year_ago(tecnico, p) {
            soma = match_pl_ftol((read_f64_at(0x968fa8) - match_man_days_since_date(p) as f64 * read_f64_at(0x96a350)) * soma as f64);
        }
        if ativo && soma * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        soma += match_pl_ftol(a / ((read_f64_at(0x968950) - b * read_f64_at(0x96a5c0)) * read_f64_at(0x968ff8)));
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        soma += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 1 != 0 && reg != 0 && masc & 1 == 0 {
        cont += 1;
        soma += (core::ptr::read_unaligned((reg + 0x35) as *const i16) as i32 - match_fatigue_offset(reg, fb(0xf))) / 50;
    }
    if flags & 4 != 0 && da_base {
        let v = match_pl_ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64_at(0x968ea8));
        if v != 0 {
            cont += 1;
            soma += v;
        }
    }
    let habilidade: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = soma * 50 / cont as i32;
        }
        l34 = habilidade(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        soma += l34 as i16 as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = potential_or_average_value(p, f, clube, soma, cont);
        }
        let mut r = habilidade(obs, p, clube, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        let fp = *((p + 0x61) as *const usize);
        if fp != 0 {
            let (d, c) = (*((fp + 0x1b) as *const i8), *((fp + 0x36) as *const i8));
            r = r.wrapping_sub(if d < 6 { 10 } else if d < 10 { 5 } else { 0 });
            r = r.wrapping_sub(if c < 6 { 15 } else if c < 10 { 10 } else { 0 });
        }
        cont += 1;
        soma += r as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x40000 != 0 && da_base {
        soma += recent_form_bonus(p, f, clube, tecnico, cont);
    }
    if flags & 2 != 0 && da_base {
        soma = form_bonus(p, f, clube, tecnico, idade, cont, soma, l3c, true, 0x3faccccd);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = soma / cont as i32;
        if l4c > 0x96 {
            l4c = match_pl_ftol((l4c - 0x96) as f64 * read_f64_at(0x968940) + read_f64_at(0x968f50));
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += match_pl_ftol((fb(0x45) - 10) as f64 * read_f64_at(0x9689b0));
    }
    let marcada = marca & 0x8000 != 0 || (marca & 0x2000 != 0 && tatica & 0x100 != 0);
    if flags & 0x8000 != 0 {
        l4c = tactic_instruction_adjust(f, marca, bytes as u8, tatica, marcada, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = contract_adjust(p, f, clube, contrato, idade, l4c);
    }
    if flags & 0x4000 != 0 {
        l4c = injury_history_penalty_2c(p, l4c);
        let id = *(p as *const i32);
        let mut edi = l4c;
        if *((*(0xae2d0c as *const usize) as isize + id as isize * 79 + 0xb) as *const u8) & 0x30 != 0 {
            if idade > 0x28 {
                return -1000000;
            }
            edi -= if idade > 0x22 { 0x4b } else { 0x19 };
            l4c = edi;
            if signed_byte_at(*((p + 0x61) as *const usize), 0x3c) < 0xa {
                edi -= 0x32;
                l4c = edi;
            }
        }
        if idade > 0x2a {
            return -1000000;
        }
        let i = idade as i32;
        if i > 0x22 && masc & 1 == 0 {
            l4c = edi + (0x22 - i) * 10;
        } else if i < 0x17 && masc & 1 != 0 {
            l4c = edi + (i - 0x17) * 10;
        } else if i > 0x20 && pos & 0x40 != 0 {
            l4c = edi + (0x20 - i) * 5;
        }
        l4c = age_and_foreigner_adjust(p, f, clube, flags, pos, masc, idade, cont, l4c);
    }
    let mut valor: i32 = 0;
    if sem_valor == 0 {
        let mut v: i32;
        if reg != 0 {
            if *((reg + 0x19) as *const i8) == -2 {
                return -1000000;
            }
            let x = core::ptr::read_unaligned((reg + 0x59) as *const f32) as f64;
            v = match_pl_ftol(x + x + core::ptr::read_unaligned((reg + 0x4d) as *const f32) as f64) as i16 as i32;
            if core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed4) as *const i16) == 0x1ef {
                v = (v + 0x5dc).min(0x2710);
            }
        } else {
            v = l24 as i16 as i32;
        }
        if flags & 0x200000 == 0 && injury_word_0c(p) > 1000 {
            v += 500 - injury_word_0c(p) as i32;
        }
        if v < 0x1676 {
            let r = v + (l40 * 5 - 0x4e2) * 20;
            return if v >= 0xdac { r } else { r - 0x7a120 };
        }
        if marcada {
            v = match_pl_ftol(v as f64 * read_f64_at(0x96a5c0) * v as f64);
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
        valor = v;
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, clube, conhece, obs, tecnico, pos, masc, idade, l4c, false);
    }
    if flags & 0x2000 != 0 && lado != 0 && *((lado + 0x3b) as *const usize) == p {
        l4c += 100;
    }
    l4c = outside_nation_penalty(p, clube, flags, l4c);
    if flags & 0x80000 != 0 && lado != 0 {
        let em = |o: usize| *((lado + o) as *const usize) == p;
        l4c += if em(6) { 0x1e } else if em(0xa) { 0x14 } else if em(0xe) { 0xa } else { 0 };
    }
    // o cansaço do minuto da partida reduz o extra
    let pelo_minuto = |v: i32| {
        if reg == 0 {
            v
        } else {
            let minuto = core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed0) as *const i16) as f64;
            match_pl_ftol((read_f64_at(0x967890) - minuto * read_f64_at(0x96d268)) * v as f64)
        }
    };
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            let mut extra = pelo_minuto(contract_extra(p, contrato, flags, l28));
            if ferias != 0 {
                extra = match_pl_ftol(extra as f64 * read_f64_at(0x9689e0));
            }
            l4c += extra;
        }
    } else if flags & 0x20000 != 0 {
        if contrato != 0 {
            l4c += pelo_minuto(contract_bond(p, contrato, flags));
        }
        if sem_valor == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, da_base, l4c);
    let mut base = l40;
    if !da_base && clube != 0 {
        let nac = *((clube + 0x53) as *const usize);
        if nac != 0 && *((nac + 0x11c) as *const u8) != 0 {
            l4c -= 0x46;
            base = l40 - 0x78;
        }
    }
    l4c = own_club_bonus(p, clube, l4c);
    l4c = combine_rating(flags, base, l4c);
    if ferias != 0 && (pos & 1 == 0 || masc & 1 != 0) {
        l4c = holiday_selection_bonus(p, clube, ferias, pos, base, sem_valor, valor, l4c);
    }
    l4c
}

/// O desconto da flag 0x4000 pelo histórico (0x62ed70, +0x2c).
unsafe fn injury_history_penalty_2c(p: usize, l4c: i32) -> i32 {
    let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let h = historico(*(0xb63d28 as *const usize), p);
    if h == 0 { l4c } else { l4c - core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32 }
}

/// O cansaço do jogador da partida `reg` a descontar do fôlego (+0x35): -2000 para o técnico humano
/// (+0x904d do lado, sem 0x5fd210, seleção ou 0x5fd480) com o jogador abaixo de 11 (+0); fora de
/// campo (+0x19 = -1) os minutos que faltam x 150 (até 3000; goleiro só +3500, os demais pelo menos
/// -2000); em campo, os minutos jogados (minuto - +0x1f) abaixo de 60 x 40.
unsafe fn match_fatigue_offset(reg: usize, goleiro: i32) -> i32 {
    let m = *((reg + 0x19e) as *const usize);
    let humano = if core::ptr::read_unaligned((m + 0x8ed4) as *const i16) == 0x1ef {
        true
    } else {
        let lado = *((reg + 0x27) as *const i8);
        let base = (m as isize + lado as isize * 0xd0) as usize;
        let t = *((base + 0x904d) as *const usize);
        let n = *(0xae2364 as *const i32);
        if t == 0 || *(t as *const i32) < n - 0x10 || *(t as *const i32) >= n || *((t + 0x5f) as *const u8) == 0 {
            false
        } else {
            let tecnicos = *(0xb63c98 as *const usize);
            let g9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
            if g9a01(tecnicos, t) != 0 {
                false
            } else if *((base + 0x904b) as *const u8) != 0 {
                let c = *((*((m + 0x4792) as *const usize) + if lado == 0 { 0x1c } else { 0x20 }) as *const usize);
                let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
                if selecao(c) != 0 {
                    true
                } else {
                    let g9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
                    g9a05(tecnicos, *((base + 0x904d) as *const usize)) != 0
                }
            } else {
                true
            }
        }
    };
    let extra = if humano && *(reg as *const i8) < 0xb { -2000 } else { 0 };
    let estado = *((reg + 0x19) as *const i8);
    let minuto = core::ptr::read_unaligned((m + 0x8ed0) as *const i16) as i32 / 11;
    if estado == -1 {
        let e = extra + ((60 - minuto) * 150).min(3000);
        if goleiro > 1 { e + 0xdac } else { e.max(-2000) }
    } else if estado >= 0 {
        let jogados = (minuto as u8).wrapping_sub(*((reg + 0x1f) as *const u8)) as i8;
        if jogados < 0x3c { extra + (jogados as i32 - 60) * 40 } else { extra }
    } else {
        extra
    }
}

/// As marcas da tática (flag 0x8000): `marca` (o u64 das instruções, parte baixa), `bytes` (o
/// byte das instruções) e `tatica` contra os atributos da ficha.
unsafe fn tactic_instruction_adjust(f: usize, lo: u32, b1: u8, tatica: u32, marcada: bool, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let x = |k: usize, c: usize| match_pl_ftol((fb(k) - 10) as f64 * read_f64_at(c));
    let t13 = tatica as u8;
    let um = lo & 1 != 0;
    if lo & 4 != 0 || (um && t13 & 2 != 0) {
        l4c += fb(0x37) / 7 - x(0x40, 0x96d5e8);
    } else if lo & 8 != 0 || (um && t13 & 4 != 0) {
        l4c += x(0x36, 0x968e00) - x(0x3c, 0x968fe8) - x(0x2e, 0x968fe8);
    } else if lo & 0x10 != 0 || (um && t13 & 8 != 0) {
        l4c += x(0x36, 0x96a670) - x(0x3c, 0x96d5e8) - x(0x2e, 0x96d5e8);
    }
    if marcada {
        let c = fb(0x3c) - 15;
        let d = fb(0x44) - 15;
        let x1 = match_pl_ftol((c.abs() * c) as f64 * read_f64_at(0x968fe8));
        let x2 = match_pl_ftol((d.abs() * d) as f64 * read_f64_at(0x968e00));
        l4c += fb(0x1e) / 10 + (x2 - x1);
    } else if b1 & 1 != 0 {
        l4c += fb(0x33) / 12 + (fb(0x43) / 12 - x(0x28, 0x96d5e8));
    }
    if b1 & 2 != 0 {
        l4c += fb(0x39) / 7;
    } else if b1 & 4 != 0 {
        l4c += fb(0x32) / 7;
    }
    if lo & 0x40 != 0 {
        l4c += match_pl_ftol((fb(0x3b) - 20) as f64 * read_f64_at(0x96d5e0));
    } else if lo & 0x100 != 0 {
        l4c += match_pl_ftol((fb(0x30) - 20) as f64 * read_f64_at(0x96d5e0));
    }
    if lo & 0x200000 != 0 {
        l4c += fb(0x1a) + fb(0x43) / 10 + fb(0x33) / 9 - 20;
    }
    if lo & 0x20000 != 0 {
        l4c += (fb(0x2e) - 10) / 4;
    } else if lo & 0x40000000 != 0 {
        l4c += (10 - fb(0x2e)) / 4;
    }
    if lo & 0x80000000 != 0 {
        l4c += (10 - fb(0x2e)) / 4;
    } else if lo & 0x80000 != 0 {
        l4c += (fb(0x2e) - 10) / 4;
    }
    if lo & 0x800000 != 0 {
        l4c += (fb(0x3c) - 10) / 2 + fb(0x43) / 15 + fb(0x27) / 15;
    }
    if lo & 0x2000000 != 0 {
        l4c += fb(0x26) / 7;
    }
    if lo & 0x8000000 != 0 {
        l4c += (fb(0x40) - 10) / 3 + (fb(0x3d) - 10) / 2;
    }
    if lo & 0x20000000 != 0 {
        l4c += (fb(0x28) - 10) / 2 + fb(0x31) / 14 + fb(0x27) / 14;
    }
    l4c
}

/// O bônus das férias: o jogador na lista (16 pares id, posição) ganha 40000 + 200 x a nota da
/// posição (pelo menos 10000) se estiver entre os 11 e o técnico (0x5fea50) aceitar a posição; senão
/// 200 x a nota (nunca negativa), com a nota negativa pela metade; pesado pelo valor do jogador.
#[allow(clippy::too_many_arguments)]
unsafe fn holiday_selection_bonus(p: usize, clube: usize, ferias: usize, pos: u32, base: i32, sem_valor: u32, valor: i32, mut l4c: i32) -> i32 {
    let id = *(p as *const i32);
    let Some(k) = (0..16usize).find(|&k| *((ferias + 8 * k) as *const i32) == id) else {
        return l4c;
    };
    let aceita = || {
        let g: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(0x5fea50usize);
        if g(*(0xb63c98 as *const usize), 2, *((clube + 0xcf) as *const usize)) == 0 {
            return true;
        }
        let w = *((ferias + 8 * k + 4) as *const i32);
        w == pos as i16 as i32 || w == -1
    };
    let mut bonus;
    if k < 0xb && aceita() {
        bonus = (base * 200).max(10000) + 40000;
        if l4c < 0 {
            l4c = 0;
        }
    } else {
        bonus = (base * 200).max(0);
        if l4c < 0 {
            l4c /= 2;
        }
    }
    if sem_valor == 0 {
        let v = valor as f64;
        let y = v * read_f64_at(0x968eb8);
        let z = if valor >= 0x251c {
            y * read_f64_at(0x96d568)
        } else if valor >= 0x222e {
            y * v * read_f64_at(0x968910)
        } else {
            v * (y * v)
        };
        bonus = match_pl_ftol((z * read_f64_at(0x96a600) - read_f64_at(0x967890)) * bonus as f64);
    }
    if bonus > 0 {
        l4c += bonus;
    }
    l4c
}

unsafe extern "C" fn comparar_off_match_score(p: usize, pos: u32, flags: u32, clube: usize, limit: i32, o: usize) -> i32 {
    let f = original_fn::<unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32>(%%I1%%);
    if is_recording_top_level() {
        return f(p, pos, flags, clube, limit, o);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(p, pos, flags, clube, limit, o), || crate::aleatorio::sem_contar(|| off_match_score(p, pos, flags, clube, limit, o)));
    PLACAR.registrar(igual && r == n, || format!("off_match_score({:#x}, pos {:#x}, flags {:#x}, limit {}) original={} rust={} {:x?} x {:x?}", p, pos as u16, flags, limit, r, n, mo, mr));
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, lado: usize, tecnico: usize, limit: i32, marca: u32, marca_alta: u32,
    bytes: u32, tatica: u32, pos2: u32, ferias: usize,
) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32>(%%I2%%);
    if is_recording_top_level() {
        return o(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias);
    }
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.push((0xae1080, 4));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias),
        || crate::aleatorio::sem_contar(|| lineup_slot_score(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias)),
    );
    PLACAR.registrar(igual && r == n, || format!("lineup_slot_score({:#x}, pos {:#x}/{:#x}, flags {:#x}, marca {:#x}) original={} rust={} {:x?} x {:x?}", p, pos as u16, pos2 as u16, flags, marca, r, n, mo, mr));
    r
}
