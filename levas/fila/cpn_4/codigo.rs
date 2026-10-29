// ---------- %%LEVA%%a leva: o jogador que se aposenta sai dos elencos e a escolha do clube do jovem: o de maior nota entre todos e o sorteado da lista da nação (player_regen.cpp) ----------

unsafe fn cpn4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpn4_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpn4_por(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

/// Tira a pessoa `p` do time em `p + campo` (+0x39 o clube, +0x24 a seleção): a tática (0x896c60 de [0xde1d08], com 0
/// e depois com 1 no 3º argumento) e a primeira das 50 vagas (+0xd7) com ela, que esvazia. O time é relido depois de
/// cada chamada, como na original.
unsafe fn cpn4_tirar_do_time(p: usize, campo: usize) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    tirar(cpn4_d(0xde1d08), cpn4_d(p + campo), p, 0, 1);
    tirar(cpn4_d(0xde1d08), cpn4_d(p + campo), p, 1, 1);
    let time = cpn4_d(p + campo);
    if let Some(k) = (0..0x32usize).find(|&k| cpn4_d(time + 0xd7 + 4 * k) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * k) as *mut usize, 0);
    }
}

/// O aviso "already retired" no buffer [0xde1f64] (nome +4, sobrenome +8, id +0).
unsafe fn cpn4_ja_aposentado(p: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    formatar(0xde1f64, 0xa70150, cpn4_d(p + 4), cpn4_d(p + 8), cpn4_d(p));
}

/// { 0x7aace0 } (stdcall, ret 4, player_regen.cpp): `PLAYER_REGEN::retire_from_playing_staff(p)`: tira `p` do clube
/// (+0x39) e da seleção (+0x24) (`cpn4_tirar_do_time`) e troca o cargo de jogador pelo da comissão (tabelas 0x7aae60 e
/// 0x7aae90): pelo cargo no clube (+0x3d, byte com sinal; fora de 0..16 vale o padrão) 12 → 5, 13 → 6, 15 → 8 (fica no
/// clube), 0 e 16 (aviso "already retired") nada, 1 a 10 a rotina acaba ali, os outros 0x10 (aposentado). Depois pelo
/// cargo na seleção (+0x28) do mesmo jeito, mas 12/13/15 gravam no +0x3d (a original grava ali, não no +0x28) e o
/// padrão grava 0x10 no +0x28. Sem cargo que fica, o clube (+0x39) e a seleção (+0x24) zeram. Sem SEH, sem sorteio.
unsafe extern "stdcall" fn aposentar_do_elenco(p: usize) {
    if cpn4_d(p + 0x39) != 0 {
        cpn4_tirar_do_time(p, 0x39);
    }
    if cpn4_d(p + 0x24) != 0 {
        cpn4_tirar_do_time(p, 0x24);
    }
    let fica_no_clube = match cpn4_b(p + 0x3d) as i32 {
        0 => false,
        1..=10 => return, // comissão: a original sai pelo fim da rotina
        12 => {
            cpn4_por(p + 0x3d, 5);
            true
        }
        13 => {
            cpn4_por(p + 0x3d, 6);
            true
        }
        15 => {
            cpn4_por(p + 0x3d, 8);
            true
        }
        16 => {
            cpn4_ja_aposentado(p);
            false
        }
        _ => {
            crate::comparacao::ramo("cpn_4 jogador aposentado");
            cpn4_por(p + 0x3d, 0x10); // aposentado
            false
        }
    };
    let fica_na_selecao = match cpn4_b(p + 0x28) as i32 {
        0 => false,
        1..=10 => return,
        12 => {
            cpn4_por(p + 0x3d, 5); // no +0x3d, como a original
            true
        }
        13 => {
            cpn4_por(p + 0x3d, 6);
            true
        }
        15 => {
            cpn4_por(p + 0x3d, 8);
            true
        }
        16 => {
            cpn4_ja_aposentado(p);
            false
        }
        _ => {
            cpn4_por(p + 0x28, 0x10);
            false
        }
    };
    if !fica_no_clube {
        core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
    }
    if !fica_na_selecao {
        core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    }
}

/// O jovem com a segunda nacionalidade (+0x1e) igual a [0x9cf424] só vai para os clubes de id [0x9d0120], [0x9d0134],
/// [0x9d0128], [0x9d01a0] e [0x9d0124]: `true` para os outros clubes.
unsafe fn cpn4_vetado(jogador: usize, clube: usize) -> bool {
    let e = cpn4_d(jogador + 0x1e);
    if e == 0 || cpn4_d(e) != cpn4_d(0x9cf424) {
        return false;
    }
    let id = cpn4_d(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| cpn4_d(g) == id)
}

/// O clube pode chegar ao jogador (0x8ce990 de [0xde1f54], sem ignorar as ofertas abertas)? O motivo fica numa local
/// (a original o grava na vaga do argumento, que ninguém lê depois).
unsafe fn cpn4_pode_contratar(jogador: usize, clube: usize) -> bool {
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let mut motivo = 0i16;
    pode(cpn4_d(0xde1f54), jogador, clube, &mut motivo, 0) != 0
}

/// { 0x7ad0e0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_best_club_for_player(jogador)`: entre os
/// clubes ([0xae23b4], 0x245 bytes; os [0xae235c] − 2 × [0xae2350] primeiros, relidos a cada volta) em que o jogador
/// não é estrangeiro (0x53d980 com a nação +0x53 e 1), que não são o time B de um par (0x540a50 com 1 achando o
/// parceiro com 0), não vetados (`cpn4_vetado`), que podem chegar a ele (0x8ce990) e com elenco (0x53c590) abaixo de 50,
/// o de maior nota (0x7ad220, sorteia; comparada em 16 bits, a partir de −25000). Nenhum: 0. Sem SEH; a local do
/// parceiro só é lida quando a 0x540a50 o acha (e a grava).
unsafe extern "thiscall" fn melhor_clube_para_o_jovem(_this: usize, jogador: usize) -> usize {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let nota_de: unsafe extern "stdcall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x7ad220usize);
    let total = || (cpn4_d(0xae235c) as i32).wrapping_sub((cpn4_d(0xae2350) as i32).wrapping_mul(2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    let mut eh_a = 0u32;
    let mut clube = cpn4_d(0xae23b4);
    let mut i = 0i32;
    while i < total() {
        let candidato = estrangeiro(jogador, cpn4_d(clube + 0x53), 1) == 0
            && (parceiro(clube, &mut eh_a, 1) == 0 || eh_a != 0)
            && !cpn4_vetado(jogador, clube);
        if candidato {
            let elenco = elenco_de(clube, 0, 0);
            if cpn4_pode_contratar(jogador, clube) && (elenco as i8) < 0x32 {
                // ponytail: a original empurra o byte do elenco com lixo nos bits altos; a 0x7ad220 só lê o byte
                let s = nota_de(jogador, clube, elenco as u32) as i16;
                if s > melhor_nota {
                    crate::comparacao::ramo("cpn_4 clube melhor para o jovem");
                    melhor = clube;
                    melhor_nota = s;
                }
            }
        }
        i += 1;
        clube += 0x245;
    }
    melhor
}

/// A nação da lista do jovem: a nação (+0x1a) dele; [0x9cf510] e [0x9cf2dc] vão para [0x9cf2e4], e [0x9cf3e4] também
/// com sorteio(5) < 2.
unsafe fn cpn4_nacao_da_lista(jogador: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cpn4_d(cpn4_d(jogador + 0x1a));
    if n == cpn4_d(0x9cf510) || n == cpn4_d(0x9cf2dc) || (n == cpn4_d(0x9cf3e4) && sorteio(5) < 2) {
        return cpn4_d(0x9cf2e4);
    }
    n
}

/// A lista de clubes da nação `n`: [[this] + 4n].
unsafe fn cpn4_lista(this: usize, n: usize) -> usize {
    cpn4_d(cpn4_d(this).wrapping_add(n.wrapping_mul(4)))
}

/// { 0x7ad590 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_club_from_nation_list(jogador)`: percorre a
/// lista de clubes da nação do jovem (`cpn4_nacao_da_lista`; a palavra +0 é a conta, o cursor +0xa recomeça de +2, o
/// iterador 0x68d1b0 devolve o id) e, entre os clubes não vetados (`cpn4_vetado`), que podem chegar a ele (0x8ce990) e
/// com elenco (0x53c590) abaixo de 50, fica com o de maior sorteio da nota (5000 + reputação +0x80 menos as distâncias
/// dela à habilidade da ficha +0x61: +0xb, +7 × 50 e +5 × 50, em 16 bits), comparado em 16 bits a partir de −25000.
/// Sem lista ou lista vazia: 0. A lista que acaba antes da conta: a caixa "Error" (linha 0x19e9) e 0. Sem SEH.
unsafe extern "thiscall" fn clube_da_nacao_para_o_jovem(this: usize, jogador: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cpn4_nacao_da_lista(jogador);
    let l = cpn4_lista(this, n);
    if l == 0 || cpn4_w(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpn4_d(l + 2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    if cpn4_w(cpn4_lista(this, n)) <= 0 {
        return 0;
    }
    let mut i = 0i32;
    loop {
        let item = proximo(cpn4_lista(this, n));
        if item == 0 {
            aviso_em(0xa6bf9c, 0x19e9);
            return 0;
        }
        let clube = cpn4_d(0xae23b4).wrapping_add(cpn4_d(item).wrapping_mul(0x245));
        if !cpn4_vetado(jogador, clube) && cpn4_pode_contratar(jogador, clube) && (elenco_de(clube, 0, 0) as i8) < 0x32 {
            let ficha = cpn4_d(jogador + 0x61);
            let rep = cpn4_w(clube + 0x80) as i32;
            let c = rep
                .wrapping_sub((cpn4_w(ficha + 0xb) as i32).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((cpn4_w(ficha + 7) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((cpn4_w(ficha + 5) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_add(0x1388);
            crate::comparacao::ramo("cpn_4 sorteio do clube da nacao");
            let nota = sorteio(c as i16 as i32) as i16;
            if nota > melhor_nota {
                melhor = clube;
                melhor_nota = nota;
            }
        }
        i += 1;
        if i as i16 >= cpn4_w(cpn4_lista(this, n)) {
            break;
        }
    }
    melhor
}

/// Comparador: lado a lado pela pessoa (+0x24..+0x3d), pelas 50 vagas do clube e da seleção e pelo aviso em
/// [0xde1f64]. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha, então a tática fica fora.
unsafe extern "stdcall" fn comparar_aposentar_do_elenco(p: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(%%I0%%);
    let mut reg = vec![(p + 0x24, 0x1a), (0xde1f64, 0x80)];
    for campo in [0x39usize, 0x24] {
        let time = cpn4_d(p + campo);
        if time != 0 {
            reg.push((time + 0xd7, 200));
        }
    }
    let (antes_clube, antes_cargo) = (cpn4_d(p + 0x39), cpn4_b(p + 0x3d));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p), || aposentar_do_elenco(p));
    PLACAR.registrar(igual, || format!("aposentar_do_elenco(id {}, clube {:x}, cargo {}) {:x?} x {:x?}", cpn4_d(p), antes_clube, antes_cargo, mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x7ad220 sorteia), pelo clube devolvido.
unsafe extern "thiscall" fn comparar_melhor_clube_para_o_jovem(this: usize, jogador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| melhor_clube_para_o_jovem(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("melhor_clube_para_o_jovem(id {}) original={:x} rust={:x} {:x?} x {:x?}", cpn4_d(jogador), r, n, mo, mr));
    r
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio, pelo clube devolvido e pelo cursor (+0xa) das listas
/// que podem ser percorridas (a da nação do jovem e a de [0x9cf2e4]).
unsafe extern "thiscall" fn comparar_clube_da_nacao_para_o_jovem(this: usize, jogador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for n in [cpn4_d(cpn4_d(jogador + 0x1a)), cpn4_d(0x9cf2e4)] {
        let l = cpn4_lista(this, n);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| clube_da_nacao_para_o_jovem(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("clube_da_nacao_para_o_jovem(id {}) original={:x} rust={:x} {:x?} x {:x?}", cpn4_d(jogador), r, n, mo, mr));
    r
}
