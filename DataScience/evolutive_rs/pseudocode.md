---
author: Daniel Sánchez
title: Evolutive Algorithms in Rust
---

## Implementation

### Library

```rust
struct Fitness {
    values: u32
} deriving(Show)

struct Chromosome {
    genes: Vec<char>, // Should be iterable
    length: u32,
    fitness: Fitness
} deriving(Show)

impl Chromosome {
    pub fn new(self, geneset: Vec<char>, length:) {
        self.genes = 
    }
    pub fn display(self, genes) {
        // Define code
    }
    fn get_fitness(genes)
}

trait Chromosome {
    trait::Ord(self, other: Chromosome) {
        match comparative {
            GT => self.fitness > other.fitness,
            EQ => not self.fitness > other.fitness &&
                not other.fitness > self.fitness,
            LS => not self.fitness > other.fitness &&
                not self.fitness == other.fitness
        }
    }
}

fn get_best(target_len
```

### Instance

```haskell


get_fitness :: Chromosome -> Fitness

```

## Reimplement in Python

```python
class Chromosome:
    def __init__(self, genes):
        self.genes = genes
        self.fitness = self._getfitness(genes)

    def _getfitness()
```
