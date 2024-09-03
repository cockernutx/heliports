extern crate reqwest;
use std::io::Cursor;

use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelipointInfo {
    #[serde(rename = "CódigoOACI")]
    pub cdigo_oaci: String,
    #[serde(rename = "CIAD")]
    pub ciad: String,
    #[serde(rename = "Nome")]
    pub nome: String,
    #[serde(rename = "TIPOUSO")]
    pub tipouso: String,
    #[serde(rename = "Município")]
    pub municpio: String,
    #[serde(rename = "UF")]
    pub uf: String,
    #[serde(rename = "Tipo")]
    pub tipo: String,
    #[serde(rename = "LATGEOPOINT")]
    pub latgeopoint: String,
    #[serde(rename = "LONGEOPOINT")]
    pub longeopoint: String,
    #[serde(rename = "Latitude")]
    pub latitude: String,
    #[serde(rename = "Longitude")]
    pub longitude: String,
    #[serde(rename = "Altitude")]
    pub altitude: String,
    #[serde(rename = "OperaçãoDiurna")]
    pub operao_diurna: String,
    #[serde(rename = "OperaçãoNoturna")]
    pub operao_noturna: String,
    #[serde(rename = "RampadeAproximação")]
    pub rampade_aproximao: String,
    #[serde(rename = "FormatodaÁreadePouso")]
    pub formatodareade_pouso: String,
    #[serde(rename = "Dimensões")]
    pub dimenses: String,
    #[serde(rename = "ResistênciadoPavimento")]
    pub resistnciado_pavimento: f64,
    #[serde(rename = "Superfície")]
    pub superfcie: String,
    #[serde(rename = "PortariadeRegistro")]
    pub portariade_registro: String,
    #[serde(rename = "LinkPortaria")]
    pub link_portaria: String,
    #[serde(rename = "Luzes_de_Aproximacao")]
    pub luzes_de_aproximacao: Option<String>,
    #[serde(rename = "Luzes_de_Area_de_Toque_Elevacao_Inicial_TLOF")]
    pub luzes_de_area_de_toque_elevacao_inicial_tlof: Option<String>,
    #[serde(rename = "Indicador_Visual_de_Rampa_de_Aproximacao")]
    pub indicador_visual_de_rampa_de_aproximacao: Option<String>,
    #[serde(rename = "Luzes_Indicadoras_de_Area_de_Aproximacao_Final_de_Decolagem_FATO")]
    pub luzes_indicadoras_de_area_de_aproximacao_final_de_decolagem_fato: Option<String>,
    #[serde(rename = "Luzes_de_Obstaculo")]
    pub luzes_de_obstaculo: Option<String>,
    #[serde(rename = "Luzes_de_Ponto_de_Visada_de_Heliponto")]
    pub luzes_de_ponto_de_visada_de_heliponto: Option<String>,
}

#[tokio::main]
async fn main() {
    // Get do json com as informacoes
    let request_res = match reqwest::get("https://sistemas.anac.gov.br/dadosabertos/Aerodromos/Aer%C3%B3dromos%20Privados/Lista%20de%20aer%C3%B3dromos%20privados/Heliponto/Helipontos.json").await {
        Ok(r) => r,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    // Extracao do bodhy da resposta
    let body_text = match request_res.text().await {
        Ok(b) => b,
        Err(why) => {
            println!("{}", why);
            return
        }
    };

    // Deserialize json
    let res: Vec<HelipointInfo> = match serde_json::from_str(&body_text) {
        Ok(r) => r,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };

    // Filtro da uf
    let res: Vec<HelipointInfo> = res
        .iter()
        .filter(|o| o.uf == "Distrito Federal".to_string())
        .cloned()
        .collect();

    // Serializacao do json
    let json = match serde_json::to_string(&res) {
        Ok(j) => j,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };

    // Criar dataframe
    let cursor = Cursor::new(json);
    let df = match JsonReader::new(cursor).finish() {
        Ok(d) => d,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    println!("{:?}", df);
}
