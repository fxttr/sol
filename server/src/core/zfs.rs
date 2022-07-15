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

use std::io::Error;
use std::fs::File;
use std::fs;

pub struct Volume {
    name: String,
    path: String
}

pub struct FsFile {
    tags: File,
    body: File,
    mode: File
}

pub struct Snapshot {
    
}

impl Volume {
    pub fn new(name: &str, path: &str) -> Self {
	Self {
	    name: name.to_string(),
	    path: path.to_string()
	}
    }

    pub fn open_file(&self, name: &str) -> Result<FsFile, Error> {
	match fs::create_dir(self.path(name)) {
	    Ok(_) => println!("Created directory."),
	    Err(e) => return Err(e)
	};

	let tags = File::create(self.path(name) + "tags").unwrap();
	let body = File::create(self.path(name) + "body").unwrap();
	let mode = File::create(self.path(name) + "mode").unwrap();

	Ok(FsFile {
	    tags,
	    body,
	    mode
	})
    }

    pub fn close_file(&self, name: &str) -> bool {
	match fs::remove_dir_all(self.path(name)) {
	    Ok(_) => true,
	    Err(_) => false
	}
    }

    pub fn snapshot(&self, name: &str) -> Snapshot {
	todo!();
    }

    fn path(&self, name: &str) -> String {
	self.path.clone() + "/" + name + "/"
    }
}
