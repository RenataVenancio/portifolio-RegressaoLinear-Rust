// tests/calculos_test.rs

use regressao_linear::regressao::{calcular_media, calcular_variancia, calcular_coeficientes_regressao, prever_valor, calcular_mse, calcular_r2}; 

#[test]
fn test_calcular_media() {
    let dados = vec![1.0, 2.0, 3.0];
    assert_eq!(calcular_media(&dados), 2.0);
}

#[test]
fn test_calcular_variancia() {
    let dados = vec![1.0, 2.0, 3.0];
    assert_eq!(calcular_variancia(&dados), 2.0 / 3.0);
}

#[test]
fn test_calcular_coeficientes_regressao() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 4.0, 6.0];
    let (a, b) = calcular_coeficientes_regressao(&x, &y);
    // Agora, a = intercepto, b = coef angular
    assert_eq!(a, 0.0); // Intercepto
    assert_eq!(b, 2.0); // Coeficiente angular
}

#[test]
fn test_prever_valor() {
    let x = 60.0;
    let a = 5.0; // Intercepto
    let b = 1.0; // Coeficiente angular

    assert_eq!(prever_valor(x, a, b), 65.0); // 5 + 1*60
}

#[test]
fn test_calcular_mse() {
    let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let y_real = vec![15.0, 25.0, 35.0, 45.0, 55.0];
    let (a, b) = calcular_coeficientes_regressao(&x, &y_real);

    // Erro quadrático médio deve ser zero, pois é uma linha perfeita
    assert_eq!(calcular_mse(&x, &y_real, a, b), 0.0);
}

#[test]
fn test_calcular_r2() {
    let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let y_real = vec![15.0, 25.0, 35.0, 45.0, 55.0];
    let (a, b) = calcular_coeficientes_regressao(&x, &y_real);

    // R² deve ser 1.0 pois é uma regressão perfeita
    assert_eq!(calcular_r2(&x, &y_real, a, b), 1.0);
}
