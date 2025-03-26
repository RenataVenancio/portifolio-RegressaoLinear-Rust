mod regressao;

fn main() {
    // // println!("Projeto de Regressão Linear em Rust!");
    // // regressao::calcular_regressao();
    
    // let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    // let y_real = vec![15.0, 25.0, 35.0, 45.0, 55.0];

    // // let media_x = regressao::calcular_media(&x);
    // // let media_y = regressao::calcular_media(&y);
    // // let variancia_x = regressao::calcular_variancia(&x);
    // // let variancia_y = regressao::calcular_variancia(&y);
    // // let covariancia_xy = regressao::calcular_covariancia(&x, &y);

    // let (a, b) = regressao::calcular_coeficientes_regressao(&x, &y_real);
    // let mse = regressao::calcular_mse(&x, &y_real, a, b);
    // let r2 = regressao::calcular_r2(&x, &y_real, a, b);

    // // Valor de X para prever Y
    // let novo_x = 60.0;
    // let previsao = regressao::prever_valor(novo_x, a, b);


    // // println!("Média de X: {:.2}", media_x);
    // // println!("Média de Y: {:.2}", media_y);
    // // println!("Variância de X: {:.2}", variancia_x);
    // // println!("Variância de y: {:.2}", variancia_y);
    // // println!("Covariância entre X e Y: {:.2}", covariancia_xy);

    // // println!("Coeficiente angular (a): {:.2}", a);
    // // println!("Intercepto (b): {:.2}", b);
    // // println!("Equação da reta: Y = {:.2}X + {:.2}", a, b);
    // println!("Para X = {:.2}, previsão de Y = {:.2}", novo_x, previsao);

    // println!("Equação da reta: Y = {:.2}X + {:.2}", a, b);
    // println!("Erro Quadrático Médio (MSE): {:.2}", mse);
    // println!("Coeficiente de Determinação (R²): {:.2}", r2);

    // let caminho_csv = "data/dados.csv";

    // match regressao::ler_csv(caminho_csv) {
    //     Ok(dados) => {
    //         println!("Dados carregados com sucesso!");
    //         println!("X: {:?}", dados.x);
    //         println!("Y: {:?}", dados.y);
    //     }
    //     Err(e) => {
    //         println!("Erro ao carregar os dados: {}", e);
    //     }
    // }

    let caminho_json = "data/dados.json";

    match regressao::ler_json(caminho_json) {
        Ok(dados) => {
            println!("Dados carregados com sucesso do JSON!");
            println!("X: {:?}", dados.x);
            println!("Y: {:?}", dados.y);
        }
        Err(e) => {
            println!("Erro ao  carregar os dados do JSON: {}", e);
        }
    }

}
