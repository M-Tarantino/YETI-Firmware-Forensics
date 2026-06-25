use crate::util::error::{YetiResult, YetiError};
use crate::core::scanner::Candidate;
use std::collections::BTreeMap;
use memmap2::Mmap;
use backhand::{FilesystemReader, InnerNode, SquashfsFileReader};

#[derive(Clone, Debug)]
pub struct VfsNode {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub children: BTreeMap<String, VfsNode>,
}

pub struct VirtualFilesystem { pub root: VfsNode }

impl VirtualFilesystem {
    pub fn new(mmap: &Mmap, candidate: &Candidate) -> YetiResult<Self> {
        let mut root = VfsNode { name: "/".into(), is_dir: true, size: 0, children: BTreeMap::new() };
        let offset = candidate.offset as usize;
        let mut data_block = mmap[offset..].to_vec();
        
        if data_block.starts_with(b"shsq") { 
            data_block[0..4].copy_from_slice(b"hsqs"); 
        }

        let fs = FilesystemReader::from_reader(std::io::Cursor::new(data_block))
            .map_err(|e| YetiError::Vfs(e.to_string()))?;

        for node in fs.files() {
            let path = node.fullpath.to_str().unwrap_or("");
            Self::build_tree(&mut root, path, &node.inner);
        }
        Ok(Self { root })
    }

    fn build_tree(root: &mut VfsNode, path: &str, inner: &InnerNode<SquashfsFileReader>) {
        let mut current = root;
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        for (i, part) in parts.iter().enumerate() {
            let is_last = i == parts.len() - 1;
            if is_last {
                current.children.insert(part.to_string(), VfsNode {
                    name: part.to_string(),
                    is_dir: matches!(inner, InnerNode::Dir(_)),
                    // Fix: Correctly accessing file size from SquashfsFile metadata
                    size: if let InnerNode::File(f) = inner { f.basic.file_size as u64 } else { 0 },
                    children: BTreeMap::new(),
                });
            } else {
                current = current.children.entry(part.to_string()).or_insert(VfsNode {
                    name: part.to_string(), is_dir: true, size: 0, children: BTreeMap::new(),
                });
            }
        }
    }

    pub fn resolve(&self, path: &str) -> Option<&VfsNode> {
        let mut current = &self.root;
        for part in path.split('/').filter(|s| !s.is_empty()) {
            current = current.children.get(part)?;
        }
        Some(current)
    }
}