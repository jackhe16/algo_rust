//! # 平衡二叉搜索树
//!
//! ## 算法思路
//! - 平衡操作 (cf: 当前节点平衡因子, lf: 左孩节点平衡因子, rf: 右孩节点平衡因子)
//!   - LL: cf == 2 && lf == 1, 当前右旋
//!   - RR: cf == -2 && rf == -1, 当前左旋
//!   - LR: cf == 2 && lf == -1, 左孩左旋再当前右旋
//!   - RL: cf == -2 && rf == 1, 右孩右旋再当前左旋
//! - 插入新节点时, 检查新节点祖先节点是否失衡, 若找到则进行平衡操作(只需一次?)
//! - 删除节点时, 检查删除节点祖先节点是否失衡, 若找到则进行平衡操作(重复检查)
//! - 插入或删除节点时, 祖先节点的高度需要更新

use std::{
    cmp::Ordering,
    mem::{swap, take},
};

#[derive(Debug)]
struct AvlNode<T> {
    value: T,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
    height: usize,
}

#[derive(Debug)]
pub struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
    size: usize,
}

impl<T> Default for AvlTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            size: Default::default(),
        }
    }
}

impl<T> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree::default()
    }

    /// 高度
    fn height(current: &Option<Box<AvlNode<T>>>) -> usize {
        current.as_ref().map_or(0, |v| v.height)
    }

    /// 更新高度
    fn update_height(node: &mut Box<AvlNode<T>>) {
        node.height = 1 + Self::height(&node.left).max(Self::height(&node.right));
    }

    /// 平衡因子
    fn banlance_factor(node: &Box<AvlNode<T>>) -> i8 {
        let left_height = Self::height(&node.left);
        let right_height = Self::height(&node.right);
        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    /// 左旋 (必有右孩)
    fn rotate_left(node: &mut Box<AvlNode<T>>) {
        // node的右孩置空,右孩所有权移至right_tree
        let mut right_tree = node.right.take().unwrap();
        // node的右孩指向右孩的左孩
        node.right = right_tree.left.take();
        // 此时node的高度可能变化,且后续不再变化
        Self::update_height(node);
        // 交换node和right_tree的内存数据
        swap(node, &mut right_tree);
        // swap后node指向right_tree这份数据, right_tree这份数据变成了原先node指向的数据
        node.left = Some(right_tree);
        // node指向right_tree这份数据,left指向变化,高度需更新
        Self::update_height(node);
    }

    /// 右旋 (必有左孩)
    fn rotate_right(node: &mut Box<AvlNode<T>>) {
        let mut left_tree = node.left.take().unwrap();
        node.left = left_tree.right.take();
        Self::update_height(node);
        swap(node, &mut left_tree);
        node.right = Some(left_tree);
        Self::update_height(node);
    }

    /// 重平衡
    fn rebanlance(node: &mut Box<AvlNode<T>>) {
        let factor = Self::banlance_factor(node);
        match factor {
            2 => {
                // LL, LR
                let mut left_tree = node.left.as_mut().unwrap();
                let left_tree_factor = Self::banlance_factor(&left_tree);
                if left_tree_factor == -1 {
                    Self::rotate_left(&mut left_tree);
                }
                Self::rotate_right(node);
            }
            -2 => {
                // RR, RL
                let mut right_tree = node.right.as_mut().unwrap();
                let right_tree_factor = Self::banlance_factor(&right_tree);
                if right_tree_factor == 1 {
                    Self::rotate_right(&mut right_tree);
                }
                Self::rotate_left(node);
            }
            _ => {}
        }
    }
}

impl<T> AvlTree<T>
where
    T: Ord,
{
    /// 插入
    pub fn insert(&mut self, value: T) -> bool {
        let is_inserted = Self::recursive_insert(&mut self.root, value);
        if is_inserted {
            self.size += 1;
        }
        is_inserted
    }
    fn recursive_insert(current: &mut Option<Box<AvlNode<T>>>, value: T) -> bool {
        match current {
            None => {
                let _ = current.insert(Box::new(AvlNode {
                    height: 1,
                    left: None,
                    right: None,
                    value,
                }));
                true
            }
            Some(node) => {
                let is_inserted = match value.cmp(&node.value) {
                    Ordering::Equal => false,
                    Ordering::Less => Self::recursive_insert(&mut node.left, value),
                    Ordering::Greater => Self::recursive_insert(&mut node.right, value),
                };
                if is_inserted {
                    Self::update_height(node);
                    Self::rebanlance(node);
                }
                is_inserted
            }
        }
    }

    /// 层序遍历
    pub fn level_order_vec(&self) -> Vec<Vec<&T>> {
        let mut elements = vec![];
        Self::recursive_level_order_vec(&self.root, &mut elements, 0);
        elements
    }
    fn recursive_level_order_vec<'a>(
        current: &'a Option<Box<AvlNode<T>>>,
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

    fn recursive_remove_max(current: &mut Option<Box<AvlNode<T>>>) -> Option<T> {
        if current.as_ref().unwrap().right.is_some() {
            let removed = Self::recursive_remove_max(&mut current.as_mut().unwrap().right);
            if removed.is_some() {
                Self::update_height(current.as_mut().unwrap());
                Self::rebanlance(current.as_mut().unwrap());
            }
            removed
        } else {
            let node = current.take()?;
            *current = node.left;
            Some(node.value)
        }
    }
}

impl<T> AvlTree<T>
where
    T: Ord + Default,
{
    /// 移除
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let removed = Self::recursive_remove(&mut self.root, value);
        if removed.is_some() {
            self.size -= 1;
        }
        removed
    }
    fn recursive_remove(current: &mut Option<Box<AvlNode<T>>>, value: &T) -> Option<T> {
        match current {
            Some(node) => {
                let removed = match value.cmp(&node.value) {
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
                };
                if removed.is_some() && current.is_some() {
                    Self::update_height(current.as_mut().unwrap());
                    Self::rebanlance(current.as_mut().unwrap());
                }
                removed
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut avl = AvlTree::new();
        avl.insert(8);
        avl.insert(3);
        avl.insert(10);
        avl.insert(2);
        avl.insert(6);
        avl.insert(9);
        avl.insert(7);
        println!("{:?}", avl.level_order_vec());
    }

    #[test]
    fn test_remove() {
        let mut avl = AvlTree::new();
        avl.insert(8);
        avl.insert(3);
        avl.insert(10);
        avl.insert(2);
        avl.insert(6);
        avl.insert(9);
        avl.insert(7);
        println!("{:?}", avl.level_order_vec());
        avl.remove(&9);
        println!("{:?}", avl.level_order_vec());
    }
}
