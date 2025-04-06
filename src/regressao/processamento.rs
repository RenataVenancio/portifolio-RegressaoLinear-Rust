// src/processamento.rs

use std::fs::File;
use std::error::Error;
use serde::{Serialize, Deserialize};
use csv::Writer;
use std::fs;
// use crate::modulo::*; // Ou use o nome correto do módulo

#[derive(Serialize, Deserialize)]
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
// Função para salvar em JSON
pub fn salvar_json(resultado: &ResultadoRegressao, caminho: &str) -> std::io::Result<()> {
    let file = File::create(caminho)?;
    serde_json::to_writer_pretty(file, resultado)?;
    Ok(())
}

// Função para salvar em CSV
pub fn salvar_csv(a: f64,b: f64,mse: f64,r2: f64,previsoes: &Vec<(f64, f64)>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data/resultado.csv")?;

    // Cabeçalho com todas as colunas
    wtr.write_record(&["a", "b", "mse", "r2", "x_previsto", "y_previsto"])?;

    // Linha com resultados com as previsões (com os valores para a,b,mse,r2, x_previsoes, y_previsoes)
    if let Some((x, y)) = previsoes.first() {
        wtr.write_record(&[
            a.to_string(),
            b.to_string(),
            mse.to_string(),
            r2.to_string(),
            x.to_string(),
            y.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}