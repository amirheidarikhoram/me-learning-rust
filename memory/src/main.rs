use std::fmt::Display;

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

struct  Mission {
    name: String,
    project: Project,
}

fn main() {
    let mut mission = Mission {
        name: String::from("Mission 1"),
        project: Project::ProjectA,
    };

    let _name = &mut mission.name;
    let _project = &mut mission.project;

    _name.push_str(" - Updated");
    *_project = Project::ProjectB;

    // print mission
    println!("Mission: {}", mission.name);
    println!("Project: {}", mission.project);
}
