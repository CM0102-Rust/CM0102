# Placar da migração

Gerado por `python engenharia-reversa/fronteiras.py --placar` a cada leva. Fronteira de função = alvo de `call` ou rotina já portada; tamanho = até a próxima fronteira.

**Motor de partidas:** 78,1% · **sem as telas:** 94,8% · **executável inteiro:** 362 de 5524 KB (6,55%) · **357 cópias Rust**

| Módulo | Funções | KB | Coberto |
|---|---|---|---|
| `match_pl.cpp` | 65/67 | 148,3/149,3 | 99,3% |
| `match_eng.cpp` | 117/117 | 114,8/114,8 | 100,0% |
| `match_screens.cpp` | 0/27 | 0,0/68,5 | 0,0% |
| `match_man.cpp` | 8/8 | 25,1/25,1 | 100,0% |
| `match_day.cpp` | 16/18 | 14,8/16,6 | 89,1% |
| `match_events.cpp` | 1/6 | 0,2/14,0 | 1,1% |
| `match_official.cpp` | 1/1 | 1,0/1,0 | 100,0% |
