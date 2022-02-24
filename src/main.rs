mod util;

use std::collections::HashMap;
use std::error::Error;

type Skillset = HashMap<String, usize>;

#[derive(Debug)]
struct Role {
    skill: String,
    level: usize,
}

#[derive(Debug)]
struct Contributor {
    name: String,
    skills: Skillset,
}

#[derive(Debug)]
struct Project {
    name: String,
    duration: usize,
    score: usize,
    best_before: usize,
    roles: Vec<Role>,
}

fn parse_input(content: String) -> (Vec<Project>, Vec<Contributor>) {
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

fn main() {
    let file_name = std::env::var("FILE")
        .expect("No input file is provided");

    let content = util::read_input_from_file_into_string(file_name)
        .expect("Can not read file input");

    let (projects, contributors) = parse_input(content);

    dbg!(projects);
}
