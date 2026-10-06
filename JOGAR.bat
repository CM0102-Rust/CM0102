@echo off
rem Abre o CM 01/02 em 1920x1080 (nativo), em janela redimensionavel, sem CD e sem trocar a resolucao do Windows.
rem Versao 800x600: troque cm0102-1080.exe por cm0102-portatil.exe.
rem As rotinas ja reescritas em Rust (jogo\ddraw.dll) rodam no lugar das originais.
rem Para conferir contra as originais: set CM_COMPARAR=1 antes do start (diferencas em jogo\ddraw.log).
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-1080.exe" -windowed
