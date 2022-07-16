/*
 * Copyright (c) 2022, Florian Büstgens
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::collections::HashMap;

use super::{file::space::Space, err::FileError, tree::Node};

pub trait TimeShift {
    fn snapshot(&self, name: &str) -> Snapshot {
        todo!();
    }

    fn rollback(&self, snapshot: Snapshot) {
        todo!();
    }
}

pub struct Volume<'a> {
    name: String,
    path: String,
    spaces: HashMap<String, Space<'a>>
}

pub struct Snapshot {}

impl<'a> Volume<'a> {
    pub fn new(name: &str, path: &str) -> Self {
	Volume::create_vdir(path, name);
	
        Self {
            name: name.to_string(),
            path: path.to_string(),
	    spaces: HashMap::new()
        }
    }

    pub fn create_space(&'a mut self, name: &str) -> Result<&'a Space<'a>, FileError> {
	let space = Space::new(name, self);

	self.spaces.insert(name.to_owned(), space);

	match self.spaces.get(name) {
	    Some(x) => Ok(x),
	    _ => Err(FileError)
	}
    }
}

impl<'a> Node for Volume<'a> {
    fn path(&self) -> String {
        self.path.clone()
    }
}
