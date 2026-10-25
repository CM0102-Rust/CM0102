// ---------- 233a leva: as regras do contrato: o clube em temporada e o clube e a nação entre as seis grandes (Database.cpp) ----------

/// { 0x547770 } (cdecl, Database.cpp): `club_is_in_season(clube)`: clube nulo, 0; senão o que
/// `key_nation_club_in_season` 0x669bd0(clube, nulo, nulo) devolve (1 = hoje dentro da temporada da nação do clube),
/// o eax inteiro (o contrato padrão 0x864550 testa o eax). Na criação: o contrato padrão das pessoas
/// (CONTRACT_MANAGER::initialise_all_staff_contracts → 0x85e960 → 0x864550) e o salário padrão das listas iniciais
/// (0x83a1b0 → 0x862720).
unsafe extern "C" fn clube_em_temporada(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);
    temporada(clube, 0, 0) // a temporada da nação do clube
}

unsafe extern "C" fn comparar_clube_em_temporada(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(681)(clube);
    let n = clube_em_temporada(clube);
    PLACAR.registrar(r == n, || format!("clube_em_temporada({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549e50 } (cdecl, Database.cpp): `club_is_in_major_nation(clube)`: 1 se a nação do clube (+0x53) tem o id
/// (+0) de uma das chaves Inglaterra 0x9cf2e4, França 0x9cf300, Alemanha 0x9cf314, Itália 0x9cf360, Espanha 0x9cf488 ou
/// Holanda 0x9cf338; clube ou nação nulos: 0. Só `al` (o resto do eax é o id da nação). Na criação, pelo contrato
/// padrão (0x864550) e pelo salário padrão (0x862720).
unsafe extern "C" fn clube_em_nacao_grande(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [0x9cf2e4usize, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338]; // Holanda (clube)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}

unsafe extern "C" fn comparar_clube_em_nacao_grande(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(682)(clube);
    let n = clube_em_nacao_grande(clube);
    PLACAR.registrar(r == n, || format!("clube_em_nacao_grande({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549ea0 } (cdecl, Database.cpp): `nation_is_major_nation(nacao)`: o mesmo teste das seis chaves de 0x549e50
/// sobre o id (+0) da própria nação; nula: 0. Só `al`. Na carreira, pela regeneração dos jovens
/// (PLAYER_REGEN::regenerate_as_youth_player 0x7abac0).
unsafe extern "C" fn nacao_grande(nacao: usize) -> u8 {
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [0x9cf2e4usize, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338]; // Holanda (nação)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}

unsafe extern "C" fn comparar_nacao_grande(nacao: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(683)(nacao);
    let n = nacao_grande(nacao);
    PLACAR.registrar(r == n, || format!("nacao_grande({:x}) original={} rust={}", nacao, r, n));
    r
}

// ---------- 233a leva: os vínculos de opinião do contrato: limpar, juntar e tirar (Database.cpp) ----------

/// O registro de contrato (79 bytes, tabela [0xae2d0c]) da `pessoa` como as 0x549c90/0x549cf0/0x549d60 o acham:
/// pessoa nula ou id (+0) ≥ [0xae2364] (comparação com sinal: id negativo passa, como na original): nenhum.
/// Os dois vínculos ficam em +0x27/+0x37 e +0x3b/+0x4b (id da outra pessoa, valor), −1 = vazio.
unsafe fn registro_de_opinioes(pessoa: usize) -> Option<usize> {
    if pessoa == 0 {
        return None;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return None;
    }
    Some((*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79)))
}

unsafe fn opiniao_ler(c: usize, d: usize) -> i32 {
    core::ptr::read_unaligned(c.wrapping_add(d) as *const i32)
}

unsafe fn opiniao_gravar(c: usize, d: usize, v: i32) {
    core::ptr::write_unaligned(c.wrapping_add(d) as *mut i32, v)
}

/// { 0x549c90 } (cdecl, Database.cpp): `staff_clear_opinion_links(pessoa)`: −1 nos dois vínculos do contrato da
/// pessoa (+0x27, +0x37, +0x3b, +0x4b). Na criação, a cada contrato gerado (0x85e960, por
/// CONTRACT_MANAGER::initialise_all_staff_contracts); na carreira, nas propostas e nas queixas.
unsafe extern "C" fn limpar_vinculos_de_opiniao(pessoa: usize) {
    if let Some(c) = registro_de_opinioes(pessoa) {
        for k in [0usize, 0x14] {
            opiniao_gravar(c, k + 0x27, -1);
            opiniao_gravar(c, k + 0x37, -1); // o valor do vínculo vazio
        }
    }
}

unsafe extern "C" fn comparar_limpar_vinculos_de_opiniao(pessoa: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(684);
    let Some(c) = registro_de_opinioes(pessoa) else { return o(pessoa) }; // não escreve nada
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa), || limpar_vinculos_de_opiniao(pessoa));
    PLACAR.registrar(igual, || format!("limpar_vinculos_de_opiniao({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x549cf0 } (cdecl, Database.cpp): `staff_add_opinion_link(pessoa, outra, valor)`: com as duas pessoas válidas
/// (não nulas, id < [0xae2364]), grava o id da `outra` e o `valor` no primeiro vínculo se ele está vazio (+0x27 = −1),
/// senão no segundo (+0x3b/+0x4b, por cima do que houver). Na carreira, pelos eventos do contrato (0x867700).
unsafe extern "C" fn juntar_vinculo_de_opiniao(pessoa: usize, outra: usize, valor: i32) {
    let Some(c) = registro_de_opinioes(pessoa) else { return };
    if outra == 0 {
        return;
    }
    let id = *(outra as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return;
    }
    if opiniao_ler(c, 0x27) == -1 {
        opiniao_gravar(c, 0x27, id);
        opiniao_gravar(c, 0x37, valor);
    } else {
        opiniao_gravar(c, 0x3b, id); // o segundo vínculo
        opiniao_gravar(c, 0x4b, valor);
    }
}

unsafe extern "C" fn comparar_juntar_vinculo_de_opiniao(pessoa: usize, outra: usize, valor: i32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, i32)>(685);
    let Some(c) = registro_de_opinioes(pessoa) else { return o(pessoa, outra, valor) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa, outra, valor),
        || juntar_vinculo_de_opiniao(pessoa, outra, valor));
    PLACAR.registrar(igual, || format!("juntar_vinculo_de_opiniao({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, outra, valor, mo, mr));
}

/// { 0x549d60 } (cdecl, Database.cpp): `staff_remove_opinion_link(pessoa, valor)`: esvazia (−1 no id e no valor) o
/// primeiro vínculo com esse `valor` (+0x37) ou, se não, o segundo (+0x4b); nenhum com o valor: nada. Na carreira,
/// quando a queixa passa (0x867240, 0x8674e0).
unsafe extern "C" fn tirar_vinculo_de_opiniao(pessoa: usize, valor: i32) {
    let Some(c) = registro_de_opinioes(pessoa) else { return };
    if opiniao_ler(c, 0x37) == valor {
        opiniao_gravar(c, 0x27, -1);
        opiniao_gravar(c, 0x37, -1);
    } else if opiniao_ler(c, 0x4b) == valor {
        opiniao_gravar(c, 0x3b, -1); // esvazia o segundo vínculo
        opiniao_gravar(c, 0x4b, -1);
    }
}

unsafe extern "C" fn comparar_tirar_vinculo_de_opiniao(pessoa: usize, valor: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32)>(686);
    let Some(c) = registro_de_opinioes(pessoa) else { return o(pessoa, valor) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa, valor), || tirar_vinculo_de_opiniao(pessoa, valor));
    PLACAR.registrar(igual, || format!("tirar_vinculo_de_opiniao({:x}, {}) {:x?} x {:x?}", pessoa, valor, mo, mr));
}

// ---------- 233a leva: a comissão do clube com a do time parceiro: o auxiliar, os treinadores e a contagem dos treinadores (Database.cpp) ----------

/// { 0x53cb80 } (cdecl, Database.cpp): `database_get_club_assistant(clube)`: o auxiliar (+0xd3) do clube; sem ele, o
/// do time parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro); nenhum: 0.
/// Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos treinadores (0x8b2880, nas
/// contratações e transferências), pela regeneração (0x7a65d0) e pelas telas.
unsafe extern "C" fn auxiliar_do_clube(clube: usize) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> usize>(687)(clube);
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let a = rd(clube + 0xd3);
    if a != 0 {
        return a;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        return 0;
    }
    rd(p + 0xd3) // o auxiliar do parceiro
}

unsafe extern "C" fn comparar_auxiliar_do_clube(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(687);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube), || auxiliar_do_clube(clube));
    PLACAR.registrar(igual && r == n, || format!("auxiliar_do_clube({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// { 0x53cc30 } (cdecl, Database.cpp): `database_get_club_coaches(clube, vetor)`: os 5 treinadores (+0x19f) do clube no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): as 5 casas como estão. Com parceiro: os não nulos do clube e
/// depois os do parceiro até 5; o resto zerado só quando os dois juntos não enchem as 5. Contador de 8 bits na
/// original (no máximo 5). Devolve 1. Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos
/// treinadores (0x8b2880).
unsafe extern "C" fn treinadores_do_clube(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(688)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        for j in 0..5 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + 0x19f + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..5 {
        let t = rd(clube + 0x19f + 4 * j);
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for j in 0..5 {
        if n >= 5 {
            return 1;
        }
        let t = rd(p + 0x19f + 4 * j); // os treinadores do parceiro
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for k in n..5 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}

unsafe extern "C" fn comparar_treinadores_do_clube(clube: usize, vetor: *mut usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(688);
    if clube == 0 {
        return o(clube, vetor); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(vetor as usize, 20), (clube + 0x244, 1)],
        || o(clube, vetor), || treinadores_do_clube(clube, vetor));
    PLACAR.registrar(igual && r == n, || format!("treinadores_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// { 0x53c8d0 } (cdecl, Database.cpp): `database_count_club_coaches(clube)`: quantos dos 5 treinadores (+0x19f) do
/// clube são pessoas válidas (não nulas, id (+0) < [0xae2364], com sinal) mais os do time parceiro A/B (0x540a50 com
/// nulo e 1, sempre chamada). Só `al` (o resto do eax é o do parceiro). Clube nulo: a original (caixa "Error" e 0).
/// Na carreira, pela fila da regeneração (PLAYER_REGEN::process_regen_queue 0x7a65d0).
unsafe extern "C" fn contar_treinadores_do_clube(clube: usize) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u8>(689)(clube);
    }
    let contar = |c: usize| {
        let total = *(0xae2364 as *const i32);
        (0..5).filter(|&j| {
            let t = core::ptr::read_unaligned((c + 0x19f + 4 * j) as *const usize);
            t != 0 && *(t as *const i32) < total // treinador válido
        }).count() as u8
    };
    let mut n = contar(clube);
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, 0, 1);
    if p != 0 {
        n = n.wrapping_add(contar(p)); // e os do parceiro
    }
    n
}

unsafe extern "C" fn comparar_contar_treinadores_do_clube(clube: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(689);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube), || contar_treinadores_do_clube(clube));
    PLACAR.registrar(igual && r == n, || format!("contar_treinadores_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

// ---------- 233a leva: os nomes em inglês da sigla da nação, da competição e da competição de pessoal (Database.cpp) ----------

/// Comparador dos nomes em inglês (0x5454c0, 0x5455a0, 0x5456a0): com o idioma dos nomes ([0x9b5984]) ≠ 0 e ≠ 1 e
/// a tabela do eng.lng ([0xae2408]) ainda nula, o primeiro uso aloca e carrega sob o quadro SEH: só a original.
/// Senão as duas, comparando o ponteiro devolvido (sem efeito na memória).
unsafe fn comparar_nome_em_ingles(nome: &str, x: usize, o: unsafe extern "C" fn(usize) -> usize, r: unsafe extern "C" fn(usize) -> usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(x);
    }
    let a = o(x);
    let b = r(x);
    PLACAR.registrar(a == b, || format!("{}({:x}) idioma {} original={:x} rust={:x}", nome, x, idioma, a, b));
    a
}

/// { 0x5454c0 } (cdecl, Database.cpp): `nation_get_english_three_letter_name(nação)`: idioma ([0x9b5984]) 0 ou
/// 1: a própria sigla (+0x53). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de
/// +0x2c, pelo id (sem conferir a quantidade), a sigla em +0x4f da linha. Quadro SEH só no primeiro uso.
unsafe extern "C" fn sigla_inglesa_da_nacao(nacao: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(0x53); // sigla da nação
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(690)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109)).wrapping_add(0x4f) // sigla na linha
}

unsafe extern "C" fn comparar_sigla_inglesa_da_nacao(nacao: usize) -> usize {
    comparar_nome_em_ingles("sigla_inglesa_da_nacao", nacao, orig::<unsafe extern "C" fn(usize) -> usize>(690), sigla_inglesa_da_nacao)
}

/// { 0x5455a0 } (cdecl, Database.cpp): `comp_get_english_name(competição)`: idioma 0 ou 1: o próprio nome (+4).
/// Senão a tabela do eng.lng, linhas de 83 bytes: id ≥ [t+0x14] (quantas de clube): a de nação `id − [t+0x14]`
/// em [t+0x28]; senão a de clube `id` em [t+0x38] (comparação com sinal). Quadro SEH só no primeiro uso.
unsafe extern "C" fn nome_ingles_da_competicao(comp: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(691)(comp);
    }
    let id = *(comp as *const i32);
    let de_clube = *((t + 0x14) as *const i32);
    if id >= de_clube {
        (*((t + 0x28) as *const usize)).wrapping_add((id.wrapping_sub(de_clube) as usize).wrapping_mul(83)) // competição de nação
    } else {
        (*((t + 0x38) as *const usize)).wrapping_add((id as usize).wrapping_mul(83)) // competição de clube
    }
}

unsafe extern "C" fn comparar_nome_ingles_da_competicao(comp: usize) -> usize {
    comparar_nome_em_ingles("nome_ingles_da_competicao", comp, orig::<unsafe extern "C" fn(usize) -> usize>(691), nome_ingles_da_competicao)
}

/// { 0x5456a0 } (cdecl, Database.cpp): `staff_comp_get_english_name(competição de pessoal)`: idioma 0 ou 1: o
/// próprio nome (+4). Senão a tabela do eng.lng, linhas de 79 bytes em [t+0x3c], pelo id. SEH só no primeiro uso.
unsafe extern "C" fn nome_ingles_da_competicao_de_pessoal(comp: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição de pessoal
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(692)(comp);
    }
    let id = *(comp as *const i32);
    (*((t + 0x3c) as *const usize)).wrapping_add((id as usize).wrapping_mul(79)) // competição de pessoal na tabela
}

unsafe extern "C" fn comparar_nome_ingles_da_competicao_de_pessoal(comp: usize) -> usize {
    comparar_nome_em_ingles("nome_ingles_da_competicao_de_pessoal", comp, orig::<unsafe extern "C" fn(usize) -> usize>(692), nome_ingles_da_competicao_de_pessoal)
}

/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação): nem a carreira nem o save passam pelo caminho do eng.lng
/// (o jogo dos testes está em inglês). 512 casos com o idioma de 0 a 13 e uma tabela do eng.lng falsa (nunca
/// nula: o primeiro uso carregaria o arquivo de verdade), quantas de clube de 0 a 9, ids de −2 a 13; as globais
/// voltam no fim.
pub unsafe fn autoteste_dbm_1() {
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x4442_4d31_6964_696f);
    let globais = [0x9b5984usize, 0xae2408];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 .. +0x1c quantos, +0x20 .. +0x3c linhas
    let mut registro = [0u8; 0x60];
    for _ in 0..512 {
        let z = s.proximo();
        *(0x9b5984 as *mut u32) = (z % 14) as u32;
        tabela[5] = ((z >> 4) % 10) as usize;
        for k in 8..16 {
            tabela[k] = 0x1000_0000 * (k - 7) + ((z >> (8 + k)) as usize & 0xffff0);
        }
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        let r = registro.as_ptr() as usize;
        comparar_sigla_inglesa_da_nacao(r);
        comparar_nome_ingles_da_competicao(r);
        comparar_nome_ingles_da_competicao_de_pessoal(r);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco idioma (nomes em inglês 1): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

// ---------- 233a leva: o nome em inglês do estádio, o .lng do idioma existe? e soltar as tabelas em inglês (Database.cpp) ----------

/// { 0x545990 } (cdecl, Database.cpp): `stadium_get_english_name(estádio)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]), linhas de 52 bytes em [t+0x34], pelo id. Quadro SEH só no
/// primeiro uso (new 0x40 e a carga 0x544fc0), que fica com a original.
unsafe extern "C" fn nome_ingles_do_estadio(estadio: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return estadio.wrapping_add(4); // nome do estádio
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(693)(estadio);
    }
    let id = *(estadio as *const i32);
    (*((t + 0x34) as *const usize)).wrapping_add((id as usize).wrapping_mul(52)) // estádio na tabela
}

unsafe extern "C" fn comparar_nome_ingles_do_estadio(estadio: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(693);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(estadio); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(estadio);
    let n = nome_ingles_do_estadio(estadio);
    PLACAR.registrar(r == n, || format!("nome_ingles_do_estadio({:x}) idioma {} original={:x} rust={:x}", estadio, idioma, r, n));
    r
}

/// Os nomes dos `.lng` no .rdata do exe por idioma: 0, 1, 11 e 12 "eng.lng"; 2 fra, 3 ger, 4 ita, 5 esp, 6 dan,
/// 7 hol, 8 nor, 9 por, 10 swe; os outros (13, negativos...) nenhum.
const ARQUIVOS_DOS_IDIOMAS: [usize; 13] = [0x9b632c, 0x9b632c, 0x9b633c, 0x9b6334, 0x9b6324, 0x9b630c, 0x9b62fc,
    0x9b6344, 0x9b6314, 0x9b631c, 0x9b6304, 0x9b632c, 0x9b632c];

/// { 0x535930 } (cdecl, Database.cpp): `database_language_file_exists(idioma)`: o caminho `CM3_TEMP\<.lng>`
/// (0x90bde0, no buffer 0xde279c) e se existe (0x90c120: `_access(.., 0) == 0`, 1 ou 0); idioma sem `.lng`, 0
/// sem montar o caminho. Chamada pelo `load_database` (0x5286e4, o idioma das preferências) e na criação.
unsafe extern "C" fn arquivo_do_idioma_existe(idioma: i32) -> i32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x90c120usize);
    match ARQUIVOS_DOS_IDIOMAS.get(idioma as u32 as usize) {
        Some(&nome) => existe(caminho(0x987da0, nome)), // "CM3_TEMP"
        None => 0,
    }
}

/// Comparador: as duas a partir do mesmo buffer do caminho (0xde279c, 0x104 bytes); compara a resposta e o caminho
/// montado (acusa o nome trocado mesmo sem o arquivo na pasta).
unsafe extern "C" fn comparar_arquivo_do_idioma_existe(idioma: i32) -> i32 {
    let o = orig::<unsafe extern "C" fn(i32) -> i32>(694);
    let (r, x, igual, _, _) = crate::comparacao::lado_a_lado(&[(0xde279c, 0x104)], || o(idioma), || arquivo_do_idioma_existe(idioma));
    PLACAR.registrar(igual && r == x, || format!("arquivo_do_idioma_existe({}) original={} rust={} caminho igual {}", idioma, r, x, igual));
    r
}

/// As 8 linhas da tabela do eng.lng (+0x20 .. +0x3c), na ordem, cada uma não nula entregue a `soltar`.
unsafe fn soltar_tabelas_em_ingles(t: usize, mut soltar: impl FnMut(usize)) {
    for k in 0..8 {
        let p = *((t + 0x20 + 4 * k) as *const usize); // linha da tabela em inglês
        if p != 0 {
            soltar(p);
        }
    }
}

/// { 0x544f30 } (thiscall, Database.cpp): `eng_lng_free_tables(tabela)`: `free` 0x9452ca de cada uma das 8 linhas
/// não nulas, de +0x20 a +0x3c (os ponteiros ficam; quem chama, `load_database` 0x528503 e a troca de idioma
/// 0x533aa6, apaga a tabela com `delete` 0x944c94 e zera [0xae2408]).
unsafe extern "thiscall" fn liberar_tabelas_em_ingles(t: usize) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    soltar_tabelas_em_ingles(t, |p| free(p));
}

/// Comparador: soltar duas vezes soltaria os mesmos blocos duas vezes.
// ponytail: só a original (como as da 232a); a cópia é provada pelo autoteste_dbm_2 (a ordem dos blocos soltos).
unsafe extern "thiscall" fn comparar_liberar_tabelas_em_ingles(t: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(695)(t)
}

/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação). (1) O estádio: 256 casos com o idioma de 0 a 13 e uma
/// tabela do eng.lng falsa (nunca nula), ids de −2 a 13. (2) O `.lng` do idioma: idiomas de −3 a 16 contra a
/// original (resposta e caminho montado). (3) Soltar as tabelas: 64 tabelas falsas com linhas nulas ou não
/// sorteadas; a cópia entrega a uma lista (sem soltar nada), que tem de ser as não nulas de +0x20 a +0x3c, na
/// ordem da original (a original solta de verdade e não dá para rodar as duas). As globais voltam no fim.
pub unsafe fn autoteste_dbm_2() {
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x4442_4d32_6573_7461);
    let globais = [0x9b5984usize, 0xae2408];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
        *(0x9b5984 as *mut u32) = (z % 14) as u32;
        tabela[13] = 0x3000_0000 + ((z >> 8) as usize & 0xffff0);
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        comparar_nome_ingles_do_estadio(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    for idioma in -3..=16 {
        comparar_arquivo_do_idioma_existe(idioma);
    }
    for _ in 0..64 {
        let z = s.proximo();
        for k in 0..8 {
            tabela[8 + k] = if (z >> k) & 1 == 0 { 0 } else { 0x1000_0000 * (k + 1) + ((z >> (8 + k)) as usize & 0xffff0) };
        }
        let esperado: Vec<usize> = tabela[8..].iter().copied().filter(|&p| p != 0).collect();
        let mut soltos = Vec::new();
        soltar_tabelas_em_ingles(tabela.as_ptr() as usize, |p| soltos.push(p));
        PLACAR.registrar(soltos == esperado, || format!("liberar_tabelas_em_ingles: soltos {:x?} esperados {:x?}", soltos, esperado));
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco idioma (estádio, .lng, soltar): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

// ---------- 233a leva: a criação do jogo: gerar a comissão que falta, cortar a tabela de clubes e podar o histórico de competições (Database.cpp) ----------

/// A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
unsafe fn caixa_do_banco_dbn1(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b5ac8, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// A caixa do Database.cpp seguida do fim do jogo (0x5cc7d0, 0x5a8f60 e exit(−1)), como a original sem memória.
unsafe fn banco_fatal_dbn1(linha: u32) -> ! {
    let antes: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let depois: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    caixa_do_banco_dbn1(linha);
    antes();
    depois();
    sair(-1)
}

unsafe fn gi_dbn1(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn rd_dbn1(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Clube da nação da chave 0x9cf2e4 com reputação (+0x80, 16 bits com sinal) até 500: 0x527240 e 0x5267c0 o tratam à parte.
unsafe fn fraco_da_chave_2e4_dbn1(c: usize) -> bool {
    let nac = rd_dbn1(c + 0x53);
    nac != 0 && gi_dbn1(nac) == gi_dbn1(0x9cf2e4) && core::ptr::read_unaligned((c + 0x80) as *const i16) <= 500
}

/// A conferência inicial da 0x527240: alguma pessoa de cargo 8 (+0x3d) com clube (+0x39) que não a tem nas 5 casas +0x19f.
unsafe fn cargo_8_fora_do_clube_dbn1() -> bool {
    let mut i = 0i32;
    while i < gi_dbn1(0xae2364) {
        let s = rd_dbn1(0xae23bc) + i as usize * 110;
        let c = rd_dbn1(s + 0x39);
        if *((s + 0x3d) as *const u8) == 8 && c != 0 && !(0..5).any(|j| rd_dbn1(c + 0x19f + 4 * j) == s) {
            return true;
        }
        i += 1;
    }
    false
}

/// Uma pessoa do pessoal extra da 0x527240: a linha [0xae2364] − gerados − 17 (das reservas, de trás para a frente),
/// primeiro e segundo nome da nação (0x539c60, 0x53a1b0) e o nome comum: a linha guardada em [0x9b59b4] ou, sem ela
/// (−1), a primeira linha de nome comum vazia (guardada) ou, sem nenhuma, uma sorteada entre todas (sem guardar).
unsafe fn pessoa_extra_dbn1(nacao: usize, gerados: i32) -> usize {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let s = rd_dbn1(0xae23bc).wrapping_add((gi_dbn1(0xae2364) - gerados - 0x11) as usize * 110);
    core::ptr::write_unaligned((s + 4) as *mut usize, primeiro(nacao));
    core::ptr::write_unaligned((s + 8) as *mut usize, segundo(nacao));
    let comum = |k: i32| rd_dbn1(0xae23f8).wrapping_add((k as usize).wrapping_mul(0x3c));
    let guardada = gi_dbn1(0x9b59b4);
    let nome = if guardada != -1 {
        comum(guardada)
    } else {
        let n = gi_dbn1(0xae23a0);
        match (0..n).find(|&j| *(comum(j) as *const u8) == 0) {
            Some(j) => {
                *(0x9b59b4 as *mut i32) = j;
                comum(j)
            }
            None => comum(sorteio(n)),
        }
    };
    core::ptr::write_unaligned((s + 0xc) as *mut usize, nome);
    s
}

/// A data de nascimento da pessoa extra (+0x10, 0x54a130): ano 1940 + sorteio(26), mês sorteio(11), dia sorteio(27) + 1,
/// sorteados nessa ordem.
unsafe fn nascimento_extra_dbn1(s: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    let ano = sorteio(0x1a) + 0x794; // 1940 + até 25
    let mes = sorteio(0xb);
    let dia = sorteio(0x1b) + 1;
    acertar_data(s + 0x10, dia as u32, mes as u32, ano as u32, -1);
}

/// { 0x527240 } (cdecl, Database.cpp): `database_generate_extra_non_playing_staff()`, uma vez na criação (chamada pela
/// 0x524d70 no começo dos cortes). Confere as pessoas de cargo 8 (caixa 0x62f na primeira fora das casas do clube, e
/// segue). Depois, em cada clube que não é seleção (id < [0xae235c] − 2 × [0xae2350]), não é fraco da chave 0x9cf2e4,
/// tem nação escolhida (+0x11c) e não é time A de um par (0x540a50 com 1): completa técnicos (+0x19f, 5, cargo 8),
/// olheiros (+0x1b3, 7, cargo 9) e fisioterapeutas (+0x1cf, 3, cargo 10) até (reputação + 500) × casas / 10000, entre 1
/// e o número de casas, com pessoas das reservas (pessoa_extra_dbn1, nação +0x1a, +0x5f = 1, nascimento sorteado) postas
/// na primeira casa livre. Por fim, cada clube sem treinador (+0xcf) que seja seleção ou (com nação, divisão +0x57, nação
/// escolhida e reputação ≠ 0) ou reputação > 7500, fora os fracos da 0x9cf2e4, ganha um treinador (cargo 5, +0x28 = 5)
/// sem clube e sem seleção (não vai para a casa). Mais de 23.000: caixa (0x65f, 0x6aa, 0x6f5, 0x734) e 0. Grava "generated
/// %ld extra non playing staff" em 0xde1f64 e devolve 1. A local do par só é lida quando 0x540a50 devolve parceiro.
unsafe extern "C" fn gerar_comissao_extra() -> u32 {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let gi = |a: usize| gi_dbn1(a);
    let rd = |a: usize| rd_dbn1(a);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16) as i32;
    let byte = |a: usize| *(a as *const u8);
    if cargo_8_fora_do_clube_dbn1() {
        caixa_do_banco_dbn1(0x62f);
    }
    let selecao = |c: usize| gi(c) >= gi(0xae235c) - 2 * gi(0xae2350);
    let mut gerados = 0i32;
    for &(d, k, cargo, linha) in &[(0x19fusize, 5usize, 8u8, 0x65fu32), (0x1b3, 7, 9, 0x6aa), (0x1cf, 3, 10, 0x6f5)] {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            let c = rd(0xae23b4) + i as usize * 581;
            i += 1;
            if fraco_da_chave_2e4_dbn1(c) || selecao(c) {
                continue;
            }
            let nac = rd(c + 0x53);
            if nac == 0 || byte(nac + 0x11c) == 0 {
                continue;
            }
            let mut eh_a = 0u32;
            if parceiro(c, &mut eh_a, 1) != 0 && eh_a != 0 {
                continue;
            }
            let mut tem = (0..k).filter(|&j| rd(c + d + 4 * j) != 0).count() as i32;
            let alvo = ((rep(c) + 500) * k as i32 / 10000).clamp(1, k as i32); // a comissão pela reputação
            while tem < alvo {
                tem += 1;
                if gerados >= 23000 {
                    caixa_do_banco_dbn1(linha);
                    return 0;
                }
                let s = pessoa_extra_dbn1(rd(c + 0x53), gerados);
                core::ptr::write_unaligned((s + 0x1a) as *mut usize, rd(c + 0x53));
                *((s + 0x5f) as *mut u8) = 1;
                nascimento_extra_dbn1(s);
                if let Some(j) = (0..k).find(|&j| rd(c + d + 4 * j) == 0) {
                    core::ptr::write_unaligned((c + d + 4 * j) as *mut usize, s);
                    core::ptr::write_unaligned((s + 0x39) as *mut usize, c);
                    *((s + 0x3d) as *mut u8) = cargo;
                }
                gerados += 1;
            }
        }
    }
    let mut i = 0i32;
    while i < gi(0xae235c) {
        let c = rd(0xae23b4) + i as usize * 581;
        i += 1;
        if fraco_da_chave_2e4_dbn1(c) {
            continue;
        }
        let nac = rd(c + 0x53);
        let vale = selecao(c) || (nac != 0 && rd(c + 0x57) != 0 && byte(nac + 0x11c) != 0 && rep(c) != 0) || rep(c) > 7500;
        if !vale || rd(c + 0xcf) != 0 {
            continue;
        }
        if gerados >= 23000 {
            caixa_do_banco_dbn1(0x734);
            return 0;
        }
        let s = pessoa_extra_dbn1(nac, gerados);
        *((s + 0x5f) as *mut u8) = 1;
        *((s + 0x3d) as *mut u8) = 5; // treinador sem clube
        core::ptr::write_unaligned((s + 0x39) as *mut usize, 0);
        core::ptr::write_unaligned((s + 0x24) as *mut usize, 0);
        *((s + 0x28) as *mut u8) = 5;
        nascimento_extra_dbn1(s);
        gerados += 1;
    }
    formatar(0xde1f64, 0x9b5b10, gerados);
    1
}

/// { 0x5267c0 } (cdecl, Database.cpp): `database_trim_club_table(tamanho)`, uma vez na criação (create_game). Pequena =
/// tamanho 1 ou 2. Por nação (calloc de 0xd0 bytes por nação: 50 ids, quantidade +0xc8, índice do menor +0xcc), os
/// clubes de maior reputação (+0x80, 16 bits com sinal): até 20 (15 na pequena); nação não escolhida (+0x11c) e fora das
/// 15 chaves de nação: até 5 (tamanho 2) ou 11 (tamanho 1). Ficam também os clubes das pessoas (+0x39, +0x24), os dos
/// históricos de competição (+0xa, +0xe, +0x12, na faixa) e, fora da pequena, os do histórico de pessoas com nação
/// escolhida; as seleções; as chaves de clube (0x60ba80 recebe cada id com a linha nova e devolve 1 se é chave); e os
/// de nação escolhida, menos (na pequena) os fracos da 0x9cf2e4. Troca os ponteiros de clube (clube +0xb3/+0xb7/+0xbb,
/// pessoa +0x39/+0x24, preferências +4..+0x18, histórico de pessoas +0xa, histórico de competições +0xa..+0x16) pelos
/// novos (nulo o que sai), monta [0xae23fc] (linha nova → id antigo, malloc), compacta a tabela (id = linha nova),
/// realloc no lugar (outro endereço: caixa 0x4ca e fim) e grava "Club table has %d entries" em 0xde1f64. Sem [0xae23a8]
/// (caixa) ou sem memória no new: a original; sem memória nos calloc: caixa e fim, como a original.
unsafe extern "C" fn cortar_tabela_de_clubes(tamanho: u32) {
    if *(0xae23a8 as *const usize) == 0 {
        return orig::<unsafe extern "C" fn(u32)>(697)(tamanho);
    }
    crate::comparacao::ramo("criação cortar a tabela de clubes");
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let remapear: unsafe extern "C" fn(i32, i32) -> u32 = core::mem::transmute(0x60ba80usize);
    let gi = |a: usize| gi_dbn1(a);
    let rd = |a: usize| rd_dbn1(a);
    let wi = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
    let rep_id = |id: i32| rep(rd(0xae23b4).wrapping_add((id as usize).wrapping_mul(581)));
    let byte = |a: usize| *(a as *const u8);
    let novos = novo((gi(0xae235c) as u32).wrapping_mul(4));
    if novos == 0 {
        return;
    }
    let t = tamanho as u8;
    let pequena = t == 2 || t == 1;
    let marcados = calloc(gi(0xae235c) as u32, 4);
    if marcados == 0 {
        banco_fatal_dbn1(0x513);
    }
    let marcar = |id: i32| wi(marcados.wrapping_add((id as usize).wrapping_mul(4)), 1);
    let ultimo = rd(0xae23b4).wrapping_add((gi(0xae235c) as usize).wrapping_mul(581)).wrapping_sub(581);
    let nacoes = calloc(gi(0xae2350) as u32, 0xd0);
    if nacoes == 0 {
        banco_fatal_dbn1(0x51b);
    }
    let limite = if pequena { 15 } else { 20 }; // clubes por nação
    let chaves = [0x9cf2e4usize, 0x9cf360, 0x9cf314, 0x9cf488, 0x9cf300, 0x9cf408, 0x9cf260, 0x9cf218, 0x9cf2c0, 0x9cf244,
        0x9cf338, 0x9cf36c, 0x9cf434, 0x9cf45c, 0x9cf4a8];
    let mut c = rd(0xae23b4);
    while c <= ultimo {
        let nac = rd(c + 0x53);
        if nac != 0 {
            let id = gi(nac);
            let l = nacoes.wrapping_add((id as usize).wrapping_mul(0xd0));
            let cont = gi(l + 0xc8);
            let forte = byte(nac + 0x11c) != 0 || chaves.iter().any(|&k| id == gi(k));
            let entra = forte || match t {
                2 => cont <= 4,
                1 => cont <= 10,
                _ => true,
            };
            if entra {
                if cont < limite {
                    wi(l + cont as usize * 4, gi(c));
                    if cont == 0 || rep(c) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                        wi(l + 0xcc, cont);
                    }
                    wi(l + 0xc8, cont + 1);
                } else if rep(c) > rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                    wi(l + gi(l + 0xcc) as usize * 4, gi(c));
                    for j in 0..cont {
                        if j == 0 || rep_id(gi(l + j as usize * 4)) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                            wi(l + 0xcc, j);
                        }
                    }
                }
            }
        }
        c += 581;
    }
    let mut k = 0i32;
    while k < gi(0xae2350) {
        let l = nacoes + k as usize * 0xd0;
        for j in 0..gi(l + 0xc8) {
            marcar(gi(l + j as usize * 4));
        }
        k += 1;
    }
    livre(nacoes);
    let mut s = rd(0xae23bc);
    let fim = s.wrapping_add((gi(0xae2364) as usize).wrapping_mul(110)).wrapping_sub(110);
    while s <= fim {
        for &d in &[0x39usize, 0x24] { // o clube e a seleção da pessoa ficam
            let p = rd(s + d);
            if p != 0 {
                marcar(gi(p));
            }
        }
        s += 110;
    }
    let mut h = rd(0xae23d4);
    let fim = h.wrapping_add((gi(0xae237c) as usize).wrapping_mul(0x1a)).wrapping_sub(0x1a);
    while h <= fim {
        for &d in &[0xausize, 0xe, 0x12] {
            let p = rd(h + d);
            if p != 0 {
                let id = gi(p);
                if id < gi(0xae235c) && id >= 0 {
                    marcar(id);
                }
            }
        }
        h += 0x1a;
    }
    if !pequena {
        let mut h = rd(0xae23cc);
        let fim = h.wrapping_add((gi(0xae2374) as usize).wrapping_mul(0x11)).wrapping_sub(0x11);
        while h <= fim {
            let clube = rd(h + 0xa);
            let q = rd(h + 4);
            if clube != 0 && q != 0 {
                let nq = rd(q + 0x1a);
                let nc = rd(clube + 0x53);
                if (nq != 0 && byte(nq + 0x11c) != 0) || (nc != 0 && byte(nc + 0x11c) != 0) {
                    marcar(gi(clube));
                }
            }
            h += 0x11;
        }
    }
    let tabela = rd(0xae23b4);
    let mut destino = tabela;
    let mut c = tabela;
    while c <= ultimo {
        let id = gi(c);
        let fica = if remapear(id, (destino.wrapping_sub(tabela) as i32) / 581) != 0
            || gi(marcados.wrapping_add((id as usize).wrapping_mul(4))) != 0
            || id >= gi(0xae235c) - 2 * gi(0xae2350)
        {
            true
        } else {
            let nac = rd(c + 0x53);
            !(nac == 0 || byte(nac + 0x11c) == 0 || (gi(nac) == gi(0x9cf2e4) && rep(c) <= 500 && pequena))
        };
        let casa = novos.wrapping_add((id as usize).wrapping_mul(4));
        if fica {
            wr(casa, destino);
            destino += 581;
        } else {
            wr(casa, 0);
        }
        c += 581;
    }
    livre(marcados);
    // os ponteiros de clube pelos novos (o id antigo está em +0 do alvo, ainda no lugar velho)
    let trocar = |campo: usize| {
        let p = rd(campo);
        wr(campo, if p == 0 { 0 } else { rd(novos.wrapping_add((gi(p) as usize).wrapping_mul(4))) });
    };
    for &(cont, base, tam, campos) in &[
        (0xae235cusize, 0xae23b4usize, 581usize, &[0xb3usize, 0xb7, 0xbb][..]),
        (0xae2364, 0xae23bc, 110, &[0x39, 0x24][..]),
        (0xae2370, 0xae23c8, 0x34, &[4, 8, 0xc, 0x10, 0x14, 0x18][..]),
        (0xae2374, 0xae23cc, 0x11, &[0xa][..]),
        (0xae237c, 0xae23d4, 0x1a, &[0xa, 0xe, 0x12, 0x16][..]),
    ] {
        let mut i = 0i32;
        while i < gi(cont) {
            for &d in campos {
                trocar(rd(base) + i as usize * tam + d);
            }
            i += 1;
        }
    }
    let n = gi(0xae235c);
    let ficam = (0..n).filter(|&i| rd(novos + i as usize * 4) != 0).count();
    let ids = alocar((ficam as u32) << 2);
    *(0xae23fc as *mut usize) = ids;
    if ids == 0 {
        apagar(novos);
        return;
    }
    let mut linha = 0i32;
    for i in 0..n {
        let d = rd(novos + i as usize * 4);
        if d != 0 {
            // para a frente sobre a própria tabela (destino ≤ origem), como o rep movsd da original
            core::ptr::copy((rd(0xae23b4) + i as usize * 581) as *const u8, d as *mut u8, 581);
            wi(rd(0xae23fc) + linha as usize * 4, i);
            wi(d, linha);
            linha += 1;
        }
    }
    *(0xae235c as *mut i32) = linha;
    let r = realocar(rd(0xae23b4), (linha as u32).wrapping_mul(581));
    *(0xae23b8 as *mut usize) = rd(0xae23b4).wrapping_add(((linha - gi(0xae2360)) as usize).wrapping_mul(581));
    if r != rd(0xae23b4) {
        banco_fatal_dbn1(0x4ca);
    }
    apagar(novos);
    formatar(0xde1f64, 0x9b5af4, gi(0xae235c));
}

/// { 0x539ae0 } (cdecl, Database.cpp): `database_prune_comp_history_table()`, uma vez na criação. Compacta o histórico
/// de competições de clube ([0xae23d4], [0xae237c] linhas de 26 bytes): fica a linha com competição (+4) cuja nação
/// (+0x5d) é nula ou escolhida (+0x11c), copiada para a frente na própria tabela. realloc (0x945667) ao tamanho novo,
/// que vira [0xae23d4] (sem conferir o endereço); [0xae237c] = linhas; [0xae23dc] (a parte das seleções) = tabela +
/// (linhas − [0xae2384]) × 26; qsort (0x945333) com 0x540780.
unsafe extern "C" fn podar_historico_de_competicoes() {
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("criação podar o histórico de competições");
    let mut k = 0usize;
    let mut i = 0i32;
    while i < gi_dbn1(0xae237c) {
        let l = rd_dbn1(0xae23d4) + i as usize * 0x1a;
        let comp = rd_dbn1(l + 4);
        if comp != 0 {
            let nac = rd_dbn1(comp + 0x5d);
            if nac == 0 || *((nac + 0x11c) as *const u8) != 0 { // a linha fica
                core::ptr::copy(l as *const u8, (rd_dbn1(0xae23d4) + k * 0x1a) as *mut u8, 0x1a);
                k += 1;
            }
        }
        i += 1;
    }
    let t = realocar(rd_dbn1(0xae23d4), (k * 0x1a) as u32);
    let selecoes = (k as i32).wrapping_sub(gi_dbn1(0xae2384));
    *(0xae23d4 as *mut usize) = t;
    *(0xae237c as *mut i32) = k as i32;
    *(0xae23dc as *mut usize) = t.wrapping_add((selecoes as usize).wrapping_mul(0x1a));
    ordenar(t, k as u32, 0x1a, 0x540780);
}

/// O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
fn diferenca_dbn1(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}

/// Comparador de 0x527240: original e cópia lado a lado sobre as tabelas de clubes e de pessoas inteiras, [0x9b59b4], o
/// texto de 0xde1f64 e o estado do sorteador (a cópia sem contar sorteios). Com a pessoa de cargo 8 fora das casas
/// (caixa), só a original. O estouro das 23.000 (caixa) não é previsto aqui: a base de fábrica gera bem menos.
unsafe extern "C" fn comparar_gerar_comissao_extra() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(696);
    if cargo_8_fora_do_clube_dbn1() {
        return o();
    }
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.push((0x9b59b4, 4));
    regioes.push((0xde1f64, 0x40));
    regioes.push((rd_dbn1(0xae23b4), gi_dbn1(0xae235c).max(0) as usize * 581));
    regioes.push((rd_dbn1(0xae23bc), gi_dbn1(0xae2364).max(0) as usize * 110));
    let (r, m, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, || o(),
        || crate::aleatorio::sem_contar(|| gerar_comissao_extra()));
    let onde = diferenca_dbn1(&mo, &mr);
    PLACAR.registrar(igual && r == m, || match onde {
        Some((a, x, y)) => format!("gerar_comissao_extra: original={} rust={}, diverge em {:x} original={:02x} rust={:02x}", r, m, a, x, y),
        None => format!("gerar_comissao_extra: original={} rust={}", r, m),
    });
    // ponytail: 29 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
    r
}

// ponytail: compacta e realoca a tabela de clubes, aloca [0xae23fc] e troca as chaves de clube; não roda duas
// vezes. Só a original; a cópia leva o ramo e é provada pela prova do save (o .qst tem a tabela cortada).
unsafe extern "C" fn comparar_cortar_tabela_de_clubes(tamanho: u32) {
    orig::<unsafe extern "C" fn(u32)>(697)(tamanho)
}

// ponytail: compacta e realoca o histórico de competições; não roda duas vezes. Só a original; a cópia é provada
// pela prova do save (o histórico podado vai para o .qst).
unsafe extern "C" fn comparar_podar_historico_de_competicoes() {
    orig::<unsafe extern "C" fn()>(698)()
}

// ---------- 233a leva: a carga do banco: trocar os índices por ponteiros e o vetor de clubes pela reputação (Database.cpp) ----------

/// A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
unsafe fn caixa_do_banco_dbn2(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b5ac8, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// As tabelas do banco: (ponteiro, quantidade, bytes por linha).
const TABELAS_DO_BANCO_DBN2: [(usize, usize, usize); 12] = [
    (0xae23b4, 0xae235c, 581), (0xae23a8, 0xae2350, 0x122), (0xae23e8, 0xae2390, 0x2b), (0xae23b0, 0xae2358, 0x4e),
    (0xae23c8, 0xae2370, 0x34), (0xae23c0, 0xae2368, 0x44), (0xae23bc, 0xae2364, 0x6e), (0xae23e0, 0xae2388, 0x65),
    (0xae23cc, 0xae2374, 0x11), (0xae23ac, 0xae2354, 0x38), (0xae23d4, 0xae237c, 0x1a), (0xae23d0, 0xae2378, 0x6b),
];

/// Os campos de pessoa que, fora da faixa, abrem a caixa na 0x5315a0: (campo, teto, tabela, bytes, texto, linha).
const PESSOA_COM_CAIXA_DBN2: [(usize, usize, usize, usize, usize, u32); 3] = [
    (0x61, 0xae236c, 0xae23c4, 0x46, 0x9b60b4, 0x18f6), // jogador
    (0x65, 0xae2370, 0xae23c8, 0x34, 0x9b6088, 0x1907), // preferências
    (0x69, 0xae2368, 0xae23c0, 0x44, 0x9b605c, 0x1918), // não jogador
];

/// { 0x5315a0 } (cdecl, Database.cpp): `resolve_database_indexes(leniente)`, na carga (load_database, com 0) e na
/// gravação (save_database, depois de gravar). Troca cada campo de id pelo ponteiro ao registro (tabela + id × bytes),
/// na ordem da original e com as regras dela: negativo → nulo; alguns com teto "maior que a quantidade" (o id igual à
/// quantidade passa), outros "a partir da quantidade", outros sem teto. Clube: nação, divisões, estádios, cores, pessoas
/// e clubes (+0x53..+0xbb, teto inclusivo; registro em 0xde1f64 nos altos de nação, última divisão e estádio); elenco
/// +0xd7 (50; com `leniente` = 0, a partir de [0xae2364] vira nulo, com registro acima de [0xae2364] + 0x52d), olheiros,
/// técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar e os 20 escalados (+0x1e0), com registro.
/// Depois nações, árbitros, estádios, preferências, não jogadores, pessoas (jogador, preferências e não jogador fora da
/// faixa: registro, caixa e nulo), competições de pessoa, histórico de pessoas, cidades, histórico de competições e
/// competições de clube. Erros do original, copiados: a última divisão alta do clube (+0x5b) zera +0x60 (que depois vira
/// a competição 0) e fica crua; o estádio vizinho alto (+0x48) zera a cidade (+0x38) e fica cru. Não lê os registros
/// alvo nem aloca.
unsafe extern "C" fn resolver_indices_do_banco(leniente: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let alvo = |base: usize, v: i32, tam: usize| rd(base).wrapping_add((v as usize).wrapping_mul(tam));
    // sem teto: negativo → nulo
    let a = |f: usize, base: usize, tam: usize| {
        let v = gi(f);
        wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
    };
    // teto inclusivo: negativo → nulo; acima de [teto] → Some(v), quem chama decide; senão o registro
    let b = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v < 0 {
            wr(f, 0);
            None
        } else if v > gi(teto) {
            Some(v)
        } else {
            wr(f, alvo(base, v, tam));
            None
        }
    };
    // teto exclusivo: a partir de [teto] → Some(v), quem chama decide; negativo → nulo; senão o registro
    let c = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v >= gi(teto) {
            Some(v)
        } else {
            wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
            None
        }
    };
    let linhas = |cont: usize, base: usize, tam: usize, f: &dyn Fn(usize, i32)| {
        let mut i = 0i32;
        while i < gi(cont) {
            f(rd(base) + i as usize * tam, i);
            i += 1;
        }
    };
    linhas(0xae235c, 0xae23b4, 581, &|e, i| {
        if let Some(v) = b(e + 0x53, 0xae2350, 0xae23a8, 0x122) {
            formatar(0xde1f64, 0x9b62b0, v);
            wr(e + 0x53, 0);
        }
        if b(e + 0x57, 0xae2378, 0xae23d0, 0x6b).is_some() {
            wr(e + 0x57, 0);
        }
        if let Some(v) = b(e + 0x5b, 0xae2378, 0xae23d0, 0x6b) {
            formatar(0xde1f64, 0x9b6274, v);
            wr(e + 0x60, 0); // erro do original: zera +0x60 e +0x5b fica cru
        }
        if b(e + 0x60, 0xae2378, 0xae23d0, 0x6b).is_some() {
            wr(e + 0x60, 0);
        }
        if let Some(v) = b(e + 0x69, 0xae2358, 0xae23b0, 0x4e) {
            formatar(0xde1f64, 0x9b6240, v);
            wr(e + 0x69, 0);
        }
        for &(d, teto, base, tam) in &[
            (0x6eusize, 0xae2358usize, 0xae23b0usize, 0x4eusize),
            (0x83, 0xae2394, 0xae23ec, 0x3a), (0x8b, 0xae2394, 0xae23ec, 0x3a), (0x93, 0xae2394, 0xae23ec, 0x3a),
            (0x87, 0xae2394, 0xae23ec, 0x3a), (0x8f, 0xae2394, 0xae23ec, 0x3a), (0x97, 0xae2394, 0xae23ec, 0x3a),
            (0x9b, 0xae2364, 0xae23bc, 0x6e), (0x9f, 0xae2364, 0xae23bc, 0x6e), (0xa3, 0xae2364, 0xae23bc, 0x6e),
            (0xa7, 0xae2364, 0xae23bc, 0x6e), (0xab, 0xae2364, 0xae23bc, 0x6e), (0xaf, 0xae2364, 0xae23bc, 0x6e),
            (0xb3, 0xae235c, 0xae23b4, 581), (0xb7, 0xae235c, 0xae23b4, 581), (0xbb, 0xae235c, 0xae23b4, 581),
        ] {
            if b(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
        for j in 0..50 {
            let f = e + 0xd7 + 4 * j;
            let v = gi(f);
            if leniente == 0 && v >= gi(0xae2364) { // o elenco sem as linhas a mais
                if v > gi(0xae2364) + 0x52d {
                    formatar(0xde1f64, 0x9b6218, i, v);
                }
                wr(f, 0);
            } else {
                wr(f, if v < 0 { 0 } else { alvo(0xae23bc, v, 0x6e) });
            }
        }
        // olheiros, técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar, os 20 escalados
        for &(d, k, texto) in &[(0x1b3usize, 7usize, 0x9b61f4usize), (0x19f, 5, 0x9b61d0), (0x1cf, 3, 0x9b61ac), (0xc3, 3, 0x9b6188),
            (0xcf, 1, 0x9b6164), (0xbf, 1, 0x9b6140), (0xd3, 1, 0x9b6110), (0x1e0, 20, 0x9b60dc)]
        {
            for j in 0..k {
                let f = e + d + 4 * j;
                if let Some(v) = c(f, 0xae2364, 0xae23bc, 0x6e) {
                    formatar(0xde1f64, texto, i, v);
                    wr(f, 0);
                }
            }
        }
    });
    linhas(0xae2350, 0xae23a8, 0x122, &|e, _| {
        for &(d, base, tam) in &[(0x71usize, 0xae23a4usize, 0xc6usize), (0x7a, 0xae23ac, 0x38), (0x80, 0xae23b0, 0x4e),
            (0x90, 0xae23ec, 0x3a), (0x98, 0xae23ec, 0x3a), (0xa0, 0xae23ec, 0x3a), (0x94, 0xae23ec, 0x3a),
            (0x9c, 0xae23ec, 0x3a), (0xa4, 0xae23ec, 0x3a), (0x110, 0xae23a8, 0x122), (0x114, 0xae23a8, 0x122),
            (0x118, 0xae23a8, 0x122)]
        {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2390, 0xae23e8, 0x2b, &|e, _| {
        for &(d, base, tam) in &[(4usize, 0xae23f0usize, 0x3cusize), (8, 0xae23f4, 0x3c), (0x16, 0xae23a8, 0x122), (0x1a, 0xae23ac, 0x38)] {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2358, 0xae23b0, 0x4e, &|e, _| {
        if b(e + 0x38, 0xae2354, 0xae23ac, 0x38).is_some() {
            wr(e + 0x38, 0);
        }
        if b(e + 0x48, 0xae2358, 0xae23b0, 0x4e).is_some() {
            wr(e + 0x38, 0); // erro do original: zera a cidade e +0x48 fica cru
        }
    });
    linhas(0xae2370, 0xae23c8, 0x34, &|e, _| {
        for d in (4..=0x18).step_by(4) {
            a(e + d, 0xae23b4, 581);
        }
        for d in (0x1c..=0x30).step_by(4) {
            a(e + d, 0xae23bc, 0x6e);
        }
    });
    linhas(0xae2368, 0xae23c0, 0x44, &|e, _| {
        for d in (0x23..=0x3f).step_by(4) {
            if c(e + d, 0xae2364, 0xae23bc, 0x6e).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2364, 0xae23bc, 0x6e, &|e, i| {
        for &(d, base, tam) in &[(4usize, 0xae23f0usize, 0x3cusize), (8, 0xae23f4, 0x3c), (0xc, 0xae23f8, 0x3c),
            (0x1a, 0xae23a8, 0x122), (0x39, 0xae23b4, 581), (0x1e, 0xae23a8, 0x122), (0x24, 0xae23b4, 581)]
        {
            a(e + d, base, tam);
        }
        for &(d, teto, base, tam, texto, linha) in PESSOA_COM_CAIXA_DBN2.iter() {
            if let Some(v) = c(e + d, teto, base, tam) {
                formatar(0xde1f64, texto, i, v);
                caixa_do_banco_dbn2(linha);
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2388, 0xae23e0, 0x65, &|e, _| {
        for &(d, teto, base, tam) in &[(0x53usize, 0xae234cusize, 0xae23a4usize, 0xc6usize), (0x57, 0xae2350, 0xae23a8, 0x122),
            (0x5b, 0xae2394, 0xae23ec, 0x3a), (0x5f, 0xae2394, 0xae23ec, 0x3a)]
        {
            if c(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2374, 0xae23cc, 0x11, &|e, _| {
        a(e + 4, 0xae23bc, 0x6e);
        a(e + 0xa, 0xae23b4, 581);
    });
    linhas(0xae2354, 0xae23ac, 0x38, &|e, _| {
        a(e + 0x1f, 0xae23a8, 0x122); // a nação da cidade
    });
    linhas(0xae237c, 0xae23d4, 0x1a, &|e, _| {
        a(e + 4, 0xae23d0, 0x6b);
        a(e + 0xa, 0xae23b4, 581);
        a(e + 0xe, 0xae23b4, 581);
        for &d in &[0x12usize, 0x16] {
            if b(e + d, 0xae235c, 0xae23b4, 581).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2378, 0xae23d0, 0x6b, &|e, _| {
        for &(d, base, tam) in &[(0x5dusize, 0xae23a8usize, 0x122usize), (0x59, 0xae23a4, 0xc6), (0x61, 0xae23ec, 0x3a), (0x65, 0xae23ec, 0x3a)] {
            a(e + d, base, tam);
        }
    });
}

/// { 0x53db40 } (cdecl, Database.cpp): `database_sort_clubs_by_reputation(refazer)`, na criação, no carregar_jogo
/// 0x82a9c0 (vetor ausente do save) e no laço diário 0x5cccd0. Com `refazer` (byte) ≠ 0, o vetor [0xae2d1c] recebe os
/// ids (+0) dos [0xae235c] clubes na ordem da tabela; depois qsort (0x945333) dos [0xae235c] ids com 0x540020 (pela
/// reputação).
unsafe extern "C" fn ordenar_clubes_pela_reputacao(refazer: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    if refazer as u8 != 0 {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            *((rd(0xae2d1c) + i as usize * 4) as *mut i32) = gi(rd(0xae23b4) + i as usize * 581);
            i += 1;
        }
    }
    ordenar(rd(0xae2d1c), gi(0xae235c) as u32, 4, 0x540020); // todos os clubes pela reputação
}

/// O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
fn diferenca_dbn2(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}

/// Comparador de 0x5315a0: original e cópia lado a lado sobre as 12 tabelas inteiras (~50 MB na carga) e o texto de
/// 0xde1f64. Pessoa com jogador, preferências ou não jogador fora da faixa (caixa): só a original.
unsafe extern "C" fn comparar_resolver_indices_do_banco(leniente: u32) {
    let o = orig::<unsafe extern "C" fn(u32)>(699);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    let (pessoas, n) = (rd(0xae23bc), gi(0xae2364));
    let caixa = (0..n.max(0) as usize).any(|i| PESSOA_COM_CAIXA_DBN2.iter().any(|&(d, teto, ..)| gi(pessoas + i * 0x6e + d) >= gi(teto)));
    if caixa {
        return o(leniente);
    }
    let mut regioes = vec![(0xde1f64usize, 0x80usize)];
    for &(p, cont, tam) in TABELAS_DO_BANCO_DBN2.iter() {
        if rd(p) != 0 && gi(cont) > 0 {
            regioes.push((rd(p), gi(cont) as usize * tam));
        }
    }
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, || o(leniente), || resolver_indices_do_banco(leniente));
    let onde = diferenca_dbn2(&mo, &mr);
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("resolver_indices_do_banco({}): diverge em {:x} original={:02x} rust={:02x}", leniente, a, x, y),
        None => format!("resolver_indices_do_banco({}): diverge", leniente),
    });
    // ponytail: ~50 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
}

/// Comparador de 0x53db40: original e cópia lado a lado sobre o vetor de ids [0xae2d1c].
unsafe extern "C" fn comparar_ordenar_clubes_pela_reputacao(refazer: u32) {
    let o = orig::<unsafe extern "C" fn(u32)>(700);
    let v = *(0xae2d1c as *const usize);
    let n = core::ptr::read_unaligned(0xae235c as *const i32);
    if v == 0 || n <= 0 {
        return o(refazer);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(v, n as usize * 4)], || o(refazer), || ordenar_clubes_pela_reputacao(refazer));
    let onde = diferenca_dbn2(&mo, &mr);
    PLACAR.registrar(igual, || format!("ordenar_clubes_pela_reputacao({}): diverge em {:x?}", refazer as u8, onde));
}
