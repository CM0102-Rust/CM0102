# 303ª leva — os inscritos das copas da Turquia e da Rússia; as regras das divisões polonesas e da segunda divisão turca B

**Rotinas:**
- `tur_cup_collect_entrants()` = 0x8fcb90 (tur_cup.cpp, thiscall, ret 0): regra: a Copa da Turquia inscreve 64 clubes, todos da classe 1: 26 da segunda divisão categoria B (os 30 primeiros na ordem do banco, ordenados pela última divisão, tomados do 26º para trás), depois os 20 primeiros da categoria A em ordem inversa, e os da elite do fim da lista para trás;
- `rus_cup_select_entrants()` = 0x7e99a0 (rus_cup.cpp, thiscall, ret 0): regra: a Copa da Rússia inscreve 48 clubes: os da Primeira Divisão (classe 2; o atual campeão da copa entre eles, classe 1) da 32ª vaga para trás, depois os das zonas da segunda divisão (ordenados por força, os 42 melhores embaralhados) até completar 32, classe 2; depois os da liga principal (classe 1) da 33ª vaga em diante; dá certo quando a lista fica cheia;
- `pol_first_set_up_rules()` = 0x7c9690 (pol_first.cpp, thiscall, ret 0): regra: a Primeira Divisão polonesa: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por confronto direto, saldo e gols marcados; 4 na faixa de acesso, ninguém cai direto (o rebaixamento sai da fase final); abaixo a Segunda Divisão; 7 reservas e 3 substituições; bits das regras 8;
- `pol_second_set_up_rules()` = 0x7cbf40 (pol_second.cpp, thiscall, ret 0): regra: a Segunda Divisão polonesa: como a Primeira, com 2 subindo direto e 2 indo ao mata-mata de acesso, 6 caindo em 2001 (20 clubes) e 4 depois; acima a Primeira; bits das regras 3;
- `tur_second_b_set_up_rules()` = 0x900460 (tur_second_b.cpp, thiscall, ret 0): regra: a segunda divisão turca categoria B: 3 pontos por vitória e 1 por empate; desempate por saldo e gols marcados; 2 ao mata-mata; acima a categoria A; 7 reservas e 3 substituições; bits das regras 2;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `mwd27_classe_da_categoria_b` escapou: a mutacao troca a classe gravada para os 26 inscritos da categoria B da segunda divisao turca (`tur_cup_collect_entrants`, `TurCup::collect_entrants` 0x8fcb90: `set_cup_entry(this, k, lower[0x19 - k], 1)` passa a gravar 2). O `set_cup_entry` escreve o clube em +0, a situacao 0 em +5 e a classe no byte +0x4 da entrada de 6 bytes da tabela de inscritos (`cup_team_entry::SEED_LEVEL` de `cup::TEAM_TABLE`). Nao muda a carreira porque ninguem le esse byte nesta familia de copas: o sorteio generico das rodadas (`Cup::draw_round` 0x51c410) indexa a tabela so em +0, o ponteiro do clube (`mov ebx, [esi+0xa7]` com `eax = i*3` e `cmp dword ptr [ebx + eax*2], 0`, e `mov ecx, [edi + eax]` nas chamadas de texto), e as unicas rotinas portadas que leem +0x4 sao as das competicoes europeias, que guardam ali a classe de sorteio. Diferente da semente da copa croata (301a leva), aqui nao ha nem sorteio envolvido: a classe e uma constante, de modo que a sequencia do gerador fica identica. **A comparacao acusa:** o comparador `compare_tur_cup_collect_entrants` usa `compare_entrants_with_seeds` com o predicado `|_| true`, isto e, compara a classe de **todas** as 64 entradas, e a mutacao muda 26 delas. Nao e lacuna: a carreira nao ve, a comparacao ve.. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (72df841b).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 de3fb984, original b0e4088f (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 302ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 303`

**Campos tocados:**
- copa +4, +0x3e, +0xa7 (6 bytes: clube, classe, situação); clube +0x57 divisão
- liga +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xba
