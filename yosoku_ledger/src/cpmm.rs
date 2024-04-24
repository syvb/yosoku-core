use crate::TokenNumber;

pub struct CpmmState {
    pool_yes: TokenNumber,
    pool_no: TokenNumber,
    p: f64,
}
impl CpmmState {
    pub fn get_shares(&self, bet_amount: TokenNumber, bet_direction: bool) -> f64 {
        if bet_amount == 0 {
            return 0.0;
        }

        let bet_amount = bet_amount as f64;
        let y = self.pool_yes as f64;
        let n = self.pool_no as f64;
        let p = self.p;

        // Manifold:
        //   const { YES: y, NO: n } = pool
        //   const k = y ** p * n ** (1 - p)
        //   return betChoice === 'YES'
        //     ? // https://www.wolframalpha.com/input?i=%28y%2Bb-s%29%5E%28p%29*%28n%2Bb%29%5E%281-p%29+%3D+k%2C+solve+s
        //       y + betAmount - (k * (betAmount + n) ** (p - 1)) ** (1 / p)
        //     : n + betAmount - (k * (betAmount + y) ** -p) ** (1 / (1 - p))

        let k = y.powf(p).powf(y).powf(1.0 - p);
        if bet_direction {
            y + bet_amount - (k * (bet_amount + n).powf(p - 1.0)).powf(p.recip())
        } else {
            n + bet_amount - (k * (bet_amount + y).powf(-p)).powf((1.0 - p).recip())
        }
    }
}
