//! # 二叉搜索树
//!
//!

use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    mem::take,
};

#[derive(Debug, Default)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Default)]
pub struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
    size: usize,
}

impl<T> Extend<T> for BinarySearchTree<T>
where
    T: Default + Ord,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter.into_iter() {
            self.insert(v);
        }
    }
}

impl<T> FromIterator<T> for BinarySearchTree<T>
where
    T: Default + Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bst = BinarySearchTree::new();
        bst.extend(iter);
        bst
    }
}

impl<T> From<Vec<T>> for BinarySearchTree<T>
where
    T: Default + Ord,
{
    fn from(value: Vec<T>) -> Self {
        let mut bst = BinarySearchTree::new();
        bst.extend(value.into_iter());
        bst
    }
}

impl<T> From<&[T]> for BinarySearchTree<T>
where
    T: Default + Ord + Clone,
{
    fn from(value: &[T]) -> Self {
        let mut bst = BinarySearchTree::new();
        bst.extend(value.iter().map(|v| (*v).clone()));
        bst
    }
}

impl<T> Clone for BinarySearchTree<T>
where
    T: Default + Ord + Clone,
{
    fn clone(&self) -> Self {
        let mut bst = BinarySearchTree::new();
        for v in self.in_order_vec() {
            bst.insert((*v).clone());
        }
        bst
    }
}

impl<T> Display for BinarySearchTree<T>
where
    T: Debug + Ord,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.in_order_vec())
    }
}

impl<T> BinarySearchTree<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Default,
{
    /// 插入节点
    /// value已存在时不插入, 返回false
    pub fn insert(&mut self, value: T) -> bool {
        let result = Self::recursive_insert(&mut self.root, value);
        if result {
            self.size += 1;
        }
        result
    }
    fn recursive_insert(current: &mut Option<Box<TreeNode<T>>>, value: T) -> bool {
        match current {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::recursive_insert(&mut node.left, value),
                Ordering::Greater => Self::recursive_insert(&mut node.right, value),
                Ordering::Equal => false,
            },
            None => {
                let new_node = Box::new(TreeNode {
                    value,
                    ..Default::default()
                });
                let _ = current.insert(new_node);
                true
            }
        }
    }

    /// 移除节点
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let result = Self::recursive_remove(&mut self.root, value);
        if result.is_some() {
            self.size -= 1;
        }
        result
    }
    fn recursive_remove(current: &mut Option<Box<TreeNode<T>>>, value: &T) -> Option<T> {
        match current {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::recursive_remove(&mut node.left, value),
                Ordering::Greater => Self::recursive_remove(&mut node.right, value),
                Ordering::Equal => {
                    let old_value = take(&mut node.value);
                    match (&node.left, &node.right) {
                        (None, None) => *current = None,
                        (Some(_), None) => *current = node.left.take(),
                        (None, Some(_)) => *current = node.right.take(),
                        (Some(_), Some(_)) => {
                            node.value = Self::recursive_remove_max(&mut node.left).unwrap()
                        }
                    }
                    Some(old_value)
                }
            },
            None => None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// 移除最大节点
    pub fn remove_max(&mut self) -> Option<T> {
        let result = Self::recursive_remove_max(&mut self.root);
        if result.is_some() {
            self.size -= 1;
        }
        result
    }
    fn recursive_remove_max(current: &mut Option<Box<TreeNode<T>>>) -> Option<T> {
        if current.as_ref().unwrap().right.is_some() {
            Self::recursive_remove_max(&mut current.as_mut().unwrap().right)
        } else {
            let node = current.take()?;
            *current = node.left;
            Some(node.value)
        }
    }

    /// 移除最小节点
    pub fn remove_min(&mut self) -> Option<T> {
        let result = Self::recursive_remove_min(&mut self.root);
        if result.is_some() {
            self.size -= 1;
        }
        result
    }
    fn recursive_remove_min(current: &mut Option<Box<TreeNode<T>>>) -> Option<T> {
        if current.as_ref().unwrap().left.is_some() {
            Self::recursive_remove_min(&mut current.as_mut().unwrap().left)
        } else {
            let node = current.take()?;
            *current = node.right;
            Some(node.value)
        }
    }

    /// 中序遍历
    pub fn in_order_vec(&self) -> Vec<&T> {
        let mut elements = vec![];
        Self::recursive_in_order_vec(&self.root, &mut elements);
        elements
    }
    fn recursive_in_order_vec<'a>(
        current: &'a Option<Box<TreeNode<T>>>,
        elements: &mut Vec<&'a T>,
    ) {
        if let Some(node) = current {
            Self::recursive_in_order_vec(&node.left, elements);
            elements.push(&node.value);
            Self::recursive_in_order_vec(&node.right, elements);
        }
    }

    /// 层序遍历
    pub fn level_order_vec(&self) -> Vec<Vec<&T>> {
        let mut elements = vec![];
        Self::recursive_level_order_vec(&self.root, &mut elements, 0);
        elements
    }
    fn recursive_level_order_vec<'a>(
        current: &'a Option<Box<TreeNode<T>>>,
        elements: &mut Vec<Vec<&'a T>>,
        depth: usize,
    ) {
        if let Some(node) = current {
            if elements.len() == depth {
                elements.push(vec![]);
            }
            elements[depth].push(&node.value);
            Self::recursive_level_order_vec(&node.left, elements, depth + 1);
            Self::recursive_level_order_vec(&node.right, elements, depth + 1);
        }
    }

    /// 树高
    pub fn height(&self) -> isize {
        Self::recursive_height(&self.root)
    }
    fn recursive_height(current: &Option<Box<TreeNode<T>>>) -> isize {
        match current {
            None => -1,
            Some(node) => {
                1 + Self::recursive_height(&node.left).max(Self::recursive_height(&node.right))
            }
        }
    }

    /// 树深
    pub fn depth(&self) -> usize {
        Self::recursive_depth(&self.root, 0)
    }
    fn recursive_depth(current: &Option<Box<TreeNode<T>>>, depth: usize) -> usize {
        match current {
            None => depth - 1,
            Some(node) => Self::recursive_depth(&node.left, depth + 1)
                .max(Self::recursive_depth(&node.right, depth + 1)),
        }
    }

    /// 节点个数
    pub fn len(&self) -> usize {
        self.size
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 是否包含值
    pub fn contains(&self, value: &T) -> bool {
        Self::recursive_contains(&self.root, value)
    }
    fn recursive_contains(current: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match current {
            None => false,
            Some(node) => match value.cmp(&node.value) {
                Ordering::Equal => true,
                Ordering::Less => Self::recursive_contains(&node.left, value),
                Ordering::Greater => Self::recursive_contains(&node.right, value),
            },
        }
    }

    /// 查找值
    pub fn get(&self, value: &T) -> Option<&T> {
        Self::recursive_get(&self.root, value)
    }
    fn recursive_get<'a>(current: &'a Option<Box<TreeNode<T>>>, value: &T) -> Option<&'a T> {
        match current {
            None => None,
            Some(node) => match value.cmp(&node.value) {
                Ordering::Equal => Some(&node.value),
                Ordering::Less => Self::recursive_get(&node.left, value),
                Ordering::Greater => Self::recursive_get(&node.right, value),
            },
        }
    }

    /// 最小值
    pub fn min(&self) -> Option<&T> {
        Self::recursive_min(&self.root)
    }
    fn recursive_min(current: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match current {
            None => None,
            Some(node) => {
                if node.left.is_some() {
                    Self::recursive_min(&node.left)
                } else {
                    Some(&node.value)
                }
            }
        }
    }

    /// 最大值
    pub fn max(&self) -> Option<&T> {
        Self::recursive_max(&self.root)
    }
    fn recursive_max(current: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match current {
            None => None,
            Some(node) => {
                if node.right.is_some() {
                    Self::recursive_max(&node.right)
                } else {
                    Some(&node.value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(10);
        bst.insert(3);
        bst.insert(7);
        bst.insert(9);

        println!("中序遍历: {:?}", bst.in_order_vec());
        println!("层序遍历: {:?}", bst.level_order_vec());
        println!("是否为空: {}", bst.is_empty());
        println!("节点个数: {}", bst.len());
        println!("高度: {}", bst.height());
        println!("树深: {}", bst.depth());
        println!("是否包含值: {}", bst.contains(&7));
        println!("查找值: {:?}", bst.get(&7));
        println!("最小值: {:?}", bst.min());
        println!("最大值: {:?}", bst.max());
        // println!("移除节点: {:?}", bst.remove(&8));
        // println!("层序遍历: {:?}", bst.level_order_vec());
        // println!("移除最小节点: {:?}", bst.remove_min());
        // println!("层序遍历: {:?}", bst.level_order_vec());
        // println!("移除最大节点: {:?}", bst.remove_max());
        // println!("层序遍历: {:?}", bst.level_order_vec());
    }
}
