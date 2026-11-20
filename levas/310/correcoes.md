# A 310ª leva: dois falsos alarmes de comparador no jogo real (19/11/2026)

O jogo real da cadeia (bancada 3, ENG BEL HOL GER DEN até 2002-08-01) acusou duas divergências. As duas eram dos
comparadores, não dos portes; os dois comparadores foram corrigidos e o jogo real foi repetido do zero na bancada 4:
**0 divergências** até 2002-08-01 (é o resultado do `cadeia.txt`).

| Rotina | O que o jogo real mostrou | Causa | Correção |
|---|---|---|---|
| `hol_first_advance_stage` | `before=3 after=5` | Ao chegar à fase 4, `HolFirst::create_play_off_groups` (0x5f6720) acrescenta as fases 4 e 5 (o virtual `+0x3c` com 4 e com 5) e o original termina na 5. O comparador comum (`compare_first_division_advance`) exigia subir uma fase só. É o mesmo falso alarme da Finlândia na 304ª. | O comparador holandês aceita a fase 5 quando a nova fase é a 4; no resto, a regra comum. |
| `ger_lge_cup_collect_entrants` | `list false` nas 6 entradas | O campeão da Copa vem de `comp_util_get_latest_placed_club` (0x4afd80), que reordena a cada chamada a tabela global do histórico das competições (`[0xae23d4]`, `[0xae237c]` registros de 0x1a bytes) com o `qsort` da CRT, que não é estável. Lado a lado, a rotina roda duas vezes, e a segunda pode ver os registros de mesma chave em outra ordem. | A cópia roda sobre o histórico como a original o viu, e o histórico da original é reposto depois; se as listas divergirem, o log mostra as duas. Na repetição a rotina rodou e bateu. |

O porte das duas rotinas foi conferido instrução por instrução contra a desmontagem e não mudou.
