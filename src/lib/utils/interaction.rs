use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

pub type ValidatorResult = Result<(), String>;
pub trait UserInteraction {
    fn confirm(&mut self, msg: &str) -> bool;
    fn input<F>(&mut self, msg: &str, validator: F) -> String
    where
        F: Fn(&String) -> ValidatorResult;
    fn select(&mut self, msg: &str, items: &[String]) -> usize;
    fn multi_select(&mut self, msg: &str, items: &[String]) -> Vec<usize>;
}

pub struct RealUserInteraction;

impl UserInteraction for RealUserInteraction {
    fn confirm(&mut self, msg: &str) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .wait_for_newline(true)
            .interact()
            .unwrap()
    }

    fn input<F>(&mut self, msg: &str, validator: F) -> String
    where
        F: Fn(&String) -> ValidatorResult,
    {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .validate_with(validator)
            .interact_text()
            .unwrap()
    }

    fn select(&mut self, msg: &str, items: &[String]) -> usize {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .default(0)
            .items(items)
            .interact()
            .unwrap()
    }

    fn multi_select(&mut self, msg: &str, items: &[String]) -> Vec<usize> {
        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .items(items)
            .interact()
            .unwrap()
    }
}

#[derive(Default)]
#[cfg(test)]
pub struct MockUserInteraction {
    pub confirm_answers: Vec<bool>,
    pub input_answers: Vec<String>,
    pub select_answers: Vec<usize>,
    pub multi_select_answers: Vec<Vec<usize>>,
}

#[cfg(test)]
impl UserInteraction for MockUserInteraction {
    fn confirm(&mut self, _msg: &str) -> bool {
        self.confirm_answers.remove(0)
    }

    fn input<F>(&mut self, _msg: &str, _validator: F) -> String
    where
        F: Fn(&String) -> Result<(), String>,
    {
        self.input_answers.remove(0)
    }

    fn select(&mut self, _msg: &str, _items: &[String]) -> usize {
        self.select_answers.remove(0)
    }

    fn multi_select(&mut self, _msg: &str, _items: &[String]) -> Vec<usize> {
        self.multi_select_answers.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lib::utils::interaction::MockUserInteraction;
    use crate::lib::utils::interaction::UserInteraction;
    use crate::lib::utils::interaction::ValidatorResult;

    #[test]
    fn test_mock_user_interaction() {
        let mut mock = MockUserInteraction {
            confirm_answers: vec![true, false],
            input_answers: vec!["input1".to_string(), "input2".to_string()],
            select_answers: vec![0, 1],
            multi_select_answers: vec![vec![0, 1], vec![0, 2, 4]],
        };
        let dummy_msg = "dummy";

        assert_eq!(mock.confirm(&dummy_msg), true);
        assert_eq!(mock.confirm(&dummy_msg), false);
        assert_eq!(mock.confirm_answers.len(), 0);

        let dummy_validator = |_: &String| -> ValidatorResult { ValidatorResult::Ok(()) };
        assert_eq!(
            mock.input(&dummy_msg, dummy_validator),
            "input1".to_string()
        );
        assert_eq!(
            mock.input(&dummy_msg, dummy_validator),
            "input2".to_string()
        );
        assert_eq!(mock.input_answers.len(), 0);

        let dummy_choices = vec![];

        assert_eq!(mock.select(&dummy_msg, &dummy_choices), 0);
        assert_eq!(mock.select(&dummy_msg, &dummy_choices), 1);
        assert_eq!(mock.select_answers.len(), 0);

        assert_eq!(mock.multi_select(&dummy_msg, &dummy_choices), vec![0, 1]);
        assert_eq!(mock.multi_select(&dummy_msg, &dummy_choices), vec![0, 2, 4]);
    }
}
