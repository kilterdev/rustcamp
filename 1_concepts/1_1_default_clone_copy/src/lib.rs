//! This crate exposes some objects to work with
//! on an 2D plane (i.e. Point and Polyline)

/// This type represents a point on a 2D plane
/// Implements a Copy semantics.
///
/// Also can be Default-initialized
/// Can contain any values that semantically represent coordinates
#[derive(Copy, Clone, Default, Debug)]
struct Point<T: Copy + Default> {
    x: T,
    y: T,
}

impl<T: PartialEq + Copy + Default> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// This type respresents a set of points (a line) on a 2D plane
///
/// Implements Clone semantics
#[derive(Clone, Debug)]
struct Polyline<T: Copy + Default>(Vec<Point<T>>);

impl<T: Copy + Default> Polyline<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, p: Point<T>) {
        self.0.push(p);
    }

    pub fn insert_at(&mut self, pos: usize, p: Point<T>) {
        self.0.insert(pos, p)
    }

    pub fn get_at(&self, pos: usize) -> Option<Point<T>> {
        self.0.get(pos).map(|p| *p)
    }

    pub fn get_last(&self) -> Option<Point<T>> {
        self.get_at(self.0.len() - 1)
    }

    pub fn remove_at(&mut self, pos: usize) -> Option<Point<T>> {
        if pos < 1 {
            return None;
        }
        Some(self.0.remove(pos))
    }

    pub fn shift(&mut self) -> Option<Point<T>> {
        self.remove_at(0)
    }

    pub fn pop(&mut self) -> Option<Point<T>> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p: Point<i32> = Default::default();
        let p2 = p;
        let p3: Point<usize> = Point { x: 12, y: 13 };

        assert_eq!(p, p2);
        assert_eq!(p3, Point { x: 12, y: 13 });
    }

    #[test]
    fn test_polyline() {
        let mut poly = Polyline::new();

        poly.add(Point::<i32>::default());
        poly.add(Point::<i32> { x: 1, y: 2 });

        let mut poly2 = poly.clone();

        assert_eq!(poly.pop(), poly2.pop());
    }
}
