use std::collections::HashMap;

pub type Skillset = HashMap<String, usize>;

#[derive(Debug)]
pub struct Role {
    pub skill: String,
    pub level: usize,
}

#[derive(Debug)]
pub struct Contributor {
    pub name: String,
    pub skills: Skillset,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub duration: usize,
    pub score: usize,
    pub best_before: usize,
    pub roles: Vec<Role>,
}

#[derive(Debug)]
pub struct ExecutedProject {
    pub name: String,
    pub contributors: Vec<String>,
}

pub struct Schedule(pub Vec<ExecutedProject>);

impl ExecutedProject {
    pub fn new(name: String, contributors: Vec<String>) -> Self {
        ExecutedProject { name, contributors }
    }

    pub fn dummy() -> Self {
        ExecutedProject { name: String::new(), contributors: vec![] }
    }
}

impl Schedule {
    pub fn to_output(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("{}\n", self.0.len()));

        for (i, project) in self.0.iter().enumerate() {
            output.push_str(&project.name);
            output.push('\n');
            output.push_str(&project.contributors.join(" "));

            if self.0.len() - 1 != i {
                output.push('\n');
            }
        }

        output
    }
}

impl From<Vec<ExecutedProject>> for Schedule {
    fn from(vec: Vec<ExecutedProject>) -> Schedule {
        Schedule(vec)
    }
}

pub fn parse_input(content: String) -> (Vec<Project>, Vec<Contributor>) {
    let mut lines = content.lines();
    let (contributor_count, project_count) = lines
        .next()
        .expect("Input file does not have first line")
        .split_once(" ")
        .expect("Can not split first line of the file");

    let contributor_count = contributor_count.parse::<usize>().unwrap();
    let project_count = project_count.parse::<usize>().unwrap();

    let mut contributors: Vec<Contributor> = Vec::with_capacity(contributor_count);
    let mut projects: Vec<Project> = Vec::with_capacity(project_count);

    for _ in 0..contributor_count {
        let contributor_str = lines
            .next()
            .expect("Contributor collect loop does not have more lines");

        let (name, skill_count) = contributor_str
            .split_once(" ")
            .expect("Can not split CONTRIBUTOR string");

        let skill_count = skill_count.parse::<usize>().unwrap();

        let skills = (&mut lines)
            .take(skill_count)
            .map(|line| {
                let (skill, level) = line
                    .split_once(" ")
                    .expect("Can not split SKILL string");

                let level = level.parse::<usize>().unwrap();

                (skill.to_string(), level)
            })
            .collect::<Skillset>();

        let contributor = Contributor { name: name.to_string(), skills };
        contributors.push(contributor);
    }

    for _ in 0..project_count {
        let project_str = lines
            .next()
            .expect("Project collect loop does not have more lines");

        let mut split = project_str.split(" ");
        let name = split.next().unwrap().to_string();

        let mut split = split.map(|n| n.parse::<usize>().unwrap());
        let duration = split.next().unwrap();
        let score = split.next().unwrap();
        let best_before = split.next().unwrap();
        let role_count = split.next().unwrap();

        let roles = (&mut lines)
            .take(role_count)
            .map(|line| {
                let (skill, level) = line
                    .split_once(" ")
                    .expect("Can not split SKILL string");

                let skill = skill.to_string();
                let level = level.parse::<usize>().unwrap();

                Role { skill, level }
            })
            .collect::<Vec<Role>>();

        let project = Project {
            name,
            duration,
            score,
            best_before,
            roles,
        };

        projects.push(project);
    }

    (projects, contributors)
}
