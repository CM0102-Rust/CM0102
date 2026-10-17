# Placar da migração

Gerado por `python engenharia-reversa/fronteiras.py --placar` a cada leva. Fronteira de função = alvo de `call` ou rotina já portada; tamanho = até a próxima fronteira.

**Motor de partidas:** 73,9% · **sem as telas:** 89,7% · **executável inteiro:** 346 de 5524 KB (6,26%) · **346 cópias Rust**

| Módulo | Funções | KB | Coberto |
|---|---|---|---|
| `match_pl.cpp` | 64/67 | 148,3/149,3 | 99,3% |
| `match_eng.cpp` | 117/117 | 114,8/114,8 | 100,0% |
| `match_screens.cpp` | 0/27 | 0,0/68,5 | 0,0% |
| `match_man.cpp` | 6/8 | 15,8/25,1 | 63,2% |
| `match_day.cpp` | 9/18 | 7,8/16,6 | 47,4% |
| `match_events.cpp` | 0/6 | 0,0/14,0 | 0,0% |
| `match_official.cpp` | 1/1 | 1,0/1,0 | 100,0% |
