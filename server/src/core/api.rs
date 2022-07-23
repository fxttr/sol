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

use std::{io::Error, fs};

type NodeIndex = usize;

pub trait Branch {
    fn path(&self) -> String;

    fn create_vdir(basepath: &str, name: &str) -> Result<(), Error> {
        fs::create_dir_all(basepath.to_owned() + "/" + name)
    }

    fn destroy_vdir(&self) -> Result<(), Error> {
        fs::remove_dir_all(self.path())
    }
}

pub struct Node<T: Branch> {
    parent: NodeIndex,
    children: Vec<NodeIndex>,
    current: T,
}

pub struct Arena<T: Branch> {
    nodes: Vec<Node<T>>,
}

impl<T: Branch> Arena<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn open(&mut self, data: T, parent: NodeIndex) -> NodeIndex {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            parent,
            children: Vec::new(),
            current: data,
		});

        let parent_node = self.nodes.get_mut(parent);

        match parent_node {
            Some(parent_node) => parent_node.children.push(next_index),
            None => println!("No parent found!")
        };

        next_index
    }

    pub fn get_index(&self, name: &str) -> NodeIndex {
        todo!()
    }

    pub fn close(&mut self, index: NodeIndex) {
        let children = match self.nodes.get(index) {
            Some(x) => x.children.clone(),
            None => return
        };
        self.nodes.remove(index);

        for child in children.iter() {
            self.close(child.clone());
        }
    }
}
