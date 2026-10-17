# Placar da migração

Gerado por `python engenharia-reversa/fronteiras.py --placar` a cada leva. Fronteira de função = alvo de `call` ou rotina já portada; tamanho = até a próxima fronteira.

**Motor de partidas:** 78,7% · **sem as telas:** 95,6% · **executável inteiro:** 364 de 5524 KB (6,59%) · **360 cópias Rust**

| Módulo | Funções | KB | Coberto |
|---|---|---|---|
| `match_pl.cpp` | 65/67 | 148,3/149,3 | 99,3% |
| `match_eng.cpp` | 117/117 | 114,8/114,8 | 100,0% |
| `match_screens.cpp` | 0/27 | 0,0/68,5 | 0,0% |
| `match_man.cpp` | 8/8 | 25,1/25,1 | 100,0% |
| `match_day.cpp` | 17/18 | 15,7/16,6 | 94,9% |
| `match_events.cpp` | 3/6 | 1,6/14,0 | 11,2% |
| `match_official.cpp` | 1/1 | 1,0/1,0 | 100,0% |
