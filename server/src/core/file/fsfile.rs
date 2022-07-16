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

use std::{fs::File, collections::HashMap};

use crate::core::{tree::Node, zfs::TimeShift};

use super::view::View;

pub struct FsFile<'a> {
    name: String,
    repr: HashMap<String, File>,
    file: File,
    view: &'a View<'a>
}

impl<'a> FsFile<'a> {
    // Todo: Error handling!
    pub fn new(name: &str, path: &str, view: &'a mut View) -> Self {
	FsFile::create_vdir(&view.path(), name);
	
	let mut repr: HashMap<String, File> = HashMap::new();

	for repr_name in ["body", "tags", "mode"] {
	    repr.insert(repr_name.to_owned(), File::create(FsFile::assemble_repr_path(repr_name, name, &view.path())).unwrap());
	}
	
	Self {
	    repr,
	    file: File::open(path).unwrap(),
	    view,
	    name: name.to_owned()
	}
    }

    fn assemble_repr_path(repr_name: &str, fsfile_name: &str, view_path: &str) -> String {
	view_path.to_owned() + "/" + fsfile_name + "/" + repr_name
    }
}

impl<'a> Node for FsFile<'a> {
    fn path(&self) -> String {
	self.view.path() + "/" + &self.name
    }
}

impl<'a> TimeShift for FsFile<'a> {}
