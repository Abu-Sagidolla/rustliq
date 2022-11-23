use std::vec::Vec;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut summar:Vec<i32> = Vec::new();
        for i in 0..nums.len()-1{
            let starter = nums[i];
            for j in i+1..nums.len(){
                if starter + nums[j] == target{
                  summar.push(i as i32);
                  summar.push(j as i32);
                  break;
               }
                else{
                    continue;
                }
            
        }
    };
        return summar;
}
}