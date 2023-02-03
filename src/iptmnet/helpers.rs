// iptmnet A CLI interface to the IPTMNet Rest API
// Copyright (C) 2022  Ali Sajid Imami
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum ItemType {
    All,
    #[clap(name = "uniprot-id")]
    UniProtID,
    ProteinGeneName,
    PMID,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum PtmType {
    Acetylation,
    CGlycosylation,
    NGlycosylation,
    OGlycosylation,
    SGlycosylation,
    Methylation,
    Myristoylation,
    SNitrosylation,
    Phosphorylation,
    Sumoylation,
    Ubiquitination,
}

impl fmt::Display for PtmType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PtmType::CGlycosylation => write!(f, "C-Glycosylation"),
            PtmType::NGlycosylation => write!(f, "N-Glycosylation"),
            PtmType::SGlycosylation => write!(f, "S-Glycosylation"),
            PtmType::OGlycosylation => write!(f, "O-Glycosylation"),
            PtmType::SNitrosylation => write!(f, "S-Nitrosylation"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum Role {
    Enzyme,
    Substrate,
    Both,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Both => write!(f, "Enzyme or Substrate"),
            _ => write!(f, "{self:?}"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SearchParameters {
    search_term: String,
    term_type: ItemType,
    role: Role,
    ptm_type: Option<PtmType>,
    organism: Option<String>,
}

impl SearchParameters {
    pub fn new(
        search_term: String,
        term_type: ItemType,
        role: Role,
        ptm_type: Option<PtmType>,
        organism: Option<String>,
    ) -> Self {
        Self {
            search_term,
            term_type,
            role,
            ptm_type,
            organism,
        }
    }
}

impl Default for SearchParameters {
    fn default() -> Self {
        Self {
            search_term: String::new(),
            term_type: ItemType::All,
            role: Role::Both,
            ptm_type: None,
            organism: None,
        }
    }
}
