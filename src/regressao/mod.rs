pub mod processamento;

pub fn calcular_media(valores: &[f64]) -> f64 {
    
    let soma: f64 = valores.iter().sum();
    let n = valores.len() as f64;

    if n == 0.0 {
        return 0.0; // Para evitar o erro de divisão por zero
    }
    soma / n
}

pub fn calcular_variancia(valores: &[f64]) -> f64 {
    
    let n = valores.len() as f64;
    if n == 0.0{
        return 0.0; // Para evitar o erro de divisão por zero
    }

    let media = calcular_media(valores);
    let soma_diferencas: f64 = valores.iter().map(|&x| (x - media).powi(2)).sum();

    soma_diferencas / n
}

pub fn calcular_covariancia(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();

    if n == 0 || n != y.len() {
        return 0.0; // Para evitar o erro de divisão por zero
    }

    let media_x = calcular_media(x);
    let media_y = calcular_media(y);

    let soma_produtos: f64 = x.iter()
        .zip(y.iter()) // Juntar os valores de X e Y
        .map(|(&xi, &yi)| (xi - media_x) * (yi - media_y))
        .sum();

    soma_produtos / n as f64
}

pub fn calcular_coeficientes_regressao(x: &[f64], y: &[f64]) -> (f64, f64) {
    
    let variancia_x = calcular_variancia(x);
    let covariancia_xy = calcular_covariancia(x, y);

    if variancia_x == 0.0 {
        return (0.0, 0.0); // Evita divisão por zero
    }

    let coeficiente = covariancia_xy / variancia_x;
    let intercepto = calcular_media(y) - coeficiente * calcular_media(x);

    (intercepto, coeficiente)
}

pub fn prever_valor(x: f64, a: f64, b: f64) -> f64 {
    a + b * x
}

pub fn calcular_mse(x: &[f64], y_real: &[f64], a: f64, b: f64) -> f64 {
    let n = x.len();

    if n == 0 || n != y_real.len() {
        return 0.0; // Para evitar o erro de divisão por zero
    }

    let soma_erros: f64 = x.iter()
        .zip(y_real.iter())
        .map(|(&xi, &yi_real)| {
            let y_previsto = a + b * xi; 
            (yi_real - y_previsto).powi(2)
        })
        .sum();

    soma_erros / n as f64
}

pub fn calcular_r2(x: &[f64], y_real: &[f64], a: f64, b: f64) -> f64 {
    let n = x.len();

    if n == 0 || n != y_real.len() {
        return 0.0;
    }

    let media_y = calcular_media(y_real);

    let soma_residuos: f64 = x.iter()
        .zip(y_real.iter())
        .map(|(&xi, &yi_real)| {
            let y_previsto = a + b * xi; 
            (yi_real - y_previsto).powi(2)
        })
        .sum();

    let soma_total: f64 = y_real.iter()
        .map(|&yi_real| (yi_real - media_y).powi(2))
        .sum();

    if soma_total == 0.0 {
        return 0.0;
    }

    1.0 - (soma_residuos / soma_total)
}
