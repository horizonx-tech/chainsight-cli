use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};

pub trait UserInteraction {
    fn confirm_to_user(&mut self, msg: &str) -> bool;
    fn multi_select_to_user(&mut self, msg: &str, items: &Vec<String>) -> Vec<usize>;
}

pub struct RealUserInteraction;

impl UserInteraction for RealUserInteraction {
    fn confirm_to_user(&mut self, msg: &str) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .wait_for_newline(true)
            .interact()
            .unwrap()
    }

    fn multi_select_to_user(&mut self, msg: &str, items: &Vec<String>) -> Vec<usize> {
        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .items(items)
            .interact()
            .unwrap()
    }
}

#[cfg(test)]
pub struct MockUserInteraction {
    pub confirm_answers: Vec<bool>,
    pub multi_select_answers: Vec<Vec<usize>>,
}

#[cfg(test)]
impl UserInteraction for MockUserInteraction {
    fn confirm_to_user(&mut self, _msg: &str) -> bool {
        self.confirm_answers.remove(0)
    }

    fn multi_select_to_user(&mut self, _msg: &str, _items: &Vec<String>) -> Vec<usize> {
        self.multi_select_answers.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lib::utils::interaction::MockUserInteraction;
    use crate::lib::utils::interaction::UserInteraction;

    #[test]
    fn test_mock_user_interaction() {
        let mut mock = MockUserInteraction {
            confirm_answers: vec![true, false],
            multi_select_answers: vec![vec![0, 1], vec![0, 2, 4]],
        };
        let dummy_msg = "dummy";

        assert_eq!(mock.confirm_to_user(&dummy_msg), true);
        assert_eq!(mock.confirm_to_user(&dummy_msg), false);
        assert_eq!(mock.confirm_answers.len(), 0);

        let dummy_choices = vec![];
        assert_eq!(
            mock.multi_select_to_user(&dummy_msg, &dummy_choices),
            vec![0, 1]
        );
        assert_eq!(
            mock.multi_select_to_user(&dummy_msg, &dummy_choices),
            vec![0, 2, 4]
        );
    }
}
