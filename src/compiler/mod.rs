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

use crate::ast::{Definition, ModulePath, Sect, TypeVarName};
use crate::errors::CompilationError;
use crate::errors::CompilationError::ModuleNotFoundError;
use crate::lex::Scanner;
use crate::parser::SectParser;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, PartialEq, Clone)]
struct InstantiatedTypeVariableName(TypeVarName, u32);

#[derive(Debug, Eq, PartialEq, Clone)]
struct InstantiatedTypeParameter {
    sect: ModulePath,
    def: Definition,
    type_var: TypeVarName,
    instantiated: InstantiatedTypeVariableName,
}

impl InstantiatedTypeParameter {
    fn new(
        sect: ModulePath,
        def: Definition,
        tv: TypeVarName,
        instantiated: InstantiatedTypeVariableName,
    ) -> Self {
        InstantiatedTypeParameter {
            sect: sect.clone(),
            def,
            type_var: tv.clone(),
            instantiated: instantiated.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
    path_prefixes: Vec<PathBuf>,
    sects: HashMap<ModulePath, Sect>,
    texts: HashMap<ModulePath, String>,
    paths: Vec<String>,
    type_parameter_instantiations: Vec<InstantiatedTypeParameter>,
    indices: HashMap<TypeVarName, u32>,
    queue: VecDeque<ModulePath>,
}

impl Compiler {
    pub fn new(path_prefixes: Vec<PathBuf>) -> Compiler {
        Compiler {
            path_prefixes,
            paths: Vec::new(),
            sects: HashMap::new(),
            type_parameter_instantiations: Vec::new(),
            indices: HashMap::new(),
            queue: VecDeque::new(),
            texts: HashMap::new()
        }
    }

    fn instantiate_type_param(
        & mut self,
        sect: ModulePath,
        def: Definition,
        type_var: TypeVarName,
    ) -> InstantiatedTypeVariableName {
        let idx = match self.indices.get_mut(&type_var) {
            Some(i) => {
                *i += 1;
                *i
            }
            None => {
                self.indices.insert(type_var.clone(), 1);
                1
            }
        };
        let new_name = InstantiatedTypeVariableName(type_var.clone(), idx);
        let inst = InstantiatedTypeParameter::new(sect, def, type_var, new_name);
        self.type_parameter_instantiations.push(inst.clone());

        inst.instantiated
    }

    fn find_file_for_sect(sect_name: ModulePath,
                          path_prefixes: &Vec<PathBuf>) -> Result<PathBuf, CompilationError> {
        let path_str = sect_name.to_file_syntax()
            + ".schism";
        let file_path: &Path = Path::new(&path_str);
        return if file_path.is_file() {
            Ok(file_path.to_owned())
        } else {
            for dir in path_prefixes {
                let qualified_path = dir.join(file_path);
                if qualified_path.is_file() {
                    return Ok(qualified_path);
                }
            }
            Err(ModuleNotFoundError(sect_name.clone()))
        }
    }

    fn read_sect(&mut self, sect: ModulePath) -> Result<(), CompilationError> {
        let path = Self::find_file_for_sect(sect.clone(),
                                            &self.path_prefixes)?;
        let path_str = path.to_str().unwrap().to_string();
        self.paths.push(path_str.clone());
        let idx = self.paths.len() - 1;
        let sect_text = fs::read_to_string(path.clone())?;
        self.texts.insert(sect.clone(), sect_text);
        let saved_text = self.texts.get(&sect).unwrap();
        let parsed = SectParser::new().parse(Scanner::new(idx, &saved_text))?;
        self.sects.insert(sect.clone(), parsed.clone());
        for use_def in &parsed.uses {
            self.queue.push_back(use_def.sect.clone())
        }
        Ok(())
    }

    fn path_to_sect(path: String) -> ModulePath {
        let without_suffix= if path.ends_with(".schism") {
            let s_pos = path.len() - 7;
            path[..s_pos].to_string()
        } else {
            path
        };

        ModulePath(without_suffix)

    }

    pub fn compile(&mut self, sources: Vec<String>) -> Result<(), CompilationError> {
        for source in sources {
            self.queue.push_back(Self::path_to_sect(source).clone());
        }
        while !self.queue.is_empty() {
            let next_module = self.queue.pop_front();
            if next_module.is_some() {
                self.read_sect(next_module.unwrap())?;
            }
        }
        Ok(())
    }
}
