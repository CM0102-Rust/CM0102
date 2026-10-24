# 224ª leva — o sorteio do primeiro e do segundo nome por nação e o nome da pessoa formatado; as cores do uniforme do clube, o técnico do clube e o nome do clube em inglês; o ajuste do moral, o vetor de atributos do não jogador e as marcas de elenco da pessoa

**Rotinas:**
- `primeiro_nome_da_nacao()` = 0x539c60 (Database.cpp, cdecl): sorteia um primeiro nome para a nação (20 sorteios pelo peso se ela tem mais de 100; senão empresta do grupo de língua);
- `segundo_nome_da_nacao()` = 0x53a1b0 (Database.cpp, cdecl): sorteia um segundo nome para a nação, do mesmo jeito;
- `formatar_nome_da_pessoa()` = 0x53dba0 (Database.cpp, cdecl): escreve o nome da pessoa num dos 7 estilos (nome sobrenome, N.sobrenome, sobrenome, nome, iniciais...), preferindo o nome comum ou o apelido;
- `cores_do_uniforme_do_clube()` = 0x53b640 (Database.cpp, cdecl): as cores do uniforme de casa, fora ou terceiro do clube em pixels de tela, com padrões quando faltam;
- `tecnico_do_clube_ou_selecao()` = 0x53ca90 (Database.cpp, cdecl): o técnico do clube, do parceiro A/B ou, na seleção sub-21, o da seleção principal;
- `nome_ingles_do_clube()` = 0x545880 (Database.cpp, cdecl): o nome do clube, ou o em inglês do eng.lng quando outro idioma está carregado;
- `ajustar_moral_da_pessoa()` = 0x544710 (Database.cpp, cdecl): soma uma variação ao moral do jogador, amortecida por sorteio perto das pontas, presa em 1..20; devolve se mudou;
- `vetor_de_atributos_do_nao_jogador()` = 0x53a8b0 (Database.cpp, cdecl): estende os atributos de treinador (+0x0e..+0x22) do não jogador num vetor de 21 células de 3 bytes, com o corte em 13;
- `marcar_elenco_da_pessoa()` = 0x540980 (Database.cpp, cdecl): grava os bits de elenco do clube ou da seleção da pessoa, mantendo o outro par;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `dq8_ingles` escapou: o nome inglês do clube só consulta o eng.lng quando o jogo está em outro idioma; a carreira e o jogo real comparando rodam em inglês (idioma 0/1), onde a rotina devolve o próprio nome. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 223ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 224`

**Campos tocados:**
- nação +0x00: id; id; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- nação +0x77: grupo de língua (1..32; 0 nenhum); valor; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- índice de nomes da nação ([0xae2338] + id × 36) +0x00 / +0x04: primeiro primeiro nome / primeiro segundo nome; id; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- índice de nomes da nação +0x18 / +0x1c: quantos primeiros / segundos nomes; valor; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- linha de nome (60 bytes em [0xae23f0] primeiros, [0xae23f4] segundos) +0x00 texto, +0x3b peso (com sinal); valor; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xae2398] / [0xae239c] total de primeiros / segundos nomes (até 0x7fff no sorteio); valor; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xae23a8] nações (0x122 bytes), [0xae2350] quantas; ponteiro; lê (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xad6c30] estado do rand() da CRT; valor; lê e grava (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- staff +0x04 / +0x08 / +0x0c: nome / sobrenome / nome comum; ponteiro (texto); lê (formatar_nome_da_pessoa)
- [0xb63c98] apelidos dados pelos humanos (0x600740, 0x6005e0); ponteiro; lê (formatar_nome_da_pessoa)
- clube +0x83 / +0x87: cores de casa (frente / fundo); ponteiro; lê (cores_do_uniforme_do_clube)
- clube +0x8b / +0x8f: cores de fora; ponteiro; lê (cores_do_uniforme_do_clube)
- clube +0x93 / +0x97: cores do terceiro uniforme; ponteiro; lê (cores_do_uniforme_do_clube)
- cor +0x37 / +0x38 / +0x39: vermelho / verde / azul; valor; lê (cores_do_uniforme_do_clube)
- [0xaebdf0] / [0xae3184] pixels padrão de frente / fundo; valor; lê (cores_do_uniforme_do_clube)
- clube +0xcf: técnico; ponteiro; lê (tecnico_do_clube_ou_selecao)
- clube +0x00: id (≥ [0xae235c] − [0xae2350] seleção); id; lê (tecnico_do_clube_ou_selecao, nome_ingles_do_clube)
- clube +0x53: nação; ponteiro; lê (tecnico_do_clube_ou_selecao)
- clube +0x244: sem nome (marca de parceiro A/B, zerada por 0x540a50 ao recusar); valor; lê e pode zerar (tecnico_do_clube_ou_selecao, por 0x540a50)
- clube +0x04: nome; texto; devolve o endereço (nome_ingles_do_clube)
- [0x9b5984] idioma; valor; lê (nome_ingles_do_clube)
- [0xae2408] nomes em inglês (+0 quantos, +0x20 linhas de 79 bytes), [0xae23fc] mapa de ids; ponteiro; lê (nome_ingles_do_clube)
- staff +0x61: jogador; ponteiro; lê (ajustar_moral_da_pessoa)
- jogador +0x45: moral (1..20); valor; lê e grava (ajustar_moral_da_pessoa)
- jogador +0x2f, staff +0x5a, staff +0x58: sem nome (a média / 3 é o teto do sorteio que amortece a queda do moral); valor; lê (ajustar_moral_da_pessoa)
- staff +0x39: clube; ponteiro; lê (ajustar_moral_da_pessoa, com limite)
- vínculo pessoa-clube (0x4cf5f0) +0x45: sem nome (não nulo impede o moral subir com limite); valor; lê (ajustar_moral_da_pessoa)
- [0xde1f5c], [0xde1f60], [0xad6c30] estado do gerador; valor; lê e grava (ajustar_moral_da_pessoa, por 0x90cfc0)
- não jogador +0x0e..+0x22: atributos de treinador (21 bytes com sinal); valor; lê (vetor_de_atributos_do_nao_jogador)
- staff +0x5e: marcas de elenco (& 3 clube, & 0xc seleção); valor; lê e grava (marcar_elenco_da_pessoa)

**Interessa a:** GPF2, CMRTE, CMScout
