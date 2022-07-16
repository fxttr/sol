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
use std::fs;
use crate::core::err::FileError;
use crate::core::tree::Node;
use crate::core::zfs::{Volume, TimeShift};

use super::view::View;

pub struct Space<'a> {
    name: String,
    views: HashMap<String, View<'a>>,
    volume: &'a Volume<'a>
}

impl<'a> Space<'a> {
    pub fn new(name: &str, volume: &'a Volume) -> Self {
	Space::create_vdir(&volume.path(), name);
	
	Self {
	    name: name.to_owned(),
	    views: HashMap::new(),
	    volume
	}
    }
    
    pub fn create_view(&'a mut self, name: &str) -> Result<&View, FileError> {
        match fs::create_dir(self.path()) {
            Ok(_) => println!("Created directory."),
            Err(_) => return Err(FileError),
        };

	self.views.insert(name.to_owned(), View::new(name, self));

	match self.views.get(name) {
	    Some(x) => Ok(x),
	    _ => Err(FileError)
	}
    }

    pub fn destroy_view(&mut self, name: &str) -> bool {
	self.views.remove(name);
	
        match fs::remove_dir_all(self.path()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn view(&self, name: &str) -> Option<&View> {
	self.views.get(name)
    }
}

impl<'a> Node for Space<'a> {
    fn path(&self) -> String {
	self.volume.path() + "/" + &self.name
    }
}

impl<'a> TimeShift for Space<'a> {}
