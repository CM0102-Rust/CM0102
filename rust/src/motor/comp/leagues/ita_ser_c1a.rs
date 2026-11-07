// ---------- 278a leva: o play-off de acesso e o play-out da Série C1 italiana (os dois grupos) e a passagem de fase da Third Division inglesa (ita_ser_c1a.cpp, ita_ser_c1b.cpp, eng_third.cpp) ----------

unsafe fn cql5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube da linha `i` da tabela [+0xb1] (0x3b bytes).
unsafe fn cql5_clube(liga: usize, i: usize) -> usize {
    cql5_d(cql5_d(liga + 0xb1) + i * 0x3b)
}

/// A passagem de fase das três (League::advance_stage, o slot +0x28; o corpo de 0x64b9e0, 0x64fba0 e 0x578330): com a
/// fase atual (+0x30) já na última (+0x2c − 1, com sinal), a caixa do fonte `arquivo` na linha `linha`; senão +0x30 + 1
/// e, ao chegar à fase 0 (a primeira depois da liga), cria as fases (`criar`: mov ecx, this -> thiscall, ret 0).
unsafe fn cql5_passar_de_fase(this: usize, arquivo: usize, linha: u32, criar: usize) {
    let fase = cql5_d(this + 0x30) as i32;
    if fase >= (cql5_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(arquivo, linha);
        return;
    }
    let nova = fase.wrapping_add(1);
    cql5_pd(this + 0x30, nova as usize);
    if nova == 0 {
        crate::comparacao::ramo("cql_5 fases do fim da liga");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(criar);
        f(this); // cql5: as fases do fim da liga
    }
}

/// { 0x64b9e0 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::advance_stage()`; cria o play-off e o play-out pela
/// 0x64ba70. Fonte 0x9ea474, linha 0x250.
unsafe extern "thiscall" fn fase_seguinte_da_c1a(this: usize) {
    cql5_passar_de_fase(this, 0x9ea474, 0x250, 0x64ba70)
}

/// { 0x64fba0 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::advance_stage()`; pela 0x64fc30. Fonte 0x9ea558, linha
/// 0x255.
unsafe extern "thiscall" fn fase_seguinte_da_c1b(this: usize) {
    cql5_passar_de_fase(this, 0x9ea558, 0x255, 0x64fc30)
}

/// { 0x578330 } (thiscall, ret 0, eng_third.cpp): `EngThird::advance_stage()`; cria o play-off da Third Division pela
/// 0x5783c0 (já em Rust). Fonte 0x9c4424, linha 0x29c.
unsafe extern "thiscall" fn fase_seguinte_da_third(this: usize) {
    cql5_passar_de_fase(this, 0x9c4424, 0x29c, 0x5783c0)
}

/// O comparador das três: sem criar fase (a fase atual só sobe), lado a lado em +0x30; na caixa ou ao criar as fases
/// (alocam), só a original (a criação tem o seu comparador).
unsafe fn cql5_comparar_passagem(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let fase = cql5_d(this + 0x30) as i32;
    if fase >= (cql5_d(this + 0x2c) as i32).wrapping_sub(1) || fase.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1856);
    cql5_comparar_passagem("fase_seguinte_da_c1a", this, || o(this), || fase_seguinte_da_c1a(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1858);
    cql5_comparar_passagem("fase_seguinte_da_c1b", this, || o(this), || fase_seguinte_da_c1b(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_third(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1860);
    cql5_comparar_passagem("fase_seguinte_da_third", this, || o(this), || fase_seguinte_da_third(this))
}

/// As linhas dos 4 clubes do play-off de acesso (2º contra 5º, 3º contra 4º: a lista é 5º, 2º, 4º, 3º) e do play-out
/// (14º contra 17º, 15º contra 16º: 17º, 14º, 16º, 15º), na ordem da lista que vai para a fase.
const CQL5_PLAYOFF: [usize; 4] = [4, 1, 3, 2]; // cql5: as linhas do play-off de acesso
const CQL5_PLAYOUT: [usize; 4] = [16, 13, 15, 14]; // cql5: as linhas do play-out

/// O corpo das duas (0x64ba70 e 0x64fc30, iguais fora o fonte, as linhas e a ordem de soltar as listas no fim):
/// 1) o campeão (a linha 0) vai para a Supercopa da Série C1 (a competição [[0xadadfc] + [0x9cf938] × 4], a lista de
///    inscritos [+0xa7] de 6 bytes): na vaga 0 se vazia, senão na vaga 1;
/// 2) a lista de 4 clubes (malloc 0x944e46 de 0x10: push; add esp, 4 -> cdecl; sem memória: a caixa da linha
///    `linhas[0]` e volta) com os da faixa do play-off (CQL5_PLAYOFF);
/// 3) o calendário da fase 0 (a virtual +0x3c: push 0; push &extra; push &rodadas; push 0; mov ecx, this -> thiscall,
///    ret 0x10; devolve o gabarito, `rodadas` e `extra` são palavras);
/// 4) a fase (operator new 0x944cf1 de 0xb2: push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00: 15 argumentos
///    empurrados, mov ecx, nova -> thiscall, ret 0x3c: a liga, os clubes (+0xbf, o byte com sinal), a lista, as
///    rodadas, o registro da competição (+4), o gabarito, o ano (+0x40), 0, 1, o `extra`, 0x14, 0, 0, 0 e as posições
///    dos 4 na tabela (registro local de 4 bytes: 4, 1, 3, 2)) vai para [+0xc][0]; sem fase: a caixa da linha
///    `linhas[1]` e volta; senão o sorteio da primeira rodada (Cup::set_up_round 0x51c800: push 0; mov ecx, fase ->
///    thiscall, ret 4) e solta o gabarito (0x9452ca: push; add esp, 4 -> cdecl);
/// 5) a mesma coisa com a fase 1 e os do play-out (CQL5_PLAYOUT; as posições 16, 13, 15, 14 no registro local e
///    1, 1 nos dois bytes) em [+0xc][1]; solta a lista e o gabarito (na ordem `lista_primeiro`); sem fase: a caixa da
///    linha `linhas[2]`; senão o sorteio da primeira rodada e +0x30 = 1.
/// Os bits de cima das palavras que a original empurra como dupla (os clubes, as rodadas, o ano e o `extra`) são lixo
/// da pilha ou de outros registradores; o construtor só lê a palavra (0x522e00: mov ax, word ptr [esp + ...]) e a
/// cópia passa zero neles.
unsafe fn cql5_criar_fases(this: usize, arquivo: usize, linhas: [u32; 3], lista_primeiro: bool) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let fase_de_copa: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
        core::mem::transmute(0x522e00usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(cql5_d(cql5_d(this) + 0x3c));
    // a Supercopa da Série C1
    let copa = cql5_d(cql5_d(0xadadfc) + cql5_d(0x9cf938) * 4);
    let inscritos = cql5_d(copa + 0xa7);
    let campeao = cql5_clube(this, 0);
    if cql5_d(inscritos) != 0 {
        cql5_pd(inscritos + 6, campeao);
    } else {
        cql5_pd(inscritos, campeao);
    }
    let lista = alocar(0x10);
    if lista == 0 {
        aviso_em(arquivo, linhas[0]);
        return;
    }
    for (k, &i) in CQL5_PLAYOFF.iter().enumerate() {
        cql5_pd(lista + k * 4, cql5_clube(this, i));
    }
    let clubes = *((this + 0xbf) as *const i8) as i16 as u16 as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let registro = cql5_d(this + 4);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let gabarito = calendario(this, 0, &mut rodadas, &mut extra, 0);
    let posicoes: [u8; 4] = [4, 1, 3, 2]; // cql5: as posições do play-off
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 0, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    cql5_pd(cql5_d(this + 0xc), fase);
    if cql5_d(cql5_d(this + 0xc)) == 0 {
        aviso_em(arquivo, linhas[1]);
        return;
    }
    sortear(cql5_d(cql5_d(this + 0xc)), 0);
    soltar(gabarito);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(cql5_d(cql5_d(this) + 0x3c));
    let gabarito = calendario(this, 1, &mut rodadas, &mut extra, 0);
    for (k, &i) in CQL5_PLAYOUT.iter().enumerate() {
        cql5_pd(lista + k * 4, cql5_clube(this, i));
    }
    let posicoes: [u8; 4] = [0x10, 0xd, 0xf, 0xe];
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 1, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    cql5_pd(cql5_d(this + 0xc) + 4, fase);
    if lista_primeiro {
        soltar(lista);
        soltar(gabarito);
    } else {
        soltar(gabarito);
        soltar(lista);
    }
    if cql5_d(cql5_d(this + 0xc) + 4) == 0 {
        aviso_em(arquivo, linhas[2]);
        return;
    }
    sortear(cql5_d(cql5_d(this + 0xc) + 4), 0);
    cql5_pd(this + 0x30, 1);
}

/// { 0x64ba70 } (thiscall, ret 0, ita_ser_c1a.cpp, quadro SEH só para o operator new): `ItaSerC1a::create_playoff_stages()`.
/// A regra da C1/A: o campeão sobe direto (e vai à Supercopa da C1); do 2º ao 5º, um play-off de acesso (2º × 5º,
/// 3º × 4º) dá a segunda vaga; do 14º ao 17º, um play-out (14º × 17º, 15º × 16º) decide quem cai junto com o 18º
/// [inferência pelas faixas da cqk_6 e pelos nomes do mapa]. Fonte 0x9ea474, linhas 0x27c, 0x290, 0x2b3.
unsafe extern "thiscall" fn fases_do_fim_da_c1a(this: usize) {
    cql5_criar_fases(this, 0x9ea474, [0x27c, 0x290, 0x2b3], true)
}

/// { 0x64fc30 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_play_off_stages()`, a mesma regra na C1/B.
/// Fonte 0x9ea558, linhas 0x281, 0x295, 0x2b8; no fim solta o gabarito antes da lista.
unsafe extern "thiscall" fn fases_do_fim_da_c1b(this: usize) {
    cql5_criar_fases(this, 0x9ea558, [0x281, 0x295, 0x2b8], false)
}

/// O comparador das duas: alocam e sorteiam: só a original. O plano, feito antes: o campeão e a vaga da Supercopa. Depois:
/// a vaga com o campeão, as duas fases em [+0xc] e +0x30 = 1.
unsafe fn cql5_comparar_fases(nome: &str, this: usize, original: impl FnOnce()) {
    let copa = cql5_d(cql5_d(0xadadfc) + cql5_d(0x9cf938) * 4);
    let inscritos = cql5_d(copa + 0xa7);
    let campeao = cql5_clube(this, 0);
    let vaga = if cql5_d(inscritos) != 0 { inscritos + 6 } else { inscritos };
    original();
    let fases = cql5_d(this + 0xc);
    let ok = cql5_d(vaga) == campeao && fases != 0 && cql5_d(fases) != 0 && cql5_d(fases + 4) != 0 && cql5_d(this + 0x30) == 1;
    PLACAR.registrar(ok, || format!("{}({:#x}): vaga da Supercopa {:#x} (campeão {:#x}), fases {:#x}, +0x30 = {}", nome, this, cql5_d(vaga), campeao, fases, cql5_d(this + 0x30) as i32));
}

unsafe extern "thiscall" fn comparar_fases_do_fim_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1857);
    cql5_comparar_fases("fases_do_fim_da_c1a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_fases_do_fim_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1859);
    cql5_comparar_fases("fases_do_fim_da_c1b", this, || o(this))
}

