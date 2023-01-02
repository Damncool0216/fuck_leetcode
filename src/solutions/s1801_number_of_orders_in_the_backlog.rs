#![allow(dead_code, unused)]
use std::cmp;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let mut buy_heap = BinaryHeap::<(i32, i32)>::new();
        let mut sell_heap = BinaryHeap::<(i32, i32)>::new();
        // sell_price <= buy_price => buy_price + (-sell_price) >= 0

        for order in orders.iter() {
            let mut price = order[0];
            let mut amount = order[1];
            let mut heap_a = &mut sell_heap;
            let mut heap_b = &mut buy_heap;
            if order[2] == 1 {
                //如果是销售订单，则取相反数
                price = -price;
                heap_a = &mut buy_heap;
                heap_b = &mut sell_heap;
            }
            while amount > 0 && !heap_a.is_empty() && heap_a.peek().unwrap().0 + price >= 0 {
                let (price_peek, mut amount_peek) = heap_a.pop().unwrap();
                let m = cmp::min(amount, amount_peek);
                amount -= m;
                amount_peek -= m;
                if amount_peek > 0 {
                    heap_a.push((price_peek, amount_peek));
                }
            }
            if amount > 0 {
                heap_b.push((price, amount));
            }
        }
        let mut ans = 0;
        while let Some(order) = buy_heap.pop() {
            ans += order.1;
            ans %= MOD;
        }
        while let Some(order) = sell_heap.pop() {
            ans += order.1;
            ans %= MOD;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let orders = vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0],
        ];
        assert_eq!(6, Solution::get_number_of_backlog_orders(orders));
    }

    #[test]
    fn test2() {
        let orders = vec![
            vec![7, 1000000000, 1],
            vec![15, 3, 0],
            vec![5, 999999995, 0],
            vec![5, 1, 1],
        ];
        assert_eq!(999999984, Solution::get_number_of_backlog_orders(orders));
    }
}
