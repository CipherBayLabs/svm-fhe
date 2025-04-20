use tfhe::prelude::*;
use tfhe::FheUint8;

struct FheDocument {
    // Simple vector of encrypted numbers representing a "document"
    vector: Vec<FheUint8>,  // e.g., [1,2,3] encrypted
}

struct SimplePrivateRAG {
    // "Database" of encrypted document vectors
    documents: Vec<FheDocument>,
    server_key: ServerKey,
}

impl SimplePrivateRAG {
    fn calculate_distance(&self, query: &[FheUint8], doc: &FheDocument) -> FheUint8 {
        let mut distance = FheUint8::encrypt(0);
        
        // Example:
        // query    = [1, 4, 2] (encrypted)
        // doc      = [2, 3, 5] (encrypted)
        // Compares: 1 vs 2, 4 vs 3, 2 vs 5
        for (q, d) in query.iter().zip(doc.vector.iter()) {
            let diff1 = q - d;  // FHE subtraction
            let diff2 = d - q;
            distance += diff1.max(&diff2);  // FHE max and add
        }
        
        distance
    }

    fn search(&self, query: Vec<FheUint8>) -> Vec<FheUint8> {
        // For each document, calculate distance to query
        self.documents.iter()
            .map(|doc| self.calculate_distance(&query, doc))
            .collect()
    }
}