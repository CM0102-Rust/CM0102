@echo off
rem Abre o CM 01/02 em 1920x1080 (nativo), em janela redimensionavel, sem CD e sem trocar a resolucao do Windows.
rem Versao 800x600: troque cm0102-1080.exe por cm0102-portatil.exe.
rem CM_COMPARAR=1: as rotinas ja reescritas em Rust rodam lado a lado com as originais e as
rem diferencas vao para jogo\ddraw.log (o jogo segue com o resultado da original).
set CM_COMPARAR=1
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-1080.exe" -windowed
