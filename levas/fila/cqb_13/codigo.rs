// ---------- %%LEVA%%a leva: o calendário: o conflito de dois jogos no mesmo dia e os anos de jogos de um clube (fix_man.cpp) ----------

/// Modo de ensaio da 0x5ae380 (o comparador): a cópia para na decisão, antes de mexer no calendário.
static mut CQB13_ENSAIO: bool = false;

unsafe fn cqb13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x5ae380 } (thiscall, ret 8, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::resolve_same_day_clash(jogo, conflitos)`: o jogo novo `jogo` cai no mesmo dia de outro, o primeiro
/// da lista `conflitos` (o cursor +0xa volta ao começo +2 e 0x68d1b0 o lê); sem um dos dois, a caixa da linha 0xde6
/// e devolve 1. Copia os dois (0x4f bytes) e acha o dia da semana alvo pela data do da lista (+0x2a, +0x28; 0x549ef0,
/// 0x417c10, 0x54a3f0): os dias 0 e 5 da 0x54a3f0 vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5 (0x54a2d0 dá a
/// data). A prioridade de cada um é a da competição (+0x69 da +0x14), +0x15 com seleção
/// como mandante (+0x1c, 0x53b900); o da lista sai do dia quando a do novo é maior (ou igual, com a reputação +0x80
/// do mandante do novo maior) e ele ainda não foi jogado (+0x43 = 0xff): tirado do calendário (0x5abc30), o novo entra
/// (0x5aa680); senão sai o novo. O que sai ganha a data nova; com conflito nela (0x5ad780, a lista destruída e
/// liberada: 0x68d040, 0x944c94), jogo fixo (+0x3a = 1) devolve 0, senão 0x5ab510 acha outra data; entra no calendário
/// (0x5aa680) com a notícia da remarcação (0x5aac70, causada pelo outro). Devolve 1.
unsafe extern "thiscall" fn resolve_same_day_clash(this: usize, jogo: usize, conflitos: usize) -> u32 {
    // as datas: ecx = a data, sem pushes -> thiscall (0x54a110, 0x54a3f0); 0x549ef0: push [+0x28]; push [+0x2a];
    // 0x417c10: push ret; push &tmp; 0x54a2d0: push &b; push alvo -> thiscall
    let padrao: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> *mut [u8; 8] = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(0x417c10usize);
    let semana: unsafe extern "thiscall" fn(*mut [u8; 8]) -> i32 = core::mem::transmute(0x54a3f0usize);
    let ajustar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, *mut [u8; 8]) -> u32 = core::mem::transmute(0x54a2d0usize);
    // 0x68d1b0: ecx = a lista, sem pushes -> thiscall; 0x53b900: push clube; add esp, 4 -> cdecl
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    padrao(&mut a);
    padrao(&mut b);
    let mut gravidade: u32 = 0; // [esp+0x10]: o alvo sem caso da tabela e a gravidade da 0x5ad780
    core::ptr::write_unaligned((conflitos + 0xa) as *mut usize, cqb13_d(conflitos + 2));
    let outro = proximo(conflitos);
    if jogo == 0 || outro == 0 {
        if !CQB13_ENSAIO {
            aviso_em(0x9cc208, 0xde6); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        }
        return 1;
    }
    let (mut novo, mut velho) = ([0u8; 0x50], [0u8; 0x50]);
    core::ptr::copy_nonoverlapping(jogo as *const u8, novo.as_mut_ptr(), 0x4f);
    core::ptr::copy_nonoverlapping(outro as *const u8, velho.as_mut_ptr(), 0x4f);
    let (mut tmp, mut tmp2) = ([0u8; 8], [0u8; 8]);
    // ponytail: os bytes de cima dos dois pushes de 16 bits não contam (0x549ef0 lê a palavra)
    let r = nova(&mut tmp, cqb13_w(outro + 0x2a) as u16 as u32, cqb13_w(outro + 0x28) as u16 as u32);
    copia(&mut a, &mut tmp2, r);
    let alvo = match semana(&mut a) as i16 {
        0 | 5 => 6,
        1 | 3 => 2,
        2 => 1,
        4 | 6 => 5,
        _ => gravidade,
    };
    crate::comparacao::ramo("cqb_13 dia alvo");
    ajustar(&mut a, alvo, &mut b); // cqb13: o dia da semana alvo
    let n = novo.as_mut_ptr() as usize;
    let v = velho.as_mut_ptr() as usize;
    let mut pn = cqb13_w(cqb13_d(n + 0x14) + 0x69);
    let mut pv = cqb13_w(cqb13_d(v + 0x14) + 0x69);
    if selecao(cqb13_d(n + 0x1c)) != 0 {
        pn = pn.wrapping_add(0x15);
    }
    if selecao(cqb13_d(v + 0x1c)) != 0 {
        pv = pv.wrapping_add(0x15);
    }
    let novo_vence = pn > pv // cqb13: a prioridade maior fica
        || (pn == pv && cqb13_w(cqb13_d(n + 0x1c) + 0x80) > cqb13_w(cqb13_d(v + 0x1c) + 0x80));
    if novo_vence {
        crate::comparacao::ramo("cqb_13 novo vence");
    }
    let sai_o_velho = novo_vence && *((v + 0x43) as *const u8) == 0xff; // cqb13: o da lista ainda não jogado
    if CQB13_ENSAIO {
        return 1;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let conflito: unsafe extern "thiscall" fn(usize, usize, *mut u32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let outra_data: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5ab510usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x5aac70usize);
    // as do calendário: ecx = this; push o jogo (0x5abc30, 0x5ab510; ret 4), push 0; push o jogo (0x5aa680; ret 8),
    // push 0; push &gravidade; push o jogo (0x5ad780; ret 0xc), push 0; push causa; push o jogo (0x5aac70; ret 0xc);
    // 0x68d040: ecx = a lista; 0x944c94: push lista; add esp, 4 -> cdecl
    let (sai, causa) = if sai_o_velho {
        crate::comparacao::ramo("cqb_13 sai o jogo da lista");
        tirar(this, outro);
        juntar(this, jogo, 0);
        (v, jogo)
    } else {
        (n, outro)
    };
    core::ptr::write_unaligned((sai + 0x2a) as *mut u16, u16::from_le_bytes([b[0], b[1]]));
    core::ptr::write_unaligned((sai + 0x28) as *mut u16, u16::from_le_bytes([b[2], b[3]]));
    let lista = conflito(this, sai, &mut gravidade, 0);
    if lista != 0 {
        destruir(lista);
        liberar(lista);
        if *((sai + 0x3a) as *const u8) == 1 {
            return 0;
        }
        outra_data(this, sai);
    }
    juntar(this, sai, 0);
    noticia(this, sai, causa, 0);
    1
}

/// Comparador em ensaio: a rotina só grava o cursor da lista (+0xa, lido pela 0x68d1b0) e o resto vai pelas
/// chamadas do calendário (tirar, juntar, conflitos que alocam a lista, a notícia), que não podem rodar duas vezes.
/// A cópia roda em ensaio (para na decisão, sem a caixa) e a original depois, a partir do mesmo estado: lado a lado no
/// cabeçalho da lista (+2 começo, +0xa cursor). Sem o jogo ou o primeiro da lista: só a original (a caixa).
unsafe extern "thiscall" fn comparar_resolve_same_day_clash(this: usize, jogo: usize, conflitos: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    let cabeca = cqb13_d(conflitos + 2);
    if jogo == 0 || cabeca == 0 || cqb13_d(cabeca) == 0 {
        return o(this, jogo, conflitos);
    }
    let reg = [(conflitos + 2, 0xc)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQB13_ENSAIO = true;
    resolve_same_day_clash(this, jogo, conflitos);
    CQB13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    let r = o(this, jogo, conflitos);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr && (r == 0 || r == 1), || format!("resolve_same_day_clash(jogo {:#x}) devolveu {}, cursor da lista {:x?} x {:x?}", jogo, r, mo, mr));
    r
}

/// { 0x5aee40 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`:
/// o primeiro ano com jogos do clube `[id]`: 2001, ou 2002 com o bit 0 da palavra dele em [0xae2a5c]. Devolve `ax`
/// (o chamador da tela compara 16 bits).
unsafe extern "thiscall" fn get_club_first_fixture_year(_this: usize, id: usize) -> u16 {
    let b = *((cqb13_d(0xae2a5c) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u8);
    0x7d1 + (b & 1) as u16
}

/// { 0x5aee60 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`:
/// o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0 (metade); sem ano, o ano corrente
/// [0xae2c92]. Devolve `ax` (o chamador da tela guarda 16 bits).
unsafe extern "thiscall" fn get_club_last_fixture_year(_this: usize, id: usize) -> u16 {
    let w = core::ptr::read_unaligned((cqb13_d(0xae2a5c) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u16);
    if w & 0xfffe != 0 { w >> 1 } else { *(0xae2c92 as *const u16) }
}

comparar_valor!(comparar_get_club_first_fixture_year, get_club_first_fixture_year, %%I1%%, (id: usize) -> u16);
comparar_valor!(comparar_get_club_last_fixture_year, get_club_last_fixture_year, %%I2%%, (id: usize) -> u16);
