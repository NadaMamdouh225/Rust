impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ones:i32 = s.chars().filter(|&c|c == '1').count() as i32;
        let mut zeros:i32 = 0;
        let mut ans:i32 = 0;
        for ch in s.chars().take(s.len()-1)
        {
            if ch == '0'{
                zeros+=1;
            }else {
                ones-=1;
            }
            ans = ans.max(ones+zeros);            
        }
        ans
    }
}
