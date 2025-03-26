# 📊 Portfólio - Regressão Linear em Rust

Este projeto implementa uma **Regressão Linear** em **Rust** para análise de séries temporais, sem utilizar bibliotecas externas para cálculos matemáticos.

## 📌 Funcionalidades
- 📥 **Importação de dados** (manual por enquanto).
- 📊 **Cálculo de estatísticas**: média, variância e covariância.
- 📈 **Implementação da regressão linear** para prever valores futuros.
- ✅ **Métricas de avaliação**: Erro Quadrático Médio (MSE) e Coeficiente de Determinação (R²).
- 🔍 **Testes unitários** para validar os cálculos.

## 🚀 Como Executar o Projeto

1. **Clone o repositório:** (se estiver no GitHub):
   ```sh
   git clone https://github.com/seu-usuario/portifolio-RegressaoLinear-Rust.git
   cd portifolio-RegressaoLinear-Rust

2. **Compile e execute o código:**
   Abra o PowerShell dentro na pasta onde está o projeto e execute o seguinte comando:
   **cargo run**

3. **Rodar os testes unitários:**
   Para executar os testes que tem nesse projeto com o Powershell aberto execute o comando abaixo:
   **cargo test**

4. **🧮 Como Funciona a Regressão Linear?**
   A regressão linear encontra a melhor reta para prever valores, seguindo a fórmula:
   **Y=aX+b**
   Os coeficientes são calculados com:
   a= 
Var(X)
Cov(X,Y)
  
5. **📌 Fórmulas Implementadas**
   ✅ Média
   ✅ Variância
   ✅ Covariância
   ✅ Coeficientes da regressão (a e b)
   ✅ Previsões futuras
   ✅ Erro Quadrático Médio (MSE)
   ✅ Coeficiente de Determinação (R²)

6. **Estrutura do Código**
   portifolio-RegressaoLinear-Rust/
   |-- src/
   |   |-- main.rs        # Arquivo principal
   |   |-- regressao      # Módulo para cálculos matemáticos
   |       |--mod.rs      # Implementação da regressão linear
   |-- cargo.toml         # Configuração do projeto Rust
   |-- README.md          # Documentação do projeto