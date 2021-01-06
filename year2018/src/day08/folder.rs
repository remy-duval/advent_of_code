//! Factorize the logic for traversing a tree into a trait

/// Fold a tree using the given [Folder](Folder) implementation
pub fn fold<T: Folder>(mut folder: T, tree: &super::Tree) -> Option<T::Output> {
    folder.fold_impl(&tree.0).map(|(out, _)| out)
}

/// A trait for folding a [Tree](super::Tree) into an output value
pub trait Folder {
    /// The type of the output that is folded from a tree
    type Output;

    /// Fold a leaf node into an output
    fn leaf(&mut self, metadata: &[u32]) -> Self::Output;

    /// Fold a parent node into an output
    fn parent(&mut self, children: &[Self::Output], metadata: &[u32]) -> Self::Output;

    /// Recursively fold the values of a tree into an output
    fn fold_impl<'a>(&mut self, mut current: &'a [u32]) -> Option<(Self::Output, &'a [u32])> {
        let mut slice = current.iter();
        let children = *slice.next()? as usize;
        let metadata = *slice.next()? as usize;
        current = slice.as_slice();

        if children == 0 {
            let output = self.leaf(current.get(0..metadata)?);
            Some((output, current.get(metadata..)?))
        } else {
            let mut children = Vec::with_capacity(children);
            for _ in 0..children.capacity() {
                let (partial, after) = self.fold_impl(current)?;
                current = after;
                children.push(partial);
            }

            let output = self.parent(&children, current.get(0..metadata)?);
            Some((output, current.get(metadata..)?))
        }
    }
}

/// A folder for summing the metadata values
pub struct MetadataSum;

impl Folder for MetadataSum {
    type Output = u32;

    fn leaf(&mut self, metadata: &[u32]) -> Self::Output {
        metadata.iter().sum()
    }

    fn parent(&mut self, children: &[Self::Output], metadata: &[u32]) -> Self::Output {
        children.iter().sum::<Self::Output>() + metadata.iter().sum::<Self::Output>()
    }
}

/// A folder for computing the value of a node
pub struct NodeValues;

impl Folder for NodeValues {
    type Output = u32;

    /// leaf node value is metadata sum as usual
    fn leaf(&mut self, metadata: &[u32]) -> Self::Output {
        metadata.iter().sum()
    }

    /// parent node value is the sum of the children node values referenced by its metadata
    fn parent(&mut self, children: &[Self::Output], metadata: &[u32]) -> Self::Output {
        metadata
            .iter()
            .filter_map(|i| (*i as usize).checked_sub(1))
            .filter_map(|i| children.get(i))
            .sum()
    }
}
