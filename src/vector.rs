use instant_distance::{Builder, HnswMap, Search};
use crate::EdisonError;
use std::io::{Read, Write};

/// A normalized f32 vector implementing the Point trait for HNSW.
#[derive(Clone, Debug)]
pub struct VectorPoint(pub Vec<f32>);

impl instant_distance::Point for VectorPoint {
    fn distance(&self, other: &Self) -> f32 {
        let dot: f32 = self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum();
        1.0 - dot.clamp(-1.0, 1.0)
    }
}

/// A single result from a vector similarity search.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The record ID.
    pub id: String,
    /// Cosine similarity score 0.0-1.0.
    pub score: f32,
}

/// HNSW-based approximate nearest neighbor search index.
pub struct VectorIndex {
    points: Vec<VectorPoint>,
    values: Vec<String>,
    map: Option<HnswMap<VectorPoint, String>>,
}

impl VectorIndex {
    /// Creates a new empty VectorIndex.
    pub fn new() -> Self {
        Self { points: Vec::new(), values: Vec::new(), map: None }
    }

    fn rebuild(&mut self) {
        if self.points.is_empty() {
            self.map = None;
        } else {
            self.map = Some(Builder::default().build(
                self.points.clone(),
                self.values.clone(),
            ));
        }
    }

    /// Insert or update a vector for the given ID.
    pub fn insert(&mut self, id: String, embedding: Vec<f32>) -> Result<(), EdisonError> {
        let mut emb = embedding;
        normalize(&mut emb);
        if let Some(pos) = self.values.iter().position(|v| v == &id) {
            self.points[pos] = VectorPoint(emb);
        } else {
            self.values.push(id);
            self.points.push(VectorPoint(emb));
        }
        self.rebuild();
        Ok(())
    }

    /// Search for the top-k nearest neighbors.
    pub fn search(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
        let map = match &self.map {
            Some(m) => m,
            None => return Vec::new(),
        };
        if k == 0 { return Vec::new(); }
        let mut q = query.to_vec();
        normalize(&mut q);
        let query_point = VectorPoint(q);
        let mut search = Search::default();
        map.search(&query_point, &mut search)
            .take(k)
            .map(|item| SearchResult {
                id: item.value.clone(),
                score: (1.0 - item.distance).clamp(0.0, 1.0),
            })
            .collect()
    }

    /// Remove a vector by ID.
    pub fn remove(&mut self, id: &str) {
        if let Some(pos) = self.values.iter().position(|v| v == id) {
            self.values.remove(pos);
            self.points.remove(pos);
            self.rebuild();
        }
    }

    /// Number of vectors in the index.
    pub fn len(&self) -> usize { self.values.len() }

    /// True if index is empty.
    pub fn is_empty(&self) -> bool { self.values.is_empty() }

    /// Persist the index to disk.
    pub fn save(&self, path: &str) -> Result<(), EdisonError> {
        let mut file = std::fs::File::create(path)
            .map_err(|_| EdisonError::SaveFailed)?;
        let len = self.values.len() as u32;
        file.write_all(&len.to_le_bytes()).map_err(|_| EdisonError::SaveFailed)?;
        for (i, id) in self.values.iter().enumerate() {
            let id_bytes = id.as_bytes();
            file.write_all(&(id_bytes.len() as u32).to_le_bytes())
                .map_err(|_| EdisonError::SaveFailed)?;
            file.write_all(id_bytes).map_err(|_| EdisonError::SaveFailed)?;
            let vec = &self.points[i].0;
            file.write_all(&(vec.len() as u32).to_le_bytes())
                .map_err(|_| EdisonError::SaveFailed)?;
            for &f in vec {
                file.write_all(&f.to_le_bytes()).map_err(|_| EdisonError::SaveFailed)?;
            }
        }
        Ok(())
    }

    /// Load the index from disk.
    pub fn load(path: &str) -> Result<Self, EdisonError> {
        let mut file = std::fs::File::open(path)
            .map_err(|_| EdisonError::LoadFailed)?;
        let mut buf4 = [0u8; 4];
        file.read_exact(&mut buf4).map_err(|_| EdisonError::LoadFailed)?;
        let len = u32::from_le_bytes(buf4) as usize;
        let mut values = Vec::with_capacity(len);
        let mut points = Vec::with_capacity(len);
        for _ in 0..len {
            file.read_exact(&mut buf4).map_err(|_| EdisonError::LoadFailed)?;
            let id_len = u32::from_le_bytes(buf4) as usize;
            let mut id_buf = vec![0u8; id_len];
            file.read_exact(&mut id_buf).map_err(|_| EdisonError::LoadFailed)?;
            let id = String::from_utf8(id_buf).map_err(|_| EdisonError::LoadFailed)?;
            file.read_exact(&mut buf4).map_err(|_| EdisonError::LoadFailed)?;
            let vec_len = u32::from_le_bytes(buf4) as usize;
            let mut vec = Vec::with_capacity(vec_len);
            let mut fbuf = [0u8; 4];
            for _ in 0..vec_len {
                file.read_exact(&mut fbuf).map_err(|_| EdisonError::LoadFailed)?;
                vec.push(f32::from_le_bytes(fbuf));
            }
            values.push(id);
            points.push(VectorPoint(vec));
        }
        let mut idx = Self { points, values, map: None };
        idx.rebuild();
        Ok(idx)
    }
}

impl Default for VectorIndex {
    fn default() -> Self { Self::new() }
}

fn normalize(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() { *x /= norm; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_insert_and_search() {
        let mut idx = VectorIndex::new();
        idx.insert("a".into(), vec![1.0, 0.0, 0.0]).unwrap();
        idx.insert("b".into(), vec![0.0, 1.0, 0.0]).unwrap();
        idx.insert("c".into(), vec![0.707, 0.707, 0.0]).unwrap();
        let res = idx.search(&[0.9, 0.1, 0.0], 3);
        assert!(!res.is_empty());
        assert_eq!(res[0].id, "a");
    }

    #[test]
    fn vector_search_respects_k() {
        let mut idx = VectorIndex::new();
        idx.insert("a".into(), vec![1.0, 0.0]).unwrap();
        idx.insert("b".into(), vec![0.0, 1.0]).unwrap();
        let res = idx.search(&[1.0, 0.0], 1);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn vector_remove() {
        let mut idx = VectorIndex::new();
        idx.insert("a".into(), vec![1.0, 0.0]).unwrap();
        idx.insert("b".into(), vec![0.0, 1.0]).unwrap();
        idx.remove("a");
        let res = idx.search(&[1.0, 0.0], 2);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, "b");
    }

    #[test]
    fn vector_empty_search() {
        let idx = VectorIndex::new();
        let res = idx.search(&[1.0, 0.0], 5);
        assert!(res.is_empty());
    }

    #[test]
    fn vector_index_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<VectorIndex>();
    }

    #[test]
    fn vector_persists_and_loads() {
        let path = "/tmp/test_vector_persist.vec";
        let _ = std::fs::remove_file(path);
        let mut idx = VectorIndex::new();
        idx.insert("rec:1".into(), vec![1.0, 0.0]).unwrap();
        idx.save(path).unwrap();
        let loaded = VectorIndex::load(path).unwrap();
        let res = loaded.search(&[1.0, 0.0], 1);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, "rec:1");
    }
}