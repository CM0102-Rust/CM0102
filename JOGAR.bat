@echo off
rem Abre o CM 01/02 em 1920x1080 (nativo), em janela redimensionavel, sem CD e sem trocar a resolucao do Windows.
rem Versao 800x600: troque cm0102-1080.exe por cm0102-portatil.exe.
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-1080.exe" -windowed
