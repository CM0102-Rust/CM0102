*Versão em inglês: [fechamento_mkd1.en.md](fechamento_mkd1.en.md). As duas são mantidas juntas; o português é o original.*

# Fechamento do mapa do mercado, leitor D1: 0x864550 e 0x85fc50 (11/11/2026)

Rotinas: `STAFF_CONTRACT::default_staff_contract` (0x864550, o salário padrão) e
`STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, o contrato desejado). Tudo abaixo vem da desmontagem inteira do
`jogo/cm0102.exe` (`desmonta.py`, pacotes `pacotes/0x864550` e `pacotes/0x85fc50`), com o endereço da instrução; as
constantes foram lidas do exe. O mapa (`docs/MAPA_MERCADO.md`, seção D, tabela, §2.3 e §7) as tinha só pela estrutura.
Para cada afirmação: o texto antigo, o novo e a prova. A sessão principal troca [inferido] por [provado].

## 1. 0x864550 `default_staff_contract` — item **mkd_2** (portado, compila; prova nas bancadas)

### 1.1 Tabela da seção D

- **Antigo:** `| 0x864550 | ... | não | — | 0x846920 (portão do salário), 0x85fc50 (×3), 0x844780, 0x842e10, 0x840084, update_wage_bill (0x598370), 0x85e960 |`
- **Novo:** `| 0x864550 | ... | não (item mkd_2) | mkd_2 | 0x846920 (portão do salário, 0x849957), 0x85fc50 (×3: 0x860310, 0x8604ad, 0x860523), 0x844780 (0x84668f), 0x842e10 (0x84371d), 0x83f3a0 (0x840893), update_wage_bill (0x5983ee), 0x85e960 (0x85eca3) |`
- **Prova:** `refs.py jogo/cm0102.exe 0x864550` (9 chamadas relativas); as fronteiras de função do `pacote.py` põem a
  chamada 0x840893 na 0x83f3a0 (o mapa dizia 0x840084).

### 1.2 Convenção [provado]

stdcall(pessoa, clube, flag, estatuto, cargo, tipo), `ret 0x18` (0x8665b4), devolve o float em st0 carregado de um f32
([esp+0x4c], 0x8665a9). O ecx é sobrescrito na entrada (`mov ecx, [0xae1050]`, 0x864550): o invólucro thiscall do
`exe_calls.rs` passa um ecx que não conta. Os slots de argumento são reusados como rascunho ([esp+0x3c] clube em
0x865bfc..; [esp+0x48] cargo em 0x864723/0x864ba4/0x864c73/0x86522b; [esp+0x4c] o resultado), sempre depois da última
leitura; o fim usa o clube em ebx.

### 1.3 §2.3, a regra

- **Antigo:** "0x864550 `default_staff_contract` e 0x862720 `calc_unset_default_wage` (originais): o salário padrão
  pela ambição, lealdade, pressão, a nação grande (0x549e50), a temporada (0x547770) e o teto (0x5968d0). [inferido]"
- **Novo [provado]** (a 0x862720 não é desta rodada):
  1. O contrato da pessoa (`get_contract` 0x864562) vai a [0xae1080] (0x864569); o clube atual = o do contrato, salvo
     tipo 5 (0x864570..0x864591).
  2. O nível: com clube, reputação / 500 + 1 num byte (0x8645d2..0x8645eb); com clube atual e estatuto profissional 2,
     menos um, entre 1 e 4 (0x8645fe..0x864613). Sem clube, a reputação mundial da ficha de jogador (+0xd) ou da de não
     jogador (+0xc) / 500, mínimo 1 (0x86461e..0x86465b).
  3. O cargo: `job_for_club` (+0x3d) se há clube atual; senão `job_for_nation` (+0x28) com seleção (+0x24); senão +0x3d;
     senão 11 para jogador; senão +0x28; senão 9 (0x864669..0x8646a2). Tabela de salto 0x8665b8 sobre cargo − 5.
  4. Comissão (cargos 5 a 10): o nível sobe ao do clube atual ou da seleção (não jogador, cargo nacional ≠ 11)
     (0x8646ba..0x864714); base = reputação atual do não jogador (+0xa)³ × k1 ou, sem ficha de não jogador,
     (id / total de pessoas × 4 + 1) × k2 × 0,0001; salário = nível × base × 0,25 (0x864718..0x864761). Por cargo:
     5 (k1 2,5e-8 0x96ff38, k2 25000): mínimo 750, acima de 25000 → **150000** (0x864767..0x8649f0); 6 e 7 (1e-8, 10000):
     mínimo 500; 8 (5e-9, 5000): 275 a 5000; 9 (1e-9, 1000): 250 a 1000; 10: 200 a 1000 (0x864cd2..0x864ce3). Com clube
     atual diferente do clube: cargos 6, 7 e 8 dão 150000 quando o clube atual está mais de três níveis acima
     (reputação / 500 + 4 > nível) — no 6 também quando o cargo pedido não é 11 nem 6; sem isso, o 6 abaixo de 2500
     × 1,35 (0x86485b..0x8648c3, 0x86498e..0x8649ba, 0x864ade..0x864b2e); 6 e 7 fora disso: acima de 25000 → 150000,
     salvo cargo pedido 11 ou 6 (0x8649c7..0x8649f0). Cargo 14 e os outros: 1000 (0x864ce3). **150000 é a marca de
     "indisponível"**, não um salário.
  5. Jogadores (cargos 11, 12, 13, 15): sem ficha, a mensagem "No player data" vai ao texto 0xde1f64 e o salário é
     `rand() × 4 / 32768 × 550` (0, 550, 1100 ou 1650; 0x865699..0x8656e1). Com ficha:
     - estatuto −1 com clube: 1 (reputação atual > a do clube, 24 a 33 anos), 2 (> 0,75 × a do clube, 21+), 4 (> 0,5 ×
       a do clube e 17+, ou 25+), 5 (potencial × 50 > 0,75 × a do clube), 6 (0x864d40..0x864e00); o byte baixo do
       argumento é regravado e vai inteiro ao `get_max_wage`;
     - a base: tipo 3 85000; tipo 1 92500 (lealdade < 15 e pressão > 15) ou 75000; tipo 2 75000 (23 a 31 anos) ou
       82500; cargo pedido 13 ou 15 com 36+ anos 10000; estatuto 5/6: 9350 (reputação atual < 2500), 11000 (< 6250);
       atacante ≥ 15: 82500 / 70000 (reputação > 6750, < 31 anos, lealdade < 15 / ≥ 15) ou 55000; volante, meia ou meia
       atacante ≥ 15: 67750 / 60000 (> 7250, < 31, lealdade < 14) ou 44000; goleiro 20: 37500 (mundial > 7250,
       lealdade < 3, < 34) ou 22000; líbero e zagueiro < 15: 27500; senão 50000 (> 7500, lealdade < 6, < 33) ou 27500
       (0x864e04..0x864fc6). **O tipo 0x14244 da terceira comparação (0x8651bc) nunca vale: defeito, o piso 6500 não
       existe;**
     - o jovem (19 a 31 anos) não preso ao clube nem à nação do clube (clube de reputação < 5750 ou liga < 12) com
       mundial > (lealdade + 230) × 25 − ambição × 10 sobe a 75000 / 92500 / 105000 (0x864fca..0x8650c9);
     - regra da FIFA ([0xae233c] = 1), acima de 11000, clube de país grande (0x549e50): × 1,15 / 1,125 / 1,1 (reputação
       atual > 6750, pela lealdade 1 / < 6 / outra) ou × 1,025 (> 4250) (0x8650cd..0x865141), sem arredondar a f32;
     - m = a maior das reputações (atual, local, mundial); mundial < 1750 e m > 3250: m = 3250 − ftol((m − 3250) × −0,2);
       tipo 1 mínimo 7250, tipo 2 mínimo 6750; base 75000/92500/105000 mínimo 7250 (0x865145..0x865201);
     - u = (atual + local + mundial + 4m + 50 × (habilidade + potencial)) / 9 + ftol(id / total × 8 − 4); estrangeiro
       (outra região, `actual_region` +0x76, que a do clube e a do clube atual): mínimo 1500 ou × 1,05; mínimo 1
       (0x865206..0x8652c3);
     - salário = base × u² × 1e-8, mínimo 175 (comparado antes de arredondar, 0x8652c8..0x8652f8); × 0,75 ou × 0,5 pelo
       `state_of_development` (+0x7e) da nação do clube atual (ou 1 com mundial > 4250, ou a da pessoa, ou 3)
       (0x865300..0x865345); × mundial × 0,0004 (clube < 2500) ou mundial / (nível × 500), entre 0,85 e 1,15
       (0x865349..0x8653b7);
     - tetos por nível: < 10 → 2500 × nível; < 15 → 2750 × nível; senão 10% acima de 5250 × nível (Inglaterra, Espanha,
       Itália com PLC +0x82), 7500 × nível (sem PLC) ou 3250 × nível (as outras) (0x8653c1..0x865513);
     - teto da posição: 20% acima de 150000 (atacante 20), 125000 (meias 20), 100000 (zagueiro 20) ou 80000; × 1,1 com a
       FIFA num país grande e atual > 7250 (0x865515..0x8655ae);
     - sem `club_job` (+0x39), salvo o veterano (33+) internacional com raça + determinação ≤ 30, id % 15 ≠ 0, em clube
       ≤ 4500: × (1,375 − d × 0,0125), mínimo 0,2, com d = dias desde `date_joined_club` (+0x3e, máximo 250), + 60 na
       temporada (0x547770), 20% acima de 30 (0x8655b2..0x865690).
  6. Com clube (0x864ceb, sem clube devolve já): para quem tem ficha, r = (reputação do clube × 0,00017 / 0,00012 /
     0,0001)⁶ (pow 0x945ff0), no máximo 1; por estatuto (tabela 0x8665e4) o salário sobe a meio caminho de 3 × reputação
     × r × (1: 0,6 / 1,5 / 1,25; 2: 0,3 / 1 / 0,6; 3: 0,25 e 0,6 / 0,3; 4: 0,1 / 0,3 / 0,15; 5: 0,05; 6: 0,02, por < 20
     anos / 24+ em país grande / outros) (0x864cf7..0x86598a); com a queixa 0x10 do contrato (+0x45): × 0,67 / 0,67 /
     0,85 / 2,5 / 3,5 / 7,5 (tabela 0x8665fc, 0x86598c..0x8659ec).
  7. A nação do clube, quando a pessoa trabalha lá (`club_job` da mesma nação, e é de lá ou lealdade > 3 ou ambição <
     15) ou é de lá sem clube (0x8659f0..0x865a47): Japão mínimo nível² × 10 e 25% acima de 7500; Inglaterra, Itália,
     Espanha nada; EUA × 0,75, MLS 15% acima de 250, nativos 20% acima de 1250 (32+ ou < 24 com mundial < 3750) ou 2750,
     estrangeiros 25% acima de 3250; Coreia, Escócia, Alemanha/França/Holanda, Suécia, Dinamarca/Noruega, Bélgica,
     Portugal/Croácia/Polônia, Brasil/Argentina × o fator (reputação do clube / 50 / `league_standard`) × 0,1 × k
     (k = 0,25 / 0,25 / 0,75 / 0,25 / 0,5 / 0,45 / 0,5 / 0,45), máximo 1, quando reputação / 50 > liga × 10, e os tetos
     de cada um; Gales/Finlândia/Irlanda/Irlanda do Norte/Austrália só tetos (200 / 100 × 0,15); as outras × 0,3 e 25%
     acima de 15000 (0x865a4d..0x86635b).
  8. Jogadores: no máximo o salário do contrato com o clube atual de estatuto 7 (`get_contract_for_club`, 0x866377..);
     com `flag`, ou sem clube atual e atual < 2250, no máximo `get_max_wage` (sem finanças devolve o valor como está,
     sem os mínimos) (0x8663af..0x866414); sem `flag` e com clube atual, o ambicioso (> 10), pouco leal (< 6), famoso
     (> 6750) atacante/meia/meia atacante 20 com 21+ anos: × 1,15/1,1 (meia atacante) ou, se salário × 1,25 < máximo,
     × 1,1/1,05 (meia) ou × 1,2/1,15 (0x866428..0x8664ff).
  9. Mínimo: clube de estatuto 1: 75 abaixo de 20 anos, senão 100; outros: 75 em clube > 2250, senão 50
     (0x866503..0x8665a9).
- **Ponto de extensão:** o salário padrão é um float devolvido em st0: um desvio que chame a original e multiplique a
  saída muda todos os quem chamam (o pedido, o portão da vontade e a folha) num lugar só. Com a cópia Rust do mkd_2,
  os pontos de ajuste fino são a base por posição (passo 5), o u² e os tetos por nação (passo 7).

### 1.4 Defeitos do original (0x864550)

- `cmp edx, 0x14244` (0x8651bf) compara o byte do tipo com um número que não cabe num byte: o piso 6500 nunca vale.
- Sem clube, um jogador com nação lê o clube pelo ponteiro nulo em 0x865270 (e de nível ≥ 15 em 0x86541b); nenhum
  chamador passa clube nulo para jogador.

## 2. 0x85fc50 `set_to_desired_contract` — item **mkd_3** fora (FORA.md); regras provadas

### 2.1 Tabela da seção D

- **Antigo:** `| 0x85fc50 | ... | não | — | ... |`
- **Novo:** `| 0x85fc50 | ... | não (mkd_3 fora: tamanho, ver FORA.md) | — | ... |` (os chamadores do mapa conferem com
  o pacote: 0x47c0b0, 0x4cda70, 0x4ce450, 0x4ce4e0, 0x4de5d0, 0x4e2280, 0x84a020, 0x866620, 0x866830, 0x8bf060,
  0x8c0080, 0x8c9720).

### 2.2 Convenção [provado]

thiscall(contrato, pessoa, clube, oferta, estatuto, cargo, tipo, &vontade), `ret 0x1c` (0x862688), devolve um int (o
valor do contrato). Os argumentos em [esp+0x278..0x290] depois do prólogo (0x85fc7e, 0x85fd66, 0x85ff4a, 0x8602f8,
0x8609d4, 0x8609f5). Quadro com tratador C++ 0x962a76.

### 2.3 §2.3, a regra

- **Antigo:** "0x85fc50 (original, 10.896 bytes): o contrato que a pessoa quer no clube. Lê ambição, lealdade, pressão,
  profissionalismo, int_apps; o valor do jogador (calc_player_value 0x4d0c60, ×7...); o salário padrão (0x864550, ×3);
  a vontade (0x846920 e 0x844780); o teto (get_max_wage, ×4); a janela (0x8cf470, 0x8cf0e0); o mês do fim da temporada
  (0x4d45f0); um sorteio. Grava o tipo 2 em +0x35, a situação em +0x4f (nibble alto), o salário/valor em +8/+0xc,
  cláusulas e a data de fim. [inferido] Teto absoluto 250.000 no ramo de comissão (fVar27 = 250000.0)."
- **Novo [provado]:** as contagens conferem (calc_player_value em 0x85ffc8, 0x8615e5, 0x8617c5, 0x8618cd, 0x8619ed,
  0x861af2, 0x861c53; o salário padrão em 0x860310, 0x8604ad, 0x860523; get_max_wage em 0x861274, 0x8612a2, 0x8612d4,
  0x861302). A ordem:
  1. Zera +0x21 e +0x20; sem clube, o clube passa a ser o atual (ou o time B ligado, `club_get_linked_b_team` 0x540a50)
     (0x85fcb0..0x85fd18); o contrato com o clube atual (`get_contract_for_club`, 0x85fd2b) é "o contrato atual".
  2. +0x35 = (+0x35 & 0x40) | 2 (tipo 2), +0x49 = 0, +0x4a = −1, +0x3b = 0, +0 = id, +0x3a = 0 (0x85fd34..0x85fd56).
  3. Estatuto −1 com ficha: o merecido (`get_deserved_squad_status` 0x4ca470) contra o contrato atual (queixa 0x10 e
     não 1 → 2) ou `get_squad_status_for_club` (0x849f00); vai ao nibble alto de +0x4f (0x85fd73..0x85fe40).
  4. Nível (byte): com clube, reputação / 500 + 1 (estatuto profissional 2: −1, entre 1 e 4), +4 = id do clube; sem
     clube, a da seleção / 500 + 1 (mínimo 3) ou a mundial / 500; + ftol(liga × 0,25 × int_apps × 0,125) (1 a 4) para
     internacionais (0x85fe3e..0x85ff42).
  5. A luva (+8) de jogador: `calc_player_value` / 5 (0x85ffc8..0x85ffe2), × 0,25 (sem club_job, atual < 2750, clube
     < 2000), × 0,5 (< 4250) ou × 0,75 (< 4750); renovação: 0 (< 19 anos e < 3750), × 0,1/0,25/0,5 para < 21 anos, ×
     0,5 em clube < 3500; × 0,9 / 0,75 / 0,75 / 0,5 / 0,25 pelo estatuto merecido (tabela 0x86268c); clube novo: × 0,5
     (atual < 4250) e × 0,5 (liga < 10) (0x85ffe5..0x86016b). Comissão: 0 (0x860177).
  6. Sem `job_for_club`: mensagem "has no job for club" e +0x3d = 11 (com ficha) ou 9 (0x86017e..0x8601a6, 0x8602f4).
  7. Comissão (0x8602f8): w = salário padrão (flag 0, tipo 0); vontade = `staff_interest_in_job_at_club` (0x844780),
     gravada em *&vontade; w + 2500 com vontade < 1,75; valor = w × max(0,9; 3,5 / max(vontade; 0,1))
     (0x861333..0x86137e); cargo 6: 250000 se > 2 × reputação + 1200 e > 4100; 8: > reputação + 500 e > 3200; 9: >
     0,75 × reputação + 250 e > 2100; 10: o mesmo com 1500; e sempre no máximo 250000 (0x861375..0x861492). **O
     "teto 250.000" é a marca de "indisponível" (0x48742400 = 250000), não um teto de salário.**
  8. Jogador, renovação no próprio clube (0x8601e2..0x86045a): o salário padrão com flag 1 (no máximo o teto do clube)
     quando a renovação é "barata" (sem queixa, sem a marca 0xf do registro de 79 bytes 0xae2d0c, atual < 7250, menos de
     lealdade × 12 + 182 dias para o fim e saída pela Bosman marcada; ou pressão < 6, < 24 anos, sem transferência
     futura, fora da lista, < 3250 e abaixo da reputação do clube + 250, < 200 − pressão dias; ou queixa sem os bits
     0x161a8 e poucos dias; ou lealdade > 18 / atual < 5250 sem a marca, com mundial < 3500); senão flag 0.
     Clube novo (0x86045f..0x860528): tipo 3 com atual > 6750, senão 0 (os testes de ambição + lealdade > 80 e > 100,
     que dariam o tipo 1/2 e o estatuto 1/2, não valem com atributos de 1 a 20).
  9. Jogador: salário pedido acima do atual: no máximo max(2 × atual; atual + 8000), e para o famoso (mundial > 6750,
     > 10000) acima disso, o teto 75000 (Inglaterra/Espanha/Itália) ou 50000, 25% acima (0x86052c..0x860699).
     A ambição: t = mundial + 25 × (ambição − lealdade) + 10 × pressão (16 bits); o reforço no país grande para 19 a 31
     anos (orçamento de salários das finanças +0x10, até 0,0001 × t × 85000/105000 na Inglaterra/Espanha/Itália, goleiro
     até 60000) × (0,9..1,0 + ajustes de idade, posição e (ambição − profissionalismo − lealdade + 4) × 0,0075 entre −0,25
     e 0,15) (0x8606c7..0x8609cc).
  10. Vontade = `staff_interest_in_joining_club` (0x846920), gravada em *&vontade; q = 3,5 / max(vontade; 0,1); w +
      5000 com vontade < 1,75 (sem contrato atual, ou com queixa e jovem/em lista/...); q mínimo 0,9; na renovação q
      máximo 1,25/1,75/1,5/4,0 pelas queixas (2 / 1 / 0x8400 / outras), 1,25/1,5/2,0/1,35/1,15 por idade e estatuto
      merecido (tabelas 0x86269c, 0x8626b0), 1,1 para o jovem (< 20, habilidade < 125 − pressão − ambição, atual < 4250,
      < 120 − pressão − ambição dias), 3,0 sem contrato (0x8609ce..0x860e27); valor = q × w (0x860e2f).
  11. No mínimo o salário atual × (reputação do clube / a do atual, mínimo 0,5) quando o clube é mais fraco
      (0x860e4b..0x860ed6); com a queixa 0x1000000, até 95% do orçamento de salários (0x860ed8..0x860f0c).
  12. Marcas de "indisponível" (250000): valor > 200000; > 35000 com 35+ anos; > 75000 com 32+; goleiro 20 > 105000;
      estatuto 7 > 5000, 6 > 8250, 5 > 17500, 4 > 35000, 3 > 57500 (mundial < 6750; clube humano `human_manager_is_
      human_club` 0x600a50, ou fora das faixas 85000/105000/125000), 2 > 125000 (mundial < 7250) (0x860f10..0x8610f0).
  13. Contrato atual mais caro, em outro clube: valor = ((20 − a) × salário atual + a × min(w; max(get_max_wage; w ×
      0,5))) × 0,05, com a de 0 a 20 (queixa 0x100 +10, em lista +10, salário > 40000, idade 34+, felicidade e moral)
      (0x86110b..0x86132a).
  14. Cláusulas (só jogador com cargo 11): +0x1d=0/+0x20=1 para o veterano; não promoção (+0x1c), rebaixamento (+0x1f),
      multa mínima (+0x1d), sem jogar (+0x1e), com a multa (+0x21) por `calc_player_value` e mínimos 150000 / 50000 /
      150000 / 1500000; as das ligas pelos virtuais +0x98/+0x9c da competição (tipo 2) (0x86149a..0x861d1b).
  15. Datas (0x861d1f..0x862371): `date_started` (+0x25) pela janela (0x8cf470 ou 0x8cf0e0 pela oferta) e logo
      regravada com hoje (0x861e14); fim = dia `utils_random(0x1c)` + 1 do mês do fim da temporada (0x4d45f0) do ano
      (seguinte, pela tabela 0xb63d77 da nação-chave 0x669360) + ftol(L + 1) anos; L por idade (3; 4; 3/2/1,5 por
      posição; 1 ou 4), × 0,75 com cláusula, pelo nível e lealdade, pelo estatuto (tabela 0x8626c4), pela reputação,
      Finlândia/Turquia × 0,5 (máx. 3), salário < 500/1000/2000 → máx. 2/3/4, máximo [0xa83608] − 1 = 9, mínimo 1;
      tipo 2 com o bit 0x80 (sem `club_job`, nível < 2 e ftol(sorteio fixo × 4, ou × 5 abaixo de nível × 500 + 1250
      de reputação) = 0); nesse último caso, até 20 anos, tipo 1 (+0x35 & 0xc1 | 1) sem anos a mais; na renovação o
      fim não recua (`date_expires_club` +0x46).
  16. Bônus (+0x10/+0x14/+0x18) = ftol(valor × sorteio fixo × 0,025 + valor × 0,025 + 25) para atacante / meio / goleiro
      20, senão −1; +0x35 |= 0x80 com valor < 125 em clube de estatuto 2, limpo acima de 1250; **+0xc = ftol(valor)**
      (0x86237a..0x862493).
  17. Retorno: ftol(salário (× 0,5 no tipo 1, × 1,05 com 0x80) + min(soma dos bônus/5 e das cláusulas × 0,013..0,1;
      valor × 0,1)) (0x862496..0x86266d).
- **Ponto de extensão (§7, item 2):** confirmado [provado]: +0xc é gravado uma vez, no fim (0x862493), e +8 antes
  (0x85ffe2..0x86016b); o retorno não relê +0xc. Um desvio que chame a original e ajuste +0xc/+8 na saída basta.
  Atenção: o retorno (o valor do contrato) já sai calculado com o salário de antes do ajuste; quem chama e usa o
  retorno em vez de +0xc não vê o ajuste (conferir chamador por chamador antes de desviar).

### 2.4 Defeitos do original (0x85fc50)

- 0x861d64..0x861d70: `if (!slot && !*slot)` no lugar de `||`: com a oferta sem vaga (0x8c61e0 devolve 0) lê o
  endereço 0 antes da caixa de erro; com a vaga e `*slot == 0` passa 0 ao 0x8cf0e0.
- [esp+0x48] (a reputação usada nas cláusulas e na duração) só é escrito com ficha + cargo de jogador ou com ficha de
  não jogador (0x85ff46..0x85ff7c); uma pessoa com ficha, cargo de comissão e sem ficha de não jogador lê lixo da pilha
  (que vira 1 se for 0).
- Os testes (100 − ambição − lealdade) × 125000 < 0 e (80 − ...) < 0 (0x86045f..0x8604df) nunca valem com atributos
  de 1 a 20: o tipo 1/2 e o estatuto 1/2 desse ramo não acontecem.
- `date_started` calculado pela janela é sobrescrito com hoje logo depois (0x861e14).
