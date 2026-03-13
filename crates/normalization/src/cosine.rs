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

use ndarray::{Array2, Axis};
use crate::NormError;

pub struct CosineNormalizer {
    pub epsilon: f64, 
}

impl Default for CosineNormalizer {
    fn default() -> Self {
        Self { epsilon: 1e-10 }
    }
}

impl CosineNormalizer {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }

    pub fn normalize(&self, data: &Array2<f64>) -> Result<Array2<f64>, NormError> {
        if data.is_empty() {
            return Err(NormError::EmptyInput);
        }

        let mut normalized = data.clone();

        for (idx, mut row) in normalized.axis_iter_mut(Axis(0)).enumerate() {
            let norm = row.mapv(|x| x.powi(2)).sum().sqrt();

            if norm < self.epsilon {
                return Err(NormError::ZeroMagnitude(idx));
            }

            row.mapv_inplace(|x| x / norm);
        }

        Ok(normalized)
    }
}