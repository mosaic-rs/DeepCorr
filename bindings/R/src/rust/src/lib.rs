/*
This file is part of DeepCorr.

DeepCorr is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

DeepCorr is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
DeepCorr. If not, see <https://www.gnu.org/licenses/>.
*/


use extendr_api::prelude::*;
use deepcorr_normalization::{normalize_data, NormMethod};
use ndarray::Array2;

#[extendr]
fn hello() -> String {
    format!("HELLOO!!!!!")
}

#[extendr]
fn echo(argument: String) -> String {
    format!("{}", argument)
}

#[extendr]
fn add(x: u32, y: u32) -> u32 {
    let z: u32 = x + y;
    z
}

#[extendr]
fn subtract(x: u32, y: u32) -> u32 {
    let z: u32 = x - y;
    z
}

#[extendr]
fn multiply(x: u32, y: u32) -> u32 {
    let z: u32 = x * y;
    z
}

#[extendr]
fn divide(x: u32, y: u32) -> u32 {
    let z: u32 = x / y;
    z
}

#[extendr]
fn exponent(x: u32, y: u32) -> u32 {
    let z: u32 = x.pow(y);
    z
}

#[extendr]
fn normalize(
    data: Robj, 
    method: String, 
    #[default = "NULL"] epsilon: Option<f64> 
) -> extendr_api::Result<Robj> {
    let eps = epsilon.unwrap_or(1e-6);

    let matrix_view: ndarray::ArrayView2<f64> = (&data).try_into()
        .map_err(|_| Error::Other(String::from("Input must be a valid numeric matrix")))?;
    
    let matrix_data: Array2<f64> = matrix_view.to_owned();
    
    let norm_method = match method.to_lowercase().as_str() {
        "cosine" => NormMethod::Cosine,
        "zscore" => NormMethod::ZScore,
        "minmax" => NormMethod::MinMaxScore,
        _ => return Err(Error::Other(String::from("Invalid method"))),
    };

    match normalize_data(&matrix_data, norm_method, eps) {
        Ok(result) => {
            let out_robj: Robj = result.try_into()
                .map_err(|_| Error::Other(String::from("Conversion failed")))?;
            Ok(out_robj)
        },
        Err(e) => Err(Error::Other(format!("Normalization error: {}", e))),
    }
}

extendr_module! {
    mod deepcorr;
    fn hello;
    fn echo;
    fn add;
    fn subtract;
    fn multiply;
    fn divide;
    fn exponent;
    fn normalize;
}