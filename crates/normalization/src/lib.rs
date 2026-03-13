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

// DeepCorr Plugins
pub mod cosine;

// External
use thiserror::Error

#[derive(Error, Debug)]
pub enum NormError {
    #[error("Input data is empty")]
    EmptyInput,
    #[error("Zero-magnitude vector found at row {0}")]
    ZeroMagnitude(usize),
}

pub use crate::cosine::normalize as cosine_normalize;