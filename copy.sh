#!/bin/bash

# Define o nome do arquivo de saída
OUTPUT="copy.md"

# Cria (ou sobrescreve) o arquivo vazio
> "$OUTPUT"

# 1. Cola o README.md inicial
if [ -f "README.md" ]; then
    echo "========================================" >> "$OUTPUT"
    echo "README.md" >> "$OUTPUT"
    echo "========================================" >> "$OUTPUT"
    cat "README.md" >> "$OUTPUT"
    echo -e "\n\n" >> "$OUTPUT"
else
    echo "Aviso: README.md não encontrado." >> "$OUTPUT"
    echo -e "\n" >> "$OUTPUT"
fi

if [ -f "Cargo.toml" ]; then
    echo "========================================" >> "$OUTPUT"
    echo "Cargo.toml" >> "$OUTPUT"
    echo "========================================" >> "$OUTPUT"
    cat "Cargo.toml" >> "$OUTPUT"
    echo -e "\n\n" >> "$OUTPUT"
else
    echo "Aviso: Cargo.toml não encontrado." >> "$OUTPUT"
    echo -e "\n" >> "$OUTPUT"
fi

# 2. Cola a estrutura de diretórios (tree -I target)
echo "========================================" >> "$OUTPUT"
echo "CÓDIGO DO PROJETO" >> "$OUTPUT"
echo "========================================" >> "$OUTPUT"
# Verifica se o comando tree está instalado
if command -v tree &> /dev/null; then
    tree -I target >> "$OUTPUT"
else
    echo "Aviso: Comando 'tree' não encontrado. Usando lista simples:" >> "$OUTPUT"
    # Alternativa simples caso o tree não exista no bash do usuário
    find . -not -path "*/target*" -not -path "*/\.git*" | sort >> "$OUTPUT"
fi
echo -e "\n\n" >> "$OUTPUT"

# 3. Indica que o código começa agora e lê o diretório src/
echo "========================================" >> "$OUTPUT"
echo "CÓDIGO FONTE (Diretório src/)" >> "$OUTPUT"
echo "========================================" >> "$OUTPUT"
echo -e "\n" >> "$OUTPUT"

if [ -d "src" ]; then
    # O comando find lista todos os arquivos, incluindo subpastas dentro de src/
    find src -type f | while read -r arquivo; do
        echo "--- Caminho do Arquivo: $arquivo ---" >> "$OUTPUT"
        cat "$arquivo" >> "$OUTPUT"
        echo -e "\n\n" >> "$OUTPUT"
    done
else
    echo "Aviso: Diretório 'src' não encontrado." >> "$OUTPUT"
fi

echo "Sucesso! Todo o texto (README, Árvore e Código) foi salvo em '$OUTPUT'."