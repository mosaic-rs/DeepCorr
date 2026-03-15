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

pub mod cosine;
pub mod z_score;
pub mod min_max;

// External
use thiserror::Error;
use ndarray::Array2;
use crate::cosine::CosineNormalizer;
use crate::z_score::ZScoreNormalizer;
use crate::min_max::MinMaxNormalizer;

#[derive(Error, Debug)]
pub enum NormError {
    #[error("Input data is empty")]
    EmptyInput,
    #[error("Zero-magnitude vector found at row {0}")]
    ZeroMagnitude(usize),
}

pub enum NormMethod {
    Cosine,
    ZScore,
    MinMaxScore,
}

pub fn normalize_data(
    data: &Array2<f64>, 
    method: NormMethod, 
    epsilon: f64
) -> Result<Array2<f64>, NormError> {
    match method {
        NormMethod::Cosine => {
            let n = CosineNormalizer::new(epsilon);
            n.normalize(data)
        },
        NormMethod::ZScore => {
            let n = ZScoreNormalizer::new(epsilon);
            n.normalize(data)
        }
        NormMethod::MinMaxScore => {
            let n = MinMaxNormalizer::new(epsilon);
            n.normalize(data)
        }
    }
}