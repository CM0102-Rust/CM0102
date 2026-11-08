// ---------- %%LEVA%%a leva: o histórico da carreira do jogador (a tabela das temporadas por clube montada do arquivo de histórico) e os anos desde que jogou num clube (player_stats.cpp) ----------

unsafe fn cqf8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqf8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf8_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqf8_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O tamanho de uma entrada da tabela (uma categoria de uma temporada num clube): +0 o ano, +2 o id do clube (−1
/// nenhum), +6..+0x26 os 32 bytes da categoria (+6 jogos, +7 ..., +0x14 a palavra), +0x26 o valor da transferência
/// (−1 nenhum), +0x2a categoria × 8 e o tipo nos 3 bits de baixo. Seis entradas (categorias 1..6) por temporada.
const HISTORY_ENTRY_SIZE: usize = 0x2b;

/// A entrada montada na pilha pela original (0x2b bytes, zerada antes de cada uma).
struct PlayerStatsEntry([u8; HISTORY_ENTRY_SIZE]);

impl PlayerStatsEntry {
    fn nova(ano: u16, clube: u32, cat: u8, transf: u32) -> PlayerStatsEntry {
        let mut r = [0u8; HISTORY_ENTRY_SIZE];
        r[0..2].copy_from_slice(&ano.to_le_bytes());
        r[2..6].copy_from_slice(&clube.to_le_bytes());
        r[0x26..0x2a].copy_from_slice(&transf.to_le_bytes());
        r[0x2a] = cat << 3;
        PlayerStatsEntry(r)
    }
    unsafe fn gravar(&self, a: usize) {
        core::ptr::copy_nonoverlapping(self.0.as_ptr(), a as *mut u8, HISTORY_ENTRY_SIZE);
    }
}

/// A caixa "Error" de player_stats.cpp (0xa78310) na linha `linha`.
unsafe fn show_player_stats_error(linha: u32) {
    show_error_at(0xa78310, linha);
}

/// A entrada `k` (contando de 1) a partir da conta `n`: buf + (n + k) × 0x2b − 0x2b.
fn history_entry_at(buf: usize, n: i32, k: i32) -> usize {
    buf.wrapping_add(((n + k) as isize * HISTORY_ENTRY_SIZE as isize - HISTORY_ENTRY_SIZE as isize) as usize)
}

/// { 0x7c3e40 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::build_career_history(pessoa, &conta)`: a tabela
/// (malloc 0x944e46 de 500 entradas, realloc 0x945667 de 500 em 500; quem chama libera com 0x9452ca) com seis entradas
/// por temporada e clube, e a conta em `*conta`. Pessoa nula (linha 0xb98) ou sem ficha (0xbaa): a caixa e 0; id a
/// partir de [0xae2364]: 0; sem memória: a caixa (0xbb2, 0xc00, 0xc26, 0xc41) e 0.
/// (1) A temporada atual: as seis categorias com o ano da temporada do clube (0x669950), o clube (+0x39; −1 sem ele), a
/// transferência −1 e os 32 bytes da categoria (0x7c3920) quando há. (2) O histórico do arquivo (FILE_LLIST::get_nodes
/// 0x58f120 do [this] pelas posições +0x18/+0x1c do registro de 0x27 bytes da ficha em [this+0x30]), do fim para o
/// começo (0x68d1d0): cada nó de outra temporada, clube ou tipo abre um grupo: a conta anda 6; anos vazios entre os dois
/// (até a conta passar de 500) viram grupos vazios (clube −1, transferência −1); o grupo do nó leva o ano, o clube, a
/// transferência e o tipo do nó; o nó vai para a entrada da sua categoria (+0x2a / 8) no grupo. A lista é destruída
/// (0x68d040 e 0x944c94). (3) A conta anda 6; cada entrada leva a transferência e o tipo da entrada seis adiante (as
/// do último grupo, −1 e 0). (4) Um grupo com um seguinte do mesmo ano e clube, sem transferência nos dois, tipo do
/// seguinte diferente de 1 e a palavra +0x14 diferente de −1 nos dois, soma os bytes +6..+0x13 e a palavra +0x14 no
/// seguinte e some (ano 0). (5) O grupo sem jogos (+6, +7 da primeira e da última entrada), com ano, sem transferência
/// e de tipo 0 some quando sem clube ou quando outro grupo tem o ano dele e o grupo seguinte é do mesmo clube; o de
/// jogos com a última entrada sem clube (+0xd9) também some. (6) Os grupos de ano 0 saem da tabela (os seguintes
/// descem) e a conta cai 6 por grupo.
unsafe extern "thiscall" fn build_career_history(this: usize, p: usize, conta: *mut i32) -> usize {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let realloc: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x669950usize);
    let categoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    *conta = 0;
    if p == 0 {
        show_player_stats_error(0xb98);
        return 0;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return 0;
    }
    let f = cqf8_d(p + 0x61);
    if f == 0 {
        show_player_stats_error(0xbaa);
        return 0;
    }
    let mut buf = malloc(0x53fc);
    if buf == 0 {
        show_player_stats_error(0xbb2);
        return 0;
    }
    let mut cap: i32 = 500;
    let clube = cqf8_d(p + 0x39);
    let mut ultimo = PlayerStatsEntry::nova(0, 0, 0, 0);
    for cat in 1..=6u8 {
        let ano = ano_do_clube(clube);
        let id = if clube == 0 { 0xffff_ffff } else { cqf8_d(clube) as u32 }; // cqf8: o clube da temporada atual
        crate::comparison::branch("cqf_8 temporada atual");
        let mut r = PlayerStatsEntry::nova(ano, id, cat, 0xffff_ffff);
        let c = categoria(this, f, cat as u32);
        if c != 0 {
            core::ptr::copy_nonoverlapping(c as *const u8, r.0.as_mut_ptr().add(6), 32);
        }
        r.gravar(history_entry_at(buf, *conta, cat as i32));
        ultimo = r;
    }
    // o ano, o clube e o tipo do grupo aberto (o tipo começa em 0xff: o primeiro nó sempre abre grupo)
    let mut ano_atual = u16::from_le_bytes([ultimo.0[0], ultimo.0[1]]);
    let mut clube_atual = u32::from_le_bytes([ultimo.0[2], ultimo.0[3], ultimo.0[4], ultimo.0[5]]);
    let mut tipo_atual: u8 = 0xff;
    let reg_ficha = cqf8_d(this + 0x30).wrapping_add((cqf8_i(f) as isize * 0x27) as usize);
    let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x58f120usize);
    let lista = nos(cqf8_d(this), cqf8_d(reg_ficha + 0x18) as u32, cqf8_d(reg_ficha + 0x1c) as u32);
    if lista != 0 {
        let anterior: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1d0usize);
        cqf8_pd(lista + 0xa, cqf8_d(lista + 6) as u32);
        let mut no = anterior(lista);
        while no != 0 {
            let tipo = cqf8_b(no + 0x2a);
            if !(cqf8_w(no) == ano_atual && cqf8_d(no + 2) as u32 == clube_atual && tipo & 7 == tipo_atual) {
                *conta += 6;
                ano_atual = ano_atual.wrapping_sub(1);
                if ano_atual > cqf8_w(no) {
                    loop {
                        let mut tam = cap as usize * HISTORY_ENTRY_SIZE;
                        for cat in 1..=6u8 {
                            let r = PlayerStatsEntry::nova(ano_atual, 0xffff_ffff, cat, 0xffff_ffff);
                            if *conta + cat as i32 - 1 >= cap {
                                tam += 0x53fc;
                                cap += 500;
                                buf = realloc(buf, tam);
                                if buf == 0 {
                                    show_player_stats_error(0xc00);
                                    return 0;
                                }
                            }
                            r.gravar(history_entry_at(buf, *conta, cat as i32));
                        }
                        *conta += 6;
                        if *conta > 500 {
                            break;
                        }
                        ano_atual = ano_atual.wrapping_sub(1);
                        if ano_atual <= cqf8_w(no) {
                            break;
                        }
                    }
                }
                let mut tam = cap as usize * HISTORY_ENTRY_SIZE;
                let ano_no = cqf8_w(no); // cqf8: o ano do grupo do nó
                let clube_no = cqf8_d(no + 2) as u32; // cqf8: o clube do grupo do nó
                for cat in 1..=6u8 {
                    let mut r = PlayerStatsEntry::nova(ano_no, clube_no, cat, cqf8_d(no + 0x26) as u32);
                    r.0[0x2a] = (cat << 3) | (tipo & 7);
                    crate::comparison::branch("cqf_8 temporada do historico");
                    if *conta + cat as i32 - 1 >= cap {
                        tam += 0x53fc;
                        cap += 500;
                        buf = realloc(buf, tam);
                        if buf == 0 {
                            show_player_stats_error(0xc26);
                            return 0;
                        }
                    }
                    r.gravar(history_entry_at(buf, *conta, cat as i32));
                }
                ano_atual = cqf8_w(no);
                clube_atual = cqf8_d(no + 2) as u32;
                tipo_atual = tipo & 7;
            }
            if (tipo >> 3) as i32 - 1 + *conta >= cap {
                cap += 500;
                buf = realloc(buf, cap as usize * HISTORY_ENTRY_SIZE);
                if buf == 0 {
                    show_player_stats_error(0xc41);
                    return 0;
                }
            }
            core::ptr::copy_nonoverlapping(no as *const u8, history_entry_at(buf, *conta, (tipo >> 3) as i32) as *mut u8, HISTORY_ENTRY_SIZE);
            no = anterior(lista);
        }
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(lista);
        apagar(lista);
    }
    *conta += 6;
    // (3) a transferência e o tipo descem do grupo seguinte
    let n = *conta;
    for i in 0..n {
        let e = buf + i as usize * HISTORY_ENTRY_SIZE;
        if i + 6 < n {
            cqf8_pd(e + 0x26, cqf8_d(e + 6 * HISTORY_ENTRY_SIZE + 0x26) as u32);
            *((e + 0x2a) as *mut u8) = (cqf8_b(e + 0x2a) & 0xf8) | (cqf8_b(e + 6 * HISTORY_ENTRY_SIZE + 0x2a) & 7);
        } else {
            cqf8_pd(e + 0x26, 0xffff_ffff);
            *((e + 0x2a) as *mut u8) = cqf8_b(e + 0x2a) & 0xf8;
        }
    }
    // (4) as entradas iguais do grupo seguinte somam e a de cima some
    let mut k = 6i32;
    while k - 6 < *conta {
        let ei = buf + (k - 6) as usize * HISTORY_ENTRY_SIZE;
        let mut j = k;
        while j < *conta {
            let ej = buf + j as usize * HISTORY_ENTRY_SIZE;
            if cqf8_w(ei) == cqf8_w(ej)
                && cqf8_b(ej + 0x2a) & 7 != 1
                && cqf8_i(ej + 0x26) == -1
                && cqf8_i(ei + 0x26) == -1
                && cqf8_d(ei + 2) == cqf8_d(ej + 2)
                && cqf8_w(ei + 0x14) != 0xffff
                && cqf8_w(ej + 0x14) != 0xffff
            {
                for o in 6..0x14 {
                    *((ej + o) as *mut u8) = cqf8_b(ej + o).wrapping_add(cqf8_b(ei + o)); // cqf8: soma da entrada repetida
                }
                cqf8_pw(ej + 0x14, cqf8_w(ej + 0x14).wrapping_add(cqf8_w(ei + 0x14)));
                if cqf8_i(ei + 0x26) != -1 || (cqf8_b(ei + 0x2a) & 7 == 1 && cqf8_i(ej + 0x26) == -1) {
                    cqf8_pd(ej + 0x26, cqf8_d(ei + 0x26) as u32);
                    *((ej + 0x2a) as *mut u8) = (cqf8_b(ej + 0x2a) & 0xf8) | (cqf8_b(ei + 0x2a) & 7);
                }
                cqf8_pw(ei, 0);
                break;
            }
            j += 6;
        }
        k += 1;
    }
    // (5) os grupos vazios somem (ano 0)
    let mut g = 0i32;
    while g < *conta {
        let e = buf + g as usize * HISTORY_ENTRY_SIZE;
        let zerar = if cqf8_b(e + 6) == 0
            && cqf8_b(e + 7) == 0
            && cqf8_b(e + 0xdd) == 0
            && cqf8_b(e + 0xde) == 0
            && cqf8_w(e) != 0
            && cqf8_i(e + 0x26) == -1
            && cqf8_b(e + 0x2a) & 7 == 0
        {
            let mut z = false;
            let mut j = 0i32;
            while j < *conta {
                if g != j {
                    let c = cqf8_i(e + 2);
                    if c == -1 {
                        z = true;
                        break;
                    }
                    let ej = buf + (j / 6) as usize * 0x102;
                    if cqf8_w(ej) == cqf8_w(e) && g + 6 < *conta && cqf8_i(e + 0x104) == c {
                        z = true;
                        break;
                    }
                }
                j += 6;
            }
            z
        } else {
            cqf8_i(e + 0xd9) == -1
        };
        if zerar {
            for t in 0..6 {
                cqf8_pw(e + t * HISTORY_ENTRY_SIZE, 0); // cqf8: o grupo some
            }
        }
        g += 6;
    }
    // (6) os grupos de ano 0 saem e os seguintes descem
    let mut i = 0i32;
    while i < *conta {
        let e = buf + (i / 6) as usize * 0x102;
        if cqf8_w(e) == 0 {
            let mut d = i + 6;
            let mut a = e + 0x102;
            while d < *conta {
                core::ptr::copy(a as *const u8, (a - 0x102) as *mut u8, 0x102);
                d += 6;
                a += 0x102;
            }
            *conta -= 6;
            i -= 6;
        }
        i += 6;
    }
    buf
}

/// Comparador: as duas montam a tabela (a original e a cópia, a partir do mesmo estado; o arquivo de histórico só é
/// lido) e comparam a conta e as entradas; a da cópia é liberada (0x9452ca) e o jogo segue com a da original. Os casos
/// de erro (caixa): só a original.
unsafe extern "thiscall" fn compare_build_career_history(this: usize, p: usize, conta: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize>(%%I0%%);
    if p == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && cqf8_d(p + 0x61) == 0) {
        return o(this, p, conta);
    }
    let r = o(this, p, conta);
    let mut n = 0i32;
    let t = build_career_history(this, p, &mut n);
    let igual = if r == 0 || t == 0 {
        r == t && *conta == n
    } else {
        *conta == n
            && core::slice::from_raw_parts(r as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
                == core::slice::from_raw_parts(t as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
    };
    SCOREBOARD.record(igual, || {
        let k = (*conta).max(n).max(0) as usize;
        let dif = (0..k).find(|&i| {
            r == 0 || t == 0 || core::slice::from_raw_parts((r + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE) != core::slice::from_raw_parts((t + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE)
        });
        format!("build_career_history(pessoa {:x}) conta original={} rust={}, primeira entrada diferente {:?}", p, *conta, n, dif)
    });
    if t != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        liberar(t);
    }
    r
}

/// { 0x7c6e30 } (stdcall, ret 8, player_stats.cpp): `player_stats_years_since_at_club(pessoa, clube)`: quantos anos
/// desde que a pessoa jogou pelo clube: 0 se está nele (+0x39), −1 se nunca; senão o ano de hoje (0xae2c92) menos o
/// maior ano dos grupos do histórico (0x7c3e40 em [0xb74350]; a última entrada de cada grupo, +0xd7 o ano, +0xd9 o
/// clube) com o clube. Pessoa nula (linha 0x133f), clube nulo (0x1347) ou sem ficha (0x135f): a caixa e −1; id a partir
/// de [0xae2364]: −1. Quem chama a põe com o ecx do PlayerStats, que ela não usa. Pela 0x5b03e0 (não roda na carreira
/// automática: passageira).
unsafe extern "stdcall" fn player_stats_years_since_at_club(p: usize, clube: usize) -> i16 {
    if p == 0 {
        show_player_stats_error(0x133f);
        return -1;
    }
    if clube == 0 {
        show_player_stats_error(0x1347);
        return -1;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return -1;
    }
    if cqf8_d(p + 0x39) == clube {
        return 0;
    }
    if cqf8_d(p + 0x61) == 0 {
        show_player_stats_error(0x135f);
        return -1;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cqf8_d(0xb74350), p, &mut n);
    let mut maior = 0x7b3i32;
    if t != 0 {
        let mut i = 0i32;
        let mut e = t + 0xd7;
        while i < n {
            let c = cqf8_i(e + 2);
            if c >= 0 && c == cqf8_i(clube) && cqf8_w(e) as i32 > maior {
                maior = cqf8_w(e) as i32;
            }
            i += 6;
            e += 0x102;
        }
    }
    liberar(t);
    if maior == 0x7b3 {
        return -1;
    }
    cqf8_w(0xae2c92).wrapping_sub(maior as u16) as i16
}

/// Comparador: a devolução (16 bits); a tabela é montada e liberada pelas duas. Os casos de erro: só a original.
unsafe extern "stdcall" fn compare_player_stats_years_since_at_club(p: usize, clube: usize) -> i16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i16>(%%I1%%);
    if p == 0 || clube == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && cqf8_d(p + 0x39) != clube && cqf8_d(p + 0x61) == 0) {
        return o(p, clube);
    }
    let r = o(p, clube);
    let n = player_stats_years_since_at_club(p, clube);
    SCOREBOARD.record(r == n, || format!("player_stats_years_since_at_club(pessoa {:x}, clube {:x}) original={} rust={}", p, clube, r, n));
    r
}

/// { 0x7c6de0 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_staff(a, b)`, a ordem do qsort dos
/// registros do staff_history.dat na criação das estatísticas (0x7bfd50, ponteiro em 0x7c05ad): `a` nulo 1, `b` nulo −1;
/// pela pessoa (+4, sem sinal), depois pelo ano (+8, palavra com sinal), depois a diferença do primeiro dword.
unsafe extern "C" fn player_stats_compare_history_by_staff(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (pa, pb) = (cqf8_d(a + 4) as u32, cqf8_d(b + 4) as u32);
    if pa > pb {
        return 1;
    }
    if pa < pb {
        return -1;
    }
    let (aa, ab) = (cqf8_w(a + 8) as i16, cqf8_w(b + 8) as i16);
    if aa > ab {
        return 1;
    }
    if aa != ab {
        return -1;
    }
    cqf8_i(a).wrapping_sub(cqf8_i(b))
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn compare_player_stats_compare_history_by_staff(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = player_stats_compare_history_by_staff(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_history_by_staff({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
