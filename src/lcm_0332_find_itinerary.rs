/*
 * @lc app=leetcode.cn id=332 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        vec![]
    }

    fn backtracking() {
        //
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec![String::from("MUC"), String::from("LHR")],
                vec![String::from("JFK"), String::from("MUC")],
                vec![String::from("SFO"), String::from("SJC")],
                vec![String::from("LHR"), String::from("SFO")]
            ]),
            vec![
                String::from("JFK"),
                String::from("MUC"),
                String::from("LHR"),
                String::from("SFO"),
                String::from("SJC"),
            ]
        );
    }

    fn test_2() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec![String::from("JFK"), String::from("SFO")],
                vec![String::from("JFK"), String::from("ATL")],
                vec![String::from("SFO"), String::from("ATL")],
                vec![String::from("ATL"), String::from("JFK")],
                vec![String::from("ATL"), String::from("SFO")]
            ]),
            vec![
                String::from("JFK"),
                String::from("ATL"),
                String::from("JFK"),
                String::from("SFO"),
                String::from("ATL"),
                String::from("SFO"),
            ]
        );
    }
}
