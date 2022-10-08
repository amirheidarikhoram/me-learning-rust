use std::fmt::Display;

#[derive(Copy, Clone)]
enum Project {
    ProjectA,
    ProjectB,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Project::ProjectA => write!(f, "ProjectA"),
            Project::ProjectB => write!(f, "ProjectB"),
        }
    }
}

#[derive(Copy, Clone)]
struct  Mission<'a> {
    name: &'a str,
    project: Project,
}

fn main() {
    let mission_a = Mission {
        name: "Mission 1",
        project: Project::ProjectA,
    };

    let mission_b = Mission {
        name: "Mission 2",
        project: Project::ProjectB,
    };

    let mut missions = [mission_a, mission_b];

    let m0 = &mut missions[0];

    m0.name = "Mission 3";

    println!("{}: {}", mission_a.project, mission_a.name);
}
