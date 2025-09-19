

fn main() {

    let client = BayesianKnowledgeTracer::new(0.1, 0.4, 0.1, 0.25);
    let test_value = client.update(1);

    println!("{}", test_value);

}


struct BayesianKnowledgeTracer{
    p_known: f64,
    p_learning: f64,
    p_correct_guess: f64,
    p_slip: f64,

    }

impl BayesianKnowledgeTracer {

    fn new(p_known: f64, p_learning: f64, p_correct_guess: f64, p_slip: f64) -> Self {

        Self{
            p_known,
            p_learning,
            p_correct_guess,
            p_slip
        }

    }

    fn test(&self) -> f64 {
        return self.p_known
    }


    fn update(&self, response: i32) -> f64 {

        let float_response = response as f64;

        let p_correct_given_known = 1.0 - self.p_slip;
        let p_correct_given_unknown = self.p_correct_guess;
        let p_correct = self.p_known * self.p_correct_given_known + (1 - self.p_known) * p_correct_given_unknown;

        if float_response == 1[
            let p_known_given_response = (self.p_known * p_correct_given_known) / p_correct;
        ]
        else {
            let p_incorrect_given_known = self.p_slip;
            let p_incorrect_given_unknown = 1 - self.p_correct_guess;
            let p_incorrect = self.p_known * self.p_incorrect_given_known + (1 - self.p_known) * p_incorrect_given_unknown;
            let p_known_given_response = (self.p_known * p_incorrect_given_known) / p_incorrect;
        }

        let p_new_known = p_known_given_response + (1 - p_known_given_response) * self.p_learning;

        return p_new_known

    }

}
