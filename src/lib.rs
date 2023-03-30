use std::ops::Deref;

/// Dirty wraps a value of type T with functions similiar to that of a Read/Write
/// lock but simply sets a dirty flag on write(), reset on clear().
/// Use read() or deref (*dirty_variable) to access the inner value.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Default, Hash)]
pub struct Dirty<T> {
    value: T,
    dirty: bool,
}

impl<T> Dirty<T> {
    /// Create a new Dirty.
    pub fn new(val: T) -> Dirty<T> {
        Dirty {
            value: val,
            dirty: true,
        }
    }

    /// Create a new Dirty with a clear dirty flag.
    pub fn new_clean(val: T) -> Dirty<T> {
        Dirty {
            value: val,
            dirty: false,
        }
    }

    /// Returns true if dirty, false otherwise.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Writable value return, sets the dirty flag.
    pub fn write(&mut self) -> &mut T {
        self.dirty = true;
        &mut self.value
    }

    /// Read the value.
    pub fn read(&self) -> &T {
        &self.value
    }

    /// Clears the dirty flag.
    pub fn clear(&mut self) {
        self.dirty = false;
    }

    /// Read the value only if modified since last read.
    pub fn read_dirty(&self) -> Option<&T> {
        match self.dirty {
            true => Some(&self.value),
            false => None,
        }
    }

    /// Write new value only if dirty, returning whether the value was written or not
    pub fn write_dirty<F>(&mut self, f: F) -> bool
    where F: Fn(&T) -> T {
        if self.dirty { self.value = f(&self.value); }
        self.dirty
    }

    /// Consumes the wrapper and returns the enclosed value
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T> Deref for Dirty<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Dirty;

    #[test]
    fn new_dirty() {
        let dirty = Dirty::new(0);
        assert!(dirty.dirty());
    }

    #[test]
    fn new_dirty_clean() {
        let dirty = Dirty::new_clean(0);
        assert!(!dirty.dirty());
    }

    #[test]
    fn read_doesnt_clear_flag() {
        let dirty = Dirty::new(0);
        assert!(dirty.dirty());
        assert!(*dirty.read() == 0);
        assert!(dirty.dirty());
    }

    #[test]
    fn write_sets_flag() {
        let mut dirty = Dirty::new(0);
        assert!(*dirty.read() == 0);
        dirty.clear();
        assert!(!dirty.dirty());
        *dirty.write() += 1;
        assert!(dirty.dirty());
    }

    #[test]
    fn read_dirty() {
        let mut dirty = Dirty::new(0);
        assert!(dirty.read_dirty().is_some());
        dirty.clear();
        assert!(!dirty.dirty());
        assert!(dirty.read_dirty() == None);
        assert!(!dirty.dirty());
        *dirty.write() += 1;
        assert!(dirty.dirty());
        assert!(dirty.read_dirty().is_some());
        dirty.clear();
        assert!(!dirty.dirty());
        assert!(dirty.read_dirty() == None);
    }

    #[test]
    fn write_dirty() {
        let mut dirty = Dirty::new_clean(0);
        assert!(!dirty.write_dirty(|_| 3));
        *dirty.write() += 3;
        assert!(dirty.write_dirty(|_| [1, 2, 3].iter().copied().reduce(|acc, x| acc + x).unwrap()));
        assert_eq!(*dirty.read(), 6);
    }

    #[test]
    fn access_inner_deref() {
        let dirty = Dirty::new(0);
        assert!(*dirty == 0);
    }

    #[test]
    fn default_value() {
        let dirty = Dirty::<i32>::default();
        assert!(*dirty == 0);
    }

    #[test]
    fn unwrap() {
        let mut dirty = Dirty::new(100);
        *dirty.write() = 200;
        assert_eq!(dirty.unwrap(), 200);
    }
}
