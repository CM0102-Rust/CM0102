# Levas do motor: como cada uma foi feita e testada

Cada leva porta rotinas do motor de partidas para Rust (`rust/src/motor.rs`) e só entra depois de passar pela cadeia de testes. A partir da 143ª leva (16/10/2026), cada uma tem uma pasta aqui com o que foi usado e o que deu, para quem quiser refazer, conferir ou continuar o trabalho. As levas anteriores estão descritas em `docs/ROTINAS.md`, com as mutações em `engenharia-reversa/mutacoes/`.

## O que há em `levas/NNN/`

| Arquivo | O que é |
|---|---|
| `LEIA.md` / `LEIA.en.md` | a leva em uma página: rotinas, decisões, o que deu errado no caminho e o resultado |
| `codigo.rs` | o código Rust que a leva acrescentou ao `motor.rs` (cópias, despacho do modo Rust e comparadores) |
| `integrar.py` | como o `codigo.rs` entrou no `motor.rs`: tabela `PROLOGOS`, listas de funções, portas de gravação que saíram, `ZERADAS` |
| `docs.py` | gera a documentação da leva (`docs/ROTINAS`, `PROJETO`, `wiki`, glossário) com `engenharia-reversa/gen_docs.py` |
| `cadeia.txt` | a saída completa da cadeia de testes (abaixo) |

As mutações ficam em `engenharia-reversa/mutacoes/levaNNN.py`, junto com as das levas antigas.

## O ciclo de uma leva

1. **Escolher** as rotinas: `python engenharia-reversa/fronteiras.py --candidatas` mostra o placar e as rotinas do motor cujas chamadas já estão portadas. O placar publicado fica em `docs/PLACAR.md` (atualizado a cada leva).
2. **Ler** o original: `python engenharia-reversa/pacote.py 0xADDR` junta numa pasta (`pacotes/`, fora do git) a desmontagem, o pseudocódigo, quem chama e o que é chamado (com o nome Rust de cada rotina já portada), as constantes de ponto flutuante com os valores, as tabelas de salto, os campos que ela usa com as rotinas do exe inteiro que os leem ou escrevem (pelo texto do OllyDbg trazido do fórum, `referencias/cm0102_3.9.68_text.zip`) e onde a rotina já aparece nas notas. Por dentro: `descompilar.py` (pseudocódigo do Ghidra) para a estrutura e `desmonta.py` (assembly) para as contas de ponto flutuante e os detalhes que o pseudocódigo esconde.
3. **Escrever** a cópia Rust com as chamadas que mexem fora do motor injetadas (`chamar`), o despacho do modo Rust e o comparador (lado a lado, ou pela fita quando há chamadas com efeito).
4. **Comparar** num jogo até 2001-08-20 (`simular.py --modo comparar`): zero divergências.
5. **Mutações**: cada uma muda um detalhe da cópia; a carreira tem de mudar (ou a comparação acusar). A que não é acusada é lacuna (vai para `docs/PENDENCIAS.md`) ou mutação equivalente (refeita).
6. **Cadeia**: `bash engenharia-reversa/cadeia.sh NNN 2001-10-01` roda as mutações, um jogo real comparando até 2002-01-01 e as carreiras Rust ×2 e original até 2002-06-01. As três carreiras têm de dar a impressão de referência.
7. **Documentar** (`levas/NNN/docs.py`) e publicar.

Detalhes, armadilhas e o porquê de cada passo: `docs/COMO_CONTINUAR.md`. Nada do jogo entra aqui: a cadeia só registra números, nomes de ramos e impressões.
