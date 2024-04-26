// Copyright 2024 Mark C. Chu-Carroll
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::ast::{Definition, LowerName, ModulePath, Sect, TypeVarName};
use crate::errors::CompilationError;
use crate::errors::CompilationError::ModuleNotFoundError;
use crate::lex::Scanner;
use crate::parser::SectParser;

#[derive(Debug, Eq, PartialEq, Clone)]
struct InstantiatedTypeVariableName(TypeVarName, u32);

#[derive(Debug, Eq, PartialEq, Clone)]
struct InstantiatedTypeParameter<'a> {
    sect: ModulePath,
    def: &'a Definition,
    type_var: TypeVarName,
    instantiated: InstantiatedTypeVariableName
}

impl<'a> InstantiatedTypeParameter<'a> {
    fn new(sect: &ModulePath, def: &'a Definition, tv: &TypeVarName,
           instantiated: InstantiatedTypeVariableName) -> Self {
        InstantiatedTypeParameter {
            sect: sect.clone(),
            def,
            type_var: tv.clone(),
            instantiated: instantiated.clone()
        }
    }
}

#[derive(Debug)]
struct Compiler<'a> {
    path_prefixes: Vec<PathBuf>,
    sects: HashMap<ModulePath, Sect>,
    type_parameter_instantiations: Vec<InstantiatedTypeParameter<'a>>,
    indices: HashMap<TypeVarName, u32>
}

impl<'a> Compiler<'a> {
    fn new(path_prefixes: Vec<PathBuf>) -> Compiler<'a> {
        Compiler{
            path_prefixes,
            sects: HashMap::new(),
            type_parameter_instantiations: Vec::new(),
            indices: HashMap::new()
        }
    }

    fn next_index_for(&mut self, name: &TypeVarName) -> u32 {
        match self.indices.get_mut(name) {
            Some(i) => {
                *i += 1;
                *i
            }
            None => {
                self.indices.insert(name.clone(), 1);
                1
            }
        }
    }
    fn instantiate_type_param(&mut self, sect: &ModulePath, def: &'a Definition, type_var: &TypeVarName) -> InstantiatedTypeVariableName {
        let idx = self.next_index_for(&type_var);
        let new_name = InstantiatedTypeVariableName(type_var.clone(), idx);
        let inst = InstantiatedTypeParameter::new(sect, def, type_var, new_name);
        self.type_parameter_instantiations.push(inst.clone());
        inst.instantiated
    }

    fn find_file_for_sect(&self, sect_name: &ModulePath) -> Result<PathBuf, CompilationError> {
        let path_str = sect_name.segments.iter().map(LowerName::to_string).collect::<Vec<String>>().join("/");
        let file_path: &Path = Path::new(&path_str);
        if file_path.is_file() {
            return Ok(file_path.to_owned())
        } else {
            for dir in &self.path_prefixes {
                let qualified_path = dir.join(file_path);
                if qualified_path.is_file() {
                    return Ok(qualified_path)
                }
            }
            return Err(ModuleNotFoundError(sect_name.clone()))
        }
    }

    fn read_sect(&mut self, sect: &ModulePath) -> Result<(), CompilationError>{
        let path = self.find_file_for_sect(sect)?;
        let sect_text = fs::read_to_string(path)?;
        let parsed = SectParser::new().parse(
            Scanner::new(&sect_text))?;
        self.sects.insert(sect.clone(), parsed);
        Ok(())

    }
}