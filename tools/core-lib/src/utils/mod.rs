use std::collections::HashMap;
use std::marker::PhantomData;

/// A generic memory structure that maps addresses to values of specific types.
///
/// The memory can handle different address sizes (u8, u16, u32) and will
/// return values of the corresponding output type. Implemented using a HashMap
/// for flexible storage.
///
/// # Type Parameters
/// - `A`: Address type (u8, u16, or u32)
/// - `D`: Data type (the output type when reading from memory)
pub struct Memory<A, D> {
    storage: HashMap<A, D>,
    default_value: D,
    _address_marker: PhantomData<A>,
    _data_marker: PhantomData<D>,
}

impl<A, D> Memory<A, D>
where
    A: std::hash::Hash + Eq + Copy,
    D: Copy + Default,
{
    /// Creates a new empty memory instance
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            _addr: PhantomData,
            _data: PhantomData,
        }
    }

    /// Writes a value to the specified address.
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The value to write
    pub fn write(&mut self, address: A, value: D) {
        self.storage.insert(address, value);
    }

    /// Reads a value from the specified address.
    ///
    /// Returns the default value if the address hasn't been written to.
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    pub fn read(&self, address: A) -> D {
        *self.storage.get(&address).unwrap_or(&self.default_value)
    }

    /// Returns the number of initialized memory locations.
    pub fn used(&self) -> usize {
        self.storage.len()
    }

    /// Checks if the memory is empty (no initialized locations).
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}