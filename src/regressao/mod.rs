use std::error::Error;
use std::fs::File;
use std::fs;
use serde::{Serialize, Deserialize};

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

    let a = covariancia_xy / variancia_x;
    let b = calcular_media(y) - a * calcular_media(x);

    (a, b)
}

pub fn prever_valor(x: f64, a: f64, b: f64) -> f64 {
    a * x + b
}

pub fn calcular_mse(x: &[f64], y_real: &[f64], a: f64, b: f64) -> f64 {
    let n = x.len();

    if n == 0 || n != y_real.len() {
        return 0.0; // Para evitar o erro de divisão por zero
    }

    let soma_erros: f64 = x.iter()
        .zip(y_real.iter()) // Junta os valores de X e Y_real
        .map(|(&xi, &yi_real)| {
            let y_previsto = a * xi + b;
            (yi_real - y_previsto).powi(2) // Calcula o erro ao quadrado
        })
        .sum();

    soma_erros / n as f64
}

pub fn calcular_r2(x: &[f64], y_real: &[f64], a: f64, b: f64) -> f64 {
    let n = x.len();

    if n == 0 || n != y_real.len() {
        return 0.0 // Para evitar o erro de divisão por zero
    }

    let media_y = calcular_media(y_real);

    let soma_residuos: f64 = x.iter()
        .zip(y_real.iter())
        .map(|(&xi, &yi_real)| {
            let y_previsto = a * xi + b;
            (yi_real - y_previsto).powi(2)
        })
        .sum();

    let soma_total: f64 = y_real.iter()
        .map(|&yi_real| (yi_real - media_y).powi(2))
        .sum();

    if soma_total == 0.0{
        return 0.0; // Para evitar o erro de divisão por zero
    }

    1.0 - (soma_residuos / soma_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_media() {
        let dados = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        
        assert_eq!(calcular_media(&dados), 30.0);
    }

    #[test]
    fn test_calcular_variancia() {
        let dados = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        
        assert_eq!(calcular_variancia(&dados), 200.0);
    }

    #[test]
    fn test_calcular_coeficientes_regressao() {
        let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let y = vec![15.0, 25.0, 35.0, 45.0, 55.0];
        let (a, b) = calcular_coeficientes_regressao(&x, &y);
        
        assert_eq!(a, 1.0);
        assert_eq!(b, 5.0);
    }

    #[test]
    fn test_prever_valor() {
        let x = 60.0;
        let a = 1.0;
        let b = 5.0;

        assert_eq!(prever_valor(x, a, b), 65.0);
    }
    
    #[test]
    fn test_calcular_mse() {
        let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let y_real = vec![15.0, 25.0, 35.0, 45.0, 55.0];
        let (a, b) = calcular_coeficientes_regressao(&x, &y_real);

        assert_eq!(calcular_mse(&x, &y_real, a, b), 0.0);
    }

    #[test]
    fn test_calcular_r2() {
        let x = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let y_real = vec![15.0, 25.0, 35.0, 45.0, 55.0];
        let (a, b) = calcular_coeficientes_regressao(&x, &y_real);

        assert_eq!(calcular_r2(&x, &y_real, a, b), 1.0);
    }
    
}

#[derive(Debug)]
pub struct DadosRegressao {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

pub fn ler_csv(caminho: &str) -> Result<DadosRegressao, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(caminho)
        .map_err(|_| format!("Erro ao abrir o arquivo CSV: {}", caminho))?;

    let mut x = Vec::new();
    let mut y = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|_| "Erro ao ler uma linha do CSV")?;
        if let (Some(valor_x), Some(valor_y)) = (record.get(0), record.get(1)) {
            let x_val: f64 = valor_x.parse().map_err(|_| "Erro ao converter X para número")?;
            let y_val: f64 = valor_y.parse().map_err(|_| "Erro ao converter Y para número")?;
            x.push(x_val);
            y.push(y_val);
        } else {
            return Err("Formato do CSV inválido!".into());
        }
    }

    Ok(DadosRegressao { x, y })
}


#[derive(Debug, Deserialize)]
struct Registro {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ResultadoRegressao {
    pub a: f64,
    pub b: f64,
    pub mse: f64,
    pub r2: f64,
    pub previsoes: Vec<(f64, f64)>,
}


pub fn ler_json(caminho: &str) -> Result<DadosRegressao, Box<dyn Error>> {
    let conteudo = fs::read_to_string(caminho)?; // Lê o arquivo JSON como String
    let registros: Vec<Registro> = serde_json::from_str(&conteudo)?; // Converte a String para uma estrutura Rust

    let mut x = Vec::new();
    let mut y = Vec::new();

    for registro in registros {
        x.push(registro.x);
        y.push(registro.y);
    }

    Ok(DadosRegressao { x, y })
}

// Função para salvar em CSV
pub fn salvar_csv(resultado: &ResultadoRegressao, caminho: &str) -> std::io::Result<()> {
    let mut wtr = csv::Writer::from_path(caminho)?;
    wtr.write_record(&["Coeficiente Angular (a)", "Intercepto (b)", "MSE", "R²"])?;
    wtr.write_record(&[
        resultado.a.to_string(),
        resultado.b.to_string(),
        resultado.mse.to_string(),
        resultado.r2.to_string(),
    ])?;
    wtr.write_record(&["X", "Y Previsto"])?;

    for (x, y) in &resultado.previsoes {
        wtr.write_record(&[x.to_string(), y.to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}

// Função para salvar em JSON
pub fn salvar_json(resultado: &ResultadoRegressao, caminho: &str) -> std::io::Result<()> {
    let file = File::create(caminho)?;
    serde_json::to_writer_pretty(file, resultado)?;
    Ok(())
}