// ---------- 266a leva: a copa: os jogos do confronto, a notícia do confronto pendente e os avisos das janelas de transferência (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqc5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4).
unsafe fn cqc5_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc5_d(0xb7431c), clube) != 0
}

/// A data da partida (+0x2a dia, +0x28 ano, pelos dwords que a original empurra: 0x549ef0 push ano; push dia; lea ecx,
/// local -> thiscall, ret 8) que não estiver depois de hoje ([0xae2c90]: ano sem sinal, dia com sinal) passa a amanhã
/// (0x54c5b0: push 1; push saída; mov ecx, 0xae2c90 -> thiscall(data, saída, n), ret 8); e a partida vai ao calendário
/// (0x5aa680: push 1; push &partida; ecx = [0xae2a58] -> thiscall, ret 8).
unsafe fn cqc5_ao_calendario(p: usize) {
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (mut t, mut saida) = ([0u8; 8], [0u8; 8]);
    let f = nova(t.as_mut_ptr() as usize, cqc5_d(p + 0x2a) as u32, cqc5_d(p + 0x28) as u32);
    let (hd, ha) = (cqc5_w(0xae2c90), cqc5_w(0xae2c92) as u16);
    let fa = cqc5_w(f + 2) as u16;
    let antes = ha < fa || (ha == fa && hd < cqc5_w(f));
    if !antes {
        let s = saida.as_mut_ptr() as usize;
        mais(0xae2c90, s, 1);
        cqc5_pw(p + 0x2a, cqc5_w(s) as u16);
        cqc5_pw(p + 0x28, cqc5_w(s + 2) as u16);
    }
    juntar(cqc5_d(0xae2a58), p, 1);
}

/// { 0x51cb90 } (thiscall, ret 0xc, Cup.cpp, quadro SEH sem destrutor no caminho normal: as datas e a notícia locais):
/// `Cup::create_tie_fixtures(rodada, confronto, adiar)`, da montagem da rodada (0x51c800) e da tela da competição.
/// Confronto −1: o primeiro da rodada com a marca +0x34 = 1 (adiar) ou 2 (criar); nenhum: 0. Adiar: marca 2. Criar:
/// marca 3; com time faltando (+0/+2 = −1), a notícia do confronto pendente (0x522950: push confronto; push rodada;
/// mov ecx, esi -> thiscall, ret 8). Senão a partida numa local de 0x4f bytes: a competição (+0x14) e o id (+0, −1
/// sem), −1 em +8, o ano da copa (+0x40) em +0x2c, +0x3a em +0x32, da rodada (0x68 bytes em +0xa3) o código +7 (+3
/// na de duas mãos, +0x21 = 2) em +0x30, +9 em +0x34, +0xb em +0x36, as regras +0xd em +0x38 (sem mão de volta e com
/// 0x800: 0x1000 se o nível (+4 do time) do mandante não é menor que o do visitante, senão 0x800), +0x17 em +0x3a, a
/// liga (+0x44) em +0x42, as marcas (0xff, 1, 0), o virtual +0x7c da copa (push &partida; mov ecx, esi), o bit 0x400
/// de +0x4d (sem mão de volta e +0x20 > 0), os clubes (+0x1c, +0x20) e ids (+0xc, +0x10; −1 sem clube), a data (dia
/// +3, ano +5 + o da copa), a fatia +6, a data da rodada seguinte (+0x6b/+0x6d; na última, 0), o estádio (0x85c260:
/// push +0x24 da copa; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc) e o calendário
/// (cqc5_ao_calendario). Com técnico humano num dos dois e sem o bit 0x100 em +0xd, a notícia 0x14 aos dois. Na de duas
/// mãos, com a copa ainda em [0xadadfc]: a volta +0x22 dias depois (0x549ef0, 0x54c5b0, 0x417c10), o código +4, a
/// fatia pelo dia da semana (0x54a3f0: sábado ou domingo 1, senão 2), mando invertido, o estádio e o calendário.
/// Devolve 1.
unsafe extern "thiscall" fn copa_criar_jogos_do_confronto(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let id = cqc5_d(cqc5_d(this + 4));
    let ri = rodada as i16 as i32;
    let (mut k, mut adiar) = (confronto, adiar);
    if confronto as i16 == -1 {
        let n = cqc5_w(cpq11_rodada(this, ri) + 0x1a);
        let mut i: u32 = 0;
        loop {
            if (i as i16) >= n {
                return 0;
            }
            let m = cqc5_b(cpq11_confronto(this, ri, i as i16 as i32) + 0x34);
            if m == 1 {
                adiar = 1;
                break;
            }
            if m == 2 {
                adiar = 0;
                break;
            }
            i = i.wrapping_add(1);
        }
        k = i;
        if k as i16 == -1 {
            return 0;
        }
    }
    let t = cpq11_confronto(this, ri, k as i16 as i32);
    if adiar != 0 {
        *((t + 0x34) as *mut u8) = 2;
        return 1;
    }
    *((t + 0x34) as *mut u8) = 3;
    if cqc5_w(t) == -1 || cqc5_w(t + 2) == -1 {
        let pendente: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x522950usize);
        pendente(this, rodada, k);
        return 1;
    }
    let mut buf = [0u8; 0x50];
    let p = buf.as_mut_ptr() as usize;
    let comp = cqc5_d(this + 4);
    if comp != 0 {
        cqc5_pd(p + 0x14, comp);
        cqc5_pd(p, cqc5_d(comp));
    } else {
        cqc5_pd(p, usize::MAX);
        cqc5_pd(p + 0x14, 0);
    }
    cqc5_pw(p + 0x2c, cqc5_w(this + 0x40) as u16);
    cqc5_pw(p + 0x32, cqc5_w(this + 0x3a) as u16);
    let e = cpq11_rodada(this, ri);
    let mut codigo = cqc5_w(e + 7);
    if cqc5_b(e + 0x21) == 2 {
        codigo = codigo.wrapping_add(3);
    }
    cqc5_pw(p + 0x30, codigo as u16);
    cqc5_pw(p + 0x34, cqc5_w(e + 9) as u16);
    cqc5_pb(p + 0x3a, cqc5_b(e + 0x17));
    cqc5_pw(p + 0x36, cqc5_w(e + 0xb) as u16);
    // ponytail: a original faz and 0xf4ff na palavra +0x4d de um local sem valor (o +0x4d e os bits 2, 4..7 de
    // +0x4e ficam com a sobra de pilha) e não grava +4, +0x18, +0x2e, +0x2f, +0x3c antes do estádio: aqui, zero
    cqc5_pw(p + 0x38, cqc5_w(e + 0xd) as u16);
    for x in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a] {
        cqc5_pb(p + x, 0xff);
    }
    cqc5_pd(p + 8, usize::MAX);
    cqc5_pb(p + 0x42, cqc5_b(this + 0x44));
    cqc5_pb(p + 0x3d, 0);
    cqc5_pb(p + 0x3e, 0);
    cqc5_pb(p + 0x3b, 0xff);
    cqc5_pb(p + 0x40, 1);
    cqc5_pb(p + 0x41, 1);
    if (cqc5_b(e + 0x21) as i8) < 2 && cqc5_w(p + 0x38) as u16 & 0x800 != 0 {
        let nc = cqc5_b(cpq11_time(this, cqc5_w(t)) + 4) as i8;
        let nf = cqc5_b(cpq11_time(this, cqc5_w(t + 2)) + 4) as i8;
        cqc5_pw(p + 0x38, if nc >= nf { 0x1000 } else { 0x800 });
    }
    let virtual_7c: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqc5_d(cqc5_d(this) + 0x7c));
    virtual_7c(this, p);
    let e = cpq11_rodada(this, ri);
    if (cqc5_b(e + 0x21) as i8) < 2 && (cqc5_b(e + 0x20) as i8) > 0 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    }
    let t = cpq11_confronto(this, ri, k as i16 as i32);
    let (lc, lf) = (cqc5_w(t), cqc5_w(t + 2)); // cqc5: os lados da ida
    let casa = cqc5_d(cpq11_time(this, lc));
    cqc5_pd(p + 0x1c, casa);
    cqc5_pd(p + 0xc, if casa != 0 { cqc5_d(casa) } else { usize::MAX });
    let fora = cqc5_d(cpq11_time(this, lf));
    cqc5_pd(p + 0x20, fora);
    cqc5_pd(p + 0x10, if fora != 0 { cqc5_d(fora) } else { usize::MAX });
    let ano_copa = cqc5_w(this + 0x40);
    cqc5_pw(p + 0x28, (cqc5_b(e + 5) as i8 as i16).wrapping_add(ano_copa) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(e + 3) as u16); // cqc5: o dia da ida
    cqc5_pb(p + 0x3f, cqc5_b(e + 6));
    if ri < cqc5_w(this + 0x3c) as i32 - 1 {
        cqc5_pw(p + 0x26, cqc5_w(e + 0x6b) as u16);
        cqc5_pw(p + 0x24, (cqc5_b(e + 0x6d) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16);
    } else {
        cqc5_pw(p + 0x26, 0);
        cqc5_pw(p + 0x24, 0);
    }
    let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    crate::comparacao::ramo("cqc_5 jogo do confronto");
    cqc5_ao_calendario(p);
    let (casa, fora) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20));
    if (cqc5_humano(casa) || cqc5_humano(fora)) && cqc5_w(cpq11_rodada(this, ri) + 0xd) as u16 & 0x100 == 0 {
        let mut noticia = [0u32; 0x38];
        let n = noticia.as_mut_ptr() as usize;
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        construir(n, 0x14, 0);
        gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
        gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
        gravar(n, 2, cqc5_w(cpq11_rodada(this, ri) + 7) as i32 as u32);
        gravar(n, 5, (cqc5_b(cpq11_rodada(this, ri) + 0x21) == 2) as u32);
        gravar(n, 6, cqc5_d(p + 4) as u32);
        gravar(n, 7, cqc5_w(p + 0x2a) as u16 as u32);
        gravar(n, 8, cqc5_w(p + 0x28) as u16 as u32);
        if cqc5_humano(casa) {
            gravar(n, 3, cqc5_d(p + 0xc) as u32);
            gravar(n, 4, cqc5_d(p + 0x10) as u32);
        } else {
            gravar(n, 3, cqc5_d(p + 0x10) as u32);
            gravar(n, 4, cqc5_d(p + 0xc) as u32);
        }
        entregar(cqc5_d(0xb7431c), n, casa);
        entregar(cqc5_d(0xb7431c), n, fora);
    }
    let e = cpq11_rodada(this, ri);
    if cqc5_d(cqc5_d(0xadadfc) + id * 4) == 0 || cqc5_b(e + 0x21) != 2 {
        return 1;
    }
    crate::comparacao::ramo("cqc_5 jogo de volta");
    if (cqc5_b(e + 0x20) as i8) < 1 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    }
    cqc5_pw(p + 0x30, cqc5_w(e + 7).wrapping_add(4) as u16);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    let (mut a, mut s, mut c, mut x) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let pa = a.as_mut_ptr() as usize;
    // ponytail: a original empurra o dia e o ano em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as
    // palavras
    nova(pa, cqc5_w(e + 3) as u16 as u32, (cqc5_b(e + 5) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16 as u32);
    let r = mais(pa, s.as_mut_ptr() as usize, cqc5_b(e + 0x22) as u32);
    copia(pa, c.as_mut_ptr() as usize, r);
    cqc5_pw(p + 0x28, cqc5_w(pa + 2) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(pa) as u16);
    let dia = nova(x.as_mut_ptr() as usize, cqc5_d(pa) as u32, cqc5_d(pa + 2) as u32);
    let sd = semana(dia) as i16;
    cqc5_pb(p + 0x3f, if sd == 5 || sd == 6 { 1 } else { 2 });
    let (c1, c2, i1, i2) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20), cqc5_d(p + 0xc), cqc5_d(p + 0x10)); // cqc5: os lados da volta
    cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & 0xf4);
    cqc5_pd(p + 0x20, c1);
    cqc5_pd(p + 0xc, i2);
    cqc5_pd(p + 0x10, i1);
    cqc5_pd(p + 0x1c, c2);
    let e = cpq11_rodada(this, ri);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    cqc5_ao_calendario(p);
    1
}

/// (ano, dia) + n dias (o dia vai de 0 a 364, ou 365 no bissexto da 0x549ef0).
fn cqc5_somar((ano, dia): (i32, i32), n: i32) -> (i32, i32) {
    let (mut a, mut d) = (ano, dia + n);
    loop {
        let t = if crate::data::bissexto(a as u16) == 1 { 366 } else { 365 };
        if d < t {
            return (a, d);
        }
        d -= t;
        a += 1;
    }
}

/// Os jogos do clube (id) na competição de hoje até (ano, dia) (0x5acbe0: lista nula, tipo 0xfe e lado 0xff
/// quaisquer), para o comparador. A janela não passa de onde a 0x5aa680 pode pôr os jogos novos + 14 dias: a busca cria
/// o bloco do ano que faltar (0x5abf10).
unsafe fn cqc5_jogos(clube: u32, comp: u32, (ano, dia): (i32, i32)) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    achar(cqc5_d(0xae2a58), cqc5_w(0xae2c90) as u16 as u32, cqc5_w(0xae2c92) as u16 as u32, dia as u32, ano as u32, clube, 0,
        comp, 0xfe, 0xff) & 0xffff
}

/// Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
unsafe extern "thiscall" fn comparar_copa_criar_jogos_do_confronto(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(1521);
    let ri = rodada as i16 as i32;
    let id = cqc5_d(cqc5_d(this + 4)) as u32;
    let (mut k, mut ad) = (confronto as i16 as i32, adiar);
    if k == -1 {
        let n = cqc5_w(cpq11_rodada(this, ri) + 0x1a) as i32;
        k = -1;
        for i in 0..n.max(0) {
            let m = cqc5_b(cpq11_confronto(this, ri, i) + 0x34);
            if m == 1 || m == 2 {
                k = i;
                ad = (m == 1) as u32;
                break;
            }
        }
        if k == -1 {
            let r = o(this, rodada, confronto, adiar);
            PLACAR.registrar(r == 0, || format!("copa_criar_jogos_do_confronto(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = cpq11_confronto(this, ri, k);
    let completo = cqc5_w(t) != -1 && cqc5_w(t + 2) != -1;
    let e = cpq11_rodada(this, ri);
    let duas = cqc5_b(e + 0x21) == 2;
    let casa = if completo { cqc5_d(cpq11_time(this, cqc5_w(t))) } else { 0 };
    let ic = if casa != 0 { cqc5_d(casa) as u32 } else { u32::MAX };
    let conferir = ad == 0 && completo && ic != u32::MAX;
    // o fim da janela: a data da rodada (+ os dias até a volta), amanhã ou a data da rodada seguinte (+0x6b/+0x6d), o
    // que vier depois, + 14 dias. Com conflito, a 0x5aa680 remarca o jogo (0x5ab510): copa (tipo 2) até a data da
    // rodada seguinte (sem ela, 2 semanas), os outros tipos até 2 semanas depois; com mais ligas acontece (a 1a rodada
    // da copa 101, 2001-225, foi a mais de 7 dias)
    let ida = (cqc5_b(e + 5) as i8 as i32 + cqc5_w(this + 0x40) as i32, cqc5_w(e + 3) as i32);
    let ultimo = if duas { cqc5_somar(ida, cqc5_b(e + 0x22) as i32) } else { ida };
    let amanha = cqc5_somar((cqc5_w(0xae2c92) as u16 as i32, cqc5_w(0xae2c90) as i32), 1);
    let seguinte = if ri < cqc5_w(this + 0x3c) as i32 - 1 {
        (cqc5_b(e + 0x6d) as i8 as i32 + cqc5_w(this + 0x40) as i32, cqc5_w(e + 0x6b) as i32)
    } else {
        ultimo
    };
    let fim = cqc5_somar(ultimo.max(amanha).max(seguinte), 14);
    let antes = if conferir { cqc5_jogos(ic, id, fim) } else { 0 };
    let r = o(this, rodada, confronto, adiar);
    let marca = cqc5_b(t + 0x34);
    let mut ok = r == 1 && marca == if ad != 0 { 2 } else { 3 };
    let mut esperado = 0;
    let mut depois = 0;
    if conferir && cqc5_d(cqc5_d(0xadadfc) + id as usize * 4) != 0 {
        esperado = 1 + duas as u32;
        depois = cqc5_jogos(ic, id, fim);
        ok = ok && depois == antes + esperado;
    }
    PLACAR.registrar(ok, || format!("copa_criar_jogos_do_confronto(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, marca, antes, depois, esperado));
    r
}

/// { 0x522950 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: a notícia local):
/// `Cup::post_draw_news(rodada, confronto)`, só da 0x51cb90. Com os dois times definidos, ou rodada (0x68 bytes em
/// +0xa3) com menos de 2 confrontos (+0x1a): nada. Cada lado sem time (+0/+2 = −1) vem do confronto de origem (rodada
/// +0xe/+0x10, confronto +0xa/+0xc; sem origem ou com time faltando lá: nada): os dois clubes possíveis; com time, o
/// clube dele (+0xa7, 6 bytes). A notícia 0x26: 0 = o id da copa, 1 = +0x3a, 2 = o código da rodada (+7), 3/4 = o
/// mandante e −1 ou os dois possíveis, 5/6 = o visitante e −1 ou os dois possíveis; aos técnicos de cada clube dela
/// (0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; os possíveis só se não nulos).
unsafe extern "thiscall" fn copa_noticia_do_confronto_pendente(this: usize, rodada: u32, confronto: u32) {
    let ri = rodada as i16 as i32;
    let t = cpq11_confronto(this, ri, confronto as i16 as i32);
    let (mut a1, mut a2, mut b1, mut b2, mut casa) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let h = cqc5_w(t);
    if h != -1 && cqc5_w(t + 2) != -1 {
        return;
    }
    if cqc5_w(cpq11_rodada(this, ri) + 0x1a) < 2 {
        return;
    }
    if h == -1 {
        let r2 = cqc5_w(t + 0xe);
        if r2 == -1 {
            return;
        }
        let o = cpq11_confronto(this, r2 as i32, cqc5_w(t + 0xa) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        a1 = cqc5_d(cpq11_time(this, x));
        a2 = cqc5_d(cpq11_time(this, y));
    } else {
        casa = cqc5_d(cpq11_time(this, h));
    }
    let fora;
    let v = cqc5_w(t + 2);
    if v == -1 {
        let r2 = cqc5_w(t + 0x10);
        if r2 == -1 {
            return;
        }
        let o = cpq11_confronto(this, r2 as i32, cqc5_w(t + 0xc) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        b1 = cqc5_d(cpq11_time(this, x));
        b2 = cqc5_d(cpq11_time(this, y));
        fora = 0;
    } else {
        fora = cqc5_d(cpq11_time(this, v));
    }
    crate::comparacao::ramo("cqc_5 confronto pendente");
    let mut noticia = [0u32; 0x38];
    let n = noticia.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    construir(n, 0x26, 0);
    gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
    gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
    gravar(n, 2, cqc5_w(cpq11_rodada(this, ri) + 7) as i32 as u32);
    if casa != 0 {
        gravar(n, 3, cqc5_d(casa) as u32);
        gravar(n, 4, u32::MAX);
    } else {
        gravar(n, 3, cqc5_d(a1) as u32);
        gravar(n, 4, cqc5_d(a2) as u32);
    }
    if fora != 0 {
        gravar(n, 5, cqc5_d(fora) as u32);
        gravar(n, 6, u32::MAX);
    } else {
        gravar(n, 5, cqc5_d(b1) as u32);
        gravar(n, 6, cqc5_d(b2) as u32);
    }
    let news = cqc5_d(0xb7431c);
    if casa != 0 {
        entregar(news, n, casa);
    } else {
        if a1 != 0 {
            entregar(news, n, a1);
        }
        if a2 != 0 {
            entregar(news, n, a2);
        }
    }
    if fora != 0 {
        entregar(news, n, fora);
    } else {
        if b1 != 0 {
            entregar(news, n, b1);
        }
        if b2 != 0 {
            entregar(news, n, b2);
        }
    }
}

/// Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
unsafe extern "thiscall" fn comparar_copa_noticia_do_confronto_pendente(this: usize, rodada: u32, confronto: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1522)(this, rodada, confronto) // ponytail: só a notícia
}

/// { 0x8c6bc0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::update_transfer_window_news(hoje)`, do dia das transferências. Os meses anterior, atual e
/// seguinte (0x54cfb0: mov ecx, hoje -> thiscall, ret 0; o anterior de janeiro é dezembro do ano −1, o seguinte de
/// dezembro é janeiro do ano +1), hoje + 1 semana (0x54c770: push 1; push saída; mov ecx, hoje -> thiscall, ret 8) e
/// ontem (0x54c8e0: idem). Para cada janela do vetor [this+0x8a4] (conta +0x8a8, relida): no primeiro dos três meses
/// igual ao dela (+4), a data dela naquele ano (0x40a7d0: push ano; push saída; mov ecx, janela -> thiscall, ret 8;
/// copiada pela 0x417c10). Hoje: o virtual +0x2c das regras da nação dela ([this+0x8ac + 4·(+0)]: push o dword do ano
/// da data; push o código 1, ou 3 se a janela fecha (+5), com os bytes altos do mesmo dword; push janela -> thiscall,
/// ret 0xc); ontem e janela que abre: o virtual com 0 e as ofertas em espera da nação das regras (+0xa; 0x8c4e10: push
/// nação; mov ecx, esi -> thiscall, ret 4); daqui a uma semana e janela que abre: o virtual com 2.
unsafe extern "thiscall" fn janelas_avisos_do_dia(this: usize, hoje: usize) {
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let data_da_janela: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x40a7d0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4e10usize);
    let mut anos = [0i16; 3];
    let mut d = [0u8; 8];
    let pd = d.as_mut_ptr() as usize;
    padrao(pd);
    let mut meses = [0u8; 3];
    meses[0] = mes(hoje).wrapping_sub(1);
    if (meses[0] as i8) < 0 {
        meses[0] = 0xb;
        anos[0] = -1;
    }
    meses[1] = mes(hoje);
    meses[2] = mes(hoje).wrapping_add(1);
    if (meses[2] as i8) > 0xb {
        meses[2] = 0;
        anos[2] = 1;
    }
    let (mut semana, mut ontem) = ([0u8; 8], [0u8; 8]);
    let (ps, po) = (semana.as_mut_ptr() as usize, ontem.as_mut_ptr() as usize);
    mais_semanas(hoje, ps, 1);
    menos(hoje, po, 1);
    let mut v = cqc5_d(this + 0x8a4);
    let mut i: i32 = 0;
    while i < cqc5_d(this + 0x8a8) as i32 {
        for j in 0..3 {
            let w = cqc5_d(v);
            if cqc5_b(w + 4) != meses[j] {
                continue;
            }
            let ano = (anos[j] as u16).wrapping_add(cqc5_w(hoje + 2) as u16);
            let (mut t, mut c) = ([0u8; 8], [0u8; 8]);
            let r = data_da_janela(w, t.as_mut_ptr() as usize, ano as u32);
            copia(pd, c.as_mut_ptr() as usize, r);
            let (hd, ha) = (cqc5_w(hoje), cqc5_w(hoje + 2));
            let (dd, da, dw) = (cqc5_w(pd), cqc5_w(pd + 2), cqc5_d(pd + 2) as u32);
            let regras = |w: usize| cqc5_d(this + 0x8ac + (cqc5_b(w) as i8 as isize * 4) as usize);
            let aviso = |w: usize, codigo: u32| {
                let rg = regras(w);
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(cqc5_d(cqc5_d(rg) + 0x2c));
                f(rg, w, codigo, dw);
            };
            if ha == da && hd == dd {
                let codigo = if cqc5_b(w + 5) != 0 { 3 } else { 1 };
                aviso(w, (dw & 0xffff_ff00) | codigo); // cqc5: o aviso do dia da janela
                break;
            }
            if cqc5_w(po + 2) == da && cqc5_w(po) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 0);
                let nacao = cqc5_d(regras(cqc5_d(v)) + 0xa);
                if nacao != 0 {
                    crate::comparacao::ramo("cqc_5 janela aberta");
                    espera(this, nacao); // cqc5: as ofertas em espera da janela que abriu
                }
                break;
            }
            if cqc5_w(ps + 2) == da && cqc5_w(ps) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 2);
                break;
            }
        }
        i += 1;
        v += 4;
    }
}

/// Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
unsafe extern "thiscall" fn comparar_janelas_avisos_do_dia(this: usize, hoje: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1523)(this, hoje) // ponytail: nada próprio a conferir
}

// ---------- 264a leva: a copa: o sorteio pela semente, o sorteio que separa as nações e o começo da temporada da copa (Cup.cpp) ----------

unsafe fn cqb11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Tira os times dos confrontos da rodada `r` para o vetor de 7 bytes do sorteio (o time +0, o vínculo +2/+4, o
/// nível +6), como nas 0x51d2d0 e 0x51d9f0: o mandante (+0, com +0xe/+0xa) e o visitante (+2, com +0x10/+0xc); com
/// time, a entrada leva o nível do time (+4 do time; a semente), `com_time(vaga)` roda com a vaga ainda intacta e a
/// vaga vira −2 (livre); sem time (−1), a entrada fica −1 com 0xff no nível (o resto da entrada fica como estava).
/// Devolve quantas entradas (a conta de 32 bits, usada em 16).
unsafe fn cqb11_tirar_times(copa: usize, r: i32, vetor: usize, mut com_time: impl FnMut(usize)) -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = cpq11_confronto(copa, r, k as i16 as i32);
            let e = cpq11_em(vetor, n as i16 as i32, 7);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = cqb11_b(cpq11_time(copa, time) + 4); // a semente do time
                com_time(t + lado);
                cpq11_por_w(t + lado, 0xfffe);
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    n
}

/// A entrada `e` do vetor vai ao confronto `cur` da rodada `r`: no mandante (+0, +0xe, +0xa) ou no visitante (+2,
/// +0x10, +0xc); com vínculo (+2 da entrada ≠ −1), o confronto ligado na rodada seguinte ganha a rodada (+0x12), o
/// confronto sorteado (+6) e o lado (+8: 1 mandante, 0 visitante).
unsafe fn cqb11_colocar(copa: usize, r: i32, rodada: u32, e: usize, cur: i32, mandante: bool) {
    let t = cpq11_confronto(copa, r, cur as i16 as i32);
    let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
    cpq11_por_w(t + lado, cpq11_w(e) as u16);
    cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
    cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
    let seguinte = cpq11_w(e + 2);
    if seguinte != -1 {
        let v = cpq11_confronto(copa, seguinte as i32, cpq11_w(e + 4) as i32);
        cpq11_por_w(v + 0x12, rodada as u16);
        cpq11_por_w(v + 6, cur as u16);
        cpq11_por_w(v + 8, mandante as u16);
    }
}

/// Os times `a` e `b` (índices em [copa + 0xa7]) têm clube e são da mesma nação (+0x53).
unsafe fn cqb11_mesma_nacao(copa: usize, a: i16, b: i16) -> bool {
    let c1 = cpq11_us(cpq11_time(copa, a));
    if c1 == 0 {
        return false;
    }
    let c2 = cpq11_us(cpq11_time(copa, b));
    c2 != 0 && cpq11_d(c2 + 0x53) == cpq11_d(c1 + 0x53)
}

/// O passo da volta pelos confrontos: o próximo (em círculo, 16 bits); de volta ao sorteado, a primeira volta vira a
/// segunda (`segunda`), na segunda cai a regra (`regra`: a nação, ou o "sem a mesma nação" da 0x51d9f0) e, sem ela,
/// devolve false (a caixa).
unsafe fn cqb11_passo(copa: usize, r: i32, cur: &mut i32, inicio: i16, segunda: &mut bool, regra: &mut bool) -> bool {
    *cur = cur.wrapping_add(1);
    if (*cur as i16) >= cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        *cur = 0;
    }
    if *cur as i16 == inicio {
        if !*segunda {
            *segunda = true;
        } else if *regra {
            *regra = false;
        } else {
            return false;
        }
    }
    true
}

/// { 0x51d2d0 } (thiscall, ret 4, Cup.cpp): `Cup::sort_round_teams(rodada)`: o sorteio pela semente (regra 0x400 da
/// rodada). 1) Tira os times dos confrontos para o vetor de 7 bytes (malloc 0x944e46 de 7 × vagas +0x18; sem memória,
/// a caixa da linha 0xbf8 e o jogo para), cada um com a semente (+4 do time). 2) Ordena pela semente, a maior primeiro
/// (qsort 0x945333 com 0x51ebc0). 3) Cada entrada com time (até a primeira sem, nível 0xff) vai a um confronto
/// sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira volta, só confrontos com as duas vagas sem
/// time (< 0): com as duas livres (−2), o lado sorteado (0x90cfc0(2) ≠ 0: mandante); só o mandante livre: mandante;
/// só o visitante: visitante. Da segunda volta em diante (a marca não volta a zero), qualquer vaga livre, mas com a
/// regra 4 da rodada (+0xb) o mandante livre não serve quando o visitante do confronto é da mesma nação, e o
/// visitante livre não serve quando o mandante é; dando a volta inteira de novo, cai a regra das nações (também para
/// as entradas seguintes) e, sem ela, a caixa da linha 0xcff e sai. Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn copa_sortear_pela_semente(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // os quatro: push n; add esp, 4 (malloc e sorteio) / push p; add esp, 4 (free) / 4 pushes; add esp, 0x10 (qsort) -> cdecl
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xbf8, 0x9b56dc); // "CUP::sort_round_teams() : Cannot allocate memory"
    }
    let mut nacao = cqb11_b(cpq11_rodada(copa, r) + 0xb) & 4 != 0; // a regra 4 (+0xb) da rodada
    let n = cqb11_tirar_times(copa, r, vetor, |_| {}) as i16;
    crate::comparacao::ramo("cqb_11 ordena pela semente");
    ordenar(vetor, n as i32, 7, 0x51ebc0); // cqb11: pela semente, a maior primeiro
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            let mut lado: Option<bool> = None;
            if segunda {
                let w0 = cpq11_w(e);
                let livre = casa == -2 && !(nacao && w0 >= 0 && fora >= 0 && cqb11_mesma_nacao(copa, w0, fora));
                if fora == -2 {
                    if livre && sorteio(2) != 0 {
                        lado = Some(true);
                    } else if livre || !(nacao && w0 >= 0 && casa >= 0 && cqb11_mesma_nacao(copa, w0, casa)) {
                        lado = Some(false);
                    }
                } else if livre {
                    lado = Some(true);
                }
            } else if casa < 0 && fora < 0 {
                if fora == -2 {
                    if casa == -2 && sorteio(2) != 0 {
                        crate::comparacao::ramo("cqb_11 semente lado sorteado");
                        lado = Some(true); // cqb11: o lado sorteado
                    } else {
                        lado = Some(false);
                    }
                } else if casa == -2 {
                    lado = Some(true);
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut nacao) {
                aviso_em(0x9b55ac, 0xcff);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante);
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_copa_sortear_pela_semente(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1488);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_pela_semente(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_pela_semente(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x51d9f0 } (thiscall, ret 4, Cup.cpp): `Cup::check_round_teams_for_nation(rodada)`: o sorteio que separa as nações
/// (regra 4 da rodada sem a 0x400). Conta os times de cada nação num vetor zerado de [0xae2350] bytes (0x945501;
/// sem memória, a caixa da linha 0xd27 e o jogo para) enquanto tira os times dos confrontos (malloc de 7 × vagas, a
/// caixa da linha 0xd34), pelo +0 da nação (+0x53) do clube; o nível de cada entrada com time vira a conta da nação
/// dele (sem nação, fica a semente); libera as contas e ordena, a maior primeiro (as nações com mais times saem
/// antes). Cada entrada (até a primeira com 0xff) vai a um confronto sorteado, em círculo: na primeira volta, só
/// confrontos sem time nas duas vagas (com as duas livres, o lado sorteado; só uma livre, ela); da segunda em diante
/// (a marca fica), qualquer vaga livre, menos (enquanto valer a regra, que volta a valer a cada entrada) um confronto
/// com um time da mesma nação do clube da entrada; dando a volta inteira de novo, cai a regra e, sem ela, a caixa da
/// linha 0xe5c e sai. Libera o vetor.
unsafe extern "thiscall" fn copa_sortear_separando_nacoes(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free e qsort: add esp, 0x14 cobre os dois -> cdecl
    let contas = zerado(cpq11_d(0xae2350), 1);
    if contas == 0 {
        cpq11_sem_memoria(0xd27, 0x9b5710); // "CUP::check_round_teams_for_nation() : cannot malloc array"
    }
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xd34, 0x9b56dc);
    }
    let n = cqb11_tirar_times(copa, r, vetor, |vaga| {
        let clube = cpq11_us(cpq11_time(copa, cpq11_w(vaga)));
        if clube != 0 {
            let nacao = cpq11_us(clube + 0x53);
            if nacao != 0 {
                let c = cpq11_us(nacao).wrapping_add(contas);
                *(c as *mut u8) = cqb11_b(c).wrapping_add(1);
            }
        }
    }) as i16;
    if n > 0 {
        let mut e = vetor;
        for _ in 0..n as i32 {
            if cpq11_w(e) != -1 {
                // ponytail: sem conferir o clube nulo, como a original (que leria o +0x53 dele)
                let nacao = cpq11_us(cpq11_us(cpq11_time(copa, cpq11_w(e))) + 0x53);
                if nacao != 0 {
                    crate::comparacao::ramo("cqb_11 conta da nacao");
                    *((e + 6) as *mut u8) = cqb11_b(cpq11_us(nacao).wrapping_add(contas)); // cqb11: o nível vira a conta da nação
                }
            }
            e += 7;
        }
    }
    liberar(contas);
    ordenar(vetor, n as i32, 7, 0x51ebc0);
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        let mut regra = true; // volta a valer a cada entrada
        let clube = if cpq11_w(e) != -1 { cpq11_us(cpq11_time(copa, cpq11_w(e))) } else { 0 };
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let mut lado: Option<bool> = None;
            if segunda {
                let mut livre = false;
                let mut m = false;
                let casa = cpq11_w(t);
                let c0 = if casa == -2 {
                    m = true;
                    livre = true;
                    0
                } else if casa == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, casa))
                };
                let fora = cpq11_w(t + 2);
                let c1 = if fora == -2 {
                    if !(livre && sorteio(2) != 0) {
                        m = false;
                        livre = true;
                    }
                    0
                } else if fora == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, fora))
                };
                let mesma = clube != 0 && regra
                    && ((c0 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c0 + 0x53)) || (c1 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c1 + 0x53)));
                if !mesma && livre {
                    lado = Some(m);
                }
            } else {
                let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
                if casa < 0 && fora < 0 {
                    if fora == -2 {
                        lado = Some(casa == -2 && sorteio(2) != 0);
                    } else if casa == -2 {
                        lado = Some(true);
                    }
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut regra) {
                aviso_em(0x9b55ac, 0xe5c);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante); // cqb11: a entrada vai ao confronto sorteado
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
unsafe extern "thiscall" fn comparar_copa_sortear_separando_nacoes(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1489);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_separando_nacoes(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_separando_nacoes(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x518780 } (thiscall, ret 0, Cup.cpp): `Cup::start_new_season()` da classe base: só zera o byte +0xb1 da copa
/// (as copas derivadas liberam as tabelas delas).
unsafe extern "thiscall" fn copa_comeco_da_temporada(copa: usize) {
    *((copa + 0xb1) as *mut u8) = 0;
}

/// Comparador: lado a lado no byte +0xb1.
unsafe extern "thiscall" fn comparar_copa_comeco_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1490);
    let reg = [(copa + 0xb1, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa), || copa_comeco_da_temporada(copa));
    PLACAR.registrar(igual, || format!("copa_comeco_da_temporada({:#x}) {:x?} x {:x?}", copa, mo, mr));
}

// ---------- 263a leva: a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time (Cup.cpp) ----------

unsafe fn cqa1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3).
unsafe fn cqa1_rodada(copa: usize, r: i32) -> usize {
    cqa1_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// A entrada de 6 bytes do time `t` (+0xa7: +0 o clube, +4 o nível, +5 o desfecho).
unsafe fn cqa1_time(copa: usize, t: i32) -> usize {
    cqa1_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` (+0xad: um vetor por rodada).
unsafe fn cqa1_confronto(copa: usize, r: i32, k: i32) -> usize {
    cqa1_d(cqa1_d(copa + 0xad).wrapping_add((r as isize * 4) as usize)).wrapping_add((k as isize * 0x35) as usize)
}

/// O registro de finanças do clube (0x5a0590: push clube; ecx = [0xae2a38]; sem add esp -> thiscall(tabela, clube),
/// ret 4).
unsafe fn cqa1_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cqa1_d(0xae2a38), clube)
}

/// O dinheiro no saldo (0x5999a0: o valor fica na pilha desde antes da 0x5a0590; ecx = as finanças; sem add esp ->
/// thiscall(finanças, valor), ret 4).
unsafe fn cqa1_somar(fin: usize, valor: i32) {
    let f: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    f(fin, valor)
}

/// O ensaio do comparador da 0x522240: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca).
static mut CQA1_ENSAIO: bool = false;

/// A notícia do prêmio (Comp 0x48e3e0: 7 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, valor, alto do
/// valor, −1, rodada seguinte, partida, desfecho), ret 0x1c); no ensaio, nada.
unsafe fn cqa1_noticia(copa: usize, clube: usize, valor: i32, seguinte: u32, partida: usize, desfecho: u8) {
    if CQA1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, u32) = core::mem::transmute(0x48e3e0usize);
    // ponytail: a original empurra o dword de um local de que só grava o byte baixo (os bits altos são sobra de pilha);
    // a notícia só lê o byte (movsx ecx, byte [esp+0x114]): aqui, o byte limpo.
    f(copa, clube, valor, valor >> 31, -1, seguinte, partida, desfecho as u32);
}

/// Os prêmios da rodada `r`: (ao vencedor, ao perdedor), do registro (+0x60/+0x64) ou do fundo (−1); None quando a
/// divisão da original cairia (divisor zero ou estouro). `contar`: o contador do ramo do fundo (não no comparador).
unsafe fn cqa1_premios(copa: usize, r: i32, contar: bool) -> Option<(i32, i32)> {
    let e = cqa1_rodada(copa, r);
    let (mut ao_vencedor, mut ao_perdedor) = (cqa1_i(e + 0x60), cqa1_i(e + 0x64));
    if ao_vencedor == -1 {
        let (fundo, resta, confrontos) = (cqa1_i(copa + 0x28), cqa1_w(copa + 0x3c) as i32 - r, cqa1_w(e + 0x1a) as i32);
        if resta == 0 || confrontos == 0 || (fundo == i32::MIN && resta == -1) {
            return None;
        }
        let q0 = fundo / resta;
        if q0 == i32::MIN && confrontos == -1 {
            return None;
        }
        if contar {
            crate::comparacao::ramo("cqa_1 premio pelo fundo");
        }
        let q = q0 / confrontos; // cqa1: o fundo pelas rodadas que faltam e pelos confrontos
        if ao_perdedor == -1 {
            ao_vencedor = q.wrapping_mul(3) / 4; // idiv por 4 com o arredondamento para zero (cdq; and 3; sar 2)
            ao_perdedor = q / 4;
        } else {
            ao_vencedor = q;
        }
    }
    Some((ao_vencedor, ao_perdedor))
}

/// { 0x522240 } (thiscall, ret 0x10, Cup.cpp): `Cup::pay_round_prize_money(vencedor, perdedor, rodada, partida)`
/// (entradas de 6 bytes dos dois times). Os prêmios da rodada (+0x60 o do vencedor, +0x64 o do perdedor); com −1 no
/// do vencedor, o fundo (+0x28) / (rodadas +0x3c − rodada) / confrontos (+0x1a), e com −1 também no do perdedor, 3/4
/// para o vencedor e 1/4 para o perdedor. O vencedor (se o prêmio não é zero) recebe a notícia (0x48e3e0: o
/// desfecho +5 do time, 3 se era 1 na partida 0x136, e a rodada seguinte: a palavra +7 dela se não é a última nem
/// 0x8c, sobre os 16 bits altos do ponteiro da rodada, como a original deixa no ebp) e o dinheiro (0x5999a0 nas
/// finanças 0x5a0590); o perdedor só o dinheiro; o fundo perde os dois.
unsafe extern "thiscall" fn copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let r = rodada as i16 as i32;
    let (ao_vencedor, ao_perdedor) = match cqa1_premios(copa, r, true) {
        Some(p) => p,
        None => return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(1419)(copa, vencedor, perdedor, rodada, partida), // ponytail: a original cai na divisão; que caia nela
    };
    let mut desfecho = cqa1_b(vencedor + 5);
    if cqa1_w(partida + 0x30) == 0x136 && desfecho == 1 {
        desfecho = 3;
    }
    let e = cqa1_rodada(copa, r);
    let mut seguinte = 0u32;
    if r + 1 < cqa1_w(copa + 0x3c) as i32 {
        let w = core::ptr::read_unaligned((e + 0x6f) as *const u16);
        if w != 0x8c {
            seguinte = (e as u32 & 0xffff_0000) | w as u32; // mov bp, [ebp+0x6f]: só a palavra baixa do ebp muda
        }
    }
    if ao_vencedor != 0 {
        cqa1_noticia(copa, cqa1_d(vencedor), ao_vencedor, seguinte, partida, desfecho);
        crate::comparacao::ramo("cqa_1 premio do vencedor");
        cqa1_somar(cqa1_financas(cqa1_d(vencedor)), ao_vencedor); // cqa1: o prêmio do vencedor
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, cqa1_i(copa + 0x28).wrapping_sub(ao_vencedor));
    }
    if ao_perdedor != 0 {
        cqa1_somar(cqa1_financas(cqa1_d(perdedor)), ao_perdedor);
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, cqa1_i(copa + 0x28).wrapping_sub(ao_perdedor));
    }
}

/// Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
unsafe extern "thiscall" fn comparar_copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(1419);
    let (cv, cp) = (cqa1_d(vencedor), cqa1_d(perdedor));
    if cv == 0 || cp == 0 || cv == cp || cqa1_premios(copa, rodada as i16 as i32, false).is_none() {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem clube ou com a divisão que cai, só a original
    }
    let (fv, fp) = (cqa1_financas(cv), cqa1_financas(cp));
    if fv == 0 || fp == 0 {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem finanças, só a original
    }
    let regioes = [(copa + 0x28, 4), (fv, 0x167), (fp, 0x167)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| copa_premio_da_rodada(copa, vencedor, perdedor, rodada, partida)));
    CQA1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(copa, vencedor, perdedor, rodada, partida);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("copa_premio_da_rodada(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", rodada as i16, cv, cp));
}

/// Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`; o fim é a palavra baixa de laco + 1, como o
/// `lea edx, [eax+1]` da original) ou, com −1, todos os da rodada (+0x1a).
unsafe fn cqa1_faixa(copa: usize, rodada: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, laco.wrapping_add(1) as i16)
    } else {
        (0, cqa1_w(cqa1_rodada(copa, rodada as i16 as i32) + 0x1a))
    }
}

/// Troca mandante e visitante no confronto `t`: os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
unsafe fn cqa1_inverter(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {
        let (x, y) = (cqa1_w(t + a), cqa1_w(t + b));
        core::ptr::write_unaligned((t + a) as *mut i16, y);
        core::ptr::write_unaligned((t + b) as *mut i16, x);
    }
}

/// { 0x51e870 } (thiscall, ret 8, Cup.cpp): `Cup::set_home_teams_for_round(rodada, laco)`: num confronto (`laco`) ou
/// em todos os da rodada (−1), com os dois times definidos (≠ −1), inverte mandante e visitante pela regra da rodada
/// (byte baixo de +0xb): bit 1, se a virtual +0x90 (club_in_higher_division(clube a, clube b)) devolve o clube b;
/// senão bit 8, se o nível (+4 do time, com sinal) de a é menor que o de b; senão, com o estádio da copa (+0x24) ≠
/// −1, se o clube b existe, tem estádio (+0x69) e o id dele (+0) é o da copa.
unsafe extern "thiscall" fn copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let r = rodada as i16 as i32;
    let (ini, fim) = cqa1_faixa(copa, rodada, laco);
    let regra = cqa1_b(cqa1_rodada(copa, r) + 0xb);
    if regra & 1 == 0 && regra & 8 == 0 && cqa1_i(copa + 0x24) == -1 {
        return;
    }
    let mut k = ini as i32;
    while k < fim as i32 {
        let t = cqa1_confronto(copa, r, k);
        let (a, b) = (cqa1_w(t), cqa1_w(t + 2));
        k += 1;
        if a == -1 || b == -1 {
            continue;
        }
        let (ta, tb) = (cqa1_time(copa, a as i32), cqa1_time(copa, b as i32));
        let inverter = if regra & 1 != 0 {
            let (ca, cb) = (cqa1_d(ta), cqa1_d(tb));
            // a virtual +0x90: push cb; push ca; ecx = a copa; sem add esp -> thiscall(copa, ca, cb), ret 8
            let maior: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(cqa1_d(cqa1_d(copa) + 0x90));
            let m = maior(copa, ca, cb);
            m != 0 && m == cb
        } else if regra & 8 != 0 {
            (cqa1_b(ta + 4) as i8) < (cqa1_b(tb + 4) as i8)
        } else {
            let cb = cqa1_d(tb);
            cb != 0 && cqa1_d(cb + 0x69) != 0 && cqa1_i(cqa1_d(cb + 0x69)) == cqa1_i(copa + 0x24) // o estádio da copa
        };
        if inverter {
            cqa1_inverter(t);
        }
    }
}

/// Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
unsafe extern "thiscall" fn comparar_copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1420);
    let (ini, fim) = cqa1_faixa(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((cqa1_confronto(copa, rodada as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada, laco), || copa_mandantes_da_rodada(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("copa_mandantes_da_rodada(rodada {}, laco {}) original={:x?} rust={:x?}", rodada as i16, laco as i16, mo, mr));
}

/// Os argumentos da virtual +0x68 que a 0x521b30 chama para o time `t` na rodada `r`: (p1, p2, p3) ou None.
unsafe fn cqa1_progresso(copa: usize, t: i32, r: i32) -> Option<(u8, u8, u8)> {
    let n = cqa1_w(copa + 0x3c) as i32;
    let d = cqa1_b(cqa1_time(copa, t) + 5); // o desfecho
    let byte_da_rodada = |k: i32, desl: usize| cqa1_b(cqa1_rodada(copa, k) + desl);
    if n > 0 && cqa1_w(cqa1_rodada(copa, n - 1) + 7) == 0x8c {
        // a última rodada é a do terceiro lugar
        if r == n - 2 {
            return match d {
                1 => Some((1, 1, 1)),
                2 => Some((2, 2, 2)),
                _ => None,
            };
        }
        if r == n - 1 {
            return match d {
                3 => Some((3, 3, 3)),
                4 => Some((4, 4, 4)),
                _ => None,
            };
        }
        if d != 0 {
            let v = byte_da_rodada(r, 0x5b);
            return Some((v, v, v));
        }
        if r + 1 == n - 2 {
            return Some((4, 1, 4));
        }
        let v = byte_da_rodada(r + 1, 0x5b);
        return Some((v, 1, v));
    }
    if r == n - 1 {
        return match d {
            1 => Some((1, 1, 1)),
            2 => {
                let v = byte_da_rodada(r, 0x1a).wrapping_add(1);
                Some((v, v, v))
            }
            _ => None,
        };
    }
    if d != 0 {
        let v = byte_da_rodada(r, 0x5b);
        return Some((v, v, v));
    }
    if r + 1 == n - 1 {
        let v = byte_da_rodada(r + 1, 0x1a).wrapping_add(1);
        return Some((v, 1, v));
    }
    let v = byte_da_rodada(r + 1, 0x5b);
    Some((v, 1, v))
}

/// { 0x521b30 } (thiscall, ret 8, Cup.cpp): `Cup::record_team_progress(time, rodada)`: quanto o time foi na copa,
/// pela virtual +0x68 (`set_club_progress(clube, competição +0x44, p1, p2, p3)`). Com a última rodada do tipo 0x8c
/// (terceiro lugar): na semifinal (rodadas − 2), desfecho 1 ou 2 → (d, d, d); na última, 3 ou 4 → (d, d, d); noutra
/// rodada, desfecho ≠ 0 → o +0x5b da rodada três vezes; 0 → (4, 1, 4) antes da semifinal, senão (x, 1, x) com o +0x5b
/// da rodada seguinte. Sem ela: na final, 1 → (1, 1, 1), 2 → os confrontos (+0x1a) da final + 1 três vezes; noutra,
/// desfecho ≠ 0 → o +0x5b três vezes; 0 → (x, 1, x) com os confrontos + 1 da final antes dela, senão o +0x5b da
/// seguinte. Os desfechos fora desses não chamam nada.
unsafe extern "thiscall" fn copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let (t, r) = (time as i16 as i32, rodada as i16 as i32);
    if let Some((p1, p2, p3)) = cqa1_progresso(copa, t, r) {
        // a virtual +0x68: 5 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, comp, p1, p2, p3), ret 0x14
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cqa1_d(cqa1_d(copa) + 0x68));
        // ponytail: a original empurra bytes sobre registradores inteiros (o ebx de quem chamou, o dword do argumento
        // `time` com o byte baixo trocado, 13 × rodada): os bits altos são sobra; as 70 versões da +0x68 só leem o byte
        // baixo de cada um (ou repassam os dwords a outra que lê): aqui, os bytes limpos.
        f(copa, cqa1_d(cqa1_time(copa, t)), cqa1_b(copa + 0x44) as u32, p1 as u32, p2 as u32, p3 as u32);
    }
}

/// Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
unsafe extern "thiscall" fn comparar_copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1421);
    let stats = cqa1_d(copa + 8);
    let clube = cqa1_d(cqa1_time(copa, time as i16 as i32));
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 && clube != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(copa, time, rodada); // ponytail: sem registro para olhar, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(copa, time, rodada), || copa_progresso_do_time(copa, time, rodada));
    PLACAR.registrar(igual, || format!("copa_progresso_do_time(time {}, rodada {}) original={:x?} rust={:x?}", time as i16, rodada as i16, mo, mr));
}

// ---------- 263a leva: o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta (Cup.cpp, comp_util.cpp) ----------

unsafe fn cqa3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3); `r` como a original o usa (o dword inteiro).
unsafe fn cqa3_rodada(copa: usize, r: u32) -> usize {
    cqa3_d(copa + 0xa3).wrapping_add((r as usize).wrapping_mul(0x68))
}

/// Os confrontos da rodada `r` (+0xad: um vetor de confrontos de 0x35 bytes por rodada).
unsafe fn cqa3_confrontos(copa: usize, r: u32) -> usize {
    cqa3_d(cqa3_d(copa + 0xad).wrapping_add((r as usize).wrapping_mul(4)))
}

/// Algum time definido (≠ −1) de algum confronto da rodada ainda sem clube (o vencedor de um confronto anterior em
/// aberto): a rodada não pode ser sorteada.
unsafe fn cqa3_time_em_aberto(copa: usize, r: u32) -> bool {
    let reg = cqa3_rodada(copa, r);
    let mut t = cqa3_confrontos(copa, r);
    let mut k = 0i32;
    while k < cqa3_w(reg + 0x1a) as i32 {
        for lado in [0usize, 2] {
            let x = cqa3_w(t + lado);
            if x != -1 && cqa3_d(cqa3_d(copa + 0xa7).wrapping_add((x as isize * 6) as usize)) == 0 {
                return true;
            }
        }
        t += 0x35;
        k += 1;
    }
    false
}

/// A data do sorteio da rodada (+0 o dia, +2 o ano em byte a partir de +0x40 da copa), numa Data local (0x549ef0:
/// push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8; os dois em registradores com lixo nos bits altos,
/// a Rust só lê as palavras).
unsafe fn cqa3_data_da_rodada(copa: usize, reg: usize, d: *mut [u8; 8]) {
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let ano = (cqa3_b(reg + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40));
    nova(d, cqa3_w(reg) as u16 as u32, ano as u16 as u32);
}

/// Grava a data `d` como a do sorteio da rodada: a palavra do dia e o byte do ano menos o byte baixo de +0x40.
unsafe fn cqa3_gravar_data(copa: usize, reg: usize, d: &[u8; 8]) {
    core::ptr::write_unaligned(reg as *mut [u8; 2], [d[0], d[1]]);
    *((reg + 2) as *mut u8) = d[2].wrapping_sub(cqa3_b(copa + 0x40));
}

/// A data `a` vem depois da `b` (o ano sem sinal, depois o dia com sinal), como a comparação da 0x51c410.
fn cqa3_depois(a: &[u8; 8], b: &[u8; 8]) -> bool {
    let (aa, ab) = (u16::from_le_bytes([a[2], a[3]]), u16::from_le_bytes([b[2], b[3]]));
    aa > ab || (aa == ab && i16::from_le_bytes([a[0], a[1]]) > i16::from_le_bytes([b[0], b[1]]))
}

/// A competição não leva a notícia do sorteio: menos de 2 confrontos, regra 0x200 (+0xd), competição especial das
/// seleções (0x76d940 em [0xb74318]: push comp; ecx = as seleções; sem add esp -> ret 4; a cópia Rust é stdcall(comp))
/// ou amistosa (0x5c6710 em [0xae2c74]: push comp; ecx = o Friendly -> thiscall, ret 4).
unsafe fn cqa3_sem_noticia(copa: usize, r: u32) -> bool {
    let reg = cqa3_rodada(copa, r);
    if cqa3_w(reg + 0x1a) < 2 || cqa3_w(reg + 0xd) & 0x200 != 0 {
        return true;
    }
    if cqa3_d(0xb74318) != 0 {
        let especial: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x76d940usize);
        if especial(cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    if cqa3_d(0xae2c74) != 0 {
        let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistosa(cqa3_d(0xae2c74), cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    false
}

/// { 0x51c410 } (thiscall, ret 4, Cup.cpp, quadro SEH sem destrutor no caminho normal): `Cup::draw_round(rodada)`;
/// devolve 1. Sem +0x45 ou com a rodada já sorteada (+0x34 do primeiro confronto): nada. Com um time de algum confronto
/// ainda sem clube: a data do sorteio anda 1 dia (0x54c5b0, 0x417c10) e as das rodadas seguintes passam a ser, no
/// mínimo, essa. Senão sorteia: regra 0x400 (+0xd) pela semente (0x51d2d0), regra 4 (+0xb) separando as nações
/// (0x51d9f0), sem 0x200 o sorteio livre (0x51e220); os mandantes (0x51e870 com −1); estádio compartilhado (+0x21 = 2,
/// 0x522610); marca os confrontos sorteados (+0x34 = 1). Com notícia (cqa3_sem_noticia): rodada que não é a do
/// terceiro lugar (+7 ≠ 0x8c) soma 1 em +0xac; a notícia 0x27 (a competição, +0x3a, o tipo, +0x44, a rodada, 0, e 1
/// se +0xab + 1 < +0xac) vai a cada um dos times ainda vivos (desfecho 0) entre os primeiros (soma das entradas +0x1c
/// das rodadas até esta) que é de técnico humano, com o id dele no dado 3.
unsafe extern "thiscall" fn copa_sortear_rodada(copa: usize, rodada: u32) -> u32 {
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(cqa3_confrontos(copa, rodada) + 0x34) != 0 {
        return 1;
    }
    let reg = cqa3_rodada(copa, rodada);
    if cqa3_time_em_aberto(copa, rodada) {
        let mais: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> *mut [u8; 8] = core::mem::transmute(0x54c5b0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(0x417c10usize);
        let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        cqa3_data_da_rodada(copa, reg, &mut a);
        crate::comparacao::ramo("cqa_3 sorteio adiado");
        let r = mais(&mut a, &mut b, 1); // cqa3: o sorteio adiado
        copia(&mut a, &mut c, r); // push &b; push &c; ecx = &a -> thiscall, ret 8
        cqa3_gravar_data(copa, reg, &a);
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < cqa3_w(copa + 0x3c) {
            let rk = cqa3_rodada(copa, k as i16 as i32 as u32);
            cqa3_data_da_rodada(copa, rk, &mut b);
            if cqa3_depois(&a, &b) {
                cqa3_gravar_data(copa, rk, &a);
            }
            k = k.wrapping_add(1);
        }
        return 1;
    }
    // os sorteios: push rodada; ecx = a copa; sem add esp -> thiscall, ret 4 (as três)
    let sorteio = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
        g(copa, rodada)
    };
    let regra = cqa3_w(reg + 0xd);
    if regra & 0x400 != 0 {
        crate::comparacao::ramo("cqa_3 sorteio pela semente");
        sorteio(0x51d2d0); // cqa3: pela semente
    } else if cqa3_b(reg + 0xb) & 4 != 0 {
        sorteio(0x51d9f0);
    } else if regra & 0x200 == 0 {
        crate::comparacao::ramo("cqa_3 sorteio livre");
        sorteio(0x51e220); // cqa3: o sorteio livre
    }
    // push −1; push rodada; ecx = a copa -> thiscall, ret 8
    let mandantes: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x51e870usize);
    mandantes(copa, rodada, u32::MAX);
    if cqa3_b(cqa3_rodada(copa, rodada) + 0x21) == 2 {
        sorteio(0x522610);
    }
    let mut k = 0i32;
    while k < cqa3_w(cqa3_rodada(copa, rodada) + 0x1a) as i32 {
        *((cqa3_confrontos(copa, rodada) + k as usize * 0x35 + 0x34) as *mut u8) = 1;
        k += 1;
    }
    if cqa3_sem_noticia(copa, rodada) {
        return 1;
    }
    let reg = cqa3_rodada(copa, rodada);
    if cqa3_w(reg + 7) != 0x8c {
        *((copa + 0xac) as *mut u8) = cqa3_b(copa + 0xac).wrapping_add(1);
    }
    let mut soma: i16 = 0;
    if (rodada as i32) >= 0 {
        for i in 0..=rodada {
            soma = soma.wrapping_add(cqa3_w(cqa3_rodada(copa, i) + 0x1c));
        }
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    construir(n, 0x27, 0); // push 0; push 0x27; ecx = a local -> thiscall, ret 8
    gravar(n, 0, cqa3_d(cqa3_d(copa + 4)) as u32); // push valor; push k; ecx = a local -> thiscall, ret 8
    gravar(n, 1, cqa3_w(copa + 0x3a) as i32 as u32);
    gravar(n, 2, cqa3_w(cqa3_rodada(copa, rodada) + 7) as i32 as u32);
    gravar(n, 4, cqa3_b(copa + 0x44) as i8 as i32 as u32);
    gravar(n, 5, rodada);
    gravar(n, 6, 0);
    gravar(n, 7, ((cqa3_b(copa + 0xab) as i8 as i32 + 1) < cqa3_b(copa + 0xac) as i8 as i32) as u32);
    for i in 0..soma.max(0) as usize {
        let e = cqa3_d(copa + 0xa7) + 6 * i;
        if cqa3_b(e + 5) != 0 || cqa3_d(e) == 0 {
            continue;
        }
        // 0x7875e0: push clube; ecx = [0xb7431c] -> ret 4 (a cópia Rust é stdcall(clube))
        if humano(cqa3_d(e)) == 0 {
            continue;
        }
        gravar(n, 3, cqa3_d(cqa3_d(cqa3_d(copa + 0xa7) + 6 * i)) as u32);
        // push clube; push &local; ecx = [0xb7431c]; sem add esp -> thiscall, ret 8
        entregar(cqa3_d(0xb7431c), n, cqa3_d(cqa3_d(copa + 0xa7) + 6 * i));
    }
    1
}

/// Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
unsafe extern "thiscall" fn comparar_copa_sortear_rodada(copa: usize, rodada: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1425);
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(cqa3_confrontos(copa, rodada) + 0x34) != 0 {
        return o(copa, rodada); // ponytail: nada a sortear, a original não faz nada
    }
    let n = cqa3_w(copa + 0x3c);
    if cqa3_time_em_aberto(copa, rodada) {
        // o plano das datas: (registro, palavra do dia, byte do ano) de cada rodada daqui em diante
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (cqa3_w(cqa3_rodada(copa, rodada)), (cqa3_b(cqa3_rodada(copa, rodada) + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16);
        // a 0x54c5b0 (mais 1): normalizar para a frente, 365 dias no bissexto (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let tam = |y: u16| if crate::data::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > tam(ano) {
            dia = dia.wrapping_sub(tam(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plano = vec![(cqa3_rodada(copa, rodada), dia, a[2].wrapping_sub(cqa3_b(copa + 0x40)))];
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < n {
            let rk = cqa3_rodada(copa, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (cqa3_b(rk + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16;
            b[0..2].copy_from_slice(&cqa3_w(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plano.push(if cqa3_depois(&a, &b) { (rk, dia, a[2].wrapping_sub(cqa3_b(copa + 0x40))) } else { (rk, cqa3_w(rk), cqa3_b(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(copa, rodada);
        let ok = plano.iter().all(|&(rk, d, y)| cqa3_w(rk) == d && cqa3_b(rk + 2) == y);
        PLACAR.registrar(ok, || format!("copa_sortear_rodada(rodada {}) as datas adiadas diferem do plano", rodada as i32));
        return r;
    }
    let antes_ac = cqa3_b(copa + 0xac);
    let r = o(copa, rodada);
    let mut ok = r == 1;
    let reg = cqa3_rodada(copa, rodada);
    for k in 0..cqa3_w(reg + 0x1a).max(0) as usize {
        ok &= cqa3_b(cqa3_confrontos(copa, rodada) + k * 0x35 + 0x34) == 1;
    }
    let soma = if !cqa3_sem_noticia(copa, rodada) && cqa3_w(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= cqa3_b(copa + 0xac) == antes_ac.wrapping_add(soma);
    PLACAR.registrar(ok, || format!("copa_sortear_rodada(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", rodada as i32, antes_ac, cqa3_b(copa + 0xac)));
    r
}

/// { 0x4bffc0 } (cdecl, comp_util.cpp): `comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo)`:
/// sem saída ou com menos de 200 bytes, a caixa (linha 0x26a1) e 0. Zera a saída; com fase e rodada, "{fase} {rodada}"
/// (0x66f4e0 com o modelo 0x9a7cfc, os nomes de 100 bytes da 0x4b4850 e os gêneros da 0x4ba8f0); só uma delas, o nome
/// dela direto na saída (0x4b4850 com o tamanho); nenhuma, só o zero. Devolve 1 (0 na caixa).
unsafe extern "C" fn nome_da_fase_e_rodada(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    if saida.is_null() || tamanho < 0xc8 {
        return orig::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(1426)(fase, rodada, saida, tamanho, estilo); // a caixa
    }
    // 0x4b4850: 4 pushes (estilo, id, tamanho, saída); add esp -> cdecl; 0x4ba8f0: push estilo; push id -> cdecl
    let nome: unsafe extern "C" fn(*mut u8, i32, u32, u32) -> usize = core::mem::transmute(0x4b4850usize);
    let genero: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x4ba8f0usize);
    *saida = 0;
    if fase != 0 && rodada != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        nome(a.as_mut_ptr(), 0x64, fase, estilo);
        let ga = genero(fase, estilo) as i8 as i32 as u32;
        nome(b.as_mut_ptr(), 0x64, rodada, estilo);
        let gb = genero(rodada, estilo) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (saída, modelo, ga, ga, gb, gb, &a, &b)
        let texto: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(0x66f4e0usize);
        texto(saida, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if fase != 0 {
        nome(saida, tamanho, fase, estilo);
    } else if rodada != 0 {
        nome(saida, tamanho, rodada, estilo);
    } else {
        return 0; // nenhuma das duas: 0 (0x4c0136)
    }
    1
}

/// Comparador da 0x4bffc0: lado a lado na saída (até o tamanho, no máximo 0x400 bytes) e na devolução.
unsafe extern "C" fn comparar_nome_da_fase_e_rodada(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    let o = orig::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(1426);
    if saida.is_null() || tamanho < 0xc8 {
        return o(fase, rodada, saida, tamanho, estilo); // a caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida as usize, (tamanho as usize).min(0x400))],
        || o(fase, rodada, saida, tamanho, estilo), || nome_da_fase_e_rodada(fase, rodada, saida, tamanho, estilo));
    PLACAR.registrar(igual && r == n, || format!("nome_da_fase_e_rodada({}, {}) {} x {} {:x?} x {:x?}", fase as i16, rodada as i16, r, n, mo, mr));
    r
}

/// O nível da divisão do clube: a palavra +0x69 da divisão (+0x57), ou 0 sem divisão.
unsafe fn cqa3_nivel(clube: usize) -> i16 {
    let d = cqa3_d(clube + 0x57);
    if d != 0 {
        cqa3_w(d + 0x69)
    } else {
        0
    }
}

/// { 0x522360 } (thiscall, ret 8, Cup.cpp; o this não é lido; a virtual +0x90 da copa base):
/// `Cup::club_in_higher_division(a, b)`: o clube de nível menor (+0x57 → +0x69, com sinal; 0 sem divisão); empate, 0.
unsafe extern "thiscall" fn clube_da_divisao_mais_alta(_this: usize, a: usize, b: usize) -> usize {
    let (na, nb) = (cqa3_nivel(a), cqa3_nivel(b));
    if na < nb {
        a
    } else if na == nb {
        0
    } else {
        b
    }
}

/// Comparador da 0x522360: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_clube_da_divisao_mais_alta(this: usize, a: usize, b: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1427)(this, a, b);
    let n = clube_da_divisao_mais_alta(this, a, b);
    PLACAR.registrar(r == n, || format!("clube_da_divisao_mais_alta({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}

// ---------- 263a leva: a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA (Cup.cpp) ----------

unsafe fn cqa4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3).
unsafe fn cqa4_rodada(copa: usize, r: i32) -> usize {
    cqa4_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// O clube do time `t` (+0xa7, 6 bytes por time, +0 o clube).
unsafe fn cqa4_clube(copa: usize, t: i32) -> usize {
    cqa4_d(cqa4_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize))
}

/// O registro de 6 bytes do estado do clube ([0xae2d14] + id × 6; +0 a competição europeia da temporada).
unsafe fn cqa4_estado(clube: usize) -> usize {
    cqa4_d(0xae2d14).wrapping_add((cqa4_i(clube) as isize * 6) as usize)
}

/// A competição é a amistosa (0x5c6710: push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4).
unsafe fn cqa4_amistosa(comp: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    f(cqa4_d(0xae2c74), comp) != 0
}

/// O índice da nação dona da competição na tabela das chaves (0x669470: push comp; add esp, 4 -> cdecl; devolve o byte).
unsafe fn cqa4_indice(comp: usize) -> u8 {
    let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    f(comp)
}

/// A competição é a copa principal guardada da nação dela ([0xb63d7e + 72 × índice]).
unsafe fn cqa4_copa_principal(comp: usize) -> bool {
    cqa4_d(0xb63d7e_usize.wrapping_add((cqa4_indice(comp) as i8 as isize * 72) as usize)) == comp
}

/// A vaga na Copa da UEFA que o campeão da copa (`copa_registros_pelo_desfecho`, desfecho 1) ganha: a Copa da Liga
/// inglesa ([0x9cf750]) sempre; senão, a copa principal de uma nação europeia ([0x9cfa10] o continente, pela nação
/// +0x5d e o continente +0x71), não amistosa, com o clube ainda sem competição europeia (−1) e a nação com índice.
unsafe fn cqa4_vaga_na_uefa(comp: usize, clube: usize) -> bool {
    if cqa4_i(comp) == cqa4_i(0x9cf750) {
        return true;
    }
    let nacao = cqa4_d(comp + 0x5d);
    if nacao == 0 {
        return false;
    }
    let continente = cqa4_d(nacao + 0x71);
    if continente == 0 || cqa4_i(continente) != cqa4_i(0x9cfa10) || cqa4_amistosa(comp) {
        return false;
    }
    if cqa4_i(cqa4_estado(clube)) != -1 || cqa4_indice(comp) == 0xff {
        return false;
    }
    cqa4_copa_principal(comp)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push final; push jogo; push clube;
/// ecx = [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc).
unsafe fn cqa4_registro_de_final(f: usize, clube: usize, jogo: usize, final_: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cqa4_d(0xdd7f00), clube, jogo, final_);
}

/// { 0x519890 } (thiscall, ret 0x18, Cup.cpp; a virtual +0x44 das copas inglesas e da base):
/// `Cup::update_staff_records_for_status(clube, desfecho, _, _, jogo, final)`; devolve 0. Nada com o jogo +0x3a = 1,
/// com +0x2c ou com +0x44 ≠ 0xff. Desfecho 1: o registro de campeão e, com a vaga (cqa4_vaga_na_uefa), o estado do
/// clube recebe a Copa da UEFA ([0x9cf6f0]); 2: o de vice; 3: o de terceiro; 0: se a rodada seguinte (+0x34 + 1) existe
/// e não é a do terceiro lugar (+7 ≠ 0x8c), a classificação (0x86bdd0) com o peso rodada × 99 / (rodadas − 1) + 1;
/// 0xff: a eliminação (0x86c000) com o peso rodada × 99 / rodadas + 1. Os registros de fase levam os dwords que a
/// original empurra: a palavra do jogo (+0x32, +0x30) ou da rodada (+7) sobre os 16 bits altos do registrador.
unsafe extern "thiscall" fn copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    if cqa4_b(jogo + 0x3a) == 1 || cqa4_d(copa + 0x2c) != 0 || cqa4_b(copa + 0x44) != 0xff {
        return 0;
    }
    let comp = cqa4_d(copa + 4);
    match desfecho as u8 {
        1 => {
            crate::comparacao::ramo("cqa_4 campeao");
            cqa4_registro_de_final(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão
            if cqa4_vaga_na_uefa(comp, clube) {
                core::ptr::write_unaligned(cqa4_estado(clube) as *mut i32, cqa4_i(0x9cf6f0));
            }
        }
        2 => {
            crate::comparacao::ramo("cqa_4 vice");
            cqa4_registro_de_final(0x86b0b0, clube, jogo, final_); // cqa4: o registro de vice
        }
        3 => cqa4_registro_de_final(0x86b710, clube, jogo, final_),
        0 => {
            let r = cqa4_w(jogo + 0x34);
            let n = cqa4_w(copa + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let w7 = core::ptr::read_unaligned((cqa4_rodada(copa, seguinte as i32) + 7) as *const u16);
            if w7 == 0x8c {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let fase = (jogo as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov cx, [ecx+0x32]
            let rodada = (cqa4_d(copa + 0xa3) as u32 & 0xffff_0000) | w7 as u32; // mov di, [edi+...+7]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        0xff => {
            let r = cqa4_w(jogo + 0x34) as i32;
            let n = cqa4_w(copa + 0x3c) as i32;
            if n == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let (q, resto) = ((r * 99) / n, (r * 99) % n);
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32;
            let fase = (peso & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov ax, [ecx+0x32]
            let rodada = (resto as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32; // mov dx, [ecx+0x30]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        _ => {}
    }
    0
}

/// Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
unsafe extern "thiscall" fn comparar_copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428);
    if clube == 0 || jogo == 0 {
        return o(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: sem clube ou jogo, só a original
    }
    let estado = cqa4_estado(clube);
    let antes = cqa4_i(estado);
    let previsto = if cqa4_b(jogo + 0x3a) != 1 && cqa4_d(copa + 0x2c) == 0 && cqa4_b(copa + 0x44) == 0xff && desfecho as u8 == 1
        && cqa4_vaga_na_uefa(cqa4_d(copa + 4), clube) { cqa4_i(0x9cf6f0) } else { antes };
    let r = o(copa, clube, desfecho, a3, a4, jogo, final_);
    let depois = cqa4_i(estado);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("copa_registros_pelo_desfecho(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", desfecho as u8, antes, depois, previsto, r));
    r
}

/// Os quatro da linha do histórico: (campeão, vice, terceiro, seleção anfitriã). O campeão, o vice e o terceiro pelo
/// desfecho dos times (1, 2, 3; até achar os três); na copa galesa ([0x9cf958]) o terceiro é o primeiro time com o
/// progresso +0x73 = 3 no Comp_Stats (0x4a4850 de +8 do objeto da competição em [0xadadfc]: push clube; ecx = as
/// estatísticas -> thiscall, ret 4). A anfitriã: com a virtual +0x88 do objeto da competição (ecx = o objeto; sem push
/// -> thiscall, ret 0) e as anfitriãs do ano (+0x40) da HostCountry ([0xb5e134], 0x5fa730: push 0; push &b; push &a;
/// push ano; push id; ecx = [0xb5e134] -> thiscall, ret 0x14): duas, a seleção principal (0x53b3d0: push nação; add
/// esp, 4 -> cdecl) da de +0x8e maior (empate, a primeira); uma, a da primeira; nenhuma, 0.
unsafe fn cqa4_honras(copa: usize) -> (usize, usize, usize, usize) {
    let n = cqa4_w(copa + 0x3e);
    let (mut campeao, mut vice, mut terceiro) = (0usize, 0usize, 0usize);
    let mut i = 0i16;
    while i < n {
        let e = cqa4_d(copa + 0xa7).wrapping_add((i as isize * 6) as usize);
        match cqa4_b(e + 5) {
            1 => campeao = cqa4_d(e),
            2 => vice = cqa4_d(e),
            3 => terceiro = cqa4_d(e),
            _ => {}
        }
        if campeao != 0 && vice != 0 && terceiro != 0 {
            break;
        }
        i += 1;
    }
    let comp = cqa4_d(copa + 4);
    let objeto = || cqa4_d(cqa4_d(0xadadfc).wrapping_add((cqa4_i(cqa4_d(copa + 4)) as isize * 4) as usize));
    if comp != 0 && cqa4_i(comp) == cqa4_i(0x9cf958) {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let mut j = 0i16;
        while j < cqa4_w(copa + 0x3e) {
            let c = cqa4_clube(copa, j as i32);
            if cqa4_b(busca(cqa4_d(objeto() + 8), c) + 0x73) == 3 {
                terceiro = c;
                break;
            }
            j += 1;
        }
    }
    let o = objeto();
    let tem: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqa4_d(cqa4_d(o) + 0x88));
    let mut anfitria = 0usize;
    if tem(o) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        let achar: unsafe extern "thiscall" fn(usize, i32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x5fa730 recebe um short
        let k = achar(cqa4_d(0xb5e134), cqa4_i(cqa4_d(copa + 4)), cqa4_w(copa + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let nacao = |x: i32| cqa4_d(0xae23a8).wrapping_add((x as isize * 0x122) as usize);
        if k == 2 {
            let (na, nb) = (nacao(a), nacao(b));
            anfitria = if cqa4_w(na + 0x8e) < cqa4_w(nb + 0x8e) { selecao(nb) } else { selecao(na) };
        } else if k != 0 {
            anfitria = selecao(nacao(a));
        }
    }
    (campeao, vice, terceiro, anfitria)
}

/// { 0x519a90 } (thiscall, ret 0, Cup.cpp; a virtual +0x30 das copas inglesas e da base): `Cup::record_season_honours()`:
/// a linha do histórico (0x4afce0: 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã)) com os
/// quatro de cqa4_honras; fora da competição amistosa, na copa principal da nação (cqa4_copa_principal), os
/// semifinalistas para a UEFA (0x519fc0: ecx = a copa -> thiscall, ret 0).
unsafe extern "thiscall" fn copa_honras_da_temporada(copa: usize) {
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let historico: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    historico(copa, campeao, vice, terceiro, anfitria);
    let comp = cqa4_d(copa + 4);
    if !cqa4_amistosa(comp) && cqa4_copa_principal(comp) {
        let semi: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x519fc0usize);
        semi(copa);
    }
}

/// Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (cpa2_linha_livre) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de cqa4_honras. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
unsafe extern "thiscall" fn comparar_copa_honras_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1429);
    let linha = cpa2_linha_livre();
    if linha == 0 || cqa4_d(copa + 4) == 0 {
        return o(copa); // ponytail: o histórico cresce (realloc) ou não há competição: só a original
    }
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    o(copa);
    let lido = (cqa4_d(linha + 4), core::ptr::read_unaligned((linha + 8) as *const u16), cqa4_d(linha + 0xa), cqa4_d(linha + 0xe), cqa4_d(linha + 0x12), cqa4_d(linha + 0x16));
    let previsto = (cqa4_d(copa + 4), ano, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("copa_honras_da_temporada: linha {:x?} prevista {:x?}", lido, previsto));
}

/// Os dois clubes (ids) que a 0x519fc0 manda à semeadura da UEFA, ou None quando ela sai sem mandar: a rodada 0x82
/// (as semifinais) e a 0x96 (a final), as duas achadas; percorre os confrontos da 0x82 e os times que não estão no
/// primeiro confronto da 0x96: o primeiro vira a, o segundo b (aí para); um time −1 (ainda sem confronto) cancela;
/// faltando, −1.
unsafe fn cqa4_semifinalistas(copa: usize) -> Option<(i32, i32)> {
    let (mut semi, mut final_) = (-1i16, -1i16);
    let mut i = 0i16;
    while i < cqa4_w(copa + 0x3c) {
        let t = cqa4_w(cqa4_rodada(copa, i as i32) + 7);
        if t == 0x82 {
            semi = i;
        } else if t == 0x96 {
            final_ = i;
        }
        if semi != -1 && final_ != -1 {
            break;
        }
        i += 1;
    }
    if semi == -1 || final_ == -1 {
        return None;
    }
    let confrontos = |r: i16| cqa4_d(cqa4_d(copa + 0xad).wrapping_add((r as isize * 4) as usize));
    let f0 = confrontos(final_);
    let (fa, fb) = (cqa4_w(f0), cqa4_w(f0 + 2));
    let n = cqa4_w(cqa4_rodada(copa, semi as i32) + 0x1a);
    let base = confrontos(semi);
    let (mut a, mut b) = (-1i32, -1i32);
    let mut k = 0i16;
    'lacos: while k < n {
        let t = base.wrapping_add((k as isize * 0x35) as usize);
        for lado in [0usize, 2] {
            let x = cqa4_w(t + lado);
            if x == fa || x == fb {
                continue;
            }
            if x == -1 {
                return None;
            }
            let id = cqa4_i(cqa4_clube(copa, x as i32));
            if a == -1 {
                a = id;
            } else {
                b = id;
                break 'lacos;
            }
        }
        k += 1;
    }
    Some((a, b))
}

/// { 0x519fc0 } (thiscall, ret 0, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: os semifinalistas fora da
/// final (cqa4_semifinalistas) vão à semeadura da UEFA (0x905790: push b; push a; push id da competição; ecx =
/// [0xde1f58]; sem add esp -> thiscall, ret 0xc).
unsafe extern "thiscall" fn copa_semifinalistas_para_a_uefa(copa: usize) {
    if let Some((a, b)) = cqa4_semifinalistas(copa) {
        let f: unsafe extern "thiscall" fn(usize, i32, i32, i32) = core::mem::transmute(0x905790usize);
        f(cqa4_d(0xde1f58), cqa4_i(cqa4_d(copa + 4)), a, b);
    }
}

/// Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
unsafe extern "thiscall" fn comparar_copa_semifinalistas_para_a_uefa(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1430);
    let s = cqa4_d(0xde1f58);
    if s == 0 {
        return o(copa); // ponytail: sem a semeadura, só a original
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(copa), || copa_semifinalistas_para_a_uefa(copa));
    PLACAR.registrar(igual, || format!("copa_semifinalistas_para_a_uefa(copa {:x}) a semeadura difere ({:?})", copa, cqa4_semifinalistas(copa)));
}

// ---------- 257a leva: a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa (league.cpp, Cup.cpp) ----------

unsafe fn cpu12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de 59 bytes da tabela da liga.
const CPU12_LINHA: usize = 0x3b;

/// Copia um registro de 59 bytes (o `rep movsd` de 14 dwords, a palavra e o byte da original).
unsafe fn cpu12_copiar(destino: usize, origem: usize) {
    core::ptr::copy(origem as *const u8, destino as *mut u8, CPU12_LINHA);
}

/// Põe o par (i, j) da tabela nas vagas k e metade + k da grade, marca as vagas e os dois clubes.
unsafe fn cpu12_par(grade: usize, posto: *mut u32, usado: *mut u32, tab: usize, i: usize, j: usize, k: &mut i32, metade: i32) {
    cpu12_copiar(grade + *k as usize * CPU12_LINHA, tab + i * CPU12_LINHA);
    let m = metade + *k;
    cpu12_copiar(grade + m as usize * CPU12_LINHA, tab + j * CPU12_LINHA);
    *posto.add(m as usize) = 1;
    *posto.add(*k as usize) = 1;
    *usado.add(j) = 1;
    *usado.add(i) = 1;
    *k += 1;
}

/// { 0x686a70 } (thiscall, ret 4, league.cpp): `League::draw_team_order(clubes)`: embaralha a tabela da liga
/// ([+0xb1], +0x3e registros de 59 bytes) para a grade de jogos: guarda um sorteio (0x90cfc0, 30000), semeia o rand
/// da CRT com +0x40 + [0xdd7120] e troca n pares sorteados (rand × n / 0x8000), e volta a semente (sorteio + 1). Liga
/// com a marca 0x100 em +0xd9: cada clube vai à vaga que a tabela virtual +0xa8 diz (se livre), os outros às
/// primeiras livres, e a +0xac recebe cada vaga. Senão: na Escócia ([+4] = [0x9cf5dc]) o Celtic e o Rangers
/// ([0x9d0118], [0x9d011c]) vão às vagas 0 e clubes / 2, em ordem sorteada (0x90cfc0, 2); os clubes do mesmo estádio
/// (+0x69) e depois os de estádios ligados (+0x48 do estádio) vão em pares às vagas k e clubes / 2 + k (passando da
/// metade dos clubes, a caixa); os outros, às primeiras vagas livres. A grade (0x944cf1, livre em 0x944c94) volta à
/// tabela. Sem memória: a caixa (linha 0xed3).
unsafe extern "thiscall" fn sortear_ordem_dos_clubes(this: usize, clubes: i32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(0x9469fausize);
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    let n = || cpu12_w(this + 0x3e) as i32;
    let tab = || cpu12_d(this + 0xb1);
    let metade_n = n() / 2;
    let usado = novo((n() * 4) as u32) as *mut u32;
    let posto = novo((n() << 2) as u32) as *mut u32;
    let grade = novo((n() * CPU12_LINHA as i32) as u32);
    let mut k: i32 = 0;
    if grade == 0 || usado.is_null() || posto.is_null() {
        aviso_em(0x9f15f4, 0xed3);
        liberar(posto as usize);
        liberar(usado as usize);
        liberar(grade);
        return;
    }
    let mut i = 0;
    while i < n() {
        *usado.add(i as usize) = 0;
        *posto.add(i as usize) = 0;
        i += 1;
    }
    let semente = sorteio(30000).wrapping_add(1);
    semear((cpu12_w(this + 0x40) as i32).wrapping_add(*(0xdd7120 as *const i32)) as u32); // cpu12: a semente
    crate::comparacao::ramo("cpu_12 ordem sorteada");
    let mut i = 0;
    while i < n() {
        let a = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let b = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let mut tmp = [0u8; CPU12_LINHA];
        let t = tab();
        core::ptr::copy(( t + a * CPU12_LINHA) as *const u8, tmp.as_mut_ptr(), CPU12_LINHA);
        cpu12_copiar(t + a * CPU12_LINHA, t + b * CPU12_LINHA);
        core::ptr::copy(tmp.as_ptr(), (tab() + b * CPU12_LINHA) as *mut u8, CPU12_LINHA); // cpu12: a troca
        i += 1;
    }
    semear(semente as u32);
    let metade = clubes / 2;
    if cpu12_w(this + 0xd9) as u16 & 0x100 != 0 {
        let mut i = 0;
        while i < n() {
            *((grade + i as usize * CPU12_LINHA) as *mut u32) = 0;
            i += 1;
        }
        let vt = cpu12_d(this);
        let vaga_do: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(cpu12_d(vt + 0xa8));
        let mut i = 0usize;
        while (i as i32) < n() {
            let s = vaga_do(this, cpu12_d(tab() + i * CPU12_LINHA)) as i8 as i32;
            if s >= 0 && s < n() && cpu12_d(grade + s as usize * CPU12_LINHA) == 0 {
                cpu12_copiar(grade + s as usize * CPU12_LINHA, tab() + i * CPU12_LINHA);
                *((tab() + i * CPU12_LINHA) as *mut u32) = 0;
            }
            i += 1;
        }
        let mut i = 0usize;
        while (i as i32) < n() {
            let lin = tab() + i * CPU12_LINHA;
            if cpu12_d(lin) != 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| cpu12_d(grade + j * CPU12_LINHA) == 0) {
                    cpu12_copiar(grade + j * CPU12_LINHA, lin);
                }
            }
            i += 1;
        }
        let gravar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cpu12_d(vt + 0xac));
        let mut i = 0;
        while i < n() {
            gravar(this, cpu12_d(grade + i as usize * CPU12_LINHA), i);
            i += 1;
        }
    } else {
        if cpu12_d(cpu12_d(this + 4)) == cpu12_d(0x9cf5dc) {
            let (mut rangers, mut celtic): (i8, i8) = (-1, -1); // bl e [esp+0x17]
            let total = cpu12_w(this + 0x3e);
            let mut c: i8 = 0;
            while total > 0 && (c as i16) < total {
                let id = cpu12_d(cpu12_d(tab() + c as isize as usize * CPU12_LINHA));
                if id == cpu12_d(0x9d0118) {
                    celtic = c;
                }
                if id == cpu12_d(0x9d011c) {
                    rangers = c;
                }
                c = c.wrapping_add(1);
            }
            if rangers != -1 && celtic != -1 {
                let (primeiro, segundo) = if sorteio(2) != 0 { (celtic, rangers) } else { (rangers, celtic) };
                cpu12_copiar(grade, tab() + primeiro as isize as usize * CPU12_LINHA);
                cpu12_copiar(grade + metade as usize * CPU12_LINHA, tab() + segundo as isize as usize * CPU12_LINHA);
                k = 1;
                *posto.add(metade as usize) = 1;
                *posto = 1;
                *usado.add(celtic as isize as usize) = 1;
                *usado.add(rangers as isize as usize) = 1;
            }
        }
        // os clubes do mesmo estádio (+0x69)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CPU12_LINHA) + 0x69);
                        if ei != 0 {
                            let ej = cpu12_d(cpu12_d(tab() + j * CPU12_LINHA) + 0x69);
                            if ej != 0 && ei == ej {
                                if k >= metade_n {
                                    aviso_em(0x9f15f4, 0xf76);
                                } else {
                                    cpu12_par(grade, posto, usado, tab(), i, j, &mut k, metade);
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os clubes de estádios ligados (+0x48 do estádio de um é o do outro)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CPU12_LINHA) + 0x69);
                        if ei == 0 {
                            break;
                        }
                        let ej = cpu12_d(cpu12_d(tab() + j * CPU12_LINHA) + 0x69);
                        if ej == 0 {
                            break;
                        }
                        if cpu12_d(ei + 0x48) == ej || cpu12_d(ej + 0x48) == ei {
                            if k >= metade_n {
                                aviso_em(0x9f15f4, 0xfa3);
                            } else {
                                cpu12_par(grade, posto, usado, tab(), i, j, &mut k, metade);
                            }
                            break;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os outros, às primeiras vagas livres
        let mut i = 0usize;
        while (i as i32) < n() {
            if *usado.add(i) == 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: a primeira vaga livre
                    crate::comparacao::ramo("cpu_12 clube na vaga livre");
                    cpu12_copiar(grade + j * CPU12_LINHA, tab() + i * CPU12_LINHA);
                    *posto.add(j) = 1;
                    *usado.add(i) = 1;
                }
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < n() {
        cpu12_copiar(tab() + i * CPU12_LINHA, grade + i * CPU12_LINHA);
        i += 1;
    }
    liberar(posto as usize);
    liberar(usado as usize);
    liberar(grade);
}

/// O núcleo da 0x5219b0: o jogo `jogo` da rodada `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35): cada lado (+0 e
/// +2) com clube vai a `saida[*conta]` (e conta sobe); lado vazio (−1) desce ao jogo de onde ele vem (+0xe/+0xa e
/// +0x10/+0xc). Rodada negativa ou jogo −1 (também nas descidas): a caixa (linha 0x1592, se `caixa`) e a conta.
/// Devolve a conta.
unsafe fn cpu12_coletar(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32, caixa: bool) -> i32 {
    if rodada < 0 || jogo == -1 {
        if caixa {
            aviso_em(0x9b55ac, 0x1592);
        }
        return *conta;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let a = cpu12_w(reg);
    if a == -1 {
        cpu12_coletar(this, cpu12_w(reg + 0xe) as i32, cpu12_w(reg + 0xa) as i32, saida, conta, caixa);
    } else {
        *saida.offset(*conta as isize) = a;
        *conta += 1;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let b = cpu12_w(reg + 2);
    if b == -1 {
        cpu12_coletar(this, cpu12_w(reg + 0x10) as i32, cpu12_w(reg + 0xc) as i32, saida, conta, caixa);
        return *conta;
    }
    *saida.offset(*conta as isize) = b;
    *conta += 1;
    *conta
}

/// { 0x5219b0 } (thiscall, ret 0x10, Cup.cpp): `Cup::collect_possible_teams(rodada, jogo, saida, *conta)`: acrescenta
/// a `saida` os clubes que ainda podem jogar esse jogo, descendo pelos jogos de onde vêm os lados vazios. Saída ou
/// conta nula: a original (a caixa).
unsafe extern "thiscall" fn clubes_possiveis_do_jogo(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    if saida.is_null() || conta.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(1229)(this, rodada, jogo, saida, conta);
    }
    cpu12_coletar(this, rodada, jogo, saida, conta, true)
}

/// { 0x521790 } (thiscall, ret 0x10, Cup.cpp): `Cup::get_fixture_side_teams(rodada, jogo, lado, saida)`: o clube de um
/// lado do jogo (lado ≠ 0: +0; 0: +2) ou, vazio, os clubes que podem chegar a ele (0x5219b0 com a conta no global
/// 0xae1880). Devolve a conta (a caixa se nenhum). Rodada fora de 0..+0x3c, copa sem clubes (+0xa7) ou jogo −1: a
/// original (a caixa e 0).
unsafe extern "thiscall" fn clubes_do_lado_do_jogo(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    if rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(1228)(this, rodada, jogo, lado, saida);
    }
    let coletar: unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32 = core::mem::transmute(0x5219b0usize);
    let conta = 0xae1880 as *mut i32;
    *conta = 0;
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    if c != -1 {
        *saida = c;
        *conta = 1;
        return 1;
    }
    coletar(this, cpu12_w(r) as i32, cpu12_w(j) as i32, saida, conta);
    if *conta == 0 {
        aviso_em(0x9b55ac, 0x157f);
    }
    *conta
}

/// Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
unsafe extern "thiscall" fn comparar_sortear_ordem_dos_clubes(this: usize, clubes: i32) {
    // ponytail: sorteia e aloca (não roda duas vezes); a cópia se prova pela carreira (a grade de jogos de toda liga)
    orig::<unsafe extern "thiscall" fn(usize, i32)>(1227)(this, clubes)
}

/// Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
unsafe extern "thiscall" fn comparar_clubes_do_lado_do_jogo(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(1228);
    if saida.is_null() || rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return o(this, rodada, jogo, lado, saida); // ponytail: a caixa, só a original
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    let mut esperado = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        esperado[0] = c;
        n = 1;
    } else {
        cpu12_coletar(this, cpu12_w(r) as i32, cpu12_w(j) as i32, esperado.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, rodada, jogo, lado, saida);
    let obtido = core::slice::from_raw_parts(saida, res.clamp(0, 0x100) as usize);
    let global = *(0xae1880 as *const i32);
    PLACAR.registrar(res == n && global == n && obtido == &esperado[..n.clamp(0, 0x100) as usize], || format!("clubes_do_lado_do_jogo({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, rodada, jogo, lado as u8, res, n, global, obtido, &esperado[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
unsafe extern "thiscall" fn comparar_clubes_possiveis_do_jogo(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(1229);
    if saida.is_null() || conta.is_null() || rodada < 0 || jogo == -1 || !(0..0x100).contains(&*conta) {
        return o(this, rodada, jogo, saida, conta); // ponytail: a caixa ou conta fora do esperado, só a original
    }
    let c0 = *conta;
    let mut esperado = vec![0i16; 0x200];
    let mut n = c0;
    cpu12_coletar(this, rodada, jogo, esperado.as_mut_ptr(), &mut n, false);
    let res = o(this, rodada, jogo, saida, conta);
    let fim = (*conta).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(saida.offset(c0 as isize), (fim - c0) as usize);
    let ok = res == n && *conta == n && obtido == &esperado[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.registrar(ok, || format!("clubes_possiveis_do_jogo({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, rodada, jogo, res, n, *conta, obtido, &esperado[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

// ---------- 255a leva: o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes (Cup.cpp, fix_man.cpp) ----------

unsafe fn cps13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps13_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Uma data montada pela 0x549ef0 (dia, ano: ela lê as palavras; os dwords vão como os da original).
unsafe fn cps13_data(dia: u32, ano: u32) -> [u8; 8] {
    let montar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut d = [0u8; 8];
    montar(&mut d, dia, ano);
    d
}

/// (dia, ano) de uma data.
fn cps13_dia_ano(d: &[u8; 8]) -> (u16, u16) {
    (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]))
}

/// A rodada `k` da copa (0x68 bytes cada, em +0xa3).
unsafe fn cps13_rodada(this: usize, k: isize) -> usize {
    (cps13_d(this + 0xa3) as isize).wrapping_add(k.wrapping_mul(0x68)) as usize
}

/// A data da rodada `k` (+0 o dia, +2 o ano a partir do ano-base +0x40 da copa, byte com sinal), como nos blocos da
/// 0x51f890 em que a metade alta do dword do dia é a do ponteiro da rodada (a do ano é zero).
unsafe fn cps13_data_da_rodada(this: usize, k: isize) -> [u8; 8] {
    let r = cps13_rodada(this, k);
    let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
    cps13_data((r as u32 & 0xffff_0000) | cps13_w(r) as u32, ano)
}

/// A virtual `off` do objeto `p` (thiscall, sem argumentos).
unsafe fn cps13_virtual0(p: usize, off: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cps13_d(cps13_d(p) + off));
    f(p)
}

/// A fase `i` da copa ([+0xc][i]).
unsafe fn cps13_fase(this: usize, i: usize) -> usize {
    cps13_d(cps13_d(this + 0xc) + i * 4)
}

/// { 0x51f890 } (thiscall, ret 8, Cup.cpp; a atualização diária de cada copa): `Cup::update_for_date(hoje, fase)`. Copa
/// sem pai (+0x44 = 0xff): com +0xb1, se as fases 0..=+0x30 estão completas (virtual +0x10 com 0, 1), passa à fase
/// seguinte (virtual +0x28, se +0x30 < +0x2c − 1) ou encerra uma vez (+0x4c: o histórico 0x442e80 de [0xad9da8], a
/// virtual +0x30, os recordes 0x4a19f0 do Comp_Stats +8); depois as notícias dos recordes (0x4a2450). Ainda sem pai,
/// cada fase recebe o dia (virtual +0x14 com 0xae2c90 e a fase). Fase 0 (com [0xae2c88] = 1): a última rodada final
/// (+7 = 0x8c) sorteia na data dela (0x51c410); sem +0xb1, sorteia as rodadas a partir de +0xac cuja data chegou, até
/// não avançar ou acabar. Fase ≠ 0 (com +0x45): o mesmo com a montagem das rodadas (0x51c800) a partir de +0xab, que
/// também monta na data pendente da rodada (+0x34 da fase da rodada). Devolve 0.
unsafe extern "thiscall" fn atualizar_copa_no_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
    let montar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c800usize);
    if cps13_b(this + 0x44) == 0xff {
        let mut completas = cps13_b(this + 0xb1) != 0;
        if completas {
            let mut i = 0usize;
            while (i as i32) <= cps13_d(this + 0x30) as i32 {
                let p = cps13_fase(this, i);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cps13_d(cps13_d(p) + 0x10));
                if completa(p, 0, 1) == 0 {
                    completas = false;
                    break;
                }
                i += 1;
            }
        }
        if completas {
            if (cps13_d(this + 0x30) as i32) < (cps13_d(this + 0x2c) as i32).wrapping_sub(1) {
                cps13_virtual0(this, 0x28); // a fase seguinte
            } else if cps13_d(this + 0x4c) == 0 {
                let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
                historico(cps13_d(0xad9da8), this);
                cps13_virtual0(this, 0x30);
                if cps13_d(this + 8) != 0 {
                    let recordes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a19f0usize);
                    recordes(cps13_d(this + 8));
                }
                core::ptr::write_unaligned((this + 0x4c) as *mut u32, 1);
            }
        }
        if cps13_d(this + 8) != 0 {
            let noticias: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a2450usize);
            noticias(cps13_d(this + 8));
        }
    }
    if cps13_b(this + 0x44) == 0xff {
        let mut i = 0usize;
        while (i as i32) <= cps13_d(this + 0x30) as i32 {
            let p = cps13_fase(this, i);
            let dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(cps13_d(cps13_d(p) + 0x14));
            dia(p, 0xae2c90, fase);
            i += 1;
        }
    }
    let (hdia, hano) = (cps13_w(hoje), cps13_w(hoje + 2));
    let n = cps13_w(this + 0x3c) as i16;
    if fase == 0 {
        if cps13_b(0xae2c88) != 1 {
            return 0;
        }
        if n > 0 && cps13_w(cps13_rodada(this, n as isize - 1) + 7) == 0x8c {
            let r = cps13_rodada(this, n as isize - 1);
            let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let d = cps13_data(cps13_w(r) as u32, ano);
            if cps13_dia_ano(&d) == (hdia, hano) {
                sortear(this, (n as i32 - 1) as u32); // a final na data dela
            }
        }
        if cps13_b(this + 0xb1) != 0 {
            return 0;
        }
        let mut k = cps13_b(this + 0xac) as i8;
        if k as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        loop {
            let (dia, ano) = cps13_dia_ano(&cps13_data_da_rodada(this, k as isize));
            // o ano sem sinal, o dia com sinal
            if ano > hano || (ano == hano && dia as i16 > hdia as i16) { // cps13: a data do sorteio
                return 0;
            }
            crate::comparacao::ramo("cps_13 sorteio da rodada");
            sortear(this, cps13_b(this + 0xac) as i8 as i32 as u32);
            let novo = cps13_b(this + 0xac) as i8;
            if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
                return 0;
            }
            k = novo;
        }
    }
    if cps13_d(this + 0x45) == 0 {
        return 0;
    }
    if n > 0 && cps13_w(cps13_rodada(this, n as isize - 1) + 7) == 0x8c {
        let r = cps13_rodada(this, n as isize - 1);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let d = cps13_data(cps13_w(r) as u32, ano);
        if cps13_dia_ano(&d) == (hdia, hano) {
            montar(this, (n as i32 - 1) as u32); // a final na data dela
        }
    }
    if cps13_b(this + 0xb1) != 0 || cps13_d(this + 0x45) == 0 {
        return 0;
    }
    let mut k = cps13_b(this + 0xab) as i8;
    if k as i16 == cps13_w(this + 0x3c) as i16 {
        return 0;
    }
    loop {
        let (dia, ano) = cps13_dia_ano(&cps13_data_da_rodada(this, k as isize));
        if (ano, dia) != (hano, hdia) { // cps13: a data da montagem
            let atual = cps13_b(this + 0xab) as i8 as isize;
            let p = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize);
            if cps13_b(p + 0x34) == 0 {
                return 0; // nem a data nem pendente
            }
        }
        crate::comparacao::ramo("cps_13 montagem da rodada");
        montar(this, cps13_b(this + 0xab) as i8 as i32 as u32);
        let novo = cps13_b(this + 0xab) as i8;
        if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        k = novo;
    }
}

/// { 0x51c800 } (thiscall, ret 4, Cup.cpp): `Cup::set_up_round(rodada)`: sem +0x45: 1. Rodada fora de 0..+0x3c − 1: a
/// caixa (linha 0xa64) e −1. Sorteia a rodada se a fase dela não tem +0x34 (0x51c410). Na rodada atual (+0xab): se um
/// confronto (0x35 bytes na fase, +0/+2 os times ou 0xffff) tem time sem clube (+0xa7, 6 bytes), adia a rodada pedida e
/// as seguintes um dia (0x54c5b0, 0x417c10; as seguintes só se ficaram antes) e devolve 1. Senão cria os jogos de cada
/// confronto marcado (+0x34 = 1 adiado, 2 já: 0x51cb90; depois do 2, sem a copa em [0xadadfc][id]: 1), leva os jogos às
/// datas da rodada (data +3/+5 num bloco de 8 bytes, 0x51ebe0, liberado depois) e, com a copa em [0xadadfc][id], passa à
/// rodada seguinte (fora da final 0x8c) e leva +0xac junto. Devolve 1.
unsafe extern "thiscall" fn montar_rodada_da_copa(this: usize, rodada: u32) -> u32 {
    let id = cps13_d(cps13_d(this + 4));
    let ativa = || cps13_d(cps13_d(0xadadfc) + id * 4) != 0;
    if cps13_d(this + 0x45) == 0 {
        return 1;
    }
    let ri = rodada as i32;
    if ri > cps13_w(this + 0x3c) as i16 as i32 - 1 || ri < 0 {
        aviso_em(0x9b55ac, 0xa64);
        return u32::MAX;
    }
    let ad = cps13_d(this + 0xad);
    if cps13_b(cps13_d(ad + rodada as usize * 4) + 0x34) == 0 {
        let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
        sortear(this, rodada);
    }
    let atual = cps13_b(this + 0xab) as i8 as isize;
    let base = cps13_d(this + 0xa3);
    let off = atual * 0x68;
    let n = *((base as isize + off + 0x1a) as *const i16) as i32;
    let mut sem_clube = false;
    for i in 0..n.max(0) as usize {
        let t = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i * 0x35;
        for lado in [0usize, 2] {
            let time = cps13_w(t + lado);
            if time != 0xffff && cps13_d(cps13_d(this + 0xa7) + (time as i16 as isize * 6) as usize) == 0 {
                sem_clube = true;
                break;
            }
        }
        if sem_clube {
            break;
        }
    }
    if sem_clube {
        // a rodada pedida (não a atual) e as seguintes ficam um dia depois
        let mais_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
        let r = cps13_rodada(this, ri as isize);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let mut d = cps13_data((cps13_d(this + 0xa7) as u32 & 0xffff_0000) | cps13_w(r) as u32, ano);
        let (mut amanha, mut copia) = ([0u8; 8], [0u8; 8]);
        let p = mais_dias(&mut d, &mut amanha, 1);
        atribuir(&mut d, &mut copia, p);
        let (dia, ano) = cps13_dia_ano(&d);
        crate::comparacao::ramo("cps_13 rodada adiada");
        let r = cps13_rodada(this, ri as isize);
        core::ptr::write_unaligned(r as *mut u16, dia);
        *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
        let mut k = ri + 1;
        while (k as i16) < cps13_w(this + 0x3c) as i16 {
            let r = cps13_rodada(this, k as i16 as isize);
            let rano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let (odia, oano) = cps13_dia_ano(&cps13_data((cps13_d(this + 0xa3) as u32 & 0xffff_0000) | cps13_w(r) as u32, rano));
            if ano > oano || (ano == oano && dia as i16 > odia as i16) {
                let r = cps13_rodada(this, k as i16 as isize);
                core::ptr::write_unaligned(r as *mut u16, dia);
                *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
            }
            k += 1;
        }
        return 1;
    }
    let criar: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(0x51cb90usize);
    let mut i = 0i32;
    while n > 0 && i < *((cps13_d(this + 0xa3) as isize + off + 0x1a) as *const i16) as i32 {
        let marca = |i: i32| cps13_b(cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i as usize * 0x35 + 0x34);
        if marca(i) == 1 {
            criar(this, atual as i32 as u32, i as u32, 1);
        }
        if marca(i) == 2 {
            criar(this, atual as i32 as u32, i as u32, 0);
            if !ativa() {
                return 1;
            }
        }
        i += 1;
    }
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let montar_data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let p = novo(8);
    let data = if p != 0 {
        let base = cps13_d(this + 0xa3);
        let r = (base as isize + off) as usize;
        let ano = (cps13_b(r + 5) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let dia = cps13_w(r + 3) as u32;
        montar_data(p, (r as u32 & 0xffff_0000) | dia, (base as u32 & 0xffff_0000) | ano)
    } else {
        0
    };
    let levar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51ebe0usize);
    crate::comparacao::ramo("cps_13 jogos da rodada");
    levar(this, data, atual as i32 as u32);
    if data != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        liberar(data);
    }
    if !ativa() {
        return 1;
    }
    if cps13_w((cps13_d(this + 0xa3) as isize + off + 7) as usize) != 0x8c {
        *((this + 0xab) as *mut u8) = cps13_b(this + 0xab).wrapping_add(1);
    }
    if (cps13_b(this + 0xac) as i8) < cps13_b(this + 0xab) as i8 {
        *((this + 0xac) as *mut u8) = cps13_b(this + 0xab);
    }
    1
}

/// Põe o risco de doença (0x62e820 de [0xb63d28], tipo 3) na véspera da data do jogo `f` (+0x2a o dia, +0x28 o ano;
/// 0x549ef0, 0x54c8e0, 0x417c10) para o clube `id` com a nação do estádio. `alto_dia`/`alto_ano`: as metades altas dos
/// dwords que a original empurra à 0x549ef0.
unsafe fn cps13_doenca(f: usize, id: usize, nacao: usize, alto_dia: u32, alto_ano: u32) {
    let menos_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let doenca: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32, usize, usize) -> u32 = core::mem::transmute(0x62e820usize);
    let mut d = cps13_data((alto_dia & 0xffff_0000) | cps13_w(f + 0x2a) as u32, (alto_ano & 0xffff_0000) | cps13_w(f + 0x28) as u32);
    let (mut vespera, mut copia) = ([0u8; 8], [0u8; 8]);
    let p = menos_dias(&mut d, &mut vespera, 1);
    atribuir(&mut d, &mut copia, p);
    doenca(cps13_d(0xb63d28), &mut d, 3, id, nacao);
}

/// { 0x5ab3c0 } (thiscall, ret 4, fix_man.cpp; o `this` não é usado): `FIXTURE_MANAGER::add_away_trip_illness_risk(jogo)`:
/// com o estádio do jogo (+0x18), a cidade (+0x38) e a nação dela (+0x1f) num país do terceiro mundo (+0x7e = 3), cada
/// clube visitante de outra nação (+0x53) — o mandante (+0x1c, id +0xc) e o visitante (+0x20, id +0x10) — ganha o risco
/// de doença na véspera do jogo.
unsafe extern "thiscall" fn risco_de_doenca_na_viagem(_this: usize, f: usize) {
    let estadio = cps13_d(f + 0x18);
    if estadio == 0 {
        return;
    }
    let cidade = cps13_d(estadio + 0x38);
    if cidade == 0 {
        return;
    }
    let nacao = cps13_d(cidade + 0x1f);
    if nacao == 0 {
        return;
    }
    if cps13_b(nacao + 0x7e) != 3 { // cps13: só o terceiro mundo
        return;
    }
    let casa = cps13_d(f + 0x1c);
    if casa != 0 && cps13_d(casa + 0x53) != nacao {
        crate::comparacao::ramo("cps_13 doenca do mandante");
        // a original empurra o ano no eax do clube e o dia no ecx da nação (metades altas dos ponteiros)
        cps13_doenca(f, cps13_d(f + 0xc), cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f), nacao as u32, casa as u32);
    }
    let fora = cps13_d(f + 0x20);
    if fora != 0 && cps13_d(fora + 0x53) != cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f) {
        crate::comparacao::ramo("cps_13 doenca do visitante");
        // o ano no ecx do estádio e o dia no edx da cidade
        let (e, c) = (cps13_d(f + 0x18), cps13_d(cps13_d(f + 0x18) + 0x38));
        cps13_doenca(f, cps13_d(f + 0x10), cps13_d(c + 0x1f), c as u32, e as u32);
    }
}

// ponytail: sorteia e monta rodadas (aloca jogos): não roda duas vezes. Só a original; a cópia leva os ramos (sorteio e
// montagem; a atualização é diária em cada copa, 29.934 chamadas até 2002-06-01: sem ramo no caminho comum).
unsafe extern "thiscall" fn comparar_atualizar_copa_no_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1152)(this, hoje, fase)
}

// ponytail: sorteia, cria os jogos e passa a rodada: não roda duas vezes. Só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_montar_rodada_da_copa(this: usize, rodada: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1153)(this, rodada)
}

// ponytail: acrescenta o risco de doença (não roda duas vezes; o caminho comum não faz nada). Só a original; a cópia leva
// os ramos.
unsafe extern "thiscall" fn comparar_risco_de_doenca_na_viagem(this: usize, f: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1154)(this, f)
}

// ---------- 253a leva: a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada (Cup.cpp, comp_util.cpp) ----------

unsafe fn cpq11_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpq11_us(a: usize) -> usize {
    cpq11_d(a) as usize
}

unsafe fn cpq11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq11_por_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// `base + i × tam` (o índice com sinal, como o movsx do original).
fn cpq11_em(base: usize, i: i32, tam: usize) -> usize {
    base.wrapping_add((i as isize).wrapping_mul(tam as isize) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` da copa: [[copa + 0xad] + 4r] + 0x35k.
unsafe fn cpq11_confronto(copa: usize, r: i32, k: i32) -> usize {
    cpq11_em(cpq11_us(cpq11_em(cpq11_us(copa + 0xad), r, 4)), k, 0x35)
}

/// A entrada da rodada `r` (0x68 bytes) em [copa + 0xa3]: +0x18 vagas de times, +0x1a confrontos.
unsafe fn cpq11_rodada(copa: usize, r: i32) -> usize {
    cpq11_em(cpq11_us(copa + 0xa3), r, 0x68)
}

/// O clube do time `t` da copa (6 bytes em [copa + 0xa7]: +0 clube, +4 nível).
unsafe fn cpq11_time(copa: usize, t: i16) -> usize {
    cpq11_em(cpq11_us(copa + 0xa7), t as i32, 6)
}

/// Os confrontos de todas as rodadas da copa (rodadas em +0x3c), para os comparadores.
unsafe fn cpq11_confrontos(copa: usize) -> Vec<(usize, usize)> {
    let mut v = vec![(0xb67a34, 4)];
    for r in 0..(cpq11_w(copa + 0x3c) as i32).max(0) {
        let f = cpq11_us(cpq11_em(cpq11_us(copa + 0xad), r, 4));
        let n = cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32;
        if f != 0 && n > 0 {
            v.push((f, n as usize * 0x35));
        }
    }
    v
}

/// A caixa "Memory Allocation Error" ("v%s %s.%s %d (%s)" de Cup.cpp na linha `linha`) e o jogo para (como a original).
unsafe fn cpq11_sem_memoria(linha: u32, texto: usize) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b55ac, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
    *(0xb67a34 as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x51e220 } (thiscall, ret 4, Cup.cpp): `Cup::randomize_round_teams(rodada)`: o sorteio dos times da rodada.
/// 1) Tira os times dos confrontos (+0x1a da rodada) para um vetor de 7 bytes por vaga (malloc de 7 × vagas +0x18;
///    sem memória, caixa da linha 0xe7f e o jogo para): o mandante (+0, com o vínculo +0xe/+0xa) e o visitante (+2,
///    com +0x10/+0xc), cada um com o nível do time (+4 do time); a vaga do confronto vira −2 (livre); sem time (−1),
///    a entrada fica −1 (com 0xff no nível).
/// 2) Embaralha: para cada posição, troca com a entrada sorteada entre todas (0x90cfc0(n)).
/// 3) Cada entrada com time vai a um confronto sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira
///    volta só confrontos com as duas vagas vazias (< 0); depois qualquer um com vaga livre (−2); dando a volta
///    inteira na segunda, caixa da linha 0xf4c e sai (a marca da segunda volta não volta a zero). Com as duas vagas
///    livres, sorteia o lado (0x90cfc0(2): ≠ 0 mandante). O time leva o vínculo (+0xe/+0xa ou +0x10/+0xc) e, com
///    vínculo, o confronto da rodada seguinte ganha a rodada (+0x12), o confronto sorteado (+6) e o lado (+8: 1
///    mandante, 0 visitante).
/// Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn copa_sortear_times_da_rodada(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let vagas = cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32;
    let vetor = alocar(vagas.wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xe7f, 0x9b574c); // "CUP::randomize_round_teams() : Cannot allocate memory"
    }
    let entrada = |i: i16| cpq11_em(vetor, i as i32, 7);
    // 1) os times saem dos confrontos
    let mut n: i16 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = cpq11_confronto(copa, r, k as i16 as i32);
            let e = entrada(n);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = *((cpq11_time(copa, time) + 4) as *const u8); // o nível do time
                cpq11_por_w(t + lado, 0xfffe); // a vaga fica livre
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    // 2) o embaralhamento
    if n > 0 {
        let mut p = vetor;
        for _ in 0..n as i32 {
            let q = entrada(sorteio(n as i32) as i16);
            let (a, b, c) = (cpq11_d(q), cpq11_w(q + 4), *((q + 6) as *const u8));
            core::ptr::write_unaligned(q as *mut u32, cpq11_d(p));
            cpq11_por_w(q + 4, cpq11_w(p + 4) as u16);
            *((q + 6) as *mut u8) = *((p + 6) as *const u8);
            core::ptr::write_unaligned(p as *mut u32, a);
            cpq11_por_w(p + 4, b as u16);
            *((p + 6) as *mut u8) = c;
            p += 7;
        }
    }
    // 3) cada time vai a um confronto sorteado
    let mut segunda_volta = false;
    let mut i: i16 = 0;
    while i < n {
        let e = entrada(i);
        i = i.wrapping_add(1);
        if cpq11_w(e) == -1 {
            continue;
        }
        let mut s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32) as i16;
        let inicio = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, s as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            if segunda_volta || (casa < 0 && fora < 0) {
                let casa_livre = casa == -2;
                if fora == -2 {
                    if casa_livre && sorteio(2) != 0 {
                        crate::comparacao::ramo("cpq_11 sorteio mandante");
                        break true;
                    }
                    break false;
                }
                if casa_livre {
                    break true;
                }
            }
            s = s.wrapping_add(1);
            if s >= cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
                s = 0;
            }
            if s == inicio {
                if segunda_volta {
                    aviso_em(0x9b55ac, 0xf4c);
                    liberar(vetor);
                    return;
                }
                segunda_volta = true;
            }
        };
        let t = cpq11_confronto(copa, r, s as i32);
        let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
        cpq11_por_w(t + lado, cpq11_w(e) as u16);
        cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
        cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
        let seguinte = cpq11_w(e + 2);
        if seguinte != -1 {
            let v = cpq11_confronto(copa, seguinte as i32, cpq11_w(e + 4) as i32);
            cpq11_por_w(v + 0x12, rodada as u16);
            cpq11_por_w(v + 6, s as u16);
            cpq11_por_w(v + 8, mandante as u16);
        }
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_copa_sortear_times_da_rodada(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1075);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_times_da_rodada(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_times_da_rodada(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Os pares de palavras do confronto `t` que a 0x522610 troca: mandante/visitante (+0/+2), +0xa/+0xc e +0xe/+0x10.
unsafe fn cpq11_inverter(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xe, 0x10), (0xa, 0xc)] {
        let (x, y) = (cpq11_w(t + a), cpq11_w(t + b));
        cpq11_por_w(t + a, y as u16);
        cpq11_por_w(t + b, x as u16);
    }
}

/// { 0x522610 } (thiscall, ret 4, Cup.cpp): `Cup::separate_shared_stadium_ties(rodada)`: para cada confronto `i` da
/// rodada, procura o primeiro `j > i` cujo mandante (ou, senão, o visitante) divide o estádio (+0x69 do clube, não
/// nulo) com o mandante (ou o visitante) de `i`; achando, inverte o confronto `j` (mandante e visitante, +0xa/+0xc e
/// +0xe/+0x10) e acerta o lado (+8) dos confrontos ligados na rodada seguinte: o de +0xe/+0xa fica 1 (mandante), o de
/// +0x10/+0xc fica 0. Os dois jogos no mesmo estádio deixam de cair no mesmo lado.
unsafe extern "thiscall" fn copa_separar_estadio_compartilhado(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let estadio = |t: i16| cpq11_d(cpq11_us(cpq11_time(copa, t)) as usize + 0x69);
    let mut n = cpq11_w(cpq11_rodada(copa, r) + 0x1a);
    let mut i: i32 = 0;
    while (i as i16) < n {
        let fi = cpq11_confronto(copa, r, i as i16 as i32);
        let casa_i = cpq11_w(fi);
        let mut j = i + 1;
        while (j as i16) < n {
            let fj = cpq11_confronto(copa, r, j as i16 as i32);
            let mut dividem = false;
            for lado in [0usize, 2] {
                let a = if lado == 0 { casa_i } else { cpq11_w(fi + 2) };
                let b = cpq11_w(fj + lado);
                if a != -1 && b != -1 {
                    crate::comparacao::ramo("cpq_11 estadio comparado");
                    let (sa, sb) = (estadio(a), estadio(b));
                    if sa == sb && sa != 0 {
                        dividem = true;
                        break;
                    }
                }
            }
            if dividem {
                cpq11_inverter(fj);
                let seguinte = cpq11_w(fj + 0xe);
                if seguinte != -1 {
                    cpq11_por_w(cpq11_confronto(copa, seguinte as i32, cpq11_w(fj + 0xa) as i32) + 8, 1);
                }
                let seguinte = cpq11_w(fj + 0x10);
                if seguinte != -1 {
                    cpq11_por_w(cpq11_confronto(copa, seguinte as i32, cpq11_w(fj + 0xc) as i32) + 8, 0);
                }
                break;
            }
            j += 1;
        }
        n = cpq11_w(cpq11_rodada(copa, r) + 0x1a);
        i += 1;
    }
}

/// Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
unsafe extern "thiscall" fn comparar_copa_separar_estadio_compartilhado(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1076);
    let reg = cpq11_confrontos(copa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada), || copa_separar_estadio_compartilhado(copa, rodada));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_separar_estadio_compartilhado(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// A conta da 0x4bf850 sem efeitos: o eax da original e o byte devolvido. Com uma rodada só, (baixo + alto) / 2
/// (truncada para o zero) sobre a soma; senão o quociente (alto − baixo) × rodada / (rodadas − 1), sem somar o baixo.
fn cpq11_escala(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> (u32, i8) {
    let (b, a) = (baixo as u8 as i8 as i32, alto as u8 as i8 as i32);
    let n = rodadas as u16 as i16 as i32;
    if n == 1 {
        let s = b.wrapping_add(a);
        let t = s.wrapping_sub(s >> 31); // cdq; sub eax, edx
        let v = (t >> 1) as i8;
        ((t as u32 & 0xffff_ff00) | v as u8 as u32, v)
    } else {
        let q = a.wrapping_sub(b).wrapping_mul(rodada as u16 as i16 as i32).wrapping_div(n - 1); // a média por rodada
        (q as u32, q as i8)
    }
}

/// { 0x4bf850 } (cdecl, comp_util.cpp): `comp_util_scale_by_round(baixo, alto, rodada, rodadas)` (bytes e palavras com
/// sinal; as palavras chegam em registradores com lixo acima, só a palavra é lida): com uma rodada só, a média
/// (baixo + alto) / 2; senão (alto − baixo) × rodada / (rodadas − 1). Fora de 0..=100: a original (caixa da linha
/// 0x256e, devolve 0 ou 100). Devolve o eax inteiro da original (quem chama empurra o registrador todo para
/// 0x86bdd0/0x86c000): o byte em al sobre a soma (rodada única) ou o quociente.
unsafe extern "C" fn escala_pela_rodada(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let (eax, v) = cpq11_escala(baixo, alto, rodada, rodadas);
    if !(0..=100).contains(&v) {
        return orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1077)(baixo, alto, rodada, rodadas);
    }
    if rodadas as u16 != 1 {
        crate::comparacao::ramo("cpq_11 escala pela rodada");
    }
    eax
}

/// Comparador: pura (pelo eax inteiro). Fora da faixa (a caixa de erro) só a original.
unsafe extern "C" fn comparar_escala_pela_rodada(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1077);
    if !(0..=100).contains(&cpq11_escala(baixo, alto, rodada, rodadas).1) {
        return o(baixo, alto, rodada, rodadas);
    }
    let r = o(baixo, alto, rodada, rodadas);
    let n = escala_pela_rodada(baixo, alto, rodada, rodadas);
    PLACAR.registrar(r == n, || format!("escala_pela_rodada({}, {}, {}, {}) original={:x} rust={:x}",
        baixo as u8 as i8, alto as u8 as i8, rodada as u16 as i16, rodadas as u16 as i16, r, n));
    r
}
// ---------- 252a leva: as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada (Cup.cpp, league.cpp) ----------

unsafe fn cpp13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O nível (+4, byte com sinal) e a reputação do clube (+0 → +0x80, palavra com sinal) de uma entrada de copa.
unsafe fn cpp13_nivel(e: usize) -> i32 {
    *((e + 4) as *const i8) as i32
}

unsafe fn cpp13_reputacao(e: usize) -> i32 {
    *((cpp13_d(e) + 0x80) as *const i16) as i32
}

/// { 0x522190 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_desc(a, b)`: o qsort das entradas das copas (sorteios da
/// Copa da UEFA, Intertoto, copas do Japão e da Escócia) pelo nível, o maior primeiro; no empate pela reputação do
/// clube, a menor primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_dos_cabecas_de_chave_decrescente(a: usize, b: usize) -> i32 {
    let r = cpp13_nivel(b) - cpp13_nivel(a); // o maior nível primeiro
    if r != 0 {
        return r;
    }
    cpp13_reputacao(a) - cpp13_reputacao(b)
}

/// { 0x5221c0 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_asc(a, b)`: o qsort das entradas da copa da liga
/// inglesa (0x56ccc0) pelo nível, o menor primeiro; no empate pela reputação do clube, a menor primeiro. Quente
/// (qsort): sem contador.
unsafe extern "C" fn ordem_dos_cabecas_de_chave_crescente(a: usize, b: usize) -> i32 {
    let r = cpp13_nivel(a) - cpp13_nivel(b); // o menor nível primeiro
    if r != 0 {
        return r;
    }
    cpp13_reputacao(a) - cpp13_reputacao(b)
}

/// Leva o byte +0x64 do clube (com sinal) até `valor`: para baixo (`baixar` ≠ 0) se está acima, para cima se está
/// abaixo.
unsafe fn cpp13_ajustar(clube: usize, valor: i8, baixar: u32) {
    let p = (clube + 0x64) as *mut i8;
    let muda = if baixar != 0 { *p > valor } else { *p < valor };
    if muda {
        crate::comparacao::ramo("cpp_13 teto da divisao");
        *p = valor; // o teto novo
    }
}

/// { 0x68a980 } (thiscall, ret 0x10, league.cpp): `League::limit_club_division_tier(valor, fora, dentro, baixar)`
/// (chamada pelo fim de temporada de cada liga: Argentina, Bélgica, Brasil, Croácia, Dinamarca, Inglaterra, França,
/// Alemanha, Holanda, Itália, Rússia, Escócia, Suécia e Turquia): para cada linha da tabela (+0xb1, 0x3b bytes, conta
/// +0x3e), ajusta o byte +0x64 do clube (+0) a `valor` quando a situação da linha (+0x37) não é `fora` (0xfd = nenhuma)
/// e de novo quando é `dentro` (0xfd = nenhuma). Os três primeiros argumentos são bytes empurrados como dword: só o
/// byte baixo conta.
unsafe extern "thiscall" fn teto_da_divisao_dos_clubes(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let (valor, fora, dentro) = (valor as u8 as i8, fora as u8, dentro as u8);
    let mut i: i16 = 0;
    while i < *((this + 0x3e) as *const i16) {
        let linha = || cpp13_d(this + 0xb1).wrapping_add(i as isize as usize * 0x3b);
        if fora != 0xfd && *((linha() + 0x37) as *const u8) != fora {
            cpp13_ajustar(cpp13_d(linha()), valor, baixar);
        }
        if dentro != 0xfd && *((linha() + 0x37) as *const u8) == dentro {
            cpp13_ajustar(cpp13_d(linha()), valor, baixar);
        }
        i = i.wrapping_add(1);
    }
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1042)(a, b);
    let n = ordem_dos_cabecas_de_chave_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1043)(a, b);
    let n = ordem_dos_cabecas_de_chave_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
unsafe extern "thiscall" fn comparar_teto_da_divisao_dos_clubes(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(1044);
    let n = *((this + 0x3e) as *const i16);
    let tabela = cpp13_d(this + 0xb1);
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (cpp13_d(tabela + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, valor, fora, dentro, baixar),
        || teto_da_divisao_dos_clubes(this, valor, fora, dentro, baixar));
    PLACAR.registrar(igual, || format!("teto_da_divisao_dos_clubes(valor {}, fora {:x}, dentro {:x}, baixar {}) {:x?} x {:x?}",
        valor as u8 as i8, fora as u8, dentro as u8, baixar, mo, mr));
}

// ---------- 249a leva: a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio (Cup.cpp) ----------

unsafe fn cpm1_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpm1_por(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

unsafe fn cpm1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O corpo da 0x51c020: `Some(lado)`, ou `None` no empate também nos pênaltis com a regra deles (a original mostra a
/// caixa "Error" com a linha 0x97e e segue para W.O. e pontos): quem chama roda a original inteira, que refaz o único
/// que já foi gravado (+0x3d/+0x3e = 1 com o bit 0x10, idempotente).
unsafe fn cpm1_jogo(this: usize, f: usize) -> Option<u32> {
    let fl = cpm1_w(f + 0x38) as u16;
    if fl & 0x10 != 0 {
        cpm1_por(f + 0x3d, 1);
        cpm1_por(f + 0x3e, 1);
    }
    let (g1, g2) = (cpm1_b(f + 0x43), cpm1_b(f + 0x44));
    if g1 > g2 {
        if cpm1_b(f + 0x3d) == 1 {
            cpm1_por(f + 0x3d, 2);
        }
        if fl & 0x2000 != 0 {
            cpm1_por(f + 0x3d, cpm1_b(f + 0x3d).wrapping_sub(3));
        }
        return Some(1);
    }
    if g1 < g2 {
        if cpm1_b(f + 0x3e) == 1 {
            cpm1_por(f + 0x3e, 2);
        }
        if fl & 0x2000 != 0 {
            cpm1_por(f + 0x3e, cpm1_b(f + 0x3e).wrapping_sub(3));
        }
        return Some(2);
    }
    if fl & 2 != 0 {
        let (a1, a2) = (cpm1_b(f + 0x47), cpm1_b(f + 0x48));
        if a1 > a2 {
            crate::comparacao::ramo("cpm_1 jogo na prorrogacao");
            let c = cpm1_b(f + 0x3d);
            if c == 1 {
                cpm1_por(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3d, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3d, 3);
            }
            return Some(1); // o mandante na prorrogação
        }
        if a1 < a2 {
            let c = cpm1_b(f + 0x3e);
            if c == 1 {
                cpm1_por(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3e, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3e, 3);
            }
            return Some(2); // o visitante na prorrogação
        }
    }
    if fl & 0x21 != 0 {
        let (p1, p2) = (cpm1_b(f + 0x49), cpm1_b(f + 0x4a));
        // o código dos pênaltis: 4, ou 5 sem o bit 1 da regra (not al; and al, 1; or al, 4)
        let penaltis = ((!(fl as u8)) & 1 | 4) as i8;
        if p1 > p2 {
            let c = cpm1_b(f + 0x3d);
            if c == 1 {
                cpm1_por(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3d, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3d, penaltis);
            }
            return Some(1);
        }
        if p1 < p2 {
            let c = cpm1_b(f + 0x3e);
            if c == 1 {
                cpm1_por(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3e, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3e, penaltis);
            }
            return Some(2);
        }
        return None; // empate nos pênaltis: a caixa da original
    }
    if fl & 0x800 != 0 {
        cpm1_por(f + 0x3d, 8); // W.O. a favor do mandante
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        cpm1_por(f + 0x3e, 8);
        return Some(2);
    }
    if fl & 0x2000 == 0 {
        return Some(0);
    }
    let a = cpm1_b(f + 0x3d).wrapping_sub(1);
    let c = cpm1_b(f + 0x3e).wrapping_sub(1);
    cpm1_por(f + 0x3d, a);
    cpm1_por(f + 0x3e, c);
    if a < c {
        return Some(1);
    }
    if a > c {
        return Some(2);
    }
    // empate nos pontos: o cabeça de chave menor (byte +4 da entrada de 6 bytes [this+0xa7], +0x3e entradas)
    let n = cpm1_w(this + 0x3e);
    if n <= 0 {
        return Some(2);
    }
    let (c1, c2) = (cpm1_d(f + 0x1c), cpm1_d(f + 0x20));
    let (mut s1, mut s2) = (0i8, 0i8);
    let t = cpm1_d(this + 0xa7);
    for k in 0..n as usize {
        let e = t + 6 * k;
        if cpm1_d(e) == c1 {
            s1 = cpm1_b(e + 4);
        }
        if cpm1_d(e) == c2 {
            s2 = cpm1_b(e + 4);
        }
    }
    Some(if s1 < s2 { 1 } else { 2 })
}

/// { 0x51c020 } (thiscall, ret 4, Cup.cpp): `Cup::decide_fixture_winner(jogo)`: quem passa num jogo de copa (1
/// mandante, 2 visitante, 0 ninguém) pela regra (+0x38): o placar (+0x43/+0x44), a prorrogação (bit 2, +0x47/+0x48), os
/// pênaltis (bits 0x21, +0x49/+0x4a), o W.O. (0x800/0x1000) e os pontos (0x2000: os códigos −1 e o cabeça de chave),
/// gravando o código de cada lado (+0x3d/+0x3e: 2 venceu, 3 na prorrogação, 4/5 nos pênaltis, 8 W.O.; −3 com 0x2000).
/// Jogo nulo ou empate nos pênaltis: a original (caixa "Error").
unsafe extern "thiscall" fn decidir_jogo_da_copa(this: usize, jogo: usize) -> u32 {
    if jogo != 0 {
        if let Some(r) = cpm1_jogo(this, jogo) {
            return r;
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(925)(this, jogo)
}

/// O corpo da 0x51f2f0: `None` no empate nos pênaltis com a regra deles (a original mostra a caixa "Error" com a linha
/// 0x11d1 e devolve 4; nada foi gravado antes).
unsafe fn cpm1_confronto(conf: usize, j: usize) -> Option<u32> {
    let t = |k: usize| cpm1_b(conf + k);
    let fl = cpm1_w(j + 0x38) as u16;
    if t(0x33) as u8 != 1 {
        // primeiro jogo (ou jogo único) do confronto: 3
        if fl & 0x10 != 0 {
            let lado1 = if t(0x14) != t(0x15) {
                t(0x14) > t(0x15)
            } else if t(0x18) != t(0x19) {
                t(0x18) > t(0x19)
            } else {
                t(0x1a) > t(0x1b)
            };
            cpm1_por(j + if lado1 { 0x3d } else { 0x3e }, 1);
        } else if fl & 0x2000 != 0 {
            let pares = [(0x14, 0x15), (0x18, 0x19), (0x1a, 0x1b)];
            match pares.iter().find(|p| t(p.0) != t(p.1)) {
                Some(&(a, b)) => {
                    let lado = if t(a) > t(b) { 0x3d } else { 0x3e };
                    cpm1_por(j + lado, cpm1_b(j + lado).wrapping_sub(3));
                }
                None => {
                    cpm1_por(j + 0x3d, cpm1_b(j + 0x3d).wrapping_sub(1));
                    cpm1_por(j + 0x3e, cpm1_b(j + 0x3e).wrapping_sub(1));
                }
            }
        }
        return Some(3);
    }
    // o jogo de volta: o agregado (os gols +0x14 e +0x1f contra +0x1e e +0x15, em 16 bits)
    let tot1 = (t(0x14) as i16).wrapping_add(t(0x1f) as i16);
    let tot2 = (t(0x1e) as i16).wrapping_add(t(0x15) as i16);
    let volta2 = t(0x15) as i16;
    if fl & 0x10 != 0 {
        // pontos: quem venceu cada jogo (placar, prorrogação, pênaltis); um de cada: 4
        let lado_volta = if t(0x14) != t(0x15) {
            t(0x14) > t(0x15)
        } else if t(0x18) != t(0x19) {
            t(0x18) > t(0x19)
        } else {
            t(0x1a) > t(0x1b)
        };
        let lado_ida = if t(0x1e) != t(0x1f) {
            t(0x1e) > t(0x1f)
        } else if t(0x22) != t(0x23) {
            t(0x22) > t(0x23)
        } else {
            t(0x24) > t(0x25)
        };
        if lado_volta == lado_ida {
            cpm1_por(j + 0x3d, 1);
            cpm1_por(j + 0x3e, 1);
            return Some(4);
        }
        if lado_volta {
            cpm1_por(j + 0x3e, 2);
            return Some(2);
        }
        cpm1_por(j + 0x3d, 2);
        return Some(1);
    }
    if fl & 0x2000 != 0 {
        let pares = [(0x43, 0x44), (0x47, 0x48), (0x49, 0x4a)];
        match pares.iter().find(|p| cpm1_b(j + p.0) != cpm1_b(j + p.1)) {
            Some(&(a, b)) => {
                let lado = if cpm1_b(j + a) > cpm1_b(j + b) { 0x3d } else { 0x3e };
                cpm1_por(j + lado, cpm1_b(j + lado).wrapping_sub(3));
            }
            None => {
                cpm1_por(j + 0x3d, cpm1_b(j + 0x3d).wrapping_sub(1));
                cpm1_por(j + 0x3e, cpm1_b(j + 0x3e).wrapping_sub(1));
            }
        }
        if cpm1_b(j + 0x3d) < -5 {
            return Some(1);
        }
        return Some(if cpm1_b(j + 0x3e) >= -5 { 4 } else { 2 });
    }
    if tot1 > tot2 {
        return Some(2); // agregado
    }
    if tot1 < tot2 {
        return Some(1);
    }
    crate::comparacao::ramo("cpm_1 confronto empatado no agregado");
    if fl & 0x800 != 0 {
        cpm1_por(j + 0x3d, 8);
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        cpm1_por(j + 0x3e, 8);
        return Some(2);
    }
    if fl & 2 != 0 && t(0x22) as u8 != 0xff {
        if fl & 0x4000 != 0 && (t(0x22) as i32 + t(0x15) as i32) == (t(0x23) as i32 + t(0x14) as i32) {
            if t(0x23) > t(0x15) {
                cpm1_por(j + 0x3e, 6);
                return Some(2);
            }
            if t(0x23) < t(0x15) {
                cpm1_por(j + 0x3d, 6);
                return Some(1);
            }
        }
        let x = t(0x23) as i32 - t(0x1f) as i32;
        let y = t(0x22) as i32 - t(0x1e) as i32;
        if y > x {
            cpm1_por(j + 0x3d, 3);
            return Some(1);
        }
        if y < x {
            cpm1_por(j + 0x3e, 3);
            return Some(2);
        }
    }
    if fl & 4 == 0 && fl & 0x4000 == 0 {
        // os gols fora (o visitante da volta, +0x15, contra o da ida, +0x1f)
        let fora = t(0x1f) as i16;
        if volta2 < fora {
            cpm1_por(j + 0x3e, 6);
            return Some(2);
        }
        if volta2 > fora {
            cpm1_por(j + 0x3d, 6);
            return Some(1);
        }
    }
    if fl & 0x21 == 0 {
        return Some(4);
    }
    let (p1, p2) = (t(0x24), t(0x25));
    let codigo = if fl & 1 != 0 { 4 } else { 5 };
    if p2 < p1 {
        cpm1_por(j + 0x3d, codigo);
        return Some(1);
    }
    if p2 > p1 {
        cpm1_por(j + 0x3e, codigo);
        return Some(2);
    }
    None
}

/// { 0x51f2f0 } (thiscall, ret 8, Cup.cpp; `this` não é lido): `Cup::decide_tie_winner(confronto, jogo)`: no primeiro
/// jogo (+0x33 ≠ 1), só marca o vencedor (pontos 0x10: código 1; 0x2000: −3, ou −1 nos dois) e devolve 3. Na volta,
/// pelo agregado (+0x14 + +0x1f contra +0x1e + +0x15): 2 o primeiro lado à frente, 1 o segundo; empatado: W.O.
/// (0x800/0x1000, código 8), a prorrogação (bit 2 com +0x22 ≠ 0xff: com 0x4000 os gols fora dela; código 6, senão a
/// diferença da prorrogação, código 3), os gols fora (sem 4 nem 0x4000, código 6) e os pênaltis (0x21, +0x24/+0x25:
/// código 4 com o bit 1, senão 5); nada decidido: 4. Pontos (0x10) e regra 0x2000 têm os ramos próprios. Empate nos
/// pênaltis: a original (caixa "Error", 4).
unsafe extern "thiscall" fn decidir_confronto_da_copa(this: usize, confronto: usize, jogo: usize) -> u32 {
    match cpm1_confronto(confronto, jogo) {
        Some(r) => r,
        None => orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(926)(this, confronto, jogo),
    }
}

/// { 0x51ebc0 } (cdecl, Cup.cpp): `cup_compare_draw_entries_by_seed(a, b)`: a comparação do qsort das entradas de 7
/// bytes do sorteio (0x51d2d0 e 0x51d9f0): o cabeça de chave (byte com sinal +6) de `b` menos o de `a` (maior primeiro).
unsafe extern "C" fn ordem_dos_cabecas_de_chave(a: usize, b: usize) -> i32 {
    crate::comparacao::ramo("cpm_1 ordem dos cabecas de chave");
    cpm1_b(b + 6) as i32 - cpm1_b(a + 6) as i32 // o maior primeiro
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
unsafe extern "thiscall" fn comparar_decidir_jogo_da_copa(this: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(925);
    if jogo == 0 {
        return o(this, jogo);
    }
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, jogo)), || cpm1_jogo(this, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decidir_jogo_da_copa(regra {:x}) original={} rust={} {:x?} x {:x?}", cpm1_w(jogo + 0x38) as u16, r, n, mo, mr));
    }
    r
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
unsafe extern "thiscall" fn comparar_decidir_confronto_da_copa(this: usize, confronto: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(926);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, confronto, jogo)), || cpm1_confronto(confronto, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decidir_confronto_da_copa(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            cpm1_w(jogo + 0x38) as u16, cpm1_b(confronto + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(927)(a, b);
    let n = ordem_dos_cabecas_de_chave(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave({}, {}) original={} rust={}", cpm1_b(a + 6), cpm1_b(b + 6), r, n));
    r
}

// ---------- 213a leva: as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados (Cup.cpp, friendly.cpp) ----------

/// A data (Date::Date, 0x549f70, num Date local de 8 bytes) de `dia`/`mes`/`ano`, levada ao dia da
/// semana `semana` (−1 = qualquer): (palavra +0 = dia do ano, byte baixo do ano +2).
unsafe fn data_da_rodada(dia: u32, mes: u32, ano: u32, semana: u32) -> (u16, u8) {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    (d[0] as u16, (d[0] >> 16) as u8)
}

/// O registro de 104 bytes da rodada `rodada` (palavra com sinal) a partir de `rodadas`.
fn registro_da_rodada(rodadas: usize, rodada: u32) -> usize {
    rodadas.wrapping_add((rodada as i16 as isize * 104) as usize)
}

/// { 0x521e60 } (cdecl, Cup.cpp): a data do sorteio da rodada: dia do ano na palavra +0 e ano − ano-base
/// (byte) em +2, da data de `dia`/`mes`/ano-base + `anos` levada ao dia da semana. Devolve o registro.
unsafe extern "C" fn data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned(r as *mut u16, d);
    *((r + 2) as *mut u8) = a.wrapping_sub(base as u8); // ano do sorteio
    r
}

/// { 0x521eb0 } (cdecl, Cup.cpp): a data do jogo da rodada, como a do sorteio, em +3 (dia) e +5 (ano −
/// ano-base); o horário (byte) em +6 e as marcas do mando em +0x13. Devolve o registro.
unsafe extern "C" fn data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned((r + 3) as *mut u16, d); // dia do jogo
    *((r + 5) as *mut u8) = a.wrapping_sub(base as u8);
    *((r + 6) as *mut u8) = hora as u8;
    core::ptr::write_unaligned((r + 0x13) as *mut u32, mando);
    r
}

/// { 0x5c64e0 } (thiscall, ret 4, friendly.cpp): tira o `clube` dos amistosos planejados: no vetor
/// [+0x34d8] ([+0x34e0] registros de 0x14 bytes) o id do clube em +8 vira −1; na lista [+0x34c8]
/// (0x68d1b0), o ponteiro do clube em +0xc vira 0.
unsafe extern "thiscall" fn tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let vetor = (amistosos + 0x34d8) as *const usize;
    let n = (amistosos + 0x34e0) as *const i32;
    if *vetor != 0 {
        let mut i = 0;
        while i < *n {
            let id = (*vetor + 8 + 0x14 * i as usize) as *mut u32;
            if *id == *(clube as *const u32) {
                *id = 0xffff_ffff;
            }
            i += 1;
        }
    }
    let lista = (amistosos + 0x34c8) as *const usize;
    if *lista == 0 {
        return;
    }
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((*lista + 0xa) as *mut u32, core::ptr::read_unaligned((*lista + 2) as *const u32));
    loop {
        let e = prox(*lista);
        if e == 0 {
            return;
        }
        if *((e + 0xc) as *const usize) == clube {
            *((e + 0xc) as *mut usize) = 0; // sai da lista
        }
    }
}

unsafe extern "C" fn comparar_data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(531);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rodadas, rodada, dia, mes, anos, semana, base),
        || data_do_sorteio_da_rodada(rodadas, rodada, dia, mes, anos, semana, base));
    PLACAR.registrar(igual && a == b, || format!("data_do_sorteio_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "C" fn comparar_data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(532);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rodadas, rodada, dia, mes, anos, semana, hora, base, mando),
        || data_do_jogo_da_rodada(rodadas, rodada, dia, mes, anos, semana, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("data_do_jogo_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn comparar_tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(533);
    let mut regioes = vec![];
    let vetor = *((amistosos + 0x34d8) as *const usize);
    let n = *((amistosos + 0x34e0) as *const i32);
    if vetor != 0 && n > 0 {
        regioes.push((vetor, n as usize * 0x14));
    }
    let lista = *((amistosos + 0x34c8) as *const usize);
    if lista != 0 {
        // o cursor e o ponteiro +0xc de cada amistoso da lista (percorrida aqui antes; as duas
        // rotinas reiniciam o cursor)
        regioes.push((lista + 0xa, 4));
        let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
        loop {
            let e = prox(lista);
            if e == 0 {
                break;
            }
            regioes.push((e + 0xc, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(amistosos, clube),
        || tirar_clube_dos_amistosos(amistosos, clube));
    PLACAR.registrar(igual, || format!("tirar_clube_dos_amistosos({:#x}) {:x?} x {:x?}", clube, mo, mr));
}

// ---------- 210a leva: as regras da rodada no jogo da copa, a chave do jogo e o país da excursão (Cup.cpp, friendly.cpp) ----------

/// A rodada `r` da copa: registro de 104 bytes em [copa+0xa3].
unsafe fn rodada_da_copa(copa: usize, r: i32) -> usize {
    (*((copa + 0xa3) as *const usize) as isize + r as isize * 104) as usize
}

/// A linha da chave `k` da rodada `r` da copa: [[copa+0xad] + 4·r] + 53·k.
unsafe fn linha_da_chave(copa: usize, r: i32, k: i32) -> usize {
    (*((*((copa + 0xad) as *const usize) as isize + r as isize * 4) as *const usize) as isize + k as isize * 53) as usize
}

/// O registro de 6 bytes da vaga `v` da copa: [copa+0xa7] + 6·v.
unsafe fn vaga_da_copa(copa: usize, v: i16) -> usize {
    (*((copa + 0xa7) as *const usize) as isize + v as isize * 6) as usize
}

/// { 0x521d60 } (thiscall, ret 8, Cup.cpp): passa as regras da rodada do jogo (palavra +0xf da rodada
/// [jogo+0x34]) para as marcas do jogo (+0x38, OU). Com a marca 0x800 (decide quem passa no empate), o
/// nível (byte +4 da vaga, com sinal) dos dois lados da chave `chave` decide: mandante menor tira 0x800 e
/// põe 0x1000; maior mantém 0x800; igual tira 0x800 e põe 1.
unsafe extern "thiscall" fn regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let r = *((jogo + 0x34) as *const i16) as i32;
    let marcas = (jogo + 0x38) as *mut u16;
    let m = core::ptr::read_unaligned(marcas) | core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0xf) as *const u16);
    core::ptr::write_unaligned(marcas, m);
    if m & 0x800 == 0 {
        return;
    }
    let linha = linha_da_chave(copa, r, chave);
    let nivel = |o: usize| *((vaga_da_copa(copa, core::ptr::read_unaligned((linha + o) as *const i16)) + 4) as *const i8);
    let (a, b) = (nivel(0), nivel(2));
    let m = if a < b { (m ^ 0x800) | 0x1000 } else if a > b { m | 0x800 } else { (m ^ 0x800) | 1 };
    core::ptr::write_unaligned(marcas, m);
}

/// { 0x519c90 } (thiscall, ret 4, Cup.cpp): a chave da rodada do jogo (+0x34) em que está o clube
/// mandante do jogo (+0xc), olhando os dois lados de cada chave (53 bytes; vaga negativa é vazia); -1 se
/// nenhuma. O número de chaves é a palavra +0x1a da rodada. Jogo nulo: a original (caixa de erro).
unsafe extern "thiscall" fn chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    if jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(512)(copa, jogo);
    }
    let r = *((jogo + 0x34) as *const i16) as i32;
    let n = core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0x1a) as *const i16) as i32;
    let mandante = *((jogo + 0xc) as *const usize);
    for i in 0..n {
        let linha = linha_da_chave(copa, r, i);
        for o in [0, 2] {
            let v = core::ptr::read_unaligned((linha + o) as *const i16);
            if v >= 0 && *(core::ptr::read_unaligned(vaga_da_copa(copa, v) as *const usize) as *const usize) == mandante {
                return i;
            }
        }
    }
    -1
}

/// { 0x5c4680 } (thiscall, ret 4, friendly.cpp): sorteia o país da excursão do clube. Pela reputação
/// (+0x80) o alcance: ≥ 8000 qualquer país, ≥ 6000 e ≥ 4000 o mesmo continente (+0x71), ≥ 2000 o mesmo
/// continente e a mesma região (+0x75); abaixo, nenhum (0). Candidatos: os outros países (lista
/// [0xae23a8], 0x122 bytes, [0xae2350] países) com +0x86 ≥ 20, +0x7e = 1, +0x84 ≤ 2 e +0x8e ≥ 5; cada um
/// tira 0x90cfc0(100) e fica o maior (estrito, comparado como byte com sinal).
unsafe extern "thiscall" fn pais_da_excursao(_this: usize, clube: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let proprio = *((clube + 0x53) as *const usize);
    let alcance: i8 = match *((clube + 0x80) as *const i16) {
        8000.. => 5,
        6000.. => 4,
        4000.. => 3,
        2000.. => 2,
        _ => return 0,
    };
    let mut escolhido = 0usize;
    let mut melhor: i8 = -1;
    if *(0xae2350 as *const i32) <= 0 {
        return 0;
    }
    let mut p = *(0xae23a8 as *const usize);
    let mut i: i16 = 0;
    loop {
        let candidato = p != proprio
            && *((p + 0x86) as *const i16) >= 0x14
            && *((p + 0x7e) as *const u8) == 1
            && *((p + 0x84) as *const i8) <= 2
            && *((p + 0x8e) as *const i16) >= 5;
        if candidato {
            let continente = || *((p + 0x71) as *const u32) == *((proprio + 0x71) as *const u32);
            let perto = match alcance {
                2 => *((p + 0x75) as *const u8) == *((proprio + 0x75) as *const u8) && continente(),
                5.. => true,
                _ => continente(),
            };
            if perto {
                let r = sorteio(100) as i8;
                if r > melhor {
                    melhor = r;
                    escolhido = p;
                }
            }
        }
        i = i.wrapping_add(1);
        p += 0x122;
        if i as i32 >= *(0xae2350 as *const i32) {
            return escolhido;
        }
    }
}

unsafe extern "thiscall" fn comparar_regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(511);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(copa, jogo, chave),
        || regras_da_rodada_no_jogo(copa, jogo, chave));
    PLACAR.registrar(igual, || format!("regras_da_rodada_no_jogo(chave {}) {:x?} x {:x?}", chave, mo, mr));
}

unsafe extern "thiscall" fn comparar_chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(512)(copa, jogo);
    if jogo == 0 {
        return r;
    }
    let n = chave_do_jogo_na_copa(copa, jogo);
    PLACAR.registrar(r == n, || format!("chave_do_jogo_na_copa original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_pais_da_excursao(this: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(513);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pais_da_excursao(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pais_da_excursao({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

