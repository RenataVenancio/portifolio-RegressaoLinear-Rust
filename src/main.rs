use std::io;
mod regressao;

// // mod modulo;
// mod processamento;

use regressao::processamento::{ler_csv, ler_json, salvar_csv, salvar_json, ResultadoRegressao};

fn main() {

    // println!("Digite o nome do arquivo CSV ou JSON para carregar os dados:");
    
    // let mut nome_arquivo = String::new();
    // io::stdin().read_line(&mut nome_arquivo).expect("Erro ao ler entrada");
    // let nome_arquivo = nome_arquivo.trim();

    // let dados = if nome_arquivo.ends_with(".csv") {
    //     regressao::ler_csv(nome_arquivo)
    // } else if nome_arquivo.ends_with(".json") {
    //     regressao::ler_json(nome_arquivo)
    // } else {
    //     println!("Formato inválido! Use CSV ou JSON.");
    //     return;
    // };

    println!("Escolha o formato do arquivo de dados:");
    println!("1 - CSV");
    println!("2 - JSON");

    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
    let escolha = escolha.trim();

    let caminho_arquivo = if escolha == "1" {
        "data/dados.csv"
    } else if escolha == "2" {
        "data/dados.json"
    } else {
        println!("Opção inválida! Saindo...");
        return;
    };

    let dados = if escolha == "1" {
        ler_csv(caminho_arquivo)
    } else {
        ler_json(caminho_arquivo)
    };

    match dados {
        Ok(dados) => {
            println!("Dados carregados com sucesso!");

            let (a, b) = regressao::calcular_coeficientes_regressao(&dados.x, &dados.y);
            println!("Equação da reta: Y = {:.2}X + {:.2}", a, b);

            let mse = regressao::calcular_mse(&dados.x, &dados.y, a, b);
            let r2 = regressao::calcular_r2(&dados.x, &dados.y, a, b);
            println!("MSE: {:.4}", mse);
            println!("R²: {:.4}", r2);

            // Permitir previsões
            println!("Deseja prever um valor de Y com base em um X fornecido? (s/n)");
            let mut resposta = String::new();
            io::stdin().read_line(&mut resposta).expect("Erro ao ler entrada");
                    
            let mut previsoes = Vec::new();
                    
            if resposta.trim().to_lowercase() == "s" {
                println!("Digite o valor de X:");
                let mut valor_x = String::new();
                io::stdin().read_line(&mut valor_x).expect("Erro ao ler entrada");
            
                match valor_x.trim().parse::<f64>() {
                    Ok(x) => {
                        let y_previsto = regressao::prever_valor(x, a, b);
                        println!("Para X = {:.2}, o valor previsto de Y é {:.2}", x, y_previsto);
                        previsoes.push((x, y_previsto));
                    }
                    Err(_) => println!("Valor inválido! Certifique-se de inserir um número."),
                }
            }


            let resultado = ResultadoRegressao { a, b, mse, r2, previsoes };

            // Perguntar formato de saída e salvar na pasta `data/`
            println!("Escolha o formato para salvar os resultados:");
            println!("1 - CSV");
            println!("2 - JSON");

            let mut formato_saida = String::new();
            io::stdin().read_line(&mut formato_saida).expect("Erro ao ler entrada");
            let formato_saida = formato_saida.trim();

            let caminho_saida = if formato_saida == "1" {
                "data/resultado.csv"
            } else if formato_saida == "2" {
                "data/resultado.json"
            } else {
                println!("Opção inválida! Resultados não foram salvos.");
                return;
            };

            if formato_saida == "1" {
                salvar_csv(resultado.a, resultado.b, resultado.mse, resultado.r2, &resultado.previsoes).expect("Erro ao salvar CSV");
                println!("Resultados salvos em '{}'", caminho_saida);
            } else {
                salvar_json(&resultado, caminho_saida).expect("Erro ao salvar JSON");
                println!("Resultados salvos em '{}'", caminho_saida);
            }
        }
        Err(e) => println!("Erro ao carregar os dados: {}", e),
    }

}
