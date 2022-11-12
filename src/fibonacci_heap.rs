//! Fibonacci Heap Implementation
//!
//! I find this data structure really interesting
//!
//! For more insight I'd recommend watching: https://www.youtube.com/watch?v=6JxvKfSV9Ns
//!
//! Key Ideas involved:
//!
//! Binary Trees
//! - Extract Min - O(log n)
//! - Insert - O(log n)
//! - GetMin - O(1)
//! - Decrease Key - O(log n)
//!
//! Fibonacci Heap
//! - Heap Property: No Child Node is smaller than its parent
//!
//! - GetMin - O(1)
//! - Insert - O(1)
//! - Extract Min - O(log n)
//!     - Remove the minimum Node
//!     - Clean Up (Binomial Trees)
//!     - Rebuild the Heap
//! - Decrease Key - O(1)
//!
