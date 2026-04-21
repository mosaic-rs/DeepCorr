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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TensorError {
    #[error("Input data is empty")]
    EmptyInput,
}

impl TensorError {
    pub fn code(&self) -> &str {
        match self {
            NormError::EmptyInput => "T101",
        }
    }

    pub fn wiki_url(&self) -> String {
        format!("https://github.com/mosaic-rs/deepcorr/wiki/Errors#{}", self.code())
    }

    pub fn formatted_message(&self) -> String {
        format!(
            "[{}] {}\nWiki: {}",
            self.code(),
            self,
            self.wiki_url()
        )
    }
}