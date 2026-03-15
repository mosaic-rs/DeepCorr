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

pub struct MinMaxNormalizer {
    pub epsilon: f64, 
}

impl Default for MinMaxNormalizer {
    fn default() -> Self {
        Self { epsilon: 1e-10 }
    }
}

impl MinMaxNormalizer {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }

    pub fn normalize(&self, data: &Array2<f64>) -> Result<Array2<f64>, NormError> {
        if data.is_empty() {
            return Err(NormError::EmptyInput);
        }

        let min = data.fold_axis(Axis(0), f64::INFINITY, |&a, &b| f64::min(a, b));
        let max = data.fold_axis(Axis(0), f64::NEG_INFINITY, |&a, &b| f64::max(a, b));

        let mut normalized = data.clone();

        for (i, mut col) in normalized.axis_iter_mut(Axis(1)).enumerate() {
            let col_min = min[i];
            let col_max = max[i];
            let range = col_max - col_min;

            col.mapv_inplace(|x| (x - col_min) / (range + self.epsilon));
        }

        Ok(normalized)
    }
}