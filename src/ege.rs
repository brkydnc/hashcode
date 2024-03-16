use super::model::{Project, Contributor, Role, Skillset, ExecutedProject, Schedule};

pub fn executor(mut projects: Vec<Project>, mut contributors: Vec<Contributor>) -> Schedule {
    projects.sort_by(|a, b| b.best_before.cmp(&a.best_before));

    let mut executed = vec![];

    while let Some(mut current_project) = projects.pop() {
        let role_len = current_project.roles.len();
        let contributors = current_project.roles
            .into_iter()
            .map(|role| contributors
                 .iter_mut()
                 .find_map(|contributor| {
                     if let Some(level) = contributor.skills.get_mut(&role.skill) {
                         if *level >= role.level {
                             *level += 1;
                             Some(contributor.name.clone())
                         } else {
                             None
                         }
                     } else {
                         None
                     }
                 })
             )
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<String>>();

        let unique = contributors
            .iter()
            .all(|c| contributors.iter().filter(|cx| c == *cx).count() == 1);

        if contributors.len() == role_len && unique {
            executed.push(ExecutedProject::new(current_project.name, contributors));
        }
    }

    executed.into()
}
